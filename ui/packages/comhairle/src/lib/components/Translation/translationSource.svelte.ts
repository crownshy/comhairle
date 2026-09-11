import { invalidateAll } from '$app/navigation';
import { useDebounce } from 'runed';
import type { Translation, Translation2 } from '@crownshy/api-client/api';
import { getLanguageName } from '$lib/config/languages';
import { tryCatchAsync } from '$lib/utils/errorHandling';
import {
	type TranslationSource,
	type TranslationStatus,
	type TranslationEntry,
	type SaveState,
	getTextInLocale,
	deriveStatus,
	saveTranslation,
	aiTranslate as aiTranslateApi,
	markOtherTranslationsAsDraft
} from './translationUtils';
import { Second } from '$lib/utils/units';
import type { Locale } from '$lib/paraglide/runtime';

/** How long after the last keystroke we wait before persisting, so typing doesn't hit the API per key. */
const SAVE_DEBOUNCE_MS = 1 * Second;

type TextContentSourceOptions = {
	/** Getter (not a value) so the source tracks the live prop across `invalidateAll()`. */
	getTranslation: () => Translation | Translation2 | undefined;
	getPrimaryLocale: () => Locale;
	getSupportedLanguages: () => Locale[];
	/** Plain field value used for the primary locale before any translation row exists (e.g. `step.name`). */
	getPrimaryFallback?: () => string;
	/**
	 * For fields whose `TextContent` is created lazily (e.g. configure's nullable rich fields): called
	 * on the first primary-locale save when no text content id exists yet. It must create the content,
	 * link it to its parent, and refresh so `getTranslation()` returns the new id afterwards.
	 */
	ensureTextContentId?: (content: string) => Promise<void>;
	/**
	 * Fired synchronously on every primary-locale edit. Used by `superForm`-bound consumers to mirror
	 * the value into their `$form` store so inline (`Form.FieldErrors`) validation keeps working; the
	 * source still owns the content (see ADR-0005).
	 */
	onEdit?: (content: string) => void;
	/**
	 * How to re-fetch server truth after a write, so the optimistic overlay reconciles against fresh
	 * data. Defaults to SvelteKit's `invalidateAll` (correct for consumers whose `getTranslation()`
	 * reads route `data`). Tools with a self-managed store (e.g. prioritization) must pass their own
	 * refresh here; otherwise `invalidateAll()` is a no-op for their list and edits reconcile against
	 * stale data, i.e. saved content visibly reverts.
	 */
	refresh?: () => Promise<void>;
};

/**
 * A {@link TranslationSource} backed by a `TextContent` entity (the common case: step name /
 * description, conversation config fields, event fields, prioritization proposals).
 *
 * `contents` is derived from the `translation` prop, with a thin **optimistic overlay** of just-typed
 * edits layered on top so it reflects a keystroke immediately; the overlay entry is cleared once the
 * save + `invalidateAll()` has brought the server truth back (unless the user has typed again since).
 * That overlay is what keeps `RichTextEditor` from resetting the cursor mid-edit (see ADR-0005).
 *
 * Must be called during component initialisation (it uses `$state`/`$derived`), like `runed`'s
 * utilities. Construct it at the consumer and pass the result to `TranslatableField source={...}`.
 */
export function createTextContentSource(options: TextContentSourceOptions): TranslationSource {
	const {
		getTranslation,
		getPrimaryLocale,
		getSupportedLanguages,
		getPrimaryFallback,
		ensureTextContentId,
		onEdit
	} = options;

	const refresh = options.refresh ?? invalidateAll;

	const textContentId = () => getTranslation()?.textContent?.id;
	const otherLanguages = () => getSupportedLanguages().filter((l) => l !== getPrimaryLocale());

	// locale -> content typed but not yet reconciled from the server.
	let overlay = $state<Record<string, string>>({});

	// --- save-state machine (shared across the two debounced channels + the immediate actions) ---
	let saveState = $state<SaveState>('idle');
	let inFlightCount = $state(0);
	let savedResetTimer: ReturnType<typeof setTimeout> | undefined;
	const activeSaves = new Set<Promise<unknown>>();

	// Flip to "saving" the instant an edit is queued (not just when the debounced request fires), so the
	// indicator reflects "unsaved changes" during the debounce window and an unsaved-changes guard can
	// see it. Mirrors what the learn Pages controller does.
	function markSaving() {
		clearTimeout(savedResetTimer);
		saveState = 'saving';
	}

	function runSave(fn: () => Promise<void>): Promise<void> {
		clearTimeout(savedResetTimer);
		saveState = 'saving';
		inFlightCount++;
		const promise = (async () => {
			const result = await tryCatchAsync(fn);
			const ok = result.err === null;
			if (!ok) console.error('Translation save failed:', result.err);
			inFlightCount--;
			// Only the last save to settle drives the terminal state, so overlapping saves don't
			// flip the indicator to "saved" while another is still in flight.
			if (inFlightCount === 0) {
				saveState = ok ? 'saved' : 'error';
				if (ok) {
					savedResetTimer = setTimeout(() => {
						if (saveState === 'saved') saveState = 'idle';
					}, 2_000);
				}
			}
			// Re-throw so callers (e.g. aiTranslate) still see the failure.
			if (!ok) throw result.err;
		})();
		activeSaves.add(promise);
		promise.catch(() => {}).finally(() => activeSaves.delete(promise));
		return promise;
	}

	const contents = $derived.by((): Record<string, string> => {
		const translation = getTranslation();
		const primaryLocale = getPrimaryLocale();
		const server: Record<string, string> = {
			[primaryLocale]: getTextInLocale(
				translation,
				primaryLocale,
				getPrimaryFallback?.() ?? ''
			)
		};
		for (const locale of otherLanguages()) {
			server[locale] = getTextInLocale(translation, locale, '');
		}
		// Optimistic edits win until the server catches up (their overlay entry is then cleared).
		return { ...server, ...overlay };
	});

	const statuses = $derived.by((): Record<string, TranslationStatus> => {
		const translation = getTranslation();
		const primaryLocale = getPrimaryLocale();
		const result: Record<string, TranslationStatus> = { [primaryLocale]: 'primary' };
		for (const locale of otherLanguages()) {
			const row = translation?.textTranslations?.find((t) => t.locale === locale);
			result[locale] = deriveStatus(false, row?.requiresValidation);
		}
		return result;
	});

	/** Clear a locale's optimistic entry once the server has it, unless the user typed something newer. */
	function reconcileOverlay(locale: string, savedContent: string) {
		if (overlay[locale] === savedContent) {
			const next = { ...overlay };
			delete next[locale];
			overlay = next;
		}
	}

	/**
	 * Persist one locale, then reconcile: clear its overlay entry only if the user hasn't typed
	 * something newer in the meantime (otherwise a pending save still owns it).
	 */
	async function persist(
		locale: string,
		content: string,
		opts: { requiresValidation: boolean; markOthersDraft?: boolean; canCreate?: boolean }
	) {
		const id = textContentId();
		if (!id) {
			// First edit of a not-yet-created field: let the consumer create + link the TextContent.
			// It refreshes, so subsequent saves take the normal path above.
			if (opts.canCreate && ensureTextContentId) {
				await ensureTextContentId(content);
				reconcileOverlay(locale, content);
			}
			return;
		}
		await saveTranslation(id, locale, content, { requiresValidation: opts.requiresValidation });
		if (opts.markOthersDraft) {
			const primaryLocale = getPrimaryLocale();
			const approved: TranslationEntry[] = otherLanguages()
				.filter((l) => statuses[l] === 'approved' && contents[l])
				.map((l) => ({
					language: l,
					languageName: getLanguageName(l),
					status: 'approved',
					content: contents[l]
				}));
			if (approved.length > 0)
				await markOtherTranslationsAsDraft(id, primaryLocale, approved);
		}
		await refresh();
		reconcileOverlay(locale, content);
	}

	const debouncedSaveSource = useDebounce((content: string) => {
		const primaryLocale = getPrimaryLocale();
		return runSave(() =>
			persist(primaryLocale, content, {
				requiresValidation: false,
				markOthersDraft: true,
				canCreate: true
			})
		);
	}, SAVE_DEBOUNCE_MS);

	const debouncedSaveTarget = useDebounce((locale: string, content: string) => {
		return runSave(() => persist(locale, content, { requiresValidation: true }));
	}, SAVE_DEBOUNCE_MS);

	return {
		get contents() {
			return contents;
		},
		get statuses() {
			return statuses;
		},
		get saveState() {
			return saveState;
		},

		saveSource(content: string) {
			onEdit?.(content);
			overlay = { ...overlay, [getPrimaryLocale()]: content };
			markSaving();
			debouncedSaveSource(content);
		},

		saveTarget(locale: string, content: string) {
			overlay = { ...overlay, [locale]: content };
			markSaving();
			debouncedSaveTarget(locale, content);
		},

		async aiTranslate(locale: string, sourceContent: string) {
			const id = textContentId();
			if (!id) throw new Error('Cannot AI-translate without a text content id');
			let result: { content: string; requiresValidation: boolean } | undefined;
			await runSave(async () => {
				// aiTranslateApi persists the generated translation against this text content id.
				result = await aiTranslateApi(id, locale, sourceContent, getPrimaryLocale());
				overlay = { ...overlay, [locale]: result.content };
				await refresh();
				reconcileOverlay(locale, result.content);
			});
			return result!;
		},

		approve(locale: string) {
			return runSave(async () => {
				await persist(locale, contents[locale] ?? '', { requiresValidation: false });
			});
		},

		markAsDraft(locale: string) {
			return runSave(async () => {
				await persist(locale, contents[locale] ?? '', { requiresValidation: true });
			});
		},

		async flush() {
			await debouncedSaveSource.runScheduledNow();
			await debouncedSaveTarget.runScheduledNow();
			await Promise.allSettled([...activeSaves]);
		}
	};
}

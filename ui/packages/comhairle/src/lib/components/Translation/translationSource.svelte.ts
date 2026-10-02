import { invalidateAll } from '$app/navigation';
import { useDebounce } from 'runed';
import type { Translation, Translation2 } from '@crownshy/api-client/api';
import { getLanguageName } from '$lib/config/languages';
import { tryCatchAsync } from '$lib/utils/errorHandling';
import { notifications } from '$lib/notifications.svelte';
import {
	type AutosaveSource,
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
	/** Getter (not a value) so the source tracks the live prop across a reload. */
	getTranslation: () => Translation | Translation2 | undefined;
	getPrimaryLocale: () => Locale;
	getSupportedLanguages: () => Locale[];
	/** Plain field value used for the primary locale before any translation row exists (e.g. `step.name`). */
	getPrimaryFallback?: () => string;
	/**
	 * Called on the first save when the field has no `TextContent` yet (e.g. optional configure
	 * fields). Must create and link it, and return the new id.
	 */
	ensureTextContentId?: (content: string) => Promise<string | undefined>;
	/**
	 * Fired synchronously on every primary-locale edit. Used by `superForm`-bound consumers to mirror
	 * the value into their `$form` store so inline (`Form.FieldErrors`) validation keeps working; the
	 * source still owns the content (see ADR-0005).
	 */
	onEdit?: (content: string) => void;
	/**
	 * Re-fetches the data `getTranslation()` reads. Saves don't call it, since the overlay already
	 * shows what was typed. `sync()` does, once, when the user leaves the page or closes a dialog.
	 * Defaults to `invalidateAll`; a store-backed list (e.g. prioritization) passes its own reload.
	 */
	refresh?: () => Promise<void>;
};

/**
 * A {@link TranslationSource} backed by a `TextContent` entity. Edits live in an overlay on top of
 * the loaded data until `sync()` reloads it, so saving never reloads the page under the editor
 * (ADR-0005). Call it during component init, since it uses runes.
 */
export function createTextContentSource(options: TextContentSourceOptions): AutosaveSource {
	const {
		getTranslation,
		getPrimaryLocale,
		getSupportedLanguages,
		getPrimaryFallback,
		ensureTextContentId,
		onEdit
	} = options;

	const refresh = options.refresh ?? invalidateAll;

	let createdTextContentId = $state<string | undefined>();
	const textContentId = () => getTranslation()?.textContent?.id ?? createdTextContentId;
	const otherLanguages = () => getSupportedLanguages().filter((l) => l !== getPrimaryLocale());

	// Saved (or saving) values the loaded data doesn't have yet, per locale.
	let overlay = $state<Record<string, string>>({});
	let statusOverlay = $state<Record<string, TranslationStatus>>({});
	// True once something saved since the data was loaded, so `sync()` knows to reload.
	let stale = $state(false);
	// Per locale, how to redo the save that last failed there.
	let failedSaves = $state<Record<string, () => Promise<void>>>({});

	let saveState = $state<SaveState>('idle');
	let inFlightCount = $state(0);
	let savedResetTimer: ReturnType<typeof setTimeout> | undefined;
	// Notify once per run of failures, not on every retry.
	let failureNotified = false;
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
				if (!ok && !failureNotified) {
					notifications.send({
						message: "Your changes couldn't be saved. Check your connection and retry.",
						priority: 'ERROR'
					});
				}
				failureNotified = !ok;
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
		return { ...result, ...statusOverlay };
	});

	type PersistOptions = {
		requiresValidation: boolean;
		markOthersDraft?: boolean;
		canCreate?: boolean;
	};

	function withoutKey<T>(record: Record<string, T>, key: string): Record<string, T> {
		const next = { ...record };
		delete next[key];
		return next;
	}

	/** Saves one locale's latest content, remembering how to retry it if the save fails. */
	function saveLocale(locale: string, opts: PersistOptions): Promise<void> {
		const attempt = () =>
			runSave(async () => {
				const result = await tryCatchAsync(() =>
					persist(locale, contents[locale] ?? '', opts)
				);
				if (result.err !== null) {
					failedSaves = { ...failedSaves, [locale]: attempt };
					throw result.err;
				}
				failedSaves = withoutKey(failedSaves, locale);
			});
		return attempt();
	}

	async function persist(locale: string, content: string, opts: PersistOptions) {
		const id = textContentId();
		if (!id) {
			if (opts.canCreate && ensureTextContentId) {
				createdTextContentId = await ensureTextContentId(content);
				if (!createdTextContentId) throw new Error('Could not create the text content');
				stale = true;
			}
			return;
		}
		await saveTranslation(id, locale, content, { requiresValidation: opts.requiresValidation });
		if (locale !== getPrimaryLocale()) {
			statusOverlay = {
				...statusOverlay,
				[locale]: opts.requiresValidation ? 'draft' : 'approved'
			};
		}
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
			if (approved.length > 0) {
				await markOtherTranslationsAsDraft(id, primaryLocale, approved);
				for (const entry of approved) {
					statusOverlay = { ...statusOverlay, [entry.language]: 'draft' };
				}
			}
		}
		stale = true;
	}

	// Nothing awaits a typed save, and a failure is already shown on the field, so don't let it
	// surface as an unhandled rejection.
	const debouncedSaveSource = useDebounce(
		() =>
			saveLocale(getPrimaryLocale(), {
				requiresValidation: false,
				markOthersDraft: true,
				canCreate: true
			}).catch(() => {}),
		SAVE_DEBOUNCE_MS
	);

	const debouncedSaveTarget = useDebounce(
		(locale: string) => saveLocale(locale, { requiresValidation: true }).catch(() => {}),
		SAVE_DEBOUNCE_MS
	);

	async function flush() {
		await debouncedSaveSource.runScheduledNow();
		await debouncedSaveTarget.runScheduledNow();
		await Promise.allSettled(activeSaves);
	}

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
		get stale() {
			return stale;
		},

		saveSource(content: string) {
			onEdit?.(content);
			overlay = { ...overlay, [getPrimaryLocale()]: content };
			markSaving();
			debouncedSaveSource();
		},

		saveTarget(locale: string, content: string) {
			overlay = { ...overlay, [locale]: content };
			markSaving();
			debouncedSaveTarget(locale);
		},

		async aiTranslate(locale: string, sourceContent: string) {
			const id = textContentId();
			if (!id) throw new Error('Cannot AI-translate without a text content id');
			let result: { content: string; requiresValidation: boolean } | undefined;
			await runSave(async () => {
				// aiTranslateApi persists the generated translation against this text content id.
				result = await aiTranslateApi(id, locale, sourceContent, getPrimaryLocale());
				overlay = { ...overlay, [locale]: result.content };
				statusOverlay = {
					...statusOverlay,
					[locale]: result.requiresValidation ? 'draft' : 'approved'
				};
				stale = true;
			});
			return result!;
		},

		approve(locale: string) {
			return saveLocale(locale, { requiresValidation: false });
		},

		markAsDraft(locale: string) {
			return saveLocale(locale, { requiresValidation: true });
		},

		flush,

		async retry() {
			await Promise.allSettled(Object.values(failedSaves).map((attempt) => attempt()));
		},

		async sync() {
			await flush();
			if (!stale || Object.keys(failedSaves).length > 0) return;
			const savedOverlay = overlay;
			const savedStatusOverlay = statusOverlay;
			stale = false;
			const result = await tryCatchAsync(refresh);
			if (result.err !== null) {
				stale = true;
				return;
			}
			overlay = keepChangedSince(overlay, savedOverlay);
			statusOverlay = keepChangedSince(statusOverlay, savedStatusOverlay);
		}
	};
}

/** The entries of `current` that were added or changed after `snapshot` was taken. */
function keepChangedSince<T>(current: Record<string, T>, snapshot: Record<string, T>) {
	return Object.fromEntries(
		Object.entries(current).filter(([key, value]) => snapshot[key] !== value)
	);
}

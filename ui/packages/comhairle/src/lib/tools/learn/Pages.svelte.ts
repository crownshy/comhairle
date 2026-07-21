import type { LocalizedPage } from '@crownshy/api-client/api';
import { SHADOW_ITEM_MARKER_PROPERTY_NAME } from 'svelte-dnd-action';
import { useDebounce } from 'runed';

type Id = string;
export type Language = string;
type IPages = Record<Id, Record<Language, ExtendedLocalizedPage>>;

export interface ExtendedLocalizedPage extends LocalizedPage {
	lang: Language;
	requires_validation: boolean;
}

type From = 'source' | 'target';
type RawSave = (options?: { invalidate?: boolean }) => Promise<void>;
type OnRestore = () => void;
type Order = { id: string; [SHADOW_ITEM_MARKER_PROPERTY_NAME]?: boolean }[]; // Matching DraggableList "items" props

/** Lifecycle of the background save, so the UI can show a truthful indicator instead of guessing. */
export type SaveState = 'idle' | 'saving' | 'saved' | 'error';

/**
 * How long after the last content edit we wait before persisting. Collapses a burst of keystrokes
 * into a single request instead of one save (and one reload) per character.
 */
const SAVE_DEBOUNCE_MS = 800;

/**
 * Owns the learn tool's page state and its persistence. The one non-obvious job here is making saves
 * **observable and flushable**: `saveState` reflects the real request lifecycle, and `flush()` commits
 * any pending edit and awaits the in-flight save. That's what lets the UI switch pages the instant a
 * save actually lands, rather than blocking on a fixed timer and hoping.
 */
class Pages {
	items = $state<IPages>({});
	currentId = $state<number>(0);
	order = $state<Order>([]);
	areDirty = $state<boolean>(false);
	saveState = $state<SaveState>('idle');

	#rawSave: RawSave = () => Promise.resolve();
	#onRestore: OnRestore = () => {};
	#inFlight: Promise<void> | null = null;
	#savedResetTimer: ReturnType<typeof setTimeout> | undefined;

	// Content edits route through here; a single shared timer means rapid typing collapses to one save.
	#debouncedSave = useDebounce(
		(invalidate: boolean) => this.#runSave(invalidate),
		SAVE_DEBOUNCE_MS
	);

	get count() {
		return Object.keys(this.items).length;
	}

	saveHandler(fn: RawSave) {
		this.#rawSave = fn;
	}

	onRestore(fn: OnRestore) {
		this.#onRestore = fn;
	}

	async #runSave(invalidate: boolean) {
		this.saveState = 'saving';
		clearTimeout(this.#savedResetTimer);
		this.#inFlight = (async () => {
			try {
				await this.#rawSave({ invalidate });
				this.saveState = 'saved';
				this.#savedResetTimer = setTimeout(() => {
					if (this.saveState === 'saved') this.saveState = 'idle';
				}, 2_000);
			} catch {
				this.saveState = 'error';
			}
		})();
		try {
			await this.#inFlight;
		} finally {
			this.#inFlight = null;
		}
	}

	/** Queue a debounced save. For content edits, where per-keystroke persistence would be wasteful. */
	#scheduleSave(invalidate = false) {
		this.areDirty = true;
		// Show "Saving" from the first keystroke through the debounce window, not just for the brief
		// moment the request is in flight, so the indicator reflects "you have unsaved changes".
		clearTimeout(this.#savedResetTimer);
		this.saveState = 'saving';
		// A later #saveNow() cancels this debounce, which rejects the pending promise with "Cancelled".
		// That's expected (the edit is already in the in-memory model, so #saveNow persists it), so
		// swallow it here rather than leak an unhandled rejection.
		this.#debouncedSave(invalidate).catch(() => {});
	}

	/**
	 * Persist immediately, cancelling any pending debounced save. For structural edits (add / delete /
	 * reorder) where the change is discrete and the user expects it to stick right away. The in-memory
	 * model already holds any un-flushed content edit, so cancelling the timer drops the timer, not data.
	 */
	async #saveNow(invalidate = true) {
		this.areDirty = true;
		this.#debouncedSave.cancel();
		if (this.#inFlight) await this.#inFlight;
		await this.#runSave(invalidate);
	}

	/**
	 * Run any pending debounced save now and wait for the in-flight save to finish. Call before leaving
	 * the current page so a late-firing save can never land an edit against the wrong page.
	 */
	async flush() {
		await this.#debouncedSave.runScheduledNow();
		if (this.#inFlight) await this.#inFlight;
	}

	/** Move to another page, committing pending edits first. Replaces the old fixed-timer switch lockout. */
	async switchTo(id: number) {
		await this.flush();
		this.currentId = id;
	}

	new(primaryLocale: Language) {
		const keys = Object.keys(this.items);
		const latestId = Number(keys[keys.length - 1]);
		const newId = (latestId + 1).toString();
		const newPage: ExtendedLocalizedPage = {
			lang: primaryLocale,
			content: '# New Page',
			type: 'markdown',
			requires_validation: false
		};
		this.items[newId] = { [primaryLocale]: newPage };
		this.order.push({ id: newId });
		this.currentId = Number(newId);
		return this.#saveNow();
	}

	load(source: ExtendedLocalizedPage[][]) {
		const order: Order = [];
		source.forEach((page, i) => {
			const extendedLocalizedPage: Record<Language, ExtendedLocalizedPage> = {};
			page.forEach((p) => {
				extendedLocalizedPage[p.lang] = p;
			});
			this.items[i] = extendedLocalizedPage;
			order.push({ id: i.toString() });
		});
		this.order = order;
	}

	toLocalizedPages(): ExtendedLocalizedPage[][] {
		return this.order.map((p) => Object.values(this.items[p.id]));
	}

	reorder(order: Order) {
		this.order = order;

		// If it's currently being dragged then don't send it to the server until it's finished dragging
		if (order.some((o) => o[SHADOW_ITEM_MARKER_PROPERTY_NAME])) {
			return;
		}
		return this.#saveNow();
	}

	dirty() {
		this.areDirty = true;
	}

	restore() {
		this.areDirty = false;
		this.#onRestore();
	}

	get current() {
		return {
			delete: async () => {
				delete this.items[this.currentId];
				const index = this.order.findIndex((p) => Number(p.id) === this.currentId);
				this.order.splice(index, 1);
				const newIndex = Math.max(index - 1, 0);
				this.currentId = Number(this.order[newIndex].id);
				return this.#saveNow();
			},

			upsertContent: (
				from: From,
				lang: Language,
				content: ExtendedLocalizedPage['content'] | undefined
			) => {
				const requires_validation = from === 'target';
				const page = this.items[this.currentId];
				if (!page) return;
				page[lang] = {
					lang,
					type: 'markdown',
					content: content ?? page[lang]?.content ?? '',
					requires_validation
				};
				switch (from) {
					case 'source':
						for (const translation in page) {
							if (page[translation].lang !== lang) {
								page[translation].requires_validation = true;
							}
						}
						break;
					case 'target':
						break;
				}
				this.items[this.currentId] = page;
				this.#scheduleSave(false);
			},

			approve: async (lang: Language, validation: boolean) => {
				const page = this.items[this.currentId];
				if (!page || !page[lang]) return;
				page[lang].requires_validation = !validation;
				this.items[this.currentId] = page;
				return this.#saveNow(false);
			}
		};
	}
}

export default Pages;

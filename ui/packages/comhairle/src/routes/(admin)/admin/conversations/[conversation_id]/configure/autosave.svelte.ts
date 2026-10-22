import { tryCatchAsync } from '$lib/utils/errorHandling';

export type SaveStatus = 'idle' | 'saving' | 'saved' | 'error';

/** How long an edit waits for more typing before its save starts. */
const DEBOUNCE_MS = 700;
/** A fast local save still shows "Saving…" this long, so the spinner registers. */
const MINIMUM_SAVING_MS = 500;

/**
 * Debounced autosave for a Configure editor that writes its whole state in one request
 * (ADR-0006). Saves run one after another: each request replaces the stored value, so two in
 * flight could land out of order and the older value would win.
 *
 * `save` sends the editor's current state and rejects on failure. Component wiring (the
 * leave-page guard, flushing on destroy) stays with the editor.
 */
export class Autosave {
	status = $state<SaveStatus>('idle');
	/** True from an edit until a save that includes it lands. Drives the leave-page guard. */
	dirty = $state(false);
	/** True once any save has landed. */
	hasSaved = false;

	#save: () => Promise<unknown>;
	#timer: ReturnType<typeof setTimeout> | undefined;
	// Bumped on every edit, so a finished save can tell whether newer edits are still waiting.
	#editVersion = 0;
	#queue: Promise<void> = Promise.resolve();

	constructor(save: () => Promise<unknown>) {
		this.#save = save;
	}

	/** Call after every edit. */
	schedule() {
		this.#editVersion++;
		this.dirty = true;
		clearTimeout(this.#timer);
		this.#timer = setTimeout(() => this.#enqueue(), DEBOUNCE_MS);
	}

	/** Starts a scheduled save now instead of waiting, e.g. when the editor closes. */
	flush() {
		if (this.#timer === undefined) return;
		clearTimeout(this.#timer);
		this.#enqueue();
	}

	/** Resolves once every save started so far has finished. */
	settled(): Promise<void> {
		return this.#queue;
	}

	#enqueue() {
		this.#timer = undefined;
		this.#queue = this.#queue.then(() => this.#run());
	}

	async #run() {
		const savingVersion = this.#editVersion;
		this.status = 'saving';
		const startedAt = performance.now();
		const result = await tryCatchAsync(this.#save);
		if (result.err !== null) {
			this.status = 'error';
			return; // stay dirty so the leave-page guard still warns
		}
		this.hasSaved = true;
		const elapsed = performance.now() - startedAt;
		if (elapsed < MINIMUM_SAVING_MS) {
			await new Promise((resolve) => setTimeout(resolve, MINIMUM_SAVING_MS - elapsed));
		}
		// An edit made while this save was out has its own save scheduled or queued.
		if (this.#editVersion !== savingVersion) return;
		this.status = 'saved';
		this.dirty = false;
	}
}

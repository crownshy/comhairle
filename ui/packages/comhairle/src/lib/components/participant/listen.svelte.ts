/**
 * Listen reads a participant page aloud with the browser's speech synthesiser (ADR-0031).
 *
 * Shared as a module because its two controls live in different components: the Learn
 * body attaches its article and shows the offer, and the step page's pager carries the
 * transport once something is playing. The synthesiser is one global per window anyway,
 * so there is nothing to gain from more than one of these.
 */
import { getLocale } from '$lib/paraglide/runtime.js';
import { prefersReducedMotion } from '$lib/utils/reducedMotion';
import { stepScroller } from '$lib/utils/stepScroll';

export type ListenStatus = 'idle' | 'playing' | 'paused';

/** The speeds the transport cycles through, in tap order. */
export const LISTEN_RATES = [1, 1.25, 1.5, 0.75] as const;
export type ListenRate = (typeof LISTEN_RATES)[number];

/** A middling read-aloud pace at 1x. The estimate is a promise of scale, not a clock. */
const WORDS_PER_MINUTE = 170;

/**
 * What counts as a block: the elements a rendered article says its sentences in. Nested
 * matches (a paragraph inside a list item) are collapsed to the innermost so nothing is
 * read twice.
 */
const BLOCK_SELECTOR = 'p, h1, h2, h3, h4, h5, h6, li, blockquote, figcaption, td, th';

/** Set on the block being read so the article can mark it. */
export const LISTEN_CURRENT_ATTRIBUTE = 'data-listen-current';

/**
 * Paraglide locale to the language prefix the voices report. Most match as they are; Dari
 * is the one voices file under Persian.
 */
const VOICE_LANGUAGE: Record<string, string> = { prs: 'fa' };

type Block = { element: HTMLElement; text: string };

let status = $state<ListenStatus>('idle');
let rate = $state<ListenRate>(1);
let index = $state(0);
let blocks = $state.raw<Block[]>([]);
let words = $state(0);
let voice = $state.raw<SpeechSynthesisVoice | null>(null);
let voicesKnown = $state(false);

let article: HTMLElement | null = null;
/**
 * Bumped whenever playback restarts or stops. An utterance's callbacks compare against it
 * so a cancelled one cannot advance the block that replaced it: cancel() reports the old
 * utterance as ended or errored asynchronously, after the new one has already begun.
 */
let generation = 0;

function synth(): SpeechSynthesis | null {
	if (typeof window === 'undefined' || !('speechSynthesis' in window)) return null;
	return window.speechSynthesis;
}

function languagePrefix(): string {
	const locale = getLocale();
	return (VOICE_LANGUAGE[locale] ?? locale).toLowerCase();
}

/**
 * The voice for the participant's locale, or null when the device has none. British
 * English is preferred where the locale is English, since that is who the platform reads
 * to; otherwise the first voice in that language, with the OS default winning a tie.
 */
function pickVoice(voices: SpeechSynthesisVoice[]): SpeechSynthesisVoice | null {
	const prefix = languagePrefix();
	const candidates = voices.filter((v) =>
		v.lang.toLowerCase().replace('_', '-').startsWith(prefix)
	);
	if (candidates.length === 0) return null;
	if (prefix === 'en') {
		const british = candidates.find((v) => v.lang.toLowerCase().replace('_', '-') === 'en-gb');
		if (british) return british;
	}
	return candidates.find((v) => v.default) ?? candidates[0];
}

function loadVoices() {
	const s = synth();
	if (!s) {
		voicesKnown = true;
		return;
	}
	const apply = () => {
		const voices = s.getVoices();
		if (voices.length === 0) return false;
		voice = pickVoice(voices);
		voicesKnown = true;
		return true;
	};
	// Chrome and Safari populate the list asynchronously on first ask.
	if (!apply()) {
		s.addEventListener('voiceschanged', () => void apply(), { once: true });
	}
}

function collectBlocks(root: HTMLElement): Block[] {
	const elements = Array.from(root.querySelectorAll<HTMLElement>(BLOCK_SELECTOR));
	return elements
		.filter((element) => !element.querySelector(BLOCK_SELECTOR))
		.map((element) => ({ element, text: element.innerText.replace(/\s+/g, ' ').trim() }))
		.filter((block) => block.text.length > 0);
}

function countWords(list: Block[]): number {
	return list.reduce((sum, block) => sum + block.text.split(' ').length, 0);
}

function markCurrent(next: Block | undefined) {
	for (const block of blocks) {
		block.element.removeAttribute(LISTEN_CURRENT_ATTRIBUTE);
	}
	if (!next) return;
	next.element.setAttribute(LISTEN_CURRENT_ATTRIBUTE, '');
	scrollIntoStep(next.element);
}

/**
 * Bring the block being read into the middle of the scroll if it is not already on
 * screen. A block that is visible is left alone, so a participant reading ahead is not
 * yanked back on every sentence.
 */
function scrollIntoStep(element: HTMLElement) {
	const scroller = stepScroller();
	const rect = element.getBoundingClientRect();
	const top = scroller instanceof Window ? 0 : scroller.getBoundingClientRect().top;
	const bottom =
		scroller instanceof Window ? window.innerHeight : scroller.getBoundingClientRect().bottom;
	if (rect.top >= top && rect.bottom <= bottom) return;
	element.scrollIntoView({
		block: 'center',
		behavior: prefersReducedMotion() ? 'auto' : 'smooth'
	});
}

function speakFrom(start: number) {
	const s = synth();
	const block = blocks[start];
	if (!s || !block) {
		finish();
		return;
	}
	const mine = ++generation;
	s.cancel();
	index = start;
	status = 'playing';
	markCurrent(block);

	const utterance = new SpeechSynthesisUtterance(block.text);
	if (voice) utterance.voice = voice;
	utterance.lang = voice?.lang ?? languagePrefix();
	utterance.rate = rate;
	utterance.onend = () => {
		if (mine !== generation) return;
		speakFrom(start + 1);
	};
	utterance.onerror = (event) => {
		if (mine !== generation) return;
		// Our own cancel() reports as one of these on the utterance it cut short.
		if (event.error === 'interrupted' || event.error === 'canceled') return;
		finish();
	};
	s.speak(utterance);
}

function finish() {
	generation += 1;
	synth()?.cancel();
	markCurrent(undefined);
	status = 'idle';
	index = 0;
}

export const listen = {
	get status() {
		return status;
	},
	get rate() {
		return rate;
	},
	/** Zero-based block being read. */
	get index() {
		return index;
	},
	get blockCount() {
		return blocks.length;
	},
	/** Fill of the page read so far, 0 to 1, counted in blocks (ADR-0031). */
	get progress() {
		if (blocks.length === 0) return 0;
		if (status === 'idle') return 0;
		return index / blocks.length;
	},
	/**
	 * Whole minutes the attached page would take at 1x, rounded up. Zero means there is
	 * nothing to read, which is how a surface with no prose is told from a short one.
	 */
	get minutes() {
		if (words === 0) return 0;
		return Math.max(1, Math.ceil(words / WORDS_PER_MINUTE));
	},
	/**
	 * Whether the offer can be made: this browser can speak, has a voice in the
	 * participant's language, and the attached page has something to read.
	 */
	get available() {
		return voicesKnown && voice !== null && words > 0;
	},

	/**
	 * Point Listen at a rendered article. Replaces whatever was attached before, stopping it
	 * if it was playing. The estimate is costed here, off the rendered text.
	 */
	attach(root: HTMLElement) {
		if (status !== 'idle') finish();
		article = root;
		blocks = collectBlocks(root);
		words = countWords(blocks);
		if (!voicesKnown) loadVoices();
	},

	/** Let go of an article, if it is still the attached one. Stops playback. */
	detach(root: HTMLElement) {
		if (article !== root) return;
		if (status !== 'idle') finish();
		article = null;
		blocks = [];
		words = 0;
	},

	play() {
		if (!listen.available) return;
		speakFrom(status === 'paused' ? index : 0);
	},

	/**
	 * Pause is a cancel that remembers the block. The synthesiser's own pause() is
	 * unreliable on Android and resumes from nowhere after a few seconds on some
	 * builds; restarting the block is the version that works everywhere.
	 */
	pause() {
		if (status !== 'playing') return;
		generation += 1;
		synth()?.cancel();
		status = 'paused';
	},

	toggle() {
		if (status === 'playing') listen.pause();
		else listen.play();
	},

	stop() {
		if (status === 'idle') return;
		finish();
	},

	/** Next speed in the cycle. A playing utterance restarts its block at the new rate. */
	cycleRate() {
		const at = LISTEN_RATES.indexOf(rate);
		rate = LISTEN_RATES[(at + 1) % LISTEN_RATES.length];
		if (status === 'playing') speakFrom(index);
	}
};

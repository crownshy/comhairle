/**
 * The Room display driver: a clock over a scripted scenario.
 *
 * Autoplays so the display can loop unattended on a booth screen, and exposes pause,
 * scrub, rate and jump-to-stage so a facilitator can stop mid-pitch and talk over a
 * frozen frame. Everything it renders is `stateAt(scenario, playheadMs)`, which is
 * pure, so scrubbing backwards is the same operation as playing forwards.
 *
 * A factory rather than a class: Svelte evaluates class-field `$derived` initialisers
 * before the constructor body runs, so a class would read its own scenario field
 * before anything had assigned it.
 *
 * This is the prototype's substitute for a live Polis. When real endpoints arrive the
 * components do not change: something else fills `state`, `stage` and `moment`.
 */

import { stateAt, stageAt, stageTimeline } from './scenario';
import { STAGE_ORDER } from './revealStage';
import {
	advance,
	initialMomentState,
	DEFAULT_MOMENT_CONFIG,
	type Moment,
	type MomentConfig,
	type MomentState
} from './moments';
import type { DisplayMode, DisplayState, RevealStage, Scenario, StagedDisplayState } from './types';

/** How long a moment holds the screen before the display returns to what it was showing. */
const MOMENT_DURATION_MS = 4500;

/** How often the moments reducer is asked whether anything should fire. */
const OBSERVATION_INTERVAL_MS = 2000;

/**
 * Whether a frame loop can run at all. False during SSR, where the page renders the
 * scenario's opening frame and the client picks up the clock on hydration. Feature
 * detection rather than `$app/environment` keeps this module runnable under plain
 * node, which is how its callers are unit-tested.
 */
const canAnimate = typeof requestAnimationFrame === 'function';

/**
 * Largest scenario time a single frame may advance, before the playback rate.
 *
 * Browsers stop firing `requestAnimationFrame` while a page is hidden, so the first
 * frame after the facilitator switches back carries a gap of however long they were
 * away. Uncapped, that lurches the display minutes forward in one frame, skipping
 * every moment in between. Clamping makes the display resume where it paused rather
 * than teleport.
 */
const MAX_FRAME_MS = 250;

export interface DriverOptions {
	scenario: Scenario;
	/** Playback speed. A 4.5 hour run is unwatchable at 1x, so demos run compressed. */
	rate?: number;
	autoplay?: boolean;
	mode?: DisplayMode;
	momentConfig?: MomentConfig;
}

export interface RoomDisplayDriver {
	readonly scenario: Scenario;
	readonly playheadMs: number;
	readonly playing: boolean;
	readonly progress: number;
	readonly state: DisplayState;
	readonly stage: RevealStage;
	readonly staged: StagedDisplayState;
	readonly moment: Moment | null;
	rate: number;
	mode: DisplayMode;
	play(): void;
	pause(): void;
	toggle(): void;
	seek(atMs: number): void;
	seekToStage(stage: RevealStage): void;
	forceMoment(moment: Moment): void;
	takeControl(): void;
	releaseControl(): void;
	destroy(): void;
}

export function createRoomDisplayDriver(options: DriverOptions): RoomDisplayDriver {
	const scenario = options.scenario;
	const timeline = stageTimeline(scenario);
	const momentConfig = options.momentConfig ?? DEFAULT_MOMENT_CONFIG;

	let playheadMs = $state(0);
	let playing = $state(false);
	let rate = $state(options.rate ?? 1);
	let mode = $state<DisplayMode>(options.mode ?? 'ambient');
	let moment = $state<Moment | null>(null);

	const state = $derived(stateAt(scenario, playheadMs));
	const stage = $derived(stageAt(timeline, playheadMs));
	const staged = $derived<StagedDisplayState>({ ...state, stage });
	const progress = $derived(scenario.durationMs > 0 ? playheadMs / scenario.durationMs : 0);

	let momentState: MomentState = initialMomentState();
	let lastObservedAtMs = -Infinity;
	let lastNodeCount = 0;
	let momentShownAtMs: number | null = null;
	let frame: number | null = null;
	let lastTickMs: number | null = null;

	/** Feeds the moments reducer at a fixed cadence and expires whatever is on screen. */
	function observe() {
		if (momentShownAtMs !== null && playheadMs - momentShownAtMs > MOMENT_DURATION_MS) {
			moment = null;
			momentShownAtMs = null;
		}

		if (playheadMs - lastObservedAtMs < OBSERVATION_INTERVAL_MS) return;
		lastObservedAtMs = playheadMs;

		const now = stateAt(scenario, playheadMs);
		const joined = Math.max(0, now.nodes.length - lastNodeCount);
		lastNodeCount = now.nodes.length;

		// Group count is what the display would read off `report_data`. Below `shaped`
		// Polis has not clustered, so it reports none.
		const currentStage = stageAt(timeline, playheadMs);
		const groupCount =
			currentStage === 'shaped' || currentStage === 'rich' ? scenario.groups.length : 0;

		const step = advance(
			momentState,
			{ atMs: playheadMs, joined, stage: currentStage, groupCount },
			momentConfig
		);
		momentState = step.state;
		if (step.moment) {
			moment = step.moment;
			momentShownAtMs = playheadMs;
		}
	}

	function seek(atMs: number) {
		playheadMs = Math.max(0, Math.min(scenario.durationMs, atMs));
		// The moments reducer records what the room has been shown, and that record is
		// meaningless once the playhead jumps somewhere else in the timeline.
		momentState = initialMomentState();
		lastObservedAtMs = -Infinity;
		lastNodeCount = stateAt(scenario, playheadMs).nodes.length;
		moment = null;
		momentShownAtMs = null;
	}

	function tick(now: number) {
		if (!playing) return;

		if (lastTickMs !== null) {
			const frameMs = Math.min(now - lastTickMs, MAX_FRAME_MS);
			const next = playheadMs + frameMs * rate;
			// Looping keeps an unattended booth screen alive. `seek` resets the moments
			// reducer, so the second pass replays the beats rather than treating every
			// stage as already announced.
			if (next >= scenario.durationMs) seek(0);
			else playheadMs = next;
		}
		lastTickMs = now;

		observe();
		if (canAnimate) frame = requestAnimationFrame(tick);
	}

	function play() {
		if (playing) return;
		playing = true;
		lastTickMs = null;
		if (canAnimate) frame = requestAnimationFrame(tick);
	}

	function pause() {
		playing = false;
		if (frame !== null && canAnimate) cancelAnimationFrame(frame);
		frame = null;
	}

	if (options.autoplay) play();

	return {
		scenario,
		get playheadMs() {
			return playheadMs;
		},
		get playing() {
			return playing;
		},
		get progress() {
			return progress;
		},
		get state() {
			return state;
		},
		get stage() {
			return stage;
		},
		get staged() {
			return staged;
		},
		get moment() {
			return moment;
		},
		get rate() {
			return rate;
		},
		set rate(value: number) {
			rate = value;
		},
		get mode() {
			return mode;
		},
		set mode(value: DisplayMode) {
			mode = value;
		},
		play,
		pause,
		toggle: () => (playing ? pause() : play()),
		seek,
		/**
		 * Jumps to the first instant the display is at least the given stage.
		 *
		 * Matched by rank, not equality: a run can skip a stage outright (a heavily
		 * seeded conversation is `rich` the moment it clusters, never passing through
		 * `shaped`), and an exact match would find nothing and silently rewind to the
		 * start.
		 */
		seekToStage(target: RevealStage) {
			const wanted = STAGE_ORDER.indexOf(target);
			const sample = timeline.find((s) => STAGE_ORDER.indexOf(s.stage) >= wanted);
			seek(sample ? sample.atMs : scenario.durationMs);
		},
		/** Puts a moment on screen on cue, for rehearsal and for showing one beat. */
		forceMoment(next: Moment) {
			moment = next;
			momentShownAtMs = playheadMs;
		},
		/** The facilitator touched something, so hover and click go live. */
		takeControl: () => {
			mode = 'driven';
		},
		releaseControl: () => {
			mode = 'ambient';
		},
		destroy: pause
	};
}

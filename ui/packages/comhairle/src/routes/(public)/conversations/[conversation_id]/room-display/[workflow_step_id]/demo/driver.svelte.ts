/**
 * Plays a scripted scenario as a Room display source, for the demo mode.
 * A factory rather than a class, because Svelte runs class-field `$derived`
 * initialisers before the constructor has assigned the scenario.
 */

import { stateAt, stageAt, stageTimeline } from './scenario';
import { STAGE_ORDER } from '../revealStage';
import {
	advance,
	initialMomentState,
	DEFAULT_MOMENT_CONFIG,
	type Moment,
	type MomentConfig,
	type MomentState
} from './moments';
import type { RoomDisplaySource } from '../source';
import type {
	DisplayMode,
	DisplayState,
	RevealStage,
	Scenario,
	StagedDisplayState
} from '../types';

/** How long a moment holds the screen before the display returns to what it was showing. */
const MOMENT_DURATION_MS = 4500;

/** How often the moments reducer is asked whether anything should fire. */
const OBSERVATION_INTERVAL_MS = 2000;

// False during SSR. Feature detection rather than `$app/environment` keeps this
// module runnable in plain Node for unit tests.
const canAnimate = typeof requestAnimationFrame === 'function';

// Browsers pause `requestAnimationFrame` in hidden tabs, so the first frame back can
// carry a gap of minutes. Capping it resumes playback where it stopped instead of jumping.
const MAX_FRAME_MS = 250;

export interface DriverOptions {
	scenario: Scenario;
	/** Playback speed multiplier. */
	rate?: number;
	autoplay?: boolean;
	mode?: DisplayMode;
	momentConfig?: MomentConfig;
}

export interface RoomDisplayDriver extends RoomDisplaySource {
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

		// Polis reports no groups before the `shaped` stage.
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
		// What the room has already been shown no longer applies after a jump.
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
			// Loop so an unattended screen keeps running. `seek` resets the moments, so
			// the next pass announces each stage again.
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
		groups: scenario.groups,
		voteMatrix: 'per-participant',
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
		 * Jumps to the first point the display reaches at least `target`. Matched by
		 * rank, not equality, because a run can skip a stage entirely.
		 */
		seekToStage(target: RevealStage) {
			const wanted = STAGE_ORDER.indexOf(target);
			const sample = timeline.find((s) => STAGE_ORDER.indexOf(s.stage) >= wanted);
			seek(sample ? sample.atMs : scenario.durationMs);
		},
		/** Shows a moment immediately, for rehearsal. */
		forceMoment(next: Moment) {
			moment = next;
			momentShownAtMs = playheadMs;
		},
		/** Switches to `driven` mode, where hover and click respond. */
		takeControl: () => {
			mode = 'driven';
		},
		releaseControl: () => {
			mode = 'ambient';
		},
		destroy: pause
	};
}

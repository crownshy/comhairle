<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import * as Dialog from '$lib/components/ui/dialog';
	import VideoRecorder from './VideoRecorder.svelte';
	import { Mic, Info, Play } from 'lucide-svelte';
	import * as m from '$lib/paraglide/messages';
	import type { OnSequenceChange } from '$lib/step-brief/toolSequence';

	type Props = {
		onDone: () => void;
		onSequenceChange?: OnSequenceChange;
	};

	let { onDone, onSequenceChange }: Props = $props();

	/**
	 * Placeholder recordings. This tool has never had a config: the three clips are hardcoded
	 * on S3 and nothing a participant records is persisted. Left as it was found, restyled.
	 */
	const clips = [
		{
			src: 'https://crownshy.s3.eu-west-2.amazonaws.com/alpha_resources/pro.mp4',
			audio: false
		},
		{
			src: 'https://crownshy.s3.eu-west-2.amazonaws.com/alpha_resources/anti.mp4',
			audio: false
		},
		{
			src: 'https://crownshy.s3.eu-west-2.amazonaws.com/alpha_resources/neutral.mp4',
			audio: true
		}
	];

	/** The clips, then the record prompt. The step's own completion screen ends it (ADR-0022). */
	const recordIndex = clips.length;
	let index = $state(0);
	let privacyOpen = $state(false);

	// Writable $derived rather than an effect: leaving the record screen drops the recorder,
	// so coming back to it lands on the prompt again.
	let recording = $derived.by(() => {
		void index;
		return false;
	});

	let clip = $derived(clips[index] ?? null);

	let videoEl = $state<HTMLVideoElement | null>(null);

	// Writable $derived rather than an effect: the overlay belongs to the clip on screen, so
	// stepping to another one puts its play button back. The {#key} above swaps the DOM but
	// leaves this component mounted, so the flag has to be tied to the index itself.
	let playing = $derived.by(() => {
		void index;
		return false;
	});

	function goTo(next: number) {
		index = Math.max(0, Math.min(recordIndex, next));
	}

	// The clips are the tool-internal sequence the pager traverses (ADR-0018). Innermost first:
	// back closes the recorder before it moves off the record screen, and an undefined `next`
	// on that screen is what pops the pager out to the step boundary.
	function back() {
		if (recording) {
			recording = false;
			return;
		}
		goTo(index - 1);
	}

	$effect(() => {
		onSequenceChange?.({
			next: index < recordIndex ? () => goTo(index + 1) : undefined,
			prev: recording || index > 0 ? back : undefined,
			progress: index / (recordIndex + 1)
		});
	});
</script>

<!-- No position marker of its own: the chrome's progress bar already fills as the clips
	advance, and a second one beside the video said the same thing twice. -->
<div class="mx-auto flex w-full grow flex-col px-4">
	{#if clip}
		<!-- One frame, shared by every clip, taking the whole height that is left, and no
			reading-width cap: on a tall screen that cap, not the space left over, is what
			would decide how big the clip gets. Sized off the height rather than the width: a
			portrait video asked to be as wide as the text comes out taller than the screen
			and its controls vanish under the pager's fade. Absolutely positioned so the frame
			has a definite height to take 100% of, and no padding of its own, unlike the
			screens that have actions under them.

			2:3 is the widest ratio the clips are shot in. Letting each video set its own
			footprint made them jump in size from slide to slide, and a narrower frame would
			shrink the wider clips to fit rather than the other way round, so the odd ones
			out letterbox inside this one instead. The frame takes the page's own background
			rather than black, so those bars read as space around the clip instead of as a
			player with the picture sitting off-centre in it. -->
		<div class="relative min-h-0 grow">
			<div class="absolute inset-0 flex items-center justify-center">
				{#key index}
					{#if clip.audio}
						<!-- A contribution with no picture. Sized to what it holds rather than
							filling the clip frame: a portrait box of empty black reads as a
							video that failed to load. -->
						<div
							class="bg-card border-border flex w-80 max-w-full flex-col items-center gap-4 rounded-2xl border px-6 py-8"
						>
							<div
								class="bg-accent text-accent-foreground flex size-16 items-center justify-center rounded-full"
							>
								<Mic class="size-8" aria-hidden="true" />
							</div>
							<p class="text-xl leading-8 font-medium">
								{m.lived_experience_audio_only()}
							</p>
							<audio controls src={clip.src} class="w-full"></audio>
						</div>
					{:else}
						<div
							class="bg-background relative aspect-[2/3] h-full max-w-full overflow-hidden rounded-2xl"
						>
							<!-- svelte-ignore a11y_media_has_caption -->
							<!-- Metadata only: enough for the browser to paint the first frame
								and know the duration, without pulling the whole clip down for
								someone who steps straight past it. The native controls arrive
								with playback. Before that the frame is a still, and the button
								over it is the only thing to press. -->
							<video
								bind:this={videoEl}
								controls={playing}
								playsinline
								preload="metadata"
								src={clip.src}
								onplay={() => (playing = true)}
								class="size-full object-contain"
							></video>

							{#if !playing}
								<button
									type="button"
									onclick={() => videoEl?.play()}
									class="focus-visible:ring-ring absolute inset-0 flex items-center justify-center focus-visible:ring-2 focus-visible:outline-none"
								>
									<span class="sr-only">{m.lived_experience_play_clip()}</span>
									<span
										class="bg-primary text-primary-foreground flex size-20 items-center justify-center rounded-full shadow-lg"
									>
										<!-- Nudged right by an eighth: a triangle centred on its
											bounding box reads as sitting left of centre. -->
										<Play
											class="size-9 translate-x-0.5 fill-current"
											aria-hidden="true"
										/>
									</span>
								</button>
							{/if}
						</div>
					{/if}
				{/key}
			</div>
		</div>
	{:else if recording}
		<VideoRecorder onDone={() => onDone()} onCancel={() => (recording = false)} />
	{:else}
		<div class="mx-auto flex w-full max-w-xl grow flex-col gap-4 py-3">
			<div class="flex grow flex-col justify-center gap-6">
				<p class="text-lg leading-7">{m.lived_experience_record_prompt()}</p>
				<button
					type="button"
					class="text-muted-foreground inline-flex items-center gap-2 self-start text-base italic hover:underline"
					onclick={() => (privacyOpen = true)}
				>
					<Info class="size-4 shrink-0" aria-hidden="true" />
					{m.lived_experience_privacy_question()}
				</button>
			</div>
			<div class="flex flex-col items-center gap-1">
				<Button class="h-12 w-full text-base" onclick={() => (recording = true)}>
					{m.lived_experience_start_recording()}
				</Button>
				<!-- Passing is finishing the step, not skipping it: there is nothing after the
					prompt but the completion screen. -->
				<Button variant="ghost" class="h-12 w-full text-base" onclick={onDone}>
					{m.lived_experience_pass()}
				</Button>
			</div>
		</div>
	{/if}
</div>

<Dialog.Root bind:open={privacyOpen}>
	<Dialog.Content class="gap-5 sm:max-w-[345px]">
		<Dialog.Header class="text-left">
			<Dialog.Title class="text-primary pr-6 text-xl leading-7 font-bold">
				{m.lived_experience_privacy_question()}
			</Dialog.Title>
			<Dialog.Description class="text-foreground text-base leading-6">
				{m.lived_experience_privacy_body()}
			</Dialog.Description>
		</Dialog.Header>
	</Dialog.Content>
</Dialog.Root>

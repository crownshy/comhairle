<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import * as Dialog from '$lib/components/ui/dialog';
	import SlideDots from '$lib/components/participant/SlideDots.svelte';
	import VideoRecorder from './VideoRecorder.svelte';
	import { Mic, Info } from 'lucide-svelte';
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

<!-- The dots are the tool's position, in the margin beside the content rather than under it,
	so the bottom of the screen stays free for the record prompt's own two actions. -->
<div class="relative mx-auto flex w-full max-w-xl grow flex-col px-4 py-3">
	<div class="pointer-events-none absolute top-1/2 right-0 z-10 -translate-y-1/2">
		<SlideDots {index} count={recordIndex + 1} orientation="vertical" />
	</div>

	{#if clip}
		<!-- The clip is sized off the height it has rather than the column's width: asked to be
			as wide as the text, a portrait video comes out taller than the screen and its
			bottom (the audio player with it) vanishes under the pager's fade. Absolutely
			positioned so the aspect box has a definite height to take 100% of. -->
		<div class="relative min-h-0 grow">
			<div class="absolute inset-0 flex items-center justify-center">
				<div
					class="relative aspect-[9/16] h-full max-w-full overflow-hidden rounded-2xl bg-black"
				>
					{#key index}
						{#if clip.audio}
							<!-- A contribution with no picture. The frame says why it is black,
								and the player sits where a video's control bar would be. -->
							<div
								class="flex size-full flex-col items-center justify-center gap-3 text-white"
							>
								<Mic class="size-12" aria-hidden="true" />
								<p class="text-xl leading-8 font-medium">
									{m.lived_experience_audio_only()}
								</p>
							</div>
							<audio
								controls
								src={clip.src}
								class="absolute inset-x-4 bottom-4 w-[calc(100%-2rem)]"
							></audio>
						{:else}
							<!-- svelte-ignore a11y_media_has_caption -->
							<video
								controls
								playsinline
								src={clip.src}
								class="size-full object-contain"
							></video>
						{/if}
					{/key}
				</div>
			</div>
		</div>
	{:else if recording}
		<VideoRecorder onDone={() => onDone()} onCancel={() => (recording = false)} />
	{:else}
		<div class="flex min-h-0 grow flex-col gap-4">
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
		<div class="flex justify-center">
			<SlideDots {index} count={recordIndex + 1} />
		</div>
	</Dialog.Content>
</Dialog.Root>

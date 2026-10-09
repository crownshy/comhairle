<script lang="ts">
	import VideoRecorder from '$lib/components/VideoRecorder.svelte';
	import type { OnSequenceChange } from '$lib/tools/toolSequence';
	import { m } from '$lib/paraglide/messages';

	type Props = {
		onSequenceChange?: OnSequenceChange;
	};

	let { onSequenceChange }: Props = $props();

	const PHASES = ['PRE', 'FirstVideo', 'SecondVideo', 'ThirdVideo', 'Recording', 'Done'] as const;
	let phaseIndex = $state(0);
	let phase = $derived(PHASES[phaseIndex]);

	const VIDEO_PHASES: readonly string[] = ['FirstVideo', 'SecondVideo', 'ThirdVideo'];
	let videoIndex = $derived(VIDEO_PHASES.indexOf(phase));

	// Recording only moves on through the recorder, so forward there skips the step.
	let canPageForward = $derived(phase !== 'Recording' && phase !== 'Done');

	$effect(() => {
		onSequenceChange?.({
			next: canPageForward ? () => phaseIndex++ : undefined,
			previous: phaseIndex > 0 ? () => phaseIndex-- : undefined,
			progress: phaseIndex / PHASES.length,
			position:
				videoIndex >= 0
					? m.video_x_of_y({ current: videoIndex + 1, total: VIDEO_PHASES.length })
					: undefined
		});
	});
</script>

{#if phase == 'PRE'}
	<div class="flex flex-col gap-4">
		<p>
			You are about to see 3 videos recorded by other participants. They will talk about their
			experience of the topic being discussed.
		</p>
		<p>
			After you are done you will get a chance to record your own video to be shared with
			others
		</p>
	</div>
{/if}

{#if phase == 'FirstVideo'}
	<div class="flex flex-col items-center justify-center gap-4">
		<video controls width="400" autoplay>
			<source
				src="https://crownshy.s3.eu-west-2.amazonaws.com/alpha_resources/pro.mp4"
				type="video/mp4"
			/>
		</video>
	</div>
{/if}

{#if phase == 'SecondVideo'}
	<div class="flex flex-col items-center justify-center gap-4">
		<video controls width="400" autoplay>
			<source
				src="https://crownshy.s3.eu-west-2.amazonaws.com/alpha_resources/anti.mp4"
				type="video/mp4"
			/>
		</video>
	</div>
{/if}

{#if phase == 'ThirdVideo'}
	<div class="flex flex-col items-center justify-center gap-4">
		<video controls width="400" autoplay>
			<source
				src="https://crownshy.s3.eu-west-2.amazonaws.com/alpha_resources/neutral.mp4"
				type="video/mp4"
			/>
		</video>
	</div>
{/if}

{#if phase == 'Recording'}
	<p class="mb-5">
		You just heard three other peoples views on the issue. Would you like to record your own to
		let others know what you think?
	</p>
	<p>
		We will only show this video to other people signed up to take part in this conversation.
		Like with any platform on the internet we can't 100% guarantee that someone wont download
		this video and use it elsewhere. If your comfortable with that go ahead if not feel free to
		skip this step.
	</p>
	<VideoRecorder onDone={() => (phaseIndex = PHASES.indexOf('Done'))} />
{/if}
{#if phase == 'Done'}
	<div class="flex flex-col items-center justify-center gap-4">
		<p>Thanks for sharing your views!</p>
	</div>
{/if}

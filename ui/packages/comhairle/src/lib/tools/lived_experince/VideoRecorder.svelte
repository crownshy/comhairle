<script lang="ts">
	import { onMount } from 'svelte';
	import { Button } from '$lib/components/ui/button';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import * as m from '$lib/paraglide/messages';

	type Props = {
		/** The take is good enough. Nothing is uploaded: see the note in LivedExperinceUI. */
		onDone: () => void;
		/** No camera to record with, or they changed their mind. Back to the prompt. */
		onCancel: () => void;
	};

	let { onDone, onCancel }: Props = $props();

	const LIMIT_SECONDS = 20;

	let phase = $state<'starting' | 'recording' | 'review' | 'blocked'>('starting');
	let secondsLeft = $state(LIMIT_SECONDS);
	let stream = $state<MediaStream | null>(null);
	let playbackUrl = $state<string | null>(null);
	let previewEl = $state<HTMLVideoElement | null>(null);

	let recorder: MediaRecorder | null = null;
	let chunks: Blob[] = [];
	let countdown: ReturnType<typeof setInterval> | null = null;

	// The camera opens when the recorder mounts, which is the tap on Start recording: the
	// browser's permission prompt is the answer to an action the participant just took.
	onMount(() => {
		void start();
		return teardown;
	});

	$effect(() => {
		if (previewEl && stream) previewEl.srcObject = stream;
	});

	async function start() {
		const media = await tryCatchAsync(() =>
			navigator.mediaDevices.getUserMedia({ video: true, audio: true })
		);
		if (media.err) {
			phase = 'blocked';
			return;
		}
		stream = media.ok;
		beginRecording();
	}

	function beginRecording() {
		if (!stream) return;
		chunks = [];
		recorder = new MediaRecorder(stream);
		recorder.ondataavailable = (event) => {
			if (event.data.size > 0) chunks.push(event.data);
		};
		recorder.onstop = () => {
			if (playbackUrl) URL.revokeObjectURL(playbackUrl);
			playbackUrl = URL.createObjectURL(new Blob(chunks, { type: 'video/webm' }));
			phase = 'review';
		};
		recorder.start();
		phase = 'recording';
		secondsLeft = LIMIT_SECONDS;
		countdown = setInterval(() => {
			secondsLeft -= 1;
			if (secondsLeft <= 0) stopRecording();
		}, 1000);
	}

	function stopRecording() {
		clearCountdown();
		if (recorder && recorder.state !== 'inactive') recorder.stop();
	}

	function recordAgain() {
		// The camera is still open through the review, so a retake starts straight away.
		if (playbackUrl) URL.revokeObjectURL(playbackUrl);
		playbackUrl = null;
		beginRecording();
	}

	function clearCountdown() {
		if (countdown) clearInterval(countdown);
		countdown = null;
	}

	function teardown() {
		clearCountdown();
		if (recorder && recorder.state !== 'inactive') recorder.stop();
		stream?.getTracks().forEach((track) => track.stop());
		stream = null;
		if (playbackUrl) URL.revokeObjectURL(playbackUrl);
	}
</script>

<div class="flex min-h-0 grow flex-col gap-4">
	<!-- Sized off the height that is left over rather than the column's width, so the frame
		never grows past the screen and pushes the actions under it out of reach. -->
	<div class="relative min-h-0 grow">
		<div class="absolute inset-0 flex items-center justify-center">
			<div
				class="relative aspect-[9/16] h-full max-w-full overflow-hidden rounded-2xl bg-black"
			>
				{#if phase === 'review' && playbackUrl}
					<!-- svelte-ignore a11y_media_has_caption -->
					<video controls playsinline src={playbackUrl} class="size-full object-cover"
					></video>
				{:else}
					<video
						bind:this={previewEl}
						autoplay
						muted
						playsinline
						class="size-full object-cover"
					></video>
				{/if}

				{#if phase === 'recording'}
					<div
						class="absolute top-4 left-4 flex items-center gap-2 rounded-full bg-black/60 px-3 py-1 text-white"
						role="status"
					>
						<span class="bg-destructive size-2 rounded-full" aria-hidden="true"></span>
						<span class="text-sm font-medium">
							{m.lived_experience_recording_countdown({ seconds: secondsLeft })}
						</span>
					</div>
				{/if}

				{#if phase !== 'recording' && phase !== 'review'}
					<p
						class="absolute inset-0 flex items-center justify-center px-6 text-center text-base text-white"
						role="status"
					>
						{phase === 'blocked'
							? m.lived_experience_camera_blocked()
							: m.lived_experience_camera_starting()}
					</p>
				{/if}
			</div>
		</div>
	</div>

	<div class="flex flex-col items-center gap-1">
		{#if phase === 'recording'}
			<Button class="h-12 w-full text-base" onclick={stopRecording}>
				{m.lived_experience_stop_recording()}
			</Button>
		{:else if phase === 'review'}
			<Button class="h-12 w-full text-base" onclick={onDone}>
				{m.lived_experience_use_recording()}
			</Button>
			<Button variant="ghost" class="h-12 w-full text-base" onclick={recordAgain}>
				{m.lived_experience_record_again()}
			</Button>
		{:else if phase === 'blocked'}
			<Button variant="ghost" class="h-12 w-full text-base" onclick={onCancel}>
				{m.lived_experience_pass()}
			</Button>
		{/if}
	</div>
</div>

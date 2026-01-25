<script lang="ts">
	import { onMount, tick } from 'svelte';
	import { Button } from '$lib/components/ui/button';
	import { Card, CardContent } from '$lib/components/ui/card';
	import { Progress } from '$lib/components/ui/progress';

	type Props = {
		onDone: () => void;
	};

	let { onDone } = $props();

	let stream: MediaStream | null = null;
	let mediaRecorder: MediaRecorder | null = null;
	let recordedChunks: Blob[] = [];

	let videoUrl: string | null = null;
	let countdown = $state(20);
	let isRecording = $state(false);
	let isRecorded = $state(false);
	let isUploading = $state(false);
	let uploadProgress = $state(0);

	let countdownInterval = $state<ReturnType<typeof setInterval> | null>(null);
	let previewVideoEl = $state<HTMLVideoElement | null>(null);

	const uploadEndpoint = 'https://your-api.com/upload'; // replace this
	$effect(() => {
		navigator.mediaDevices.getUserMedia({ video: true, audio: true }).then((video_stream) => {
			stream = video_stream;
			previewVideoEl!.srcObject = stream;
		});
	});

	async function startRecording() {
		if (!stream) return;
		try {
			recordedChunks = [];
			mediaRecorder = new MediaRecorder(stream);

			mediaRecorder.ondataavailable = (event: BlobEvent) => {
				if (event.data.size > 0) {
					recordedChunks.push(event.data);
				}
			};

			mediaRecorder.onstop = () => {
				const blob = new Blob(recordedChunks, { type: 'video/webm' });
				videoUrl = URL.createObjectURL(blob);
				isRecorded = true;
				stopStream();
			};

			mediaRecorder.start();
			isRecording = true;
			countdown = 20;

			countdownInterval = setInterval(() => {
				countdown--;
				if (countdown <= 0) {
					stopRecording();
				}
			}, 1000);

			setTimeout(() => {
				if (isRecording) stopRecording();
			}, 20000);
		} catch (e) {
			console.error('Error starting recording:', e);
		}
	}

	function stopRecording() {
		if (mediaRecorder && mediaRecorder.state !== 'inactive') {
			mediaRecorder.stop();
		}
		isRecording = false;
		clearCountdown();
	}

	function stopStream() {
		stream?.getTracks().forEach((track) => track.stop());
		stream = null;
	}

	function clearCountdown() {
		if (countdownInterval) {
			clearInterval(countdownInterval);
			countdownInterval = null;
		}
	}

	function reset() {
		isRecorded = false;
		videoUrl = null;
		uploadProgress = 0;
		isUploading = false;
	}

	function skip() {
		onDone();
	}

	function uploadVideo() {
		onDone();
		return;
		if (!recordedChunks.length) return;

		const blob = new Blob(recordedChunks, { type: 'video/webm' });
		const formData = new FormData();
		formData.append('video', blob, 'recording.webm');

		const xhr = new XMLHttpRequest();
		xhr.open('POST', uploadEndpoint, true);

		xhr.upload.onprogress = (event: ProgressEvent) => {
			if (event.lengthComputable) {
				uploadProgress = Math.round((event.loaded / event.total) * 100);
			}
		};

		xhr.onload = () => {
			if (xhr.status >= 200 && xhr.status < 300) {
				alert('Upload successful!');
			} else {
				alert('Upload failed.');
			}
			isUploading = false;
		};

		xhr.onerror = () => {
			alert('Upload failed.');
			isUploading = false;
		};

		isUploading = true;
		xhr.send(formData);
	}
</script>

<Card class="mx-auto mt-10 max-w-xl space-y-6 rounded-[0.2rem] p-6">
	<h2 class="text-2xl font-bold">🎥 Record Your Video</h2>

	{#if !isRecording && !isRecorded}
		<video
			bind:this={previewVideoEl}
			autoplay
			muted
			playsinline
			class="mt-4 w-full rounded-[0.2rem] shadow-sm"
		/>
		<div class="flex flex-col gap-2">
			<Button onclick={startRecording}>Start Recording</Button>
			<Button variant="secondary" onclick={skip}>Pass on this step</Button>
		</div>
	{/if}

	<div class="space-y-2" style:display={isRecording ? null : 'none'}>
		<p class="font-semibold text-red-600">Recording... {countdown}s left</p>
		<Progress value={(countdown / 20) * 100} class="h-2" />
		<video
			bind:this={previewVideoEl}
			autoplay
			muted
			playsinline
			class="mt-4 w-full rounded-[0.2rem] shadow-sm"
		/>
		<Button variant="destructive" class="w-full" onclick={stopRecording}>Stop Now</Button>
	</div>

	{#if isRecorded && videoUrl}
		<CardContent class="space-y-4">
			<video controls src={videoUrl} class="w-full rounded-[0.2rem] shadow-sm" />
			<div class="flex gap-4">
				<Button onclick={uploadVideo} disabled={isUploading}>Upload</Button>
				<Button variant="outline" on:click={reset} disabled={isUploading}
					>Record Again</Button
				>
			</div>

			{#if isUploading}
				<div class="space-y-2">
					<p class="text-sm text-gray-500">Uploading: {uploadProgress}%</p>
					<Progress value={uploadProgress} class="h-2" />
				</div>
			{/if}
		</CardContent>
	{/if}
</Card>

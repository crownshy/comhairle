<script lang="ts">
	import Button from '$lib/components/ui/button/button.svelte';
	import VideoRecorder from '$lib/components/VideoRecorder.svelte';

	type Props = {
		onDone: () => void;
	};

	let { onDone }: Props = $props();
	let phase = $state('PRE');
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

		<Button variant="secondary" onclick={() => (phase = 'FirstVideo')}>Begin</Button>
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
		<Button onclick={() => (phase = 'SecondVideo')} variant="secondary">Next</Button>
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
		<Button onclick={() => (phase = 'ThirdVideo')} variant="secondary">Next</Button>
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
		<Button onclick={() => (phase = 'Recording')} variant="secondary">Next</Button>
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
	<VideoRecorder onDone={() => (phase = 'Done')} />
{/if}
{#if phase == 'Done'}
	<div class="flex flex-col items-center justify-center gap-4">
		<p>Thanks for sharing your views!</p>
		<Button onclick={onDone} variant="secondary">Continue</Button>
	</div>
{/if}

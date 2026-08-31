<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import VideoRecorder from '$lib/components/VideoRecorder.svelte';
	import { Mic, Info, X } from 'lucide-svelte';
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

	/** The clips, then the record prompt, then a thank you. */
	const lastIndex = clips.length + 1;
	let index = $state(0);
	let privacyOpen = $state(false);

	let clip = $derived(clips[index] ?? null);
	let isRecord = $derived(index === clips.length);
	let isDone = $derived(index === lastIndex);

	function goTo(next: number) {
		index = Math.max(0, Math.min(lastIndex, next));
	}

	// The clips are the tool-internal sequence the pager traverses (ADR-0018). Undefined
	// `next` on the thank-you screen is what pops the pager out to the step boundary.
	$effect(() => {
		onSequenceChange?.({
			next: index < lastIndex ? () => goTo(index + 1) : undefined,
			prev: index > 0 ? () => goTo(index - 1) : undefined,
			progress: index / lastIndex
		});
	});
</script>

<div class="mx-auto flex w-full max-w-xl grow flex-col gap-6 py-4">
	{#if clip}
		<div class="relative aspect-[9/16] max-h-[60vh] overflow-hidden rounded-2xl bg-black">
			{#if clip.audio}
				<div class="flex size-full flex-col items-center justify-center gap-3">
					<Mic class="size-12 text-white" aria-hidden="true" />
					<p class="text-xl leading-8 font-medium text-white">Audio only</p>
					<audio controls class="w-4/5">
						<source src={clip.src} />
					</audio>
				</div>
			{:else}
				<!-- svelte-ignore a11y_media_has_caption -->
				<video controls class="size-full object-cover" src={clip.src}></video>
			{/if}
		</div>
		<p class="text-muted-foreground text-center text-base">
			Recording {index + 1} of {clips.length}
		</p>
	{:else if isRecord}
		<div class="flex flex-col gap-4">
			<p class="text-base leading-6">
				You just heard three other people's views on the issue. Would you like to record
				your own to let others know what you think?
			</p>
			<button
				type="button"
				class="text-primary inline-flex items-center gap-2 self-start text-base font-medium underline"
				onclick={() => (privacyOpen = true)}
			>
				<Info class="size-4" />
				Will others see my recording?
			</button>
			<VideoRecorder onDone={() => goTo(lastIndex)} />
		</div>
	{:else if isDone}
		<div class="flex grow flex-col items-center justify-center gap-4 text-center">
			<h2 class="text-primary text-3xl leading-10 font-bold">
				Thanks for sharing your views
			</h2>
			<Button onclick={onDone} variant="secondary">Continue</Button>
		</div>
	{/if}
</div>

{#if privacyOpen}
	<div class="fixed inset-0 z-50 flex items-center justify-center p-6">
		<button
			type="button"
			class="bg-foreground/70 absolute inset-0"
			aria-label="Close"
			onclick={() => (privacyOpen = false)}
		></button>
		<div
			class="border-border bg-card relative z-10 w-full max-w-[345px] rounded-[10px] border p-6 shadow-lg"
			role="dialog"
			aria-modal="true"
			aria-labelledby="lived-privacy-title"
		>
			<button
				type="button"
				class="text-muted-foreground absolute top-4 right-4 inline-flex size-5 items-center justify-center"
				aria-label="Close"
				onclick={() => (privacyOpen = false)}
			>
				<X class="size-4" />
			</button>
			<h2 id="lived-privacy-title" class="text-primary pt-2 text-xl font-bold">
				Will others see my recording?
			</h2>
			<p class="mt-3 text-base leading-6">
				We will only show this video to other people signed up to take part in this
				conversation. Like with any platform on the internet we can't guarantee that someone
				won't download this video and use it elsewhere. If you're comfortable with that go
				ahead, and if not feel free to skip this step.
			</p>
		</div>
	</div>
{/if}

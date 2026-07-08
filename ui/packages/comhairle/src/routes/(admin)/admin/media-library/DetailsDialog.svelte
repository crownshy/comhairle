<script lang="ts">
	import * as Dialog from '$lib/components/ui/dialog';
	import type { HTMLMediaElement } from '$lib/utils/types';
	import '$lib/components/Media/MediaBackground.css';
	import Label from '$lib/components/ui/label/label.svelte';
	import Input from '$lib/components/ui/input/input.svelte';
	import Button from '$lib/components/ui/button/button.svelte';
	import { Trash2 } from 'lucide-svelte';

	interface Props {
		type: HTMLMediaElement | undefined;
		filename: string;
		src: string;
		alt: string;
	}

	const { type, filename: currentFilename, alt: currentAlt, src }: Props = $props();

	const LABEL_SPACING = 'mb-2';
	const INPUT_SPACING = 'mb-8';

	let filename = $derived(currentFilename.split('.')[0]);
	const extension = $derived(`.${currentFilename.split('.')[1]}`);

	let alt = $derived(currentAlt);
</script>

<Dialog.Portal>
	<Dialog.Overlay>
		<Dialog.Content class="flex h-[80vh] min-w-[90vw] flex-col overflow-y-scroll">
			<Dialog.Header class="text-xl font-bold">Details</Dialog.Header>
			<div class="grid-columns grid gap-7">
				<div
					class="chequered-background flex flex-col justify-center rounded p-1 lg:min-h-[50vh] lg:p-6"
				>
					{#if type === undefined}
						<p>Error reading file type</p>
					{:else if type === 'audio'}
						<audio {src} controls autoplay={false}></audio>
					{:else if type === 'video'}
						<video {src} controls autoplay={false} class="max-h-[60vh]">
							<track kind="captions" />
						</video>
					{:else}
						<img {src} alt="" class="max-h-[60vh] object-contain" />
					{/if}
				</div>
				<aside class="mr-0 flex w-full flex-col lg:mr-auto lg:w-9/10">
					<Label class={`${LABEL_SPACING} text-muted-foreground`} for="filename"
						>Filename</Label
					>
					<div class={`${INPUT_SPACING} flex h-9 flex-row items-center`}>
						<Input
							id="filename"
							type="text"
							bind:value={filename}
							class="rounded-none rounded-s-lg border-e-0"
						/>
						<div
							class="flex flex-col items-center justify-center self-stretch rounded-e-lg border border-s-0 px-4 text-xs"
						>
							{extension}
						</div>
					</div>
					<Label class={`${LABEL_SPACING} text-muted-foreground`} for="alt"
						>Alt text</Label
					>
					<Input class={INPUT_SPACING} id="alt" type="text" bind:value={alt} />
					<div class="mt-5 flex flex-row items-center self-end lg:mt-10">
						<Button
							variant="outline"
							class="mr-5"
							aria-label="Delete media"
							title="Delete media"><Trash2 /></Button
						>
						<Button>Update</Button>
					</div>
				</aside>
			</div>
		</Dialog.Content>
	</Dialog.Overlay>
</Dialog.Portal>

<style>
	.grid-columns {
		grid-template-columns: 2fr 1fr;

		@media (width <= 64rem) {
			grid-template-columns: 1fr;
		}
	}
</style>

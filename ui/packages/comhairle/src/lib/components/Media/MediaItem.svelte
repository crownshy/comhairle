<script lang="ts">
	import { capitalise } from '$lib/utils/string';
	import type { HTMLMediaElement } from '$lib/utils/types';
	import type { MediaDto } from '@crownshy/api-client/api';
	import { type Icon, Music, Video } from 'lucide-svelte';
	// FIX: Upgrade lucide icons to remove deprecated types
	import type { ComponentType } from 'svelte';
	import Badge from '$lib/components/ui/badge/badge.svelte';

	interface Props extends MediaDto {
		type: HTMLMediaElement;
		alt: string;
	}

	const { type, alt, ...props }: Props = $props();
</script>

<div
	class="background container aspect-video max-w-96 cursor-pointer items-center overflow-hidden rounded-lg"
>
	<div class="media-background justify-self-center overflow-hidden select-none">
		{#snippet placeholder(Icon: ComponentType<Icon>)}
			<Icon size={80} strokeWidth={1.7} />
		{/snippet}
		{#if type === 'audio'}
			<!-- <audio {src} controls autoplay={false} class="h-full w-full"></audio> -->
			{@render placeholder(Music)}
		{:else if type === 'video'}
			<!-- <video {src} controls autoplay={false} class="h-full w-full" -->
			<!-- 	><track kind="captions" /></video -->
			<!-- > -->
			{@render placeholder(Video)}
		{:else}
			<img src={props.url} {alt} class="aspect-video max-h-32 object-contain" />
		{/if}
	</div>
	<div
		class="bg-card text-card-foreground flex flex-col items-start self-stretch px-5 py-2 text-xs"
		title={props.filename}
	>
		<div class="grid-auto grid items-center justify-between self-stretch">
			<div class="truncate font-bold">
				{props.filename}
			</div>
			<Badge variant="secondary" class="justify-self-end">
				{capitalise(type)}
			</Badge>
		</div>
		<span class="text-muted-foreground">{props.contentType.split('/')[1].toUpperCase()}</span>
	</div>
</div>

<style>
	.container {
		display: grid;
		grid-template-rows: 4fr 1fr;
		grid-template-columns: 1fr;
		border: 2px solid var(--selected, 'transparent');
	}
	.background {
		--size: 24px;
		--bg-color: oklch(0.4 5% 270);
		--bg-alt-color: oklch(from var(--bg-color) calc(l * 0.85) c h);

		background: repeating-conic-gradient(
				var(--bg-color) 0%,
				var(--bg-color) 25%,
				var(--bg-alt-color) 0%,
				var(--bg-alt-color) 50%
			)
			50% center / var(--size) var(--size);
	}
	.grid-auto {
		grid-template-columns: auto auto;
	}
</style>

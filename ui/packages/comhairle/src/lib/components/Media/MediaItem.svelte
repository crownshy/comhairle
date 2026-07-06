<script lang="ts">
	import type { HTMLMediaElement } from '$lib/utils/types';
	import type { MediaDto } from '@crownshy/api-client/api';
	import { type Icon, Music, Video } from 'lucide-svelte';
	// FIX: Upgrade lucide icons to remove deprecated types
	import type { ComponentType } from 'svelte';

	interface Props extends MediaDto {
		type: HTMLMediaElement;
		alt: string;
	}

	const { type, alt, ...props }: Props = $props();
</script>

<div class="container aspect-video max-w-96 items-center overflow-hidden rounded-lg bg-gray-600">
	<div class="justify-self-center">
		{#snippet placeholder(Icon: ComponentType<Icon>)}
			<Icon size={50} />
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
			<img src={props.url} {alt} class="aspect-video max-h-28 object-contain" />
		{/if}
	</div>
	<div class="bg-primary text-primary-foreground self-stretch px-5 py-2">
		{props.filename}
	</div>
</div>

<style>
	.container {
		display: grid;
		grid-template-rows: 4fr 1fr;
		grid-template-columns: 1fr;
		border: 2px solid var(--selected, 'transparent');
	}
</style>

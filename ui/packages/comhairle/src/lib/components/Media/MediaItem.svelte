<script lang="ts">
	import { type Icon, Music, Video } from 'lucide-svelte';
	// FIX: Upgrade lucide icons to remove deprecated types
	import type { ComponentType } from 'svelte';

	type Props = {
		type: 'audio' | 'video' | 'img';
		src: string;
		alt: string;
	};

	const { type, src, alt }: Props = $props();
</script>

<div class="flex h-28 w-3xs flex-row items-center justify-center rounded bg-gray-600">
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
		<img {src} {alt} class="h-full w-full overflow-hidden object-cover" />
	{/if}
</div>

<style>
	div {
		border: 2px solid var(--selected, 'transparent');
	}
</style>

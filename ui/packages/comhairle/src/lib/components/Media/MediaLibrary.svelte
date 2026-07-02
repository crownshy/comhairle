<script lang="ts">
	import { htmlFromMediaType, type HTMLMediaElement } from '$lib/utils/types';
	import type { Snippet } from 'svelte';
	import type { MockData } from '../../../routes/(admin)/admin/media-library/+page.server';

	interface Props {
		data: MockData[];
		rowHeight: number;
		media: Snippet<[type: HTMLMediaElement, d: MockData]>;
	}

	const { data, rowHeight, media }: Props = $props();
</script>

<ul class="flex flex-wrap gap-2">
	{#if data.length === 0}
		<div class="mt-8">
			<p class="text-muted-foreground font-bold">No media found</p>
		</div>
	{:else}
		{#each data as d (d.id)}
			{@const type = htmlFromMediaType(d.contentType)}
			{#if type}
				<li class={`relative h-[${rowHeight}vh] grow overflow-hidden rounded-sm`}>
					{@render media(type, d)}
				</li>
			{/if}
		{/each}
	{/if}
</ul>

<style>
	li {
		transition: all 300ms ease-out;

		&:is(:focus, :hover) {
			filter: brightness(85%);
			transform: scale(1.01);
		}
	}
</style>

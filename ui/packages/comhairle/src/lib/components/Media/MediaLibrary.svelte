<script lang="ts">
	import { htmlFromMediaType, type HTMLMediaElement } from '$lib/utils/types';
	import type { Snippet } from 'svelte';
	import type { MediaDto } from '@crownshy/api-client/api';

	interface Props<T extends MediaDto> {
		data: T[];
		media: Snippet<[type: HTMLMediaElement, d: T]>;
	}

	const { data, media }: Props<MediaDto> = $props();
</script>

<ul class="grid gap-5">
	{#if data.length === 0}
		<div class="mt-8">
			<p class="text-muted-foreground font-bold">No media found</p>
		</div>
	{:else}
		{#each data as d (d.id)}
			{@const type = htmlFromMediaType(d.contentType)}
			{#if type}
				<li class="relative overflow-hidden rounded-sm">
					{@render media(type, d)}
				</li>
			{/if}
		{/each}
	{/if}
</ul>

<style>
	ul {
		--min-width: 275px;
		grid-template-columns: repeat(auto-fit, minmax(var(--min-width), 1fr));
	}

	li {
		transition: all 300ms ease-out;

		&:is(:focus, :hover) {
			filter: brightness(85%);
			transform: scale(1.01);
		}
	}
</style>

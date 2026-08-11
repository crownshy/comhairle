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

<ul class="flex flex-row flex-wrap gap-5">
	{#if data.length === 0}
		<div class="mt-8">
			<p class="text-muted-foreground font-bold">No media found</p>
		</div>
	{:else}
		{#each data as d (d.id)}
			{@const type = htmlFromMediaType(d.contentType)}
			{#if type}
				<li class="relative">
					{@render media(type, d)}
				</li>
			{/if}
		{/each}
	{/if}
</ul>

<script lang="ts">
	import { htmlFromMediaType, type HTMLMediaElement } from '$lib/utils/types';
	import type { ComponentType } from 'svelte';
	import type { MediaDto } from '@crownshy/api-client/api';
	import { Music, Video, type Icon } from 'lucide-svelte';
	import { Badge } from '$lib/components/ui/badge';
	import { capitalise } from '$lib/utils/casingUtils';
	import { Button } from '../ui/button';
	import './MediaBackground.css';

	type BaseProps = {
		media: MediaDto[];
	};
	type ViewMode = {
		mode: 'view';
		onDetailsOpen?: (media: MediaDto) => void;
		selected?: undefined;
	};
	type SingleSelectMode = {
		mode: 'single-select';
		onDetailsOpen?: undefined;
		selected: MediaDto | null;
	};
	type MultiSelectMode = {
		mode: 'multi-select';
		onDetailsOpen?: undefined;
		selected: MediaDto[];
	};
	type Props = BaseProps & (ViewMode | SingleSelectMode | MultiSelectMode);

	let { media, mode, onDetailsOpen, selected = $bindable() }: Props = $props();
</script>

{#snippet MediaItem(media: MediaDto)}
	{@const type: HTMLMediaElement | undefined = htmlFromMediaType(media.contentType) }
	<div
		class="chequered-background border-accent media-item grid aspect-video min-h-48 w-full cursor-pointer grid-cols-1 grid-rows-[4fr_1fr] items-center overflow-hidden rounded-lg border-2"
	>
		<div class="justify-self-center overflow-hidden select-none">
			{#snippet placeholder(Icon: ComponentType<Icon>)}
				<Icon size={80} strokeWidth={1.7} />
			{/snippet}
			{#if type === 'audio'}
				{@render placeholder(Music)}
			{:else if type === 'video'}
				{@render placeholder(Video)}
			{:else}
				<img src={media.url} alt={media.alt} class="aspect-video max-h-32 object-contain" />
			{/if}
		</div>
		<div
			class="bg-card text-card-foreground flex flex-col items-start self-stretch px-5 py-2 text-xs"
			title={media.name}
		>
			<div class="grid grid-cols-[auto_auto] items-center justify-between self-stretch">
				<div class="truncate font-bold">
					{media.name}
				</div>
				<Badge variant="secondary" class="justify-self-end">
					{capitalise(type ?? '')}
				</Badge>
			</div>
			<span class="text-muted-foreground">
				{media.contentType.split('/')[1].toUpperCase()}
			</span>
		</div>
	</div>
{/snippet}

<ul class="mt-5 flex flex-row flex-wrap gap-5">
	{#each media as m (m.id)}
		<li class="relative max-w-75">
			{#if mode === 'view'}
				<Button
					class="block aspect-video h-full w-full"
					command="open-dialog:media-details"
					plain
					onclick={() => {
						onDetailsOpen?.(m);
					}}
				>
					{@render MediaItem(m)}
				</Button>
			{:else if mode === 'single-select'}
				<label class="cursor-pointer">
					<input
						type="radio"
						name="selected"
						value={m.id}
						class="hidden"
						bind:group={selected}
					/>
					{@render MediaItem(m)}
				</label>
			{:else if mode === 'multi-select'}
				<label class="cursor-pointer">
					<input
						type="checkbox"
						name="media"
						value={m.id.toString()}
						class="accent-primary absolute top-2 left-2 z-2 h-4"
						bind:group={selected}
					/>
					{@render MediaItem(m)}
				</label>
			{/if}
		</li>
	{:else}
		<span class="inline-block mt-8 text-muted-foreground font-bold">No media found</span>
	{/each}
</ul>

<style>
	li {
		transition: translate 150ms ease-out;
		translate: 0px 0px;

		&:is(:focus-visible, :hover) {
			translate: 0px -3px;
		}

		& input[type='checkbox']:checked + .media-item {
			opacity: 70%;
			border-color: var(--ring);
		}

		& input[type='radio']:checked + .media-item {
			border-color: var(--ring);
		}
	}
</style>

<script lang="ts">
	import { htmlFromMediaType, type HTMLMediaElement } from '$lib/utils/types';
	import type { Component } from 'svelte';
	import type { MediaDto } from '@crownshy/api-client/api';
	import { Music, Video, type IconProps } from '@lucide/svelte';
	import { Badge } from '$lib/components/ui/badge';
	import { capitalise } from '$lib/utils/casingUtils';
	import './MediaBackground.css';
	import ErrorMessage from '../ErrorMessage.svelte';
	import type { ErrorType, Result } from '$lib/utils/errorHandling';
	import Media from '$lib/interfaces/Media.svelte';

	type BaseProps = {
		media: Result<MediaDto[], ErrorType>;
	};
	type ViewMode = {
		mode: 'view';
		onDetailsOpen?: (media: MediaDto) => void;
		selected?: undefined;
	};
	type SingleSelectMode = {
		mode: 'single-select';
		onDetailsOpen?: undefined;
		selected: MediaDto['id'] | null;
	};
	type MultiSelectMode = {
		mode: 'multi-select';
		onDetailsOpen?: undefined;
		selected: MediaDto['id'][];
	};
	type Props = BaseProps & (ViewMode | SingleSelectMode | MultiSelectMode);

	let { media, mode, onDetailsOpen, selected = $bindable() }: Props = $props();
</script>

{#snippet MediaItem(media: MediaDto)}
	{@const type: HTMLMediaElement | undefined = htmlFromMediaType(media.contentType) }
	<div
		class="chequered-background border-muted media-item hover:border-accent focus-visible:border-accent grid aspect-video min-h-48 w-full cursor-pointer grid-cols-1 grid-rows-[4fr_1fr] items-center overflow-hidden rounded-lg border-2"
	>
		<div class="justify-self-center overflow-hidden select-none">
			{#snippet placeholder(Icon: Component<IconProps>)}
				<Icon size={80} strokeWidth={1.7} />
			{/snippet}
			{#if type === 'audio'}
				{@render placeholder(Music)}
			{:else if type === 'video'}
				{@render placeholder(Video)}
			{:else}
				<img
					src={media.url}
					alt={media.alt}
					class="aspect-video max-h-48 object-contain md:max-h-32"
				/>
			{/if}
		</div>
		<div
			class="bg-card text-card-foreground grid grid-cols-[auto_auto] items-start justify-between gap-1 self-stretch px-5 py-2 text-xs"
			title={media.name}
		>
			<div class="truncate font-bold">
				{media.name}
			</div>
			<Badge variant="secondary" class="justify-self-end">
				{capitalise(type ?? '')}
			</Badge>
			<span class="text-muted-foreground justify-self-start font-semibold">
				{media.contentType.split('/')[1].toUpperCase()}
			</span>
			<!-- For old media the size wasn't provided on creation, so it defaults to 0, in this case just hide the file size -->
			{#if media.size > 0}
				<span class="text-muted-foreground font-mono font-semibold">
					{Media.formatBytes(media.size, { dp: 0 })}
				</span>
			{/if}
		</div>
	</div>
{/snippet}

<ul class="dynamic-grid gap-5">
	{#if media.err !== null}
		{console.error(media.err)}
		<ErrorMessage>Could not load media please try again</ErrorMessage>
	{:else}
		{#each media.ok as m (m.id)}
			<li class="relative">
				{#if mode === 'view'}
					<button
						type="button"
						class="h-full w-full"
						onclick={() => {
							onDetailsOpen?.(m);
						}}
					>
						{@render MediaItem(m)}
					</button>
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
	{/if}
</ul>

<style>
	li {
		transition: translate 150ms ease-out;
		translate: 0px 0px;

		@media (prefers-reduced-motion: no-preference) {
			&:is(:focus-visible, :hover) {
				translate: 0px -2px;
			}
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

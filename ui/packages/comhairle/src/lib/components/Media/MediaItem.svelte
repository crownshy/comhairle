<script lang="ts">
	import { capitalise } from '$lib/utils/casingUtils';
	import type { HTMLMediaElement } from '$lib/utils/types';
	import type { MediaDto } from '@crownshy/api-client/api';
	import { type Icon, Music, Video } from 'lucide-svelte';
	// FIX: Upgrade lucide icons to remove deprecated types
	import type { ComponentType } from 'svelte';
	import Badge from '$lib/components/ui/badge/badge.svelte';
	import './MediaBackground.css';

	interface Props extends MediaDto {
		type: HTMLMediaElement;
		alt: string;
	}

	const { type, alt, ...props }: Props = $props();
</script>

<div
	class="chequered-background border-muted-foreground container grid aspect-video min-h-48 w-full cursor-pointer grid-cols-1 grid-rows-[4fr_1fr] items-center overflow-hidden rounded-lg border-2"
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
			<img src={props.url} {alt} class="aspect-video max-h-32 object-contain" />
		{/if}
	</div>
	<div
		class="bg-card text-card-foreground flex flex-col items-start self-stretch px-5 py-2 text-xs"
		title={props.name}
	>
		<div class="grid grid-cols-[auto_auto] items-center justify-between self-stretch">
			<div class="truncate font-bold">
				{props.name}
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
		border-color: var(--selected, var(--color-accent, 'transparent'));
		transition: translate 150ms ease-out;
		translate: 0px 0px;

		&:is(:focus, :hover) {
			translate: 0px -3px;
		}
	}
</style>

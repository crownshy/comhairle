<script lang="ts">
	import ChevronLeft from 'lucide-svelte/icons/chevron-left';
	import ChevronRight from 'lucide-svelte/icons/chevron-right';
	import { Spinner } from '$lib/components/ui/spinner';
	import { cn } from '$lib/utils';
	import { PAGER_BUTTON_CLASS } from './pager';

	type Props = {
		direction: 'back' | 'forward';
		/** Forward shows it as text; back is an arrow, so it becomes the accessible name. */
		label: string;
		/** A link when given, so the browser's own back and new-tab work; a button otherwise. */
		href?: string;
		onclick?: () => void;
		disabled?: boolean;
		loading?: boolean;
		class?: string;
	};

	let {
		direction,
		label,
		href,
		onclick,
		disabled = false,
		loading = false,
		class: className
	}: Props = $props();

	let forward = $derived(direction === 'forward');
	let classes = $derived(cn(PAGER_BUTTON_CLASS, forward && 'ms-auto', className));
</script>

{#snippet inner()}
	{#if forward}
		{#if loading}
			<Spinner class="size-5" />
		{/if}
		<span class="text-base font-medium">{label}</span>
		<ChevronRight class="size-6 shrink-0 rtl:-scale-x-100" aria-hidden="true" />
	{:else}
		<ChevronLeft class="size-6 shrink-0 rtl:-scale-x-100" aria-hidden="true" />
	{/if}
{/snippet}

<!-- The hrefs come in already resolved by the caller. -->
<!-- eslint-disable svelte/no-navigation-without-resolve -->
{#if href && !disabled}
	<a {href} class={classes} aria-label={forward ? undefined : label}>
		{@render inner()}
	</a>
{:else}
	<button
		type="button"
		class={classes}
		aria-label={forward ? undefined : label}
		disabled={disabled || loading}
		aria-busy={loading}
		{onclick}
	>
		{@render inner()}
	</button>
{/if}

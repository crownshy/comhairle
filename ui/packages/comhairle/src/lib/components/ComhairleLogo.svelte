<script lang="ts">
	import IconLogo from '$lib/assets/comhairle_logo.svg';
	import FullLogo from '$lib/assets/comhairle_full_logo.svg';
	import WavesLogo from '$lib/assets/waves-logo-lg.png';
	import { themeStore } from '$lib/stores/theme.svelte';
	import { cn } from '$lib/utils';

	type Props = {
		href?: string;
		/** Names the link when the mark alone doesn't say where it goes. */
		ariaLabel?: string;
		showText?: boolean;
		logoSize?: 'sm' | 'md' | 'lg';
		color?: string;
		class?: string;
	};

	let {
		href = '/',
		ariaLabel,
		showText = true,
		logoSize = 'md',
		color = 'text-primary-foreground',
		class: className = ''
	}: Props = $props();

	const fullSizeMap = {
		sm: 'h-8 w-34',
		md: 'h-10 w-42',
		lg: 'h-12 w-52'
	};

	const iconSizeMap = {
		sm: 'h-8 w-8',
		md: 'h-10 w-10',
		lg: 'h-12 w-12'
	};

	let isWaves = $derived(themeStore.name === 'waves');
	let logoSrc = $derived(showText ? FullLogo : IconLogo);
	let sizeClass = $derived(showText ? fullSizeMap[logoSize] : iconSizeMap[logoSize]);
</script>

{#snippet logoContent()}
	{#if isWaves}
		<img
			src={WavesLogo}
			alt="Logo"
			class="inline-block h-12 shrink-0 overflow-hidden object-contain {sizeClass}"
		/>
	{:else}
		<span
			role="img"
			aria-label="Comhairle Logo"
			class="inline-block shrink-0 bg-current {sizeClass}"
			style="-webkit-mask-image: url({logoSrc}); mask-image: url({logoSrc}); -webkit-mask-size: contain; mask-size: contain; -webkit-mask-repeat: no-repeat; mask-repeat: no-repeat; -webkit-mask-position: center; mask-position: center;"
		></span>
	{/if}
{/snippet}

{#if href}
	<!-- eslint-disable-next-line svelte/no-navigation-without-resolve -->
	<a {href} aria-label={ariaLabel} class={cn('flex items-center', color, className)}>
		{@render logoContent()}
	</a>
{:else}
	<div class={cn('flex items-center', color, className)}>
		{@render logoContent()}
	</div>
{/if}

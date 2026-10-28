<script lang="ts">
	import { env } from '$env/dynamic/public';
	import IconLogo from '$lib/assets/comhairle_logo.svg';
	import FullLogo from '$lib/assets/comhairle_full_logo.svg';
	import WavesLogo from '$lib/assets/waves-logo-lg.png';
	import ScotGovLogo from '$lib/assets/Scottish_Government_Logo_White_text.svg';
	import ScotGovLogoDark from '$lib/assets/Scottish_Government_Logo_Dark_text.svg.webp';
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
		/** What the logo sits on, so brand logos with fixed colours pick the readable version. */
		surface?: 'dark' | 'light';
	};

	let {
		href = '/',
		ariaLabel,
		showText = true,
		logoSize = 'md',
		color = 'text-primary-foreground',
		class: className = '',
		surface = 'dark'
	}: Props = $props();

	const fullSizeMap = {
		sm: 'h-8 w-34',
		md: 'h-10 w-42',
		lg: 'h-12 w-52'
	};

	const brandHeightMap = {
		sm: 'h-7',
		md: 'h-8',
		lg: 'h-10'
	};

	const iconSizeMap = {
		sm: 'h-8 w-8',
		md: 'h-10 w-10',
		lg: 'h-12 w-12'
	};

	let customLogoUrl = env.PUBLIC_LOGO_URL;
	let isWaves = $derived(!customLogoUrl && themeStore.name === 'waves');
	let isScotGov = $derived(!customLogoUrl && themeStore.name === 'scot-gov');
	let scotGovSrc = $derived(surface === 'light' ? ScotGovLogoDark : ScotGovLogo);
	let logoSrc = $derived(showText ? FullLogo : IconLogo);
	let sizeClass = $derived(showText ? fullSizeMap[logoSize] : iconSizeMap[logoSize]);
</script>

{#snippet logoContent()}
	{#if customLogoUrl}
		<img
			src={customLogoUrl}
			alt="Logo"
			class="inline-block h-12 shrink-0 overflow-hidden object-contain {sizeClass}"
		/>
	{:else if isScotGov}
		<img
			src={scotGovSrc}
			alt="Scottish Government"
			class="inline-block w-auto shrink-0 {brandHeightMap[logoSize]}"
		/>
	{:else if isWaves}
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

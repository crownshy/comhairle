<script lang="ts">
	import { env } from '$env/dynamic/public';
	import IconLogo from '$lib/assets/comhairle_logo.svg';
	import FullLogo from '$lib/assets/comhairle_full_logo.svg';
	import WavesLogo from '$lib/assets/waves-logo-lg.png';
	import ScotGovLogo from '$lib/assets/scot-gov-logo-white-text.svg';
	import ScotGovLogoDark from '$lib/assets/scot-gov-logo-dark-text.webp';
	import { themeStore } from '$lib/stores/theme.svelte';
	import type { ThemeName } from '$lib/types/theme';
	import { cn } from '$lib/utils';

	type Props = {
		href?: string;
		/** Names the link when the mark alone doesn't say where it goes. */
		ariaLabel?: string;
		showText?: boolean;
		logoSize?: 'sm' | 'md' | 'lg';
		color?: string;
		class?: string;
		/**
		 * What the logo sits on, so brand logos with fixed colours pick the readable version.
		 * `page` is the page background, which follows light and dark mode.
		 */
		surface?: 'dark' | 'light' | 'page';
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

	/** An image logo with a version for light surfaces (dark text) and one for dark surfaces. */
	type BrandLogo = { light: string; dark: string; alt: string; fit: 'box' | 'height' };

	/** The configured logo is the light surface one, which emails use too. */
	function configuredLogo(): BrandLogo | undefined {
		const light = env.PUBLIC_LOGO_URL;
		if (!light) return undefined;
		return { light, dark: env.PUBLIC_LOGO_URL_DARK_SURFACE || light, alt: 'Logo', fit: 'box' };
	}

	function themeLogo(theme: ThemeName): BrandLogo | undefined {
		if (theme === 'scot-gov') {
			return {
				light: ScotGovLogoDark,
				dark: ScotGovLogo,
				alt: 'Scottish Government',
				fit: 'height'
			};
		}
		if (theme === 'waves') {
			return { light: WavesLogo, dark: WavesLogo, alt: 'Logo', fit: 'box' };
		}
		return undefined;
	}

	const configured = configuredLogo();
	let brand = $derived(configured ?? themeLogo(themeStore.name));
	let logoSrc = $derived(showText ? FullLogo : IconLogo);
	let sizeClass = $derived(showText ? fullSizeMap[logoSize] : iconSizeMap[logoSize]);
	let brandClass = $derived(
		brand?.fit === 'height'
			? `w-auto ${brandHeightMap[logoSize]}`
			: `overflow-hidden object-contain ${sizeClass}`
	);
</script>

{#snippet brandImage(src: string, extraClass = '')}
	<img {src} alt={brand?.alt} class={cn('inline-block shrink-0', brandClass, extraClass)} />
{/snippet}

{#snippet logoContent()}
	{#if brand}
		{#if surface === 'page' && brand.light !== brand.dark}
			<!-- Both render so the server markup is right before the client knows the mode. -->
			{@render brandImage(brand.light, 'dark:hidden')}
			{@render brandImage(brand.dark, 'hidden dark:inline-block')}
		{:else}
			{@render brandImage(surface === 'dark' ? brand.dark : brand.light)}
		{/if}
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

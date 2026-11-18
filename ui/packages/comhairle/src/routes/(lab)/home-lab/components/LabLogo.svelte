<script lang="ts">
	import FullLogo from '$lib/assets/comhairle_full_logo.svg';
	import WavesLogo from '$lib/assets/waves-logo-lg.png';
	// Prototype-only copy with the white parts turned Waves dark grey (#444749), for the light
	// bar. An official dark version from the council should replace it.
	import WavesLogoDark from '../media/waves-logo-dark.png';
	import ScotGovLogo from '$lib/assets/scot-gov-logo-white-text.svg';
	import ScotGovLogoDark from '$lib/assets/scot-gov-logo-dark-text.webp';
	import type { ThemeName } from '$lib/types/theme';

	// The same logo files the staging ComhairleLogo uses. That component reads the global theme
	// store, which the prototype only swaps after the page loads, so the top bar picks the logo
	// from the chosen organisation itself and shows the right one from the first paint.
	// `surface` is what the logo sits on: the theme-colour bar (dark) or the light hero.
	let { theme, surface = 'dark' }: { theme: ThemeName; surface?: 'dark' | 'light' } = $props();
</script>

{#if theme === 'scot-gov'}
	<img
		src={surface === 'light' ? ScotGovLogoDark : ScotGovLogo}
		alt="Scottish Government"
		class="h-8 w-auto"
	/>
{:else if theme === 'waves'}
	<img
		src={(surface === 'light' ? WavesLogoDark : WavesLogo) as string}
		alt="South Staffordshire Council"
		class="h-9 w-auto md:h-10"
	/>
{:else}
	<span
		role="img"
		aria-label="Comhairle"
		class="{surface === 'light'
			? 'text-foreground'
			: 'text-primary-foreground'} inline-block h-10 w-42 bg-current"
		style="mask: url({FullLogo}) left center / contain no-repeat; -webkit-mask: url({FullLogo}) left center / contain no-repeat;"
	></span>
{/if}

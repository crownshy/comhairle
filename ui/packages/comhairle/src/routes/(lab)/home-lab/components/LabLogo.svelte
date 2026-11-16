<script lang="ts">
	import FullLogo from '$lib/assets/comhairle_full_logo.svg';
	import WavesLogo from '$lib/assets/waves-logo-lg.png';
	import ScotGovLogo from '$lib/assets/scot-gov-logo-white-text.svg';
	import type { ThemeName } from '$lib/types/theme';

	// The same logo files the staging ComhairleLogo uses, in their versions for a dark surface,
	// because the top bar is the theme colour. That component reads the global theme store,
	// which the prototype only swaps after the page loads, so the top bar picks the logo from
	// the chosen organisation itself and shows the right one from the first paint.
	let { theme }: { theme: ThemeName } = $props();
</script>

{#if theme === 'scot-gov'}
	<img src={ScotGovLogo} alt="Scottish Government" class="h-8 w-auto" />
{:else if theme === 'waves'}
	<img src={WavesLogo} alt="South Staffordshire Council" class="h-9 w-auto md:h-10" />
{:else}
	<span
		role="img"
		aria-label="Comhairle"
		class="text-primary-foreground inline-block h-10 w-42 bg-current"
		style="mask: url({FullLogo}) left center / contain no-repeat; -webkit-mask: url({FullLogo}) left center / contain no-repeat;"
	></span>
{/if}

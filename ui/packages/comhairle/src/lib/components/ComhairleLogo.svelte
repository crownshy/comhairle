<script lang="ts">
	import Logo from '$lib/assets/comhairle_logo.svg';

	type Props = {
		href?: string;
		showText?: boolean;
		logoSize?: 'sm' | 'md' | 'lg';
		color?: string;
		class?: string;
	};

	let {
		href = '/',
		showText = true,
		logoSize = 'md',
		color = 'text-primary-foreground',
		class: className = ''
	}: Props = $props();

	const logoSizeMap = {
		sm: 'w-8 h-8',
		md: 'w-10 h-10',
		lg: 'h-[46px] w-auto'
	};
</script>

{#snippet logoContent()}
	<span
		role="img"
		aria-label="Comhairle Logo"
		class="inline-block shrink-0 bg-current {logoSizeMap[logoSize]}"
		style="-webkit-mask-image: url({Logo}); mask-image: url({Logo}); -webkit-mask-size: contain; mask-size: contain; -webkit-mask-repeat: no-repeat; mask-repeat: no-repeat; -webkit-mask-position: center; mask-position: center;"
	></span>
	{#if showText}
		<h1 class="text-xl font-bold">Comhairle</h1>
	{/if}
{/snippet}

{#if href}
	<a {href} class="flex items-center gap-3 {color} {className}">
		{@render logoContent()}
	</a>
{:else}
	<div class="flex items-center gap-3 {color} {className}">
		{@render logoContent()}
	</div>
{/if}

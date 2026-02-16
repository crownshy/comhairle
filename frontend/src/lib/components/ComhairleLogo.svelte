<script lang="ts">
	import Logo from '$lib/assets/comhairle_logo.svg';

	type Props = {
		href?: string;
		showText?: boolean;
		logoSize?: 'sm' | 'md' | 'lg';
		textClass?: string;
		class?: string;
	};

	let {
		href = '/',
		showText = true,
		logoSize = 'md',
		textClass = '',
		class: className = ''
	}: Props = $props();

	const logoSizeMap = {
		sm: 'w-8 h-8',
		md: 'w-10 h-10',
		lg: 'h-[46px] w-auto'
	};
</script>

{#if href}
	<a {href} class="flex items-center gap-3 {className}">
		<img class={logoSizeMap[logoSize]} src={Logo} alt="Comhairle Logo" />
		{#if showText}
			<h1 class="text-xl font-bold {textClass}">Comhairle</h1>
		{/if}
	</a>
{:else}
	<div class="flex items-center gap-3 {className}">
		<img class={logoSizeMap[logoSize]} src={Logo} alt="Comhairle Logo" />
		{#if showText}
			<h1 class="text-xl font-bold {textClass}">Comhairle</h1>
		{/if}
	</div>
{/if}

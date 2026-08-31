<script lang="ts">
	import ComhairleLogo from '$lib/components/ComhairleLogo.svelte';
	import StepDropdown from './StepDropdown.svelte';
	import StepProgressBar from './StepProgressBar.svelte';
	import type { StepItem } from './stepItems';

	let {
		steps,
		currentIndex,
		label,
		fill,
		legalLinks = [],
		count
	}: {
		steps: StepItem[];
		currentIndex: number;
		label: string;
		fill: number;
		legalLinks?: { href: string; label: string }[];
		/** Optional within-step count, e.g. Polis's "Opinion 3 of 12". */
		count?: string;
	} = $props();
</script>

<header class="bg-background sticky top-0 z-40 shrink-0 pb-2">
	<div class="mx-auto flex h-[72px] w-full max-w-5xl items-center gap-5 px-5 md:h-20 md:px-6">
		<ComhairleLogo
			href="/"
			showText={false}
			logoSize="sm"
			color="text-primary"
			class="size-9 shrink-0"
		/>
		<div class="flex min-w-0 flex-1 items-center justify-end gap-3">
			{#if count}
				<span class="text-muted-foreground shrink-0 text-sm md:text-base">{count}</span>
			{/if}
			<StepDropdown {steps} {currentIndex} {label} {legalLinks} />
		</div>
	</div>
	<div class="mx-auto w-full max-w-5xl">
		<StepProgressBar {steps} {currentIndex} {fill} />
	</div>
</header>

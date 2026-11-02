<script lang="ts">
	import type { Snippet } from 'svelte';
	import { m } from '$lib/paraglide/messages';
	import { cn } from '$lib/utils';

	type Props = {
		/** Height in rem the content is cut to until the reader expands it. */
		collapsedHeight: number;
		children: Snippet;
	};

	let { collapsedHeight, children }: Props = $props();

	const REM_IN_PIXELS = 16;
	let expanded = $state(false);
	let contentHeight = $state(0);
	let overflows = $derived(contentHeight > collapsedHeight * REM_IN_PIXELS);
	const contentId = $props.id();
</script>

<!-- Cut to the collapsed height from the first render, so long content doesn't jump once the
	page measures it. The button appears only after measuring shows something is hidden. -->
<div
	id={contentId}
	class={cn(
		'overflow-hidden',
		!expanded && overflows && '[mask-image:linear-gradient(to_bottom,black_75%,transparent)]'
	)}
	style:max-height={expanded ? undefined : `${collapsedHeight}rem`}
>
	<div bind:clientHeight={contentHeight}>
		{@render children()}
	</div>
</div>
{#if overflows}
	<button
		type="button"
		class="text-primary mt-2 text-lg font-semibold"
		aria-expanded={expanded}
		aria-controls={contentId}
		onclick={() => (expanded = !expanded)}
	>
		{expanded ? m.report_show_less() : m.report_show_more()}
	</button>
{/if}

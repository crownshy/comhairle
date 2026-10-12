<script lang="ts">
	import type { Snippet } from 'svelte';
	import { Info, TriangleAlert } from 'lucide-svelte';

	type Props = {
		/** The one-sentence message. */
		children: Snippet;
		/** Buttons or links on the right, e.g. "View now". */
		actions?: Snippet;
		/** warning: orange, something needs attention. info: blue, a suggestion or tip. */
		tone?: 'warning' | 'info';
	};

	let { children, actions, tone = 'warning' }: Props = $props();

	const TONES = {
		warning: { surface: 'bg-orange-50', icon: TriangleAlert, iconClass: 'text-orange-600' },
		info: { surface: 'bg-accent', icon: Info, iconClass: 'text-primary' }
	};
	let look = $derived(TONES[tone]);
	let Icon = $derived(look.icon);
</script>

<!-- A one-line message with its next step, at the top of a tab. -->
<div class="flex flex-wrap items-center justify-between gap-4 rounded-2xl px-5 py-4 {look.surface}">
	<div class="flex min-w-0 flex-[1_1_28rem] items-center gap-3">
		<Icon class="size-5 shrink-0 {look.iconClass}" aria-hidden="true" />
		<p class="text-base">{@render children()}</p>
	</div>
	{#if actions}
		<div class="flex flex-wrap gap-2">{@render actions()}</div>
	{/if}
</div>

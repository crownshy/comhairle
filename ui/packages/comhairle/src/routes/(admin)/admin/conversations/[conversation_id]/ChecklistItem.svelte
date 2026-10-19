<script lang="ts">
	import { ArrowUpRight, ChevronRight } from 'lucide-svelte';

	type Props = {
		title: string;
		detail: string;
		/** Built with resolve() by the caller. Without one the row is plain text. */
		href?: string | null;
		/** Shown in a circle before the title, for ordered steps. */
		number?: number;
		external?: boolean;
	};
	let { title, detail, href = null, number, external = false }: Props = $props();
</script>

<!-- Callers build href with resolve(). -->
<!-- eslint-disable svelte/no-navigation-without-resolve -->

{#snippet body()}
	{#if number !== undefined}
		<span
			class="bg-muted text-foreground flex size-7 shrink-0 items-center justify-center rounded-full text-sm font-semibold"
			>{number}</span
		>
	{/if}
	<span class="flex flex-1 flex-col gap-0.5">
		<span class="font-medium">{title}</span>
		<span class="text-muted-foreground text-sm">{detail}</span>
	</span>
{/snippet}

<li>
	{#if href}
		<a
			{href}
			target={external ? '_blank' : undefined}
			class="hover:bg-muted/60 flex items-center gap-4 px-5 py-4"
		>
			{@render body()}
			{#if external}
				<ArrowUpRight class="text-muted-foreground size-4 shrink-0" />
			{:else}
				<ChevronRight class="text-muted-foreground size-4 shrink-0" />
			{/if}
		</a>
	{:else}
		<div class="flex items-center gap-4 px-5 py-4">
			{@render body()}
		</div>
	{/if}
</li>

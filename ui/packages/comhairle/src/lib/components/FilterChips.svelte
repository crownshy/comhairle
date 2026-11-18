<script lang="ts" generics="T extends string">
	import { goto } from '$app/navigation';
	import { Button } from '$lib/components/ui/button';
	import * as Select from '$lib/components/ui/select';
	import { cn } from '$lib/utils';

	type Props = {
		/** The choices, in order. */
		options: readonly { value: T; label: string }[];
		/** The value that's selected now. */
		active: T;
		/** Builds the link for a value, usually the current URL with one query changed. */
		hrefFor: (value: T) => string;
		/** Accessible name for the group, e.g. "Filter conversations". */
		label: string;
	};

	let { options, active, hrefFor, label }: Props = $props();

	let activeLabel = $derived(options.find((option) => option.value === active)?.label);

	function selectFilter(value: string) {
		const option = options.find((candidate) => candidate.value === value);
		if (!option) return;
		// hrefFor only swaps a query on the current page's URL.
		// eslint-disable-next-line svelte/no-navigation-without-resolve
		goto(hrefFor(option.value), { noScroll: true });
	}
</script>

<!-- Chips wrap awkwardly in a narrow column, so below @lg they collapse into a dropdown. -->
<nav aria-label="Filter conversations" class="hidden @lg:block">
	<ul class="flex flex-wrap gap-2">
		{#each options as option (option.value)}
			{@const isActive = option.value === active}
			<li>
				<Button
					href={hrefFor(option.value)}
					data-sveltekit-noscroll
					aria-current={isActive ? 'page' : undefined}
					variant={isActive ? 'default' : 'ghost'}
					size="lg"
					class={cn(
						'border px-4 text-base',
						isActive ? 'border-primary' : 'bg-card text-foreground border-border'
					)}
				>
					{option.label}
				</Button>
			</li>
		{/each}
	</ul>
</nav>

<div class="@lg:hidden">
	<Select.Root type="single" value={active} onValueChange={selectFilter}>
		<Select.Trigger aria-label={label} class="bg-card h-10 w-full text-base">
			{activeLabel}
		</Select.Trigger>
		<Select.Content class="w-(--bits-select-anchor-width)">
			{#each options as option (option.value)}
				<Select.Item value={option.value} label={option.label} class="text-base" />
			{/each}
		</Select.Content>
	</Select.Root>
</div>

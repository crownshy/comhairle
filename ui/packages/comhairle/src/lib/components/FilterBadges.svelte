<script lang="ts">
	import QueryParams from '$lib/interfaces/QueryParams.svelte';
	import { cn } from '$lib/utils';

	type Props = {
		param: string;
		options: [value: string, label: string][];
		label: string;
		allLabel: string;
	};

	let { param, options, label, allLabel }: Props = $props();

	let chips = $derived<[value: string, label: string][]>([['', allLabel], ...options]);

	function select(value: string) {
		QueryParams.set(param, value).set('page', '1').update();
	}
</script>

<!-- One row that scrolls sideways on narrow screens, so every chip stays one tap away. -->
<nav aria-label={label} class="-mx-4 min-w-0 overflow-x-auto px-4 py-1 sm:mx-0 sm:px-0">
	<ul class="flex w-max gap-2">
		{#each chips as [value, chipLabel] (value)}
			{@const isActive = value === (QueryParams.get(param) ?? '')}
			<li>
				<button
					type="button"
					onclick={() => select(value)}
					aria-current={isActive ? 'page' : undefined}
					class={cn(
						'focus-visible:ring-ring flex h-9 items-center rounded-full border px-3.5 text-base whitespace-nowrap transition-colors focus-visible:ring-2 focus-visible:outline-none',
						isActive
							? 'bg-primary text-primary-foreground border-primary'
							: 'border-border bg-card hover:bg-muted'
					)}
				>
					{chipLabel}
				</button>
			</li>
		{/each}
	</ul>
</nav>

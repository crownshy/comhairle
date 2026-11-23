<script lang="ts">
	import * as Select from '$lib/components/ui/select';
	import { IsMobile } from '$lib/hooks/is-mobile.svelte';
	import QueryParams, { type ConversationsParams } from '$lib/interfaces/QueryParams.svelte';
	import { X } from '@lucide/svelte';
	import Badge from './ui/badge/badge.svelte';
	import Button from './ui/button/button.svelte';

	type Props = {
		param: string;
		options: [value: string, label: string][];
		label: string;
	};

	let { param, options, label }: Props = $props();

	const isMobile = new IsMobile();

	const PLACEHOLDER = 'Select a filter...';
</script>

<!-- Chips wrap awkwardly in a narrow column, so below @lg they collapse into a dropdown. -->
{#if !isMobile.current}
	<nav aria-label={label}>
		<ul class="flex flex-wrap gap-2">
			{#each options as [value, label] (value)}
				{@const isActive = value === QueryParams.get(param)}
				<li>
					<button
						type="button"
						onclick={() => QueryParams.set(param, isActive ? '' : value).update()}
						aria-current={isActive ? 'page' : undefined}
					>
						<Badge variant={isActive ? 'primary' : 'outline'}>
							{label}
						</Badge>
					</button>
				</li>
			{/each}
		</ul>
	</nav>
{:else}
	{@const selected = options.find((option) => option[0] === QueryParams.get(param))}
	<div class="flex items-center gap-2">
		<Select.Root
			type="single"
			value={QueryParams.get(param) as string}
			onValueChange={(value) => QueryParams.set(param, value).update()}
		>
			<Select.Trigger
				aria-label={label}
				class="bg-card h-10 text-base {!selected ? 'text-muted-foreground!' : ''}"
			>
				{selected?.[1] ?? PLACEHOLDER}
			</Select.Trigger>
			<Select.Content class="w-(--bits-select-anchor-width)">
				{#each options as [value, label] (value)}
					<Select.Item {value} {label} class="text-base" />
				{/each}
			</Select.Content>
		</Select.Root>
		<Button
			icon={X}
			title="Remove filter"
			aria-label="Remove filter"
			class="rounded-lg"
			variant="secondary"
			onclick={() => QueryParams.remove<ConversationsParams>('filter').update()}
		/>
	</div>
{/if}

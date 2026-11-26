<script lang="ts">
	import { Select } from '$lib/components/EasyForm';
	import { IsMobile } from '$lib/hooks/is-mobile.svelte';
	import QueryParams from '$lib/interfaces/QueryParams.svelte';
	import Badge from './ui/badge/badge.svelte';

	type Props = {
		param: string;
		/** Use an empty value for an "All" badge, which clears the filter. */
		options: [value: string, label: string][];
		label: string;
	};

	let { param, options, label }: Props = $props();

	const isMobile = new IsMobile();
</script>

<!-- Badges wrap awkwardly in a narrow column, so on phones they become a select that opens a sheet. -->
{#if !isMobile.current}
	<nav aria-label={label}>
		<ul class="flex flex-wrap gap-2">
			{#each options as [value, badgeLabel] (value)}
				{@const isActive = value === (QueryParams.get(param) ?? '')}
				<li>
					<button
						type="button"
						onclick={() =>
							QueryParams.set(param, isActive ? '' : value)
								.set('page', '1')
								.update()}
						aria-current={isActive ? 'page' : undefined}
					>
						<Badge
							variant={isActive ? 'primary' : 'outline'}
							class="h-9 px-3.5 text-base"
						>
							{badgeLabel}
						</Badge>
					</button>
				</li>
			{/each}
		</ul>
	</nav>
{:else}
	<Select
		id="{param}-filter"
		name={param}
		question={label}
		options={options.map(([value, optionLabel]) => ({ value, label: optionLabel }))}
		defaultSelected={(QueryParams.get(param) as string | undefined) ?? ''}
		onValueChange={(value) => QueryParams.set(param, value).set('page', '1').update()}
	/>
{/if}

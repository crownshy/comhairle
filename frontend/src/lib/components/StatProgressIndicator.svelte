<script lang="ts">
	import TrendingUpIcon from '@lucide/svelte/icons/trending-up';
	import * as Chart from '$lib/components/ui/chart/index.js';
	import * as Card from '$lib/components/ui/card/index.js';
	import { PieChart, Text } from 'layerchart';

	type Props = {
		title: string;
		varName: string;
		description: string;
		currentValue: number;
		entityType: string;
		message: string;
		total: number;
	};

	let { title, description, varName, currentValue, total, entityType, message }: Props = $props();

	const chartConfig = {
		main: { label: varName, color: 'var(--secondary)' }
	} satisfies Chart.ChartConfig;
</script>

<Card.Root class="flex flex-col">
	<Card.Header class="items-center">
		<Card.Title>{title}</Card.Title>
		<Card.Description>{description}</Card.Description>
	</Card.Header>
	<Card.Content class="">
		<Chart.Container config={chartConfig} class="mx-auto aspect-square max-h-[250px]">
			<PieChart
				data={[{ [varName]: (currentValue * 100) / total, color: chartConfig.main.color }]}
				key="platform"
				value={varName}
				c="color"
				innerRadius={76}
				padding={29}
				range={[-90, 90]}
				props={{ pie: { sort: null } }}
				cornerRadius={4}
			>
				{#snippet aboveMarks()}
					<Text
						value={currentValue}
						textAnchor="middle"
						verticalAnchor="middle"
						class="fill-foreground text-2xl! font-bold"
						dy={-24}
					/>
					<Text
						value={entityType}
						textAnchor="middle"
						verticalAnchor="middle"
						class="fill-muted-foreground! text-muted-foreground"
						dy={-4}
					/>
				{/snippet}
			</PieChart>
		</Chart.Container>
	</Card.Content>
	<Card.Footer class="flex-col gap-2 text-sm">
		<div class="flex items-center gap-2 leading-none font-medium">
			{((currentValue * 100) / total).toLocaleString(undefined, { minimumFractionDigits: 0 })}% of {entityType}
			({currentValue} out of {total}) have {message}
		</div>
	</Card.Footer>
</Card.Root>

<script lang="ts">
	import { ChevronUp, ChevronRight } from 'lucide-svelte';

	export type ScatterPoint = {
		id: string;
		x: number;
		y: number;
	};

	type Props = {
		xAxisLabel: string;
		yAxisLabel: string;
		xDomain: [number, number];
		yDomain: [number, number];
		height?: string;
		points: ScatterPoint[];
	};

	let { xAxisLabel, yAxisLabel, xDomain, yDomain, height = '50vh', points }: Props = $props();

	function toLeftPercent(x: number, [min, max]: readonly [number, number]) {
		return ((x - min) / (max - min)) * 100;
	}

	function toTopPercent(y: number, [min, max]: readonly [number, number]) {
		return 100 - ((y - min) / (max - min)) * 100;
	}
</script>

<article class="relative w-full" style="height: {height}">
	<div class="grid h-full grid-cols-[85%_1fr] gap-4">
		<div class="h-full">
			<div class="mb-2 flex justify-center">
				<span>{yAxisLabel}</span>
			</div>
			<div class="h-full w-full px-4 py-4">
				<div class="relative h-[90%] w-full">
					<!-- y axis -->
					<div
						class="bg-muted-foreground/50 absolute top-0 left-1/2 h-full w-0.5 -translate-x-1/2"
					>
						<ChevronUp class="text-muted-foreground/50 absolute -top-2.5 -left-2.75" />
					</div>

					<!-- x axis -->
					<div
						class="bg-muted-foreground/50 absolute top-1/2 left-0 h-0.5 w-full -translate-y-1/2"
					>
						<ChevronRight
							class="text-muted-foreground/50 absolute -top-2.75 -right-2.5"
						/>
					</div>

					<!-- points -->
					{#each points as point (point.id)}
						<div
							class="border-primary absolute h-4 w-4 -translate-x-1/2 -translate-y-1/2 rounded-full border-2 bg-transparent"
							style="left: {toLeftPercent(point.x, xDomain)}%; top: {toTopPercent(
								point.y,
								yDomain
							)}%"
						></div>
					{/each}
				</div>
			</div>
		</div>
		<div class="flex items-center">
			<span>{xAxisLabel}</span>
		</div>
	</div>
</article>

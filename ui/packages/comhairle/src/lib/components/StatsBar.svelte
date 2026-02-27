<script lang="ts">
	type Stat = {
		name: string;
		amount: number;
		unit: string | null;
		change?: string;
		from?: string;
	};
	type Props = {
		stats: Array<Stat>;
	};

	let { stats }: Props = $props();
</script>

<div class="bg-sidebar mb-5 inline-flex w-full items-start justify-around gap-6 rounded-3xl p-6">
	{#each stats as stat, index (stat.name)}
		<div
			class="inline-flex flex-1 flex-col items-start class:border-l-1={index >
				0} justify-start gap-1 pl-5"
		>
			<div class="inline-flex items-center justify-center gap-2 self-stretch">
				<div
					class="flex-1 justify-start font-['Inter'] text-sm font-medium leading-tight text-lime-50"
				>
					{stat.name}
				</div>
			</div>

			<div
				class="justify-start self-stretch font-['Inter'] text-3xl font-semibold leading-9 text-lime-50"
			>
				{stat.amount}
				{stat.unit}
			</div>
			{#if stat.change}
				<div
					class="justify-start self-stretch font-['Inter'] text-sm font-semibold leading-tight text-stone-400"
				>
					{stat.change}
					{stat.from}
				</div>
			{/if}
		</div>
	{/each}
</div>

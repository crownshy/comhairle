<script lang="ts">
	type Props = {
		superText: string;
		metric: number;
		subText: string;
	};

	let { superText, metric, subText }: Props = $props();
</script>

<article class="bg-card flex w-fit min-w-48 flex-col gap-1 rounded-md border p-4">
	<p class="text-sm">{superText}</p>
	<p class="text-4xl font-bold">{metric}</p>
	<p class="text-xs">{subText}</p>
</article>

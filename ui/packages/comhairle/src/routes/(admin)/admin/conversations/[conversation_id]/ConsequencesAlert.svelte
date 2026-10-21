<script lang="ts">
	import { Alert, AlertTitle, AlertDescription } from '$lib/components/ui/alert';
	import { TriangleAlert } from 'lucide-svelte';

	type Props = { title: string; consequences: string[] };
	let { title, consequences }: Props = $props();
</script>

<!-- The themes have no warning token yet, so the orange is set here once. -->
<Alert class="border-orange-300 bg-orange-50 dark:border-orange-400/40 dark:bg-orange-400/10">
	<TriangleAlert class="text-orange-500" />
	<AlertTitle>{title}</AlertTitle>
	<AlertDescription>
		<ul class="list-disc space-y-1 pl-4">
			{#each consequences as consequence (consequence)}
				<li>{consequence}</li>
			{/each}
		</ul>
	</AlertDescription>
</Alert>

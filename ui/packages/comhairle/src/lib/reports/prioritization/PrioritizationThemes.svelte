<script lang="ts">
	import type { PublicTheme } from '@crownshy/api-client/api';
	import { Badge } from '$lib/components/ui/badge';

	/**
	 * The step's themes with how many answers each groups. The answers stay with the
	 * organisers, so readers only get the count (ADR-0061).
	 */
	let { themes }: { themes: PublicTheme[] } = $props();

	const answerCount = (count: number) => `${count} ${count === 1 ? 'answer' : 'answers'}`;
</script>

{#if themes.length === 0}
	<p class="text-muted-foreground text-base">No themes have been generated for this step yet.</p>
{:else}
	<ul class="flex flex-col gap-2">
		<!-- Themes carry no id in the public data, and titles can repeat. -->
		{#each themes as theme, index (index)}
			<li class="bg-card flex flex-col gap-1 rounded-xl border p-4">
				<span class="text-base font-semibold">
					{theme.title}
					<Badge variant="secondary">{answerCount(theme.contributions)}</Badge>
				</span>
				<span class="text-muted-foreground text-base">{theme.description}</span>
			</li>
		{/each}
	</ul>
{/if}

<script lang="ts">
	import * as Card from '$lib/components/ui/card/index.js';
	import type { PublicStepResults } from '$lib/reports/embeds';

	type Props = {
		/** What the summary and body embed right now, including unsaved typing. */
		results: PublicStepResults[];
		reportIsPublic: boolean;
	};

	let { results, reportIsPublic }: Props = $props();
</script>

<Card.Root data-tour="report-public-results">
	<Card.Header>
		<Card.Title>{reportIsPublic ? 'Public now' : 'Public when you publish'}</Card.Title>
		<Card.Description class="text-base">
			Only the results embedded in this report are shared. Everything else from your steps
			stays private.
		</Card.Description>
	</Card.Header>
	<Card.Content>
		{#if results.length === 0}
			<p class="text-muted-foreground text-base">
				Nothing yet. Results you embed in the summary or body will be listed here.
			</p>
		{:else}
			<ul class="flex flex-col gap-2">
				{#each results as step (step.stepId)}
					<li class="text-base">
						<span class="font-medium">{step.stepName}:</span>
						{step.sectionBlocks.map((block) => block.label).join(', ')}
					</li>
				{/each}
			</ul>
		{/if}
	</Card.Content>
</Card.Root>

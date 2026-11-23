<script lang="ts">
	import { Badge } from '$lib/components/ui/badge';
	import * as Accordion from '$lib/components/ui/accordion';
	import type { GroupSize } from '$lib/tools/polis/reportTypes';
	import StatementVoteBlock from './StatementVoteBlock.svelte';
	import type { PolisEmbedTheme } from './embedData';

	/**
	 * The step's themes as the published report shows them, each opening onto the accepted
	 * statements filed under it with their vote bars.
	 */
	type Props = {
		themes: PolisEmbedTheme[];
		groups: GroupSize[];
	};

	let { themes, groups }: Props = $props();

	const statementCount = (count: number) =>
		`${count} ${count === 1 ? 'statement' : 'statements'}`;
</script>

{#if themes.length === 0}
	<p class="text-muted-foreground text-base">No themes have been generated for this step yet.</p>
{:else}
	<Accordion.Root type="multiple" class="w-full space-y-2">
		<!-- Themes carry no id in the public data, and titles can repeat. -->
		{#each themes as theme, index (index)}
			<Accordion.Item value={String(index)} class="bg-card rounded-xl border px-4">
				<Accordion.Trigger class="py-3 hover:no-underline">
					<span class="flex flex-col items-start gap-1 text-left">
						<span class="text-base font-semibold">
							{theme.title}
							<Badge variant="secondary"
								>{statementCount(theme.statements.length)}</Badge
							>
						</span>
						<span class="text-muted-foreground text-base font-normal">
							{theme.description}
						</span>
					</span>
				</Accordion.Trigger>
				<Accordion.Content class="flex flex-col gap-3 pb-4">
					{#each theme.statements as statement (statement.tid)}
						<StatementVoteBlock comment={statement} {groups} />
					{:else}
						<p class="text-muted-foreground text-base">
							None of this theme's statements are in the published report.
						</p>
					{/each}
				</Accordion.Content>
			</Accordion.Item>
		{/each}
	</Accordion.Root>
{/if}

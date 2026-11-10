<script
	lang="ts"
	generics="Theme extends { title: string; description: string; contributions: number }"
>
	import type { Snippet } from 'svelte';
	import { Badge } from '$lib/components/ui/badge';
	import * as Accordion from '$lib/components/ui/accordion';

	/**
	 * A step's themes as the published report shows them: title, description and how many
	 * contributions each groups. A tool that may show the contributions themselves passes
	 * them through `details`; the others only show the count.
	 */
	type Props = {
		themes: Theme[];
		/** What the count counts, such as statements or summaries. */
		noun: { one: string; other: string };
		details?: Snippet<[Theme]>;
	};

	let { themes, noun, details }: Props = $props();

	const countLabel = (count: number) => `${count} ${count === 1 ? noun.one : noun.other}`;
</script>

{#if themes.length === 0}
	<p class="text-muted-foreground text-base">No themes have been generated for this step yet.</p>
{:else if details}
	<Accordion.Root type="multiple" class="w-full space-y-2">
		{#each themes as theme, index (index)}
			<Accordion.Item value={String(index)} class="bg-card rounded-xl border px-4">
				<Accordion.Trigger class="py-3 hover:no-underline">
					<span class="flex flex-col items-start gap-1 text-left">
						<span class="text-base font-semibold">
							{theme.title}
							<Badge variant="secondary">{countLabel(theme.contributions)}</Badge>
						</span>
						<span class="text-muted-foreground text-base font-normal">
							{theme.description}
						</span>
					</span>
				</Accordion.Trigger>
				<Accordion.Content class="flex flex-col gap-3 pb-4">
					{@render details(theme)}
				</Accordion.Content>
			</Accordion.Item>
		{/each}
	</Accordion.Root>
{:else}
	<ul class="flex flex-col gap-2">
		{#each themes as theme, index (index)}
			<li class="bg-card flex flex-col gap-1 rounded-xl border p-4">
				<span class="text-base font-semibold">
					{theme.title}
					<Badge variant="secondary">{countLabel(theme.contributions)}</Badge>
				</span>
				<span class="text-muted-foreground text-base">{theme.description}</span>
			</li>
		{/each}
	</ul>
{/if}

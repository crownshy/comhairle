<script lang="ts">
	import { page } from '$app/state';
	import { FlaskConical } from 'lucide-svelte';
	import { cn } from '$lib/utils';
	import { DATASETS, ORGANISATIONS, readScenario, scenarioQuery } from '../scenario';

	const scenario = $derived(readScenario(page.url));
	const hrefWith = (patch: Parameters<typeof scenarioQuery>[1]) =>
		page.url.pathname + scenarioQuery(page.url, patch);

	const chip = (active: boolean) =>
		cn(
			'flex h-10 items-center rounded-full border px-3 text-base',
			active ? 'bg-foreground text-background border-foreground' : 'border-border'
		);
</script>

<!-- Links are resolve() plus the scenario query string, which this rule can't see. -->
<!-- eslint-disable svelte/no-navigation-without-resolve -->

<!-- Testing aid only: not part of the design being evaluated. -->
<details
	class="bg-card text-card-foreground border-border fixed bottom-4 left-4 z-50 max-w-[calc(100vw-2rem)] rounded-2xl border shadow-lg"
>
	<summary class="flex h-12 cursor-pointer items-center gap-2 px-4 text-base font-semibold">
		<FlaskConical class="size-5" /> Prototype scenario
	</summary>
	<div class="flex flex-col gap-4 px-4 pb-4">
		<fieldset class="flex flex-col gap-2">
			<legend class="text-muted-foreground pb-2 text-base">Visitor</legend>
			<div class="flex flex-wrap gap-2">
				<a href={hrefWith({ loggedIn: false })} class={chip(!scenario.loggedIn)}
					>Logged out</a
				>
				<a href={hrefWith({ loggedIn: true })} class={chip(scenario.loggedIn)}>Logged in</a>
			</div>
		</fieldset>
		<fieldset class="flex flex-col gap-2">
			<legend class="text-muted-foreground pb-2 text-base">Conversations</legend>
			<div class="flex flex-wrap gap-2">
				{#each DATASETS as dataset (dataset.value)}
					<a
						href={hrefWith({ dataset: dataset.value })}
						class={chip(scenario.dataset === dataset.value)}>{dataset.label}</a
					>
				{/each}
			</div>
		</fieldset>
		<fieldset class="flex flex-col gap-2">
			<legend class="text-muted-foreground pb-2 text-base">Organisation</legend>
			<div class="flex flex-wrap gap-2">
				{#each ORGANISATIONS as org (org.id)}
					<a href={hrefWith({ orgId: org.id })} class={chip(scenario.org.id === org.id)}
						>{org.name}</a
					>
				{/each}
			</div>
		</fieldset>
	</div>
</details>

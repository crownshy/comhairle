<script lang="ts">
	import * as Accordion from '$lib/components/ui/accordion';
	import AccordionItem from '$lib/components/ui/accordion/accordion-item.svelte';

	let { steps, currentStep }: { steps: any; currentStep: any } = $props();
</script>

<Accordion.Root>
	{#each steps as step}
		<Accordion.Item value={`step-${step.step_order}`}>
			<Accordion.Trigger><span>{step.step_order}</span>{step.name}</Accordion.Trigger>
			<Accordion.Content>
				<p>{step.description}</p>
				<a href="#">Link</a>
			</Accordion.Content>
		</Accordion.Item>
	{/each}
</Accordion.Root>

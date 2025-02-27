<script lang="ts">
	import PolisEmbed from '$lib/components/PolisEmbed.svelte';
	import ProcessDates from '$lib/components/ProcessDates.svelte';
	import * as Accordion from '$lib/components/ui/accordion';
	import AccordionItem from '$lib/components/ui/accordion/accordion-item.svelte';
	import * as Breadcrumb from '$lib/components/ui/breadcrumb/index.js';
	import { Accessibility } from 'svelte-radix';
	let { data } = $props();
	let { conversation, step } = data;
</script>

{#if conversation && step}
	<Breadcrumb.Root class="mb-16">
		<Breadcrumb.List>
			<Breadcrumb.Item>
				<Breadcrumb.Link href="/">Home</Breadcrumb.Link>
			</Breadcrumb.Item>
			<Breadcrumb.Separator />
			<Breadcrumb.Item>
				<Breadcrumb.Link href="/conversations">Conversations</Breadcrumb.Link>
			</Breadcrumb.Item>
			<Breadcrumb.Separator />
			<Breadcrumb.Item>
				<Breadcrumb.Page>{conversation.name}</Breadcrumb.Page>
			</Breadcrumb.Item>
			<Breadcrumb.Separator />
			<Breadcrumb.Item>
				<Breadcrumb.Page>{step.title}</Breadcrumb.Page>
			</Breadcrumb.Item>
		</Breadcrumb.List>
	</Breadcrumb.Root>

	<div class="grid w-full grid-cols-5 gap-8">
		<h1 class="col-start-1 col-end-4 row-start-1 row-end-1 text-8xl font-bold">
			Input to the consulation
		</h1>
		<h2 class="col-start-1 col-end-4 row-start-2 row-end-2 text-xl font-bold">
			Crowd Sourced Survey
		</h2>
		<div class="col-start-1 col-end-4 row-start-3">
			<p>
				Polis is a real-time system for gathering, analyzing and understanding what large groups of
				people think in their own words, enabled by advanced statistics and machine learning.
			</p>
			<div class="h-[1200px]">
				<PolisEmbed polis_id={step.tool_id} />
			</div>
		</div>
		<div class="col-start-4 col-end-6 row-start-3 w-full">
			<ProcessDates startDate={new Date(2025, 1, 1)} endDate={new Date(2025, 1, 28)} />
			<div class="b-green-950 mt-2 border-b-2 border-t-4 p-4 text-xl font-bold">
				Part of {conversation.name}: step by step
			</div>
			<div>
				<Accordion.Root>
					<Accordion.Item value="step-1">
						<Accordion.Trigger><span>1</span>Learn</Accordion.Trigger>
						<Accordion.Content>
							<a href="#">Link</a>
						</Accordion.Content>
					</Accordion.Item>
					<Accordion.Item value="step-2">
						<Accordion.Trigger>
							<span>2</span> Explore the opinion landscape
						</Accordion.Trigger>
						<Accordion.Content>
							<a href="#">Link</a>
						</Accordion.Content>
					</Accordion.Item>
					<Accordion.Item value="step-3">
						<Accordion.Trigger>
							<span>3</span> Summary
						</Accordion.Trigger>
						<Accordion.Content>
							<a href="../report">Link</a>
						</Accordion.Content>
					</Accordion.Item>
				</Accordion.Root>
			</div>
		</div>
	</div>
{:else}
	<h1>Failed to find conversation</h1>
{/if}

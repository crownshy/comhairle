<script lang="ts">
	/**
	 * The way back into a finished flow: every step the participant went through, in order,
	 * as stations on one rail. The ones still open are the ones they can change.
	 */
	/* Step links are built by $lib/urls, not from a typed route id, so resolve() has nothing
	   to resolve. */
	/* eslint-disable svelte/no-navigation-without-resolve */
	import { Check, ChevronRight, Lock, type Icon } from 'lucide-svelte';
	import type {
		LocalizedConversationDto,
		LocalizedWorkflowStepDto
	} from '@crownshy/api-client/api';
	import { conversation_url, thank_you_page, workflow_step_url } from '$lib/urls';
	import StepChrome from '$lib/components/participant/StepChrome.svelte';
	import type { StepItem } from '$lib/components/participant/stepItems';
	import { learningAssistantAvailable } from '$lib/components/LearningAssistant/availability';
	import { stepMeta, toMetaToolConfig } from '$lib/step-brief/slideMeta';
	import { TOOL_META, type ToolType } from '$lib/tool_meta';
	import { cn } from '$lib/utils';
	import * as m from '$lib/paraglide/messages';

	let {
		conversation,
		steps,
		workflowId,
		preview,
		hasKnowledgeBaseDocs,
		queryString
	}: {
		conversation: LocalizedConversationDto;
		/** Every step of the workflow, in order, not only the revisitable ones. */
		steps: LocalizedWorkflowStepDto[];
		workflowId: string;
		preview: boolean;
		hasKnowledgeBaseDocs: boolean;
		/** Carried onto every link so embed mode and the like survive the hop. */
		queryString: string;
	} = $props();

	type Station = {
		id: string;
		name: string;
		icon?: typeof Icon;
		/**
		 * The step's own meta labels, the ones its cover quotes. The duration leads; the
		 * counts after it are detail the rail only has room for on a wide screen.
		 */
		meta: string[];
		open: boolean;
		href?: string;
	};

	let assistantAvailable = $derived(
		learningAssistantAvailable(conversation, hasKnowledgeBaseDocs)
	);

	let introUrl = $derived(conversation_url(conversation.id, preview) + queryString);

	let stations = $derived<Station[]>(
		steps.map((step) => {
			const config = toMetaToolConfig(step.toolConfig ?? step.previewToolConfig);
			const type = config?.type as ToolType | undefined;
			return {
				id: step.id,
				name: step.name,
				icon: type ? TOOL_META[type]?.icon : undefined,
				meta: stepMeta(config).map((item) => item.label),
				open: step.canRevisit,
				href: step.canRevisit
					? workflow_step_url(conversation.id, workflowId, step.id, preview) + queryString
					: undefined
			};
		})
	);

	// Everything is behind the participant, the way the thank-you page reads it: a step they
	// can still open is offered as completed, one they cannot as completed and locked.
	let chromeSteps = $derived<StepItem[]>([
		{
			id: 'landing',
			name: m.landing_before_you_start(),
			status: 'completed',
			href: introUrl,
			isIntro: true
		},
		...stations.map((station) => ({
			id: station.id,
			name: station.name,
			status: (station.open ? 'completed' : 'completed-locked') as StepItem['status'],
			href: station.href
		}))
	]);
</script>

{#snippet station(item: Station)}
	{@const StationIcon = item.icon}
	<span
		class={cn(
			'relative z-10 flex size-12 shrink-0 items-center justify-center rounded-full',
			item.open ? 'bg-primary text-primary-foreground' : 'bg-accent text-accent-foreground'
		)}
		aria-hidden="true"
	>
		{#if StationIcon}
			<StationIcon class="size-5" />
		{:else}
			<Check class="size-5" />
		{/if}
	</span>

	<span class="flex min-w-0 flex-1 flex-col">
		<span class={cn('truncate text-base font-medium', !item.open && 'text-muted-foreground')}>
			{item.name}
		</span>
		<span class="text-muted-foreground flex min-w-0 items-center gap-2 text-base">
			{#if item.open}
				<span class="text-primary shrink-0 font-medium">
					{m.conversations_return_open_again()}
				</span>
			{:else}
				<Lock class="size-4 shrink-0" aria-hidden="true" />
				<span class="shrink-0">{m.conversations_return_closed()}</span>
			{/if}
			{#if item.meta.length}
				<span class="shrink-0" aria-hidden="true">·</span>
				<span class="shrink-0">{item.meta[0]}</span>
			{/if}
			{#each item.meta.slice(1) as label (label)}
				<span class="hidden shrink-0 md:inline" aria-hidden="true">·</span>
				<span class="hidden truncate md:inline">{label}</span>
			{/each}
		</span>
	</span>

	{#if item.open}
		<ChevronRight class="text-muted-foreground size-5 shrink-0" aria-hidden="true" />
	{:else}
		<Check class="text-muted-foreground size-5 shrink-0" aria-hidden="true" />
	{/if}
{/snippet}

<!-- The way back in is still the flow: same chrome, same one screen as the steps and the
	thank-you page it sits between. -->
<div class="grid h-[100dvh] grid-cols-[minmax(0,1fr)] grid-rows-[auto_1fr] overflow-hidden">
	<StepChrome
		steps={chromeSteps}
		currentIndex={steps.length}
		label={m.jump_back_in()}
		fill={1}
		{assistantAvailable}
		{introUrl}
		{preview}
	/>

	<main class="flex min-h-0 w-full flex-col overflow-y-auto mask-b-from-[calc(100%-2.5rem)]">
		<div class="mx-auto flex w-full max-w-2xl flex-col gap-8 px-5 pt-[5vh] pb-16 md:px-6">
			<div class="flex flex-col gap-2">
				<span class="text-muted-foreground text-base">{conversation.title}</span>
				<h1 class="text-3xl leading-tight font-bold md:text-4xl">
					{m.conversations_return_heading()}
				</h1>
				<p class="text-muted-foreground text-base">{m.conversations_return_message()}</p>
			</div>

			<!-- A rail rather than a list of cards: the line is the journey, and each step is a
				station on it that is either still open or closed behind them. -->
			<ol class="relative flex flex-col">
				<span
					class="bg-accent absolute top-6 bottom-6 left-6 w-0.5 -translate-x-1/2"
					aria-hidden="true"
				></span>
				{#each stations as item (item.id)}
					<li class="relative">
						{#if item.open}
							<a
								href={item.href}
								class="hover:bg-accent/60 flex items-center gap-4 rounded-2xl py-3 pr-3 transition-colors"
							>
								{@render station(item)}
							</a>
						{:else}
							<div class="flex items-center gap-4 rounded-2xl py-3 pr-3">
								{@render station(item)}
							</div>
						{/if}
					</li>
				{/each}
			</ol>

			<a
				class="text-muted-foreground self-start text-base underline underline-offset-4"
				href={thank_you_page(conversation.id, workflowId, preview) + queryString}
			>
				{m.conversations_return_summary_link()}
			</a>
		</div>
	</main>
</div>

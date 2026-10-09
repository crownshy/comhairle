<script lang="ts">
	import { BookOpen, ClipboardList, Plus, Settings2, Trash2 } from 'lucide-svelte';
	import { Badge } from '$lib/components/ui/badge';
	import { Button } from '$lib/components/ui/button';
	import {
		askedCount,
		createStep,
		overrideCount,
		requiredCount,
		versionLabel,
		type DemographicStep,
		type StepSettings
	} from '../../conversationStep';
	import { createInitialForms, createInitialQuestions } from '../../demographicPrototypeData';
	import AddDemographicStepDialog from './AddDemographicStepDialog.svelte';
	import ConfigureStepSheet from './ConfigureStepSheet.svelte';

	// Static prototype: forms and questions are in-memory mock data, nothing is saved.
	const forms = createInitialForms();
	const questions = createInitialQuestions();

	const tabs = ['Overview', 'Process design', 'Knowledge', 'Events', 'Reach', 'Report'];

	let steps = $state<DemographicStep[]>([]);
	let adding = $state(false);
	let configuringId = $state<string | null>(null);
	let counter = 0;

	const configuring = $derived(steps.find((step) => step.id === configuringId));

	function formFor(step: DemographicStep) {
		return forms.find((form) => form.id === step.formId) ?? forms[0];
	}

	function addStep(formId: string, settings: StepSettings) {
		const form = forms.find((candidate) => candidate.id === formId);
		if (!form) return;
		counter += 1;
		steps.push(createStep(`step-${counter}`, form, settings));
		adding = false;
	}

	function saveStep(updated: DemographicStep) {
		const index = steps.findIndex((step) => step.id === updated.id);
		if (index >= 0) steps[index] = updated;
		configuringId = null;
	}

	function removeStep(id: string) {
		steps = steps.filter((step) => step.id !== id);
	}
</script>

<div class="flex min-h-full flex-col">
	<header
		class="border-border bg-background md:pl-gutter flex w-full shrink-0 items-center justify-between border-b py-2 pr-3 pl-14 md:pr-6"
	>
		<h1
			class="text-primary max-w-[22ch] truncate text-lg leading-7 font-semibold sm:max-w-[40ch]"
		>
			Hybrid works
		</h1>
		<Badge variant="primary">Prototype</Badge>
	</header>

	<nav
		class="border-border bg-background md:px-gutter flex shrink-0 gap-6 overflow-x-auto border-b px-4"
		aria-label="Conversation sections (static)"
	>
		{#each tabs as tab (tab)}
			<span
				class="py-3 text-base whitespace-nowrap {tab === 'Process design'
					? 'border-primary text-foreground border-b-2 font-medium'
					: 'text-muted-foreground'}"
				aria-current={tab === 'Process design' ? 'page' : undefined}
			>
				{tab}
			</span>
		{/each}
	</nav>

	<div class="bg-muted flex min-h-0 w-full flex-1 overflow-hidden">
		<div class="px-gutter pt-page-top flex w-full max-w-5xl flex-col gap-4 pb-8">
			<div class="flex flex-wrap items-center justify-between gap-3">
				<div class="flex flex-col">
					<h2 class="text-2xl font-bold">Process steps</h2>
					<p class="text-muted-foreground text-base">
						Design and configure your engagement, one step at a time.
					</p>
				</div>
				<div class="flex gap-2">
					<Button variant="outline" disabled title="Not part of this prototype">
						<Plus class="size-4" />Add step
					</Button>
					<Button onclick={() => (adding = true)}>
						<Plus class="size-4" />Add demographic step
					</Button>
				</div>
			</div>

			<ol class="flex flex-col gap-3">
				<li
					class="border-border bg-card flex items-center gap-4 rounded-xl border px-5 py-4"
				>
					<BookOpen class="text-primary size-6 shrink-0" />
					<div class="flex min-w-0 flex-1 flex-col">
						<span class="text-lg font-medium">About the topic</span>
						<span class="text-muted-foreground text-base">Learn</span>
					</div>
				</li>

				{#each steps as step (step.id)}
					{@const form = formFor(step)}
					{@const overrides = overrideCount(step, form)}
					<li
						class="border-border bg-card flex flex-wrap items-center gap-4 rounded-xl border px-5 py-4"
					>
						<ClipboardList class="text-primary size-6 shrink-0" />
						<div class="flex min-w-0 flex-1 flex-col gap-1">
							<span class="text-lg font-medium">{step.name}</span>
							<span class="text-muted-foreground text-base">
								Demographic · {form.name}
							</span>
							<span class="flex flex-wrap items-center gap-2">
								<Badge variant={step.policy === 'pinned' ? 'draft' : 'default'}>
									{versionLabel(step, form)}
								</Badge>
								<span class="text-base">
									{askedCount(step)} asked · {requiredCount(step)} required
								</span>
								{#if overrides > 0}
									<Badge variant="primary">
										{overrides}
										{overrides === 1 ? 'override' : 'overrides'}
									</Badge>
								{/if}
								{#if step.mustComplete}
									<Badge variant="secondary">Must complete</Badge>
								{/if}
								{#if step.allowReturn}
									<Badge variant="secondary">Can return</Badge>
								{/if}
							</span>
						</div>
						<div class="flex gap-2">
							<Button variant="outline" onclick={() => (configuringId = step.id)}>
								<Settings2 class="size-4" />Configure
							</Button>
							<Button
								variant="ghost"
								size="icon"
								aria-label={`Remove ${step.name}`}
								onclick={() => removeStep(step.id)}
							>
								<Trash2 class="size-4" />
							</Button>
						</div>
					</li>
				{/each}
			</ol>

			{#if steps.length === 0}
				<p class="text-muted-foreground text-base">
					Add a demographic step to try choosing a form and configuring which questions it
					asks. Only the Add demographic step button works in this prototype.
				</p>
			{/if}
		</div>
	</div>
</div>

{#if adding}
	<AddDemographicStepDialog
		{forms}
		{questions}
		onAdd={addStep}
		onClose={() => (adding = false)}
	/>
{/if}

{#if configuring}
	<ConfigureStepSheet
		step={configuring}
		form={formFor(configuring)}
		{questions}
		onSave={saveStep}
		onClose={() => (configuringId = null)}
	/>
{/if}

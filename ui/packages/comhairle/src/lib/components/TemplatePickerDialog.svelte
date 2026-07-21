<script lang="ts">
	import * as Dialog from '$lib/components/ui/dialog';
	import * as Accordion from '$lib/components/ui/accordion';
	import { buttonVariants, LoadingButton } from '$lib/components/ui/button';
	import { conversationTemplates, type ConversationTemplate } from '$lib/conversation_templates';
	import SelectableOptionRow from '$lib/components/SelectableOptionRow.svelte';
	import TemplateIllustration from '$lib/components/TemplateIllustration.svelte';
	import { cn } from '$lib/utils';

	type Props = {
		/** Two-way bound so either side can close the dialog. */
		open: boolean;
		/** Puts the confirm button in its pending state and blocks re-submits. */
		submitting?: boolean;
		/** Guidance under the title. Differs by caller: creating vs re-templating. */
		description?: string;
		/** Label for the confirm button (e.g. "Get started", "Choose template"). */
		confirmLabel?: string;
		/** Receives the template the user settled on. The caller decides what to do with it. */
		onConfirm: (template: ConversationTemplate) => void;
	};

	let {
		open = $bindable(),
		submitting = false,
		description = 'Select a workflow template from the options below. You will have the opportunity to customise the workflow in the next step.',
		confirmLabel = 'Get started',
		onConfirm
	}: Props = $props();

	let selectedKey = $state(conversationTemplates[0].key);
	const selected = $derived(
		conversationTemplates.find((t) => t.key === selectedKey) ?? conversationTemplates[0]
	);
</script>

<Dialog.Root bind:open>
	<Dialog.Content
		class="flex max-h-[90vh] w-full max-w-5xl flex-col gap-6 overflow-hidden sm:max-w-5xl"
	>
		<Dialog.Header>
			<Dialog.Title class="text-center text-2xl font-semibold">Choose a template</Dialog.Title
			>
			<Dialog.Description class="text-center">{description}</Dialog.Description>
		</Dialog.Header>

		<div class="flex min-h-0 flex-1 items-start gap-6 overflow-hidden">
			<!-- Left: selectable template list -->
			<div
				class="flex w-96 shrink-0 flex-col gap-3 self-stretch overflow-y-auto pr-4 [scrollbar-gutter:stable]"
			>
				{#each conversationTemplates as template (template.key)}
					<SelectableOptionRow
						selected={selectedKey === template.key}
						name={template.name}
						description={template.description}
						onSelect={() => (selectedKey = template.key)}
					/>
				{/each}
			</div>

			<!-- Right: selected template detail -->
			<div
				class="bg-card flex min-h-0 flex-1 flex-col gap-4 self-stretch overflow-y-auto rounded-xl p-6"
			>
				<div class="flex items-center justify-between gap-2">
					<h3 class="text-card-foreground text-base font-bold">{selected.name}</h3>
					<div class="flex max-w-80 flex-wrap justify-end gap-2">
						{#each selected.badges as badge (badge.label)}
							{@const Icon = badge.icon}
							<span
								class={cn(
									'text-foreground inline-flex items-center gap-1 rounded-full px-2 py-0.5 text-xs font-medium',
									badge.class
								)}
							>
								<Icon class="size-4 shrink-0" />
								{badge.label}
							</span>
						{/each}
					</div>
				</div>

				<TemplateIllustration templateKey={selected.key} />

				<div class="flex flex-col gap-1">
					<h4 class="text-card-foreground text-sm font-bold">Workflow steps</h4>
					<Accordion.Root type="single">
						{#each selected.displaySteps as displayStep, i (displayStep.label)}
							<Accordion.Item value={`step-${i}`}>
								<Accordion.Trigger class="text-foreground text-sm font-medium">
									Step {i + 1}: {displayStep.label}
								</Accordion.Trigger>
								<Accordion.Content class="text-muted-foreground text-sm">
									{displayStep.description}
								</Accordion.Content>
							</Accordion.Item>
						{/each}
					</Accordion.Root>
				</div>
			</div>
		</div>

		<Dialog.Footer>
			<Dialog.Close class={buttonVariants({ variant: 'outline', size: 'sm' })}
				>Close</Dialog.Close
			>
			<LoadingButton
				variant="default"
				size="sm"
				loading={submitting}
				onclick={() => onConfirm(selected)}
			>
				{confirmLabel}
			</LoadingButton>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>

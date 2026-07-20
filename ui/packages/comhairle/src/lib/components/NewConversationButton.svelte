<script lang="ts">
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import * as Dialog from '$lib/components/ui/dialog';
	import * as Accordion from '$lib/components/ui/accordion';
	import { buttonVariants } from '$lib/components/ui/button';
	import { Plus } from 'lucide-svelte';
	import { goto } from '$app/navigation';
	import { notifications } from '$lib/notifications.svelte';
	import { manage_conversation_url } from '$lib/urls';
	import { createConversation } from '$lib/createConversation';
	import { conversationTemplates } from '$lib/conversation_templates';
	import SelectableOptionRow from '$lib/components/SelectableOptionRow.svelte';
	import TemplateIllustration from '$lib/components/TemplateIllustration.svelte';
	import { cn } from '$lib/utils';

	let {
		class: className = '',
		label = 'New conversation',
		labelClass = ''
	}: { class?: string; label?: string; labelClass?: string } = $props();

	let dialogOpen = $state(false);
	let submitting = $state(false);
	let selectedKey = $state(conversationTemplates[0].key);
	const selected = $derived(
		conversationTemplates.find((t) => t.key === selectedKey) ?? conversationTemplates[0]
	);

	async function create(templateKey?: string) {
		if (submitting) return;
		submitting = true;
		try {
			const conversation = await createConversation(templateKey ? { templateKey } : {});
			notifications.addFlash({ message: 'Conversation created' });
			dialogOpen = false;
			await goto(manage_conversation_url(conversation.id), { invalidateAll: true });
		} catch (e) {
			console.warn(e);
			notifications.send({ message: 'Something went wrong creating the conversation' });
		} finally {
			submitting = false;
		}
	}
</script>

<DropdownMenu.Root>
	<DropdownMenu.Trigger class={cn(buttonVariants({ variant: 'default' }), className)}>
		<Plus class="size-4" />
		<span class={labelClass}>{label}</span>
	</DropdownMenu.Trigger>
	<DropdownMenu.Content align="start" class="w-56">
		<DropdownMenu.Item onclick={() => create()}>Start from blank</DropdownMenu.Item>
		<DropdownMenu.Item onclick={() => (dialogOpen = true)}
			>Choose from templates</DropdownMenu.Item
		>
	</DropdownMenu.Content>
</DropdownMenu.Root>

<Dialog.Root bind:open={dialogOpen}>
	<Dialog.Content
		class="flex max-h-[90vh] w-full max-w-5xl flex-col gap-6 overflow-hidden sm:max-w-5xl"
	>
		<Dialog.Header>
			<Dialog.Title class="text-center text-2xl font-semibold">Choose a template</Dialog.Title
			>
			<Dialog.Description class="text-center">
				Select a workflow template from the options below. You will have the opportunity to
				customise the workflow in the next step.
			</Dialog.Description>
		</Dialog.Header>

		<div class="flex min-h-0 flex-1 items-start gap-6 overflow-hidden">
			<!-- Left: selectable template list -->
			<div class="flex w-96 shrink-0 flex-col gap-3 self-stretch overflow-y-auto pr-1">
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
			<button
				type="button"
				disabled={submitting}
				onclick={() => create(selected.key)}
				class={buttonVariants({ variant: 'default', size: 'sm' })}
			>
				Get started
			</button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>

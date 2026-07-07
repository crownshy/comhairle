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
					<button
						type="button"
						onclick={() => (selectedKey = template.key)}
						class={cn(
							'flex items-start gap-3 rounded-lg border p-4 text-left transition-colors',
							selectedKey === template.key
								? 'border-primary bg-accent'
								: 'border-border bg-card hover:bg-accent/50'
						)}
					>
						<span
							class={cn(
								'mt-0.5 flex size-4 shrink-0 items-center justify-center rounded-full border',
								selectedKey === template.key ? 'border-primary' : 'border-border'
							)}
						>
							{#if selectedKey === template.key}
								<span class="bg-primary size-2 rounded-full"></span>
							{/if}
						</span>
						<span class="flex flex-col gap-1 overflow-hidden">
							<span class="text-foreground text-sm">{template.name}</span>
							<span class="text-muted-foreground text-sm">{template.description}</span
							>
						</span>
					</button>
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
							<span
								class={cn(
									'text-foreground rounded-full px-2 py-0.5 text-xs font-medium',
									badge.class
								)}
							>
								{badge.label}
							</span>
						{/each}
					</div>
				</div>

				<!-- Decorative wireframe placeholder (per design) -->
				<div class="relative h-44 overflow-hidden rounded-md bg-neutral-200">
					<div
						class="absolute top-[37px] left-[38px] flex h-24 w-32 flex-col items-start gap-[5px] rounded-lg bg-stone-300 pt-3.5 pr-3.5 pb-5 pl-3"
					>
						<div class="h-2.5 w-24 rounded-lg bg-gray-200"></div>
						<div class="h-2.5 w-24 rounded-lg bg-gray-200"></div>
						<div class="h-2.5 w-20 rounded-lg bg-gray-200"></div>
						<div class="h-2.5 w-16 rounded-lg bg-gray-200"></div>
					</div>
					<div
						class="absolute top-[37px] left-[202px] flex h-24 w-32 flex-col items-end justify-center gap-[5px] rounded-lg bg-stone-300 pt-3.5 pr-3.5 pb-5 pl-3"
					>
						<div class="h-1.5 w-24 rounded-lg bg-gray-200"></div>
						<div class="h-10 w-24 rounded-lg bg-gray-200"></div>
						<div class="h-2.5 w-6 rounded-lg bg-gray-200"></div>
					</div>
					<div
						class="absolute top-[37px] left-[375px] flex h-24 w-32 flex-col items-center justify-center gap-[5px] rounded-lg bg-stone-300 pt-3.5 pr-3.5 pb-5 pl-3"
					>
						<div
							class="flex flex-wrap content-start items-start gap-[5px] self-stretch"
						>
							{#each Array.from({ length: 14 }) as _, i (i)}
								<div class="h-1.5 w-6 rounded-lg bg-gray-200"></div>
							{/each}
						</div>
					</div>
					<div
						class="absolute top-[87px] left-[170px] h-0 w-6 outline outline-2 -outline-offset-1 outline-stone-300"
					></div>
					<div
						class="absolute top-[87px] left-[340px] h-0 w-6 outline outline-2 -outline-offset-1 outline-stone-300"
					></div>
				</div>

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

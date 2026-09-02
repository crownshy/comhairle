<script lang="ts">
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import { buttonVariants } from '$lib/components/ui/button';
	import LoadingOverlay from '$lib/components/LoadingOverlay.svelte';
	import { Plus } from 'lucide-svelte';
	import { goto } from '$app/navigation';
	import { notifications } from '$lib/notifications.svelte';
	import { manage_conversation_url } from '$lib/urls';
	import { createConversation } from '$lib/createConversation';
	import { justCreatedConversation } from '$lib/stores/justCreatedConversation.svelte';
	import TemplatePickerDialog from '$lib/components/TemplatePickerDialog.svelte';
	import { cn } from '$lib/utils';
	import { key } from '$lib/utils/invalidationKey';

	type Props = {
		class?: string;
		label?: string;
		labelClass?: string;
	};
	let { class: className = '', label = 'New conversation', labelClass = '' }: Props = $props();

	let dialogOpen = $state(false);
	let submitting = $state(false);

	async function create(templateKey?: string) {
		if (submitting) return;
		submitting = true;
		try {
			const conversation = await createConversation(templateKey ? { templateKey } : {});
			// Flag it so the configure page it lands on makes its newness obvious.
			justCreatedConversation.flag(conversation.id);
			notifications.addFlash({ message: 'Conversation created' });
			dialogOpen = false;
			await goto(manage_conversation_url(conversation.id), {
				invalidate: [key('conversations')]
			});
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

<TemplatePickerDialog
	bind:open={dialogOpen}
	{submitting}
	onConfirm={(template) => create(template.key)}
/>

<!-- Immediate, screen-level feedback while the conversation is being created (covers
	both "Start from blank", where the dropdown has already closed, and a template). -->
<LoadingOverlay open={submitting} message="Creating your conversation…" />

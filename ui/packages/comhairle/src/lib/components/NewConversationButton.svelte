<script lang="ts">
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import { buttonVariants } from '$lib/components/ui/button';
	import LoadingOverlay from '$lib/components/LoadingOverlay.svelte';
	import { Plus } from 'lucide-svelte';
	import { goto } from '$app/navigation';
	import { resolve } from '$app/paths';
	import { notifications } from '$lib/notifications.svelte';
	import { createConversation } from '$lib/createConversation';
	import { justCreatedConversation } from '$lib/stores/justCreatedConversation.svelte';
	import TemplatePickerDialog from '$lib/components/TemplatePickerDialog.svelte';
	import { key } from '$lib/utils/invalidationKey';
	import { cn } from '$lib/utils';
	import { tryCatchAsync } from '$lib/utils/errorHandling';

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
		const created = await tryCatchAsync(() =>
			createConversation(templateKey ? { templateKey } : {})
		);
		if (created.err !== null) {
			notifications.send({ message: 'Something went wrong creating the conversation' });
			submitting = false;
			return;
		}
		// Flag it so the configure page it lands on makes its newness obvious.
		justCreatedConversation.flag(created.ok.id);
		notifications.addFlash({ message: 'Conversation created' });
		dialogOpen = false;
		// The sidebar list lives in the admin layout, keyed on the plural key.
		await goto(
			resolve('/(admin)/admin/conversations/[conversation_id]/configure', {
				conversation_id: created.ok.id
			}),
			{
				invalidate: [key('admin/conversations')]
			}
		);
		submitting = false;
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

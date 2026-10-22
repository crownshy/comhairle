<script lang="ts">
	import { Button, LoadingButton } from '$lib/components/ui/button';
	import type { SaveStatus } from '$lib/utils/emailTemplateForm';

	type Props = {
		/** The id of the form the save button submits. */
		formId: string;
		status: SaveStatus;
		submitLabel: string;
		cancelHref: string;
		disabled?: boolean;
	};

	let { formId, status, submitLabel, cancelHref, disabled = false }: Props = $props();

	const STATUS_TEXT: Record<SaveStatus, string> = {
		idle: '',
		unsaved: 'Unsaved changes',
		saving: '',
		saved: 'Saved',
		error: 'Could not save. Please try again.'
	};
</script>

<div class="flex flex-wrap items-center justify-end gap-3">
	<span role="status" class="text-muted-foreground text-sm">{STATUS_TEXT[status]}</span>
	<Button href={cancelHref} variant="outline">Cancel</Button>
	<LoadingButton type="submit" form={formId} loading={status === 'saving'} {disabled}>
		{submitLabel}
	</LoadingButton>
</div>

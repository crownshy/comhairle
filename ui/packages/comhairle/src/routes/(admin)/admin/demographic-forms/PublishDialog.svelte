<script lang="ts">
	import * as AlertDialog from '$lib/components/ui/alert-dialog';
	type Props = {
		title: string;
		description: string;
		actionLabel: string;
		onConfirm: () => void;
		onClose: () => void;
	};

	let { title, description, actionLabel, onConfirm, onClose }: Props = $props();
</script>

<AlertDialog.Root open onOpenChange={(open) => !open && onClose()}>
	<AlertDialog.Content>
		<AlertDialog.Header>
			<AlertDialog.Title>{title}</AlertDialog.Title>
			<AlertDialog.Description>{description}</AlertDialog.Description>
		</AlertDialog.Header>
		<AlertDialog.Footer>
			<AlertDialog.Cancel>Cancel</AlertDialog.Cancel>
			<AlertDialog.Action onclick={onConfirm}>{actionLabel}</AlertDialog.Action>
		</AlertDialog.Footer>
	</AlertDialog.Content>
</AlertDialog.Root>

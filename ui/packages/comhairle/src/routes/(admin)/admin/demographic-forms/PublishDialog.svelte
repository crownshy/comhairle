<script lang="ts">
	import * as AlertDialog from '$lib/components/ui/alert-dialog';
	import { publishImpact, type DemographicForm } from './demographicPrototypeData';

	type Props = { form: DemographicForm; onConfirm: () => void; onClose: () => void };

	let { form, onConfirm, onClose }: Props = $props();
</script>

<AlertDialog.Root open onOpenChange={(open) => !open && onClose()}>
	<AlertDialog.Content>
		<AlertDialog.Header>
			<AlertDialog.Title>Publish {form.name} as v{form.version + 1}?</AlertDialog.Title>
			<AlertDialog.Description>
				{publishImpact(form)}
			</AlertDialog.Description>
		</AlertDialog.Header>
		<AlertDialog.Footer>
			<AlertDialog.Cancel>Cancel</AlertDialog.Cancel>
			<AlertDialog.Action onclick={onConfirm}>Publish v{form.version + 1}</AlertDialog.Action>
		</AlertDialog.Footer>
	</AlertDialog.Content>
</AlertDialog.Root>

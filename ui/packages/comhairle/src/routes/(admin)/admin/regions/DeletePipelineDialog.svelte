<script lang="ts">
	import * as AlertDialog from '$lib/components/ui/alert-dialog';
	import { Checkbox } from '$lib/components/ui/checkbox';

	type Props = {
		name: string;
		onConfirm: (deleteAreas: boolean) => void;
		onClose: () => void;
	};

	let { name, onConfirm, onClose }: Props = $props();
	let deleteAreas = $state(false);
</script>

<AlertDialog.Root open onOpenChange={(open) => !open && onClose()}>
	<AlertDialog.Content>
		<AlertDialog.Header>
			<AlertDialog.Title>Delete {name}?</AlertDialog.Title>
			<AlertDialog.Description class="text-base">
				Scheduled refreshes stop and the pipeline is removed. This cannot be undone.
			</AlertDialog.Description>
		</AlertDialog.Header>
		<div class="flex items-start gap-3">
			<Checkbox
				id="delete-pipeline-areas"
				checked={deleteAreas}
				onCheckedChange={(value) => (deleteAreas = value === true)}
				class="mt-1"
			/>
			<label for="delete-pipeline-areas" class="grid gap-1 text-base">
				<span class="font-medium">Also delete its geographic areas</span>
				<span class="text-muted-foreground">
					Removes every area this pipeline imported and the Region created for each. Leave
					unticked to keep them as ordinary, editable areas and Regions.
				</span>
			</label>
		</div>
		<AlertDialog.Footer>
			<AlertDialog.Cancel>Cancel</AlertDialog.Cancel>
			<AlertDialog.Action
				class="bg-destructive hover:bg-destructive/90 text-white"
				onclick={() => onConfirm(deleteAreas)}
			>
				{deleteAreas ? 'Delete pipeline and areas' : 'Delete pipeline'}
			</AlertDialog.Action>
		</AlertDialog.Footer>
	</AlertDialog.Content>
</AlertDialog.Root>

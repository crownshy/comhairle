<script lang="ts">
	import * as AlertDialog from '$lib/components/ui/alert-dialog';
	import { Button, LoadingButton } from '$lib/components/ui/button';
	import type { PolisStatementAux } from '@crownshy/api-client/api';
	import { pluralise } from '$lib/utils/pluralise';

	type Props = {
		/** The split original to restore. Null while closed. */
		original: PolisStatementAux | null;
		replacementCount: number;
		working: boolean;
		onConfirm: () => void;
		onCancel: () => void;
	};

	let { original, replacementCount, working, onConfirm, onCancel }: Props = $props();

	const replacements = $derived(
		`${replacementCount} ${pluralise(replacementCount, 'replacement statement')}`
	);
</script>

<AlertDialog.Root open={original !== null}>
	<AlertDialog.Content>
		<AlertDialog.Header>
			<AlertDialog.Title>Undo split?</AlertDialog.Title>
			<AlertDialog.Description class="text-base">
				The {replacements} will be rejected, and any votes on them are lost. The original goes
				back to Waiting for a new decision.
			</AlertDialog.Description>
		</AlertDialog.Header>
		{#if original}
			<p class="text-muted-foreground line-clamp-3 text-base italic">
				“{original.statement_text}”
			</p>
		{/if}
		<AlertDialog.Footer>
			<Button variant="ghost" disabled={working} onclick={onCancel}>Cancel</Button>
			<LoadingButton variant="destructive" loading={working} onclick={onConfirm}>
				Undo split
			</LoadingButton>
		</AlertDialog.Footer>
	</AlertDialog.Content>
</AlertDialog.Root>

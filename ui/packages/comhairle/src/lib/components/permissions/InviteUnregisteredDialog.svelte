<script lang="ts">
	import { LoaderCircle } from 'lucide-svelte';
	import * as AlertDialog from '$lib/components/ui/alert-dialog';

	type Props = {
		open: boolean;
		emails: string[];
		saving: boolean;
		onConfirm: () => Promise<void>;
	};

	let { open = $bindable(), emails, saving, onConfirm }: Props = $props();
</script>

<AlertDialog.Root bind:open>
	<AlertDialog.Content>
		<AlertDialog.Header>
			<AlertDialog.Title>Invite people without an account?</AlertDialog.Title>
			<AlertDialog.Description>
				These email addresses do not have an account yet. We can email them an invitation to
				join the platform. They will be given the selected roles once they sign up.
			</AlertDialog.Description>
		</AlertDialog.Header>
		<ul class="max-h-60 list-disc space-y-1 overflow-y-auto pl-6 text-base">
			{#each emails as email (email)}<li>{email}</li>{/each}
		</ul>
		<AlertDialog.Footer>
			<AlertDialog.Cancel disabled={saving}>Cancel</AlertDialog.Cancel>
			<AlertDialog.Action
				disabled={saving}
				onclick={(event) => {
					event.preventDefault();
					onConfirm();
				}}
			>
				{#if saving}<LoaderCircle class="size-4 animate-spin" />{/if}
				Invite and assign
			</AlertDialog.Action>
		</AlertDialog.Footer>
	</AlertDialog.Content>
</AlertDialog.Root>

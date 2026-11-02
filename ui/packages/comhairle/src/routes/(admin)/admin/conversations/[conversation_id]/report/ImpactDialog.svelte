<script lang="ts">
	import { untrack } from 'svelte';
	import type { CreateImpactDTO } from '@crownshy/api-client/api';
	import * as Dialog from '$lib/components/ui/dialog';
	import * as Select from '$lib/components/ui/select';
	import { Button } from '$lib/components/ui/button';
	import LoadingButton from '$lib/components/ui/button/loading-button.svelte';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import { Textarea } from '$lib/components/ui/textarea';
	import { IMPACT_KINDS, impactKindLabel } from './impactKinds';

	type Props = {
		open: boolean;
		/** The impact being edited, or nothing to add a new one. Read when the dialog mounts. */
		initial?: CreateImpactDTO;
		onSave: (impact: CreateImpactDTO) => Promise<boolean>;
	};
	let { open = $bindable(), initial, onSave }: Props = $props();

	const empty = (): CreateImpactDTO => ({ title: '', details: '', kind: 'policy' });

	let draft = $state<CreateImpactDTO>(untrack(() => (initial ? { ...initial } : empty())));
	let saving = $state(false);

	const canSave = $derived(draft.title.trim().length > 0);

	async function save(event: SubmitEvent) {
		event.preventDefault();
		if (!canSave) return;
		saving = true;
		const saved = await onSave({ ...draft, title: draft.title.trim() });
		saving = false;
		if (saved) open = false;
	}
</script>

<Dialog.Root bind:open>
	<Dialog.Content class="sm:max-w-md">
		<Dialog.Header>
			<Dialog.Title>{initial ? 'Edit impact' : 'Add an impact'}</Dialog.Title>
			<Dialog.Description class="text-base">
				Something that happened because of this conversation.
			</Dialog.Description>
		</Dialog.Header>
		<form class="flex flex-col gap-4" onsubmit={save}>
			<div class="flex flex-col gap-2">
				<Label for="impact-title">Title</Label>
				<Input id="impact-title" bind:value={draft.title} required />
			</div>
			<div class="flex flex-col gap-2">
				<Label for="impact-details">Details</Label>
				<Textarea
					id="impact-details"
					placeholder="What happened, and where?"
					bind:value={draft.details}
				/>
			</div>
			<div class="flex flex-col gap-2">
				<Label for="impact-kind">Type</Label>
				<Select.Root type="single" bind:value={draft.kind}>
					<Select.Trigger id="impact-kind" class="w-full">
						{impactKindLabel(draft.kind)}
					</Select.Trigger>
					<Select.Content>
						{#each IMPACT_KINDS as kind (kind.value)}
							<Select.Item value={kind.value} label={kind.label} />
						{/each}
					</Select.Content>
				</Select.Root>
			</div>
			<Dialog.Footer>
				<Button type="button" variant="ghost" onclick={() => (open = false)}>Cancel</Button>
				<LoadingButton type="submit" loading={saving} disabled={!canSave}>
					{initial ? 'Save' : 'Add impact'}
				</LoadingButton>
			</Dialog.Footer>
		</form>
	</Dialog.Content>
</Dialog.Root>

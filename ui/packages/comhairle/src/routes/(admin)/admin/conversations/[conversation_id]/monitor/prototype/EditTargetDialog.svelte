<script lang="ts">
	import * as Dialog from '$lib/components/ui/dialog';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';

	type Props = {
		/** Current target; 0 means none set yet. */
		target: number;
		joined: number;
		onSave: (target: number) => void;
	};

	let { target, joined, onSave }: Props = $props();

	let open = $state(false);
	let draft = $state('');
	let error = $state<string | null>(null);

	// Start from the current target every time the dialog opens.
	$effect(() => {
		if (open) {
			draft = target > 0 ? String(target) : '';
			error = null;
		}
	});

	function save(event: SubmitEvent) {
		event.preventDefault();
		const value = Number(draft);
		if (!Number.isInteger(value) || value < 1) {
			error = 'Enter a whole number of 1 or more.';
			return;
		}
		onSave(value);
		open = false;
	}
</script>

<!-- TODO(backend): onSave only updates page state; persist the target with the conversation. -->
<Dialog.Root bind:open>
	<Dialog.Trigger class="text-primary text-base font-semibold hover:underline">
		{target > 0 ? 'Edit target' : 'Set a target'}
	</Dialog.Trigger>
	<Dialog.Content class="sm:max-w-md">
		<form class="flex flex-col gap-5" onsubmit={save} novalidate>
			<Dialog.Header>
				<Dialog.Title>Recruitment target</Dialog.Title>
				<Dialog.Description class="text-base">
					How many people do you want to take part? The progress bar, pace line and charts
					update to match.
				</Dialog.Description>
			</Dialog.Header>

			<div class="flex flex-col gap-2">
				<Label for="recruitment-target" class="text-base"
					>Target number of participants</Label
				>
				<Input
					id="recruitment-target"
					type="number"
					inputmode="numeric"
					min="1"
					step="1"
					bind:value={draft}
					aria-invalid={error ? 'true' : undefined}
					aria-describedby="recruitment-target-help{error
						? ' recruitment-target-error'
						: ''}"
					class="h-11 text-base"
				/>
				<p id="recruitment-target-help" class="text-muted-foreground text-sm">
					{joined} people have joined so far.
				</p>
				{#if error}
					<p id="recruitment-target-error" class="text-destructive text-sm" role="alert">
						{error}
					</p>
				{/if}
			</div>

			<Dialog.Footer class="gap-2">
				<Dialog.Close>
					{#snippet child({ props })}
						<Button {...props} variant="outline" class="rounded-full">Cancel</Button>
					{/snippet}
				</Dialog.Close>
				<Button type="submit" class="rounded-full">Save target</Button>
			</Dialog.Footer>
		</form>
	</Dialog.Content>
</Dialog.Root>

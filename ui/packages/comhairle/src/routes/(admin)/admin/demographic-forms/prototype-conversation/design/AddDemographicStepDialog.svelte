<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { Checkbox } from '$lib/components/ui/checkbox';
	import * as Dialog from '$lib/components/ui/dialog';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import * as RadioGroup from '$lib/components/ui/radio-group';
	import * as Select from '$lib/components/ui/select';
	import {
		pinnableVersions,
		type StepSettings,
		type VersionPolicy
	} from '../../conversationStep';
	import { specialCategoryCount, type DemographicForm } from '../../demographicPrototypeData';
	import type { DemographicQuestion } from '../../demographicPrototypeData';

	type Props = {
		forms: DemographicForm[];
		questions: DemographicQuestion[];
		onAdd: (formId: string, settings: StepSettings) => void;
		onClose: () => void;
	};

	let { forms, questions, onAdd, onClose }: Props = $props();

	const firstPublished = forms.find((form) => form.status === 'published');

	let formId = $state(firstPublished?.id ?? '');
	let name = $state('About you');
	let policy = $state<VersionPolicy>('latest');
	let pinnedVersion = $state<number | null>(null);
	let allowReturn = $state(true);
	let mustComplete = $state(true);

	const selected = $derived(forms.find((form) => form.id === formId));
	const versions = $derived(selected ? pinnableVersions(selected) : []);
	const pinned = $derived(pinnedVersion ?? selected?.version ?? 1);

	function chooseForm(id: string) {
		formId = id;
		pinnedVersion = null;
	}

	function submit() {
		if (!selected) return;
		onAdd(selected.id, {
			name,
			policy,
			pinnedVersion: policy === 'pinned' ? pinned : null,
			allowReturn,
			mustComplete
		});
	}
</script>

<Dialog.Root open onOpenChange={(open) => !open && onClose()}>
	<Dialog.Content class="flex max-h-[90vh] flex-col gap-5 overflow-y-auto sm:max-w-xl">
		<Dialog.Header>
			<Dialog.Title class="text-2xl font-semibold">Add demographic step</Dialog.Title>
			<Dialog.Description class="text-base">
				Choose one of your organisation's demographic forms. You can choose which of its
				questions to ask and require next.
			</Dialog.Description>
		</Dialog.Header>

		<fieldset class="flex flex-col gap-2">
			<legend class="mb-2 text-base font-semibold">Demographic form</legend>
			<RadioGroup.Root value={formId} onValueChange={chooseForm} class="gap-2">
				{#each forms as form (form.id)}
					{@const isDraft = form.status !== 'published'}
					<label
						for={`form-${form.id}`}
						class="flex items-start gap-3 rounded-xl border px-4 py-3 {isDraft
							? 'bg-muted text-muted-foreground cursor-not-allowed'
							: formId === form.id
								? 'border-primary bg-primary/10 cursor-pointer'
								: 'border-border hover:bg-muted cursor-pointer'}"
					>
						<RadioGroup.Item
							id={`form-${form.id}`}
							value={form.id}
							disabled={isDraft}
							class="mt-1"
						/>
						<span class="flex flex-col">
							<span class="text-base font-medium">{form.name}</span>
							<span class="text-sm">
								{#if isDraft}
									Draft. Publish it in Demographic forms to use it here.
								{:else}
									{form.questions.length} questions · {specialCategoryCount(
										form,
										questions
									)} special category · Published v{form.version}
								{/if}
							</span>
						</span>
					</label>
				{/each}
			</RadioGroup.Root>
		</fieldset>

		{#if selected}
			<fieldset class="flex flex-col gap-2">
				<legend class="mb-2 text-base font-semibold">When {selected.name} changes</legend>
				<RadioGroup.Root bind:value={policy} class="gap-2">
					<label
						for="policy-latest"
						class="flex cursor-pointer items-start gap-3 rounded-xl border px-4 py-3 {policy ===
						'latest'
							? 'border-primary bg-primary/10'
							: 'border-border hover:bg-muted'}"
					>
						<RadioGroup.Item id="policy-latest" value="latest" class="mt-1" />
						<span class="flex flex-col">
							<span class="text-base font-medium">Follow the latest version</span>
							<span class="text-muted-foreground text-sm">
								Recommended. When the form is published again, this step picks up
								the new wording and questions automatically.
							</span>
						</span>
					</label>
					<label
						for="policy-pinned"
						class="flex cursor-pointer items-start gap-3 rounded-xl border px-4 py-3 {policy ===
						'pinned'
							? 'border-primary bg-primary/10'
							: 'border-border hover:bg-muted'}"
					>
						<RadioGroup.Item id="policy-pinned" value="pinned" class="mt-1" />
						<span class="flex flex-1 flex-col gap-2">
							<span class="text-base font-medium">Pin to one version</span>
							<span class="text-muted-foreground text-sm">
								Use this when answers must stay comparable, for example once the
								conversation is live. Later versions will not change this step.
							</span>
							{#if policy === 'pinned'}
								<Select.Root
									type="single"
									value={String(pinned)}
									onValueChange={(value) => (pinnedVersion = Number(value))}
								>
									<Select.Trigger class="w-48" aria-label="Version to pin">
										Version {pinned}{pinned === selected.version
											? ' (latest)'
											: ''}
									</Select.Trigger>
									<Select.Content>
										{#each versions as version (version)}
											<Select.Item
												value={String(version)}
												label={`Version ${version}`}
											>
												Version {version}{version === selected.version
													? ' (latest)'
													: ''}
											</Select.Item>
										{/each}
									</Select.Content>
								</Select.Root>
							{/if}
						</span>
					</label>
				</RadioGroup.Root>
			</fieldset>
		{/if}

		<div class="flex flex-col gap-3">
			<h3 class="text-base font-semibold">Step settings</h3>
			<div class="flex flex-col gap-2">
				<Label for="step-name" class="text-base">Step name</Label>
				<Input id="step-name" bind:value={name} />
			</div>
			<div class="flex items-center gap-3">
				<Checkbox id="allow-return" bind:checked={allowReturn} />
				<Label for="allow-return" class="text-base font-normal">
					Allow participants to return to this step
				</Label>
			</div>
			<div class="flex items-center gap-3">
				<Checkbox id="must-complete" bind:checked={mustComplete} />
				<Label for="must-complete" class="text-base font-normal">
					Participants must complete this step to continue
				</Label>
			</div>
		</div>

		<p class="bg-primary/10 rounded-lg px-4 py-3 text-base">
			After adding, open the step's Configure tab to choose which questions to ask and which
			to require. Participants who already saved their answers see them prefilled.
		</p>

		<Dialog.Footer>
			<Button variant="outline" onclick={onClose}>Cancel</Button>
			<Button disabled={!selected} onclick={submit}>Add step</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>

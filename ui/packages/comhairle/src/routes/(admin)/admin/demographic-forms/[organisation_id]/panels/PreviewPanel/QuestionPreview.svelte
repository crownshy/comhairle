<script lang="ts">
	import DraggableList from '$lib/components/DraggableList.svelte';
	import { Input } from '$lib/components/EasyForm';
	import { GripVertical, Plus, X } from 'lucide-svelte';
	import { PREFER_NOT_TO_SAY } from '../../../demographicAnswers';
	import { getPlaceholder } from '../../../demographicModel';
	import type { QuestionLatestDto } from '@crownshy/api-client/api';
	import { Button } from '$lib/components/ui/button';
	import { typed } from '$lib/utils/types';
	import { max } from '$lib/utils/maths';
	import InlineInput from '../../InlineInput.svelte';

	type Option = QuestionLatestDto['options'][number];

	type Props = {
		question: QuestionLatestDto;
	};

	let { question = $bindable() }: Props = $props();

	let options = $state<Option[]>([]);

	const Option = {
		create() {
			const maxPosition = max(options, (option) => option.position);
			options.push(
				typed<Option>({
					id: '',
					label: '',
					position: maxPosition + 1
				})
			);
		},
		delete(label: string) {
			const index = options.findIndex((option) => option.label === label);
			if (!index) {
				return;
			}
			options.splice(index, 1);
		}
	};
</script>

<InlineInput
	placeholder="Question..."
	class="mb-3 text-xl! font-semibold"
	multiline
	value={question.text}
	oninput={(e) => void (question.text = e.currentTarget.value)}
/>

{#if question.questionType === 'text' && question.multi}
	<textarea placeholder={getPlaceholder(question.questionType)}></textarea>
{:else if question.questionType === 'text' || question.questionType === 'number' || question.questionType === 'date'}
	<Input
		type={question.questionType}
		placeholder={getPlaceholder(question.questionType)}
		class="py-5 text-lg!"
	/>
{:else if question.questionType === 'select'}
	<DraggableList items={options} onReorder={(next) => void (options = next)}>
		{#snippet children(row)}
			<div class="flex justify-between">
				<div class="flex items-center">
					<GripVertical class="size-4" />
					<Input
						value={row.label}
						placeholder="Option..."
						oninput={(e) => {
							row.id = e.currentTarget.value;
							row.label = e.currentTarget.value;
						}}
					/>
				</div>
				<Button
					aria-label="Remove {row.label}"
					onclick={() => void Option.delete(row.label)}
				>
					<X class="size-4" />
				</Button>
			</div>
		{/snippet}
	</DraggableList>
	<Button onclick={Option.create}>
		<Plus class="size-4" />Add option
	</Button>
{/if}
{#if question.preferNotToSay}
	<button
		type="button"
		class="flex h-12.5 w-full items-center justify-between rounded-lg border px-4 text-left text-sm transition-colors"
	>
		<span>{PREFER_NOT_TO_SAY}</span>
	</button>
{/if}
{#if question.specialCategory}
	<div class="border-border mt-6 flex flex-col gap-4 border-t pt-4">
		Special category placeholder
	</div>
{/if}

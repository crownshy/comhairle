<script lang="ts">
	import { Input } from '$lib/components/ui/input';
	import {
		defaultPlaceholder,
		isChoiceKind,
		type DemographicQuestion
	} from './demographicPrototypeData';

	type Props = {
		question: Pick<
			DemographicQuestion,
			'kind' | 'options' | 'allowOther' | 'preferNotToSay' | 'placeholder'
		>;
	};

	let { question }: Props = $props();

	const choices = $derived([
		...question.options.filter((option) => option.trim() !== ''),
		...(question.allowOther ? ['Other (please specify)'] : []),
		...(question.preferNotToSay ? ['Prefer not to say'] : [])
	]);
</script>

{#if isChoiceKind(question.kind)}
	<ul class="flex flex-col gap-2">
		{#each choices as choice (choice)}
			<li class="border-border bg-card rounded-lg border px-4 py-3 text-base">{choice}</li>
		{/each}
	</ul>
{:else}
	<div class="flex flex-col gap-2">
		<Input
			readonly
			tabindex={-1}
			placeholder={question.placeholder || defaultPlaceholder(question.kind)}
		/>
		{#if question.preferNotToSay}
			<p class="text-muted-foreground text-sm">Prefer not to say</p>
		{/if}
	</div>
{/if}

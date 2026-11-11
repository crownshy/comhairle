<script lang="ts">
	import InlineInput from '../../../InlineInput.svelte';
	import ErrorMessage from '$lib/components/ErrorMessage.svelte';
	import { Button } from '$lib/components/ui/button';
	import { Spinner } from '$lib/components/ui/spinner';
	import { QuestionLatestDto } from '@crownshy/api-client/api';
	import FullScreenContent from '../../../FullScreenContent.svelte';
	import { FormPanel, PreviewPanel, QuestionOptionsPanel } from '../../../panels';

	const { data, params } = $props();

	let questions = $state<QuestionLatestDto[]>([]);
	let selectedQuestionId = $state<QuestionLatestDto['id']>();
</script>

{#await data.streamed}
	<div class="flex h-full w-full items-center justify-center">
		<Spinner
			class="text-muted-foreground h-1/4 w-1/4"
			strokeWidth={0.5}
			aria-label="Loading form"
		/>
	</div>
{:then demographics}
	{#if demographics.err !== null}
		<ErrorMessage>Could not load form, please try again</ErrorMessage>
	{:else}
		<FullScreenContent>
			{#snippet Header()}
				<section class="mx-5 flex w-full flex-row items-center justify-between gap-x-5">
					<InlineInput
						class="max-w-120 text-3xl! font-bold"
						placeholder="New form"
						autofocus
						value={demographics.ok.form.name}
						required
					/>
					<Button class="mr-10">Create</Button>
				</section>
			{/snippet}
			{#snippet Body()}
				<div class="grid h-full grid-cols-[1fr_2fr_1fr] gap-x-5">
					<FormPanel
						bind:questions
						bind:selectedQuestionId
						organisationId={params.organisation_id}
					/>
					<PreviewPanel bind:questions {selectedQuestionId} />
					<QuestionOptionsPanel bind:questions {selectedQuestionId} />
				</div>
			{/snippet}
		</FullScreenContent>
	{/if}
{/await}

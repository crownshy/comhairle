<script lang="ts">
	import { slide } from 'svelte/transition';
	import * as Table from '$lib/components/ui/table';
	import * as Dialog from '$lib/components/ui/dialog';
	import type { ThinkingSpaceUserInsights } from '@crownshy/api-client/api';
	import { CornerDownRight } from '@lucide/svelte';
	import { ChevronDown } from 'lucide-svelte';

	type Props = {
		userInsights: ThinkingSpaceUserInsights[];
	};
	let { userInsights }: Props = $props();

	const ROW_ANIMATION_DURATION_MS = 1000;
	let rowLimit = $state(4);
	let selectedUserInsights = $state<ThinkingSpaceUserInsights | null>(null);
	let openDialog = $derived(selectedUserInsights !== null);
	let visibleInsightsRows = $derived(userInsights.slice(0, rowLimit));
</script>

<Table.Root class="w-full table-fixed">
	<Table.Header>
		<Table.Row class="text-xs">
			<Table.Head class="text-muted-foreground w-[10%] px-5">PID</Table.Head>
			<Table.Head class="text-muted-foreground w-2/5 px-5">Summary</Table.Head>
			<Table.Head class="text-muted-foreground w-2/5 px-5">Question/Responses</Table.Head>
		</Table.Row>
	</Table.Header>
	<Table.Body>
		{#each visibleInsightsRows as insight (insight.userId)}
			<Table.Row
				class="hover:cursor-pointer"
				onclick={() => (selectedUserInsights = insight)}
			>
				<Table.Cell class="text-muted-foreground border-l-6 px-5 py-9">
					<div transition:slide={{ duration: ROW_ANIMATION_DURATION_MS }}>
						{insight.userId.substring(0, 8).concat('...')}
					</div>
				</Table.Cell>
				<Table.Cell class="text-foreground h-auto px-5 text-wrap!">
					<div transition:slide={{ duration: ROW_ANIMATION_DURATION_MS }}>
						{#if insight.summary.summary.length > 120}
							{insight.summary.summary.substring(0, 120).concat('...')}
						{:else}
							{insight.summary.summary}
						{/if}
					</div>
				</Table.Cell>
				<Table.Cell class="text-muted-foreground h-auto px-5 text-wrap!">
					<div transition:slide={{ duration: ROW_ANIMATION_DURATION_MS }}>
						{#if insight.answers[0].root.question.length > 120}
							{insight.answers[0].root.question.substring(0, 120).concat('...')}
						{:else}
							{insight.answers[0].root.question}
						{/if}
					</div>
				</Table.Cell>
			</Table.Row>
		{/each}
	</Table.Body>
</Table.Root>

{#if rowLimit <= userInsights.length}
	<button
		class="bg-border flex w-full items-center justify-center gap-2 py-3 text-center"
		type="button"
		onclick={() => (rowLimit = userInsights.length)}
		>See all {userInsights.length} deep thoughts <ChevronDown /></button
	>
{/if}

<Dialog.Root bind:open={openDialog}>
	<Dialog.Content class="flex h-[70vh] w-full max-w-6xl flex-col gap-8 sm:w-full sm:max-w-6xl">
		<Dialog.Header>
			<Dialog.Title class="text-2xl">Participant</Dialog.Title>
			<span>{selectedUserInsights?.userId}</span>
			<Dialog.Description class="text-muted-foreground text-sm"
				>Thinking space summary, follow-up questions and responses</Dialog.Description
			>
		</Dialog.Header>
		<div class="">
			<h3 class="mb-4 font-bold">Summary</h3>
			<p class="bg-muted p-4">{selectedUserInsights?.summary.summary}</p>
		</div>
		<div class="overflow-y-auto">
			<div class="flex flex-col gap-8">
				{#each selectedUserInsights?.answers as answer, index (answer.root.id)}
					<div class="my-4 flex flex-col gap-4">
						<div class="grid grid-cols-[auto_1fr] items-center gap-4">
							<h3 class="text-muted-foreground">Question {index + 1}</h3>
							<div class="bg-muted h-0.5"></div>
						</div>
						<div class="rounded-md border">
							<p class="text-primary px-4 py-3">{answer.root.question}</p>
							<p class="bg-muted flex items-center gap-4 px-4 py-3">
								<CornerDownRight /><span>{answer.root.answer}</span>
							</p>
						</div>
						<div class="mx-5 mt-8 flex flex-col gap-4 border-l-2 px-4">
							<h4 class="flex items-center gap-4 text-xs font-semibold uppercase">
								Follow-ups
							</h4>
							<div class="flex flex-col gap-2">
								{#each answer.followUps as followUp, index (followUp.id)}
									<div class="rounded-md border">
										<div class="text-primary flex items-start gap-3 px-4 py-3">
											<span
												class="bg-muted inline-flex h-1.25 w-1.25 items-center justify-center rounded-full p-2.5 text-xs"
												>{index + 1}</span
											>{followUp.question}
											<p class="text-primary"></p>
										</div>
										<p class="bg-muted flex items-center gap-4 px-4 py-3">
											<CornerDownRight /><span>{followUp.answer}</span>
										</p>
									</div>
								{/each}
							</div>
						</div>
					</div>
				{/each}
			</div>
		</div>
	</Dialog.Content>
</Dialog.Root>

<script lang="ts">
	import * as Table from '$lib/components/ui/table';
	import * as Dialog from '$lib/components/ui/dialog';
	import type { ThinkingSpaceUserInsights } from '@crownshy/api-client/api';
	import { CornerDownRight } from '@lucide/svelte';

	type Props = {
		userInsights: ThinkingSpaceUserInsights[];
	};
	let { userInsights }: Props = $props();

	let selectedUserInsights = $state<ThinkingSpaceUserInsights | null>(null);
	let openDialog = $derived(selectedUserInsights !== null);
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
		{#each userInsights as insight (insight.userId)}
			<Table.Row
				onclick={() =>
					(selectedUserInsights =
						userInsights.find((i) => i.userId === insight.userId) ?? null)}
			>
				<Table.Cell class="text-muted-foreground border-l-6 px-5 py-9"
					>{insight.userId.substring(0, 8).concat('...')}</Table.Cell
				>
				<Table.Cell class="text-foreground h-auto px-5 text-wrap!">
					{#if insight.summary.summary.length > 120}
						{insight.summary.summary.substring(0, 120).concat('...')}
					{:else}
						{insight.summary.summary}
					{/if}
				</Table.Cell>
				<Table.Cell class="text-muted-foreground h-auto px-5 text-wrap!">
					{#if insight.answers[0].root.question.length > 120}
						{insight.answers[0].root.question.substring(0, 120).concat('...')}
					{:else}
						{insight.answers[0].root.question}
					{/if}
				</Table.Cell>
			</Table.Row>
		{/each}
	</Table.Body>
</Table.Root>

<Dialog.Root bind:open={openDialog}>
	<Dialog.Content class="flex h-[70vh] w-full max-w-6xl flex-col gap-8 sm:w-full sm:max-w-6xl">
		<Dialog.Header>
			<Dialog.Title class="text-2xl">Participant #{selectedUserInsights?.userId}</Dialog.Title
			>
			<Dialog.Description class="text-muted-foreground text-sm"
				>Thinking space summary, follow-up questions and responses</Dialog.Description
			>
		</Dialog.Header>
		<div class="">
			<h3 class="mb-4 font-bold">Summary</h3>
			<p>{selectedUserInsights?.summary.summary}</p>
		</div>
		<div class="overflow-y-auto">
			<h3 class="mb-4 font-bold">Root questions (and responses)</h3>
			<div class="flex flex-col gap-8">
				{#each selectedUserInsights?.answers as answer (answer.root.id)}
					<div class="my-4 flex flex-col gap-4">
						<p class="text-primary">{answer.root.question}</p>
						<p class="flex items-center gap-4">
							<CornerDownRight /><span>{answer.root.answer}</span>
						</p>
						<div class="mt-8 flex flex-col gap-4">
							<h4 class="flex items-center gap-4 font-bold">
								<CornerDownRight /> Follow-up questions (and responses)
							</h4>
							<div class="flex flex-col gap-4 pl-8">
								{#each answer.followUps as followUp (followUp.id)}
									<p class="text-primary">{followUp.question}</p>
									<p class="flex items-center gap-4">
										<CornerDownRight /><span>{followUp.answer}</span>
									</p>
								{/each}
							</div>
						</div>
					</div>
				{/each}
			</div>
		</div>
	</Dialog.Content>
</Dialog.Root>

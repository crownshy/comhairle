<!-- Dev-only admin tool for exercising the demographics API end-to-end: create
	 questions, attach them to a conversation, and post/inspect responses. -->
<script lang="ts">
	import PageHeader from '$lib/components/PageHeader.svelte';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import * as Select from '$lib/components/ui/select';
	import * as Card from '$lib/components/ui/card';
	import * as Table from '$lib/components/ui/table';
	import { notifications } from '$lib/notifications.svelte';
	import { apiClient } from '@crownshy/api-client/client';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import type {
		ConversationDemographics,
		DemographicsQuestion,
		DemographicsQuestionResponseType,
		DemographicsResponse
	} from '@crownshy/api-client/api';

	let { data } = $props();

	// Writable derived: resyncs from `data.questions` when the load reruns (e.g. after
	// `depends('dev:demographics')` invalidation), while still allowing local mutation.
	let questions = $derived(data.questions);
	let conversations = $derived(data.conversations);

	type BucketRow = { min: string; max: string; label: string };
	type OptionRow = { value: string; label: string };

	function buildBucketConfig(
		responseType: DemographicsQuestionResponseType,
		buckets: BucketRow[],
		options: OptionRow[]
	) {
		if (responseType === 'number' && buckets.length > 0) {
			return {
				type: 'numeric' as const,
				buckets: buckets.map((b) => ({
					min: b.min === '' ? null : Number(b.min),
					max: b.max === '' ? null : Number(b.max),
					label: b.label
				}))
			};
		}
		if (responseType === 'string' && options.length > 0) {
			return { type: 'string' as const, options };
		}
		return undefined;
	}

	// --- New question form -------------------------------------------------
	let newSlug = $state('');
	let newDisplayName = $state('');
	let newResponseType = $state<DemographicsQuestionResponseType>('number');
	let numericBuckets = $state<BucketRow[]>([]);
	let choiceOptions = $state<OptionRow[]>([]);
	let creatingQuestion = $state(false);

	function addNumericBucket() {
		numericBuckets = [...numericBuckets, { min: '', max: '', label: '' }];
	}

	function addChoiceOption() {
		choiceOptions = [...choiceOptions, { value: '', label: '' }];
	}

	async function createQuestion(e: SubmitEvent) {
		e.preventDefault();
		const form = e.currentTarget as HTMLFormElement;
		creatingQuestion = true;

		const bucketConfig = buildBucketConfig(newResponseType, numericBuckets, choiceOptions);

		const result = await tryCatchAsync(() =>
			apiClient.CreateDemographicsQuestion({
				slug: newSlug,
				displayName: newDisplayName,
				responseType: newResponseType,
				bucketConfig
			})
		);

		if (result.err !== null) {
			notifications.send({
				priority: 'ERROR',
				message: 'Failed to create demographics question'
			});
		} else {
			questions = [...questions, result.ok];
			newSlug = '';
			newDisplayName = '';
			numericBuckets = [];
			choiceOptions = [];
			form?.reset();
			notifications.send({
				priority: 'INFO',
				message: `Created question "${result.ok.slug}"`
			});
		}

		creatingQuestion = false;
	}

	// --- Edit existing question's buckets/options ---------------------------
	let editingSlug = $state<string | undefined>(undefined);
	let editNumericBuckets = $state<BucketRow[]>([]);
	let editChoiceOptions = $state<OptionRow[]>([]);
	let savingEdit = $state(false);

	function startEditQuestion(question: DemographicsQuestion) {
		editingSlug = question.slug;
		const config = question.bucketConfig;
		editNumericBuckets =
			config?.type === 'numeric'
				? config.buckets.map((b) => ({
						min: b.min === null ? '' : String(b.min),
						max: b.max === null ? '' : String(b.max),
						label: b.label
					}))
				: [];
		editChoiceOptions = config?.type === 'string' ? config.options.map((o) => ({ ...o })) : [];
	}

	function addEditNumericBucket() {
		editNumericBuckets = [...editNumericBuckets, { min: '', max: '', label: '' }];
	}

	function addEditChoiceOption() {
		editChoiceOptions = [...editChoiceOptions, { value: '', label: '' }];
	}

	function cancelEditQuestion() {
		editingSlug = undefined;
	}

	async function saveEditQuestion(question: DemographicsQuestion) {
		savingEdit = true;

		const bucketConfig =
			buildBucketConfig(question.responseType, editNumericBuckets, editChoiceOptions) ?? null;

		const result = await tryCatchAsync(() =>
			apiClient.UpdateDemographicsQuestion(
				{ bucketConfig },
				{ params: { question_slug: question.slug } }
			)
		);

		if (result.err !== null) {
			notifications.send({
				priority: 'ERROR',
				message: `Failed to update "${question.slug}"`
			});
		} else {
			questions = questions.map((q) => (q.slug === question.slug ? result.ok : q));
			editingSlug = undefined;
			notifications.send({
				priority: 'INFO',
				message: `Updated question "${question.slug}"`
			});
		}

		savingEdit = false;
	}

	async function deleteQuestion(slug: string) {
		const result = await tryCatchAsync(() =>
			apiClient.DeleteDemographicsQuestion(undefined, { params: { question_slug: slug } })
		);

		if (result.err !== null) {
			notifications.send({ priority: 'ERROR', message: `Failed to delete "${slug}"` });
			return;
		}

		questions = questions.filter((q) => q.slug !== slug);
		if (selectedConversationQuestions.some((cq) => cq.questionSlug === slug)) {
			selectedConversationQuestions = selectedConversationQuestions.filter(
				(cq) => cq.questionSlug !== slug
			);
		}
	}

	// --- Conversation associations ------------------------------------------
	let selectedConversationId = $state<string | undefined>(undefined);
	let selectedConversationQuestions = $state<ConversationDemographics[]>([]);
	let associationSlugToAdd = $state<string | undefined>(undefined);
	let loadingAssociations = $state(false);

	async function loadAssociations() {
		if (!selectedConversationId) return;
		loadingAssociations = true;

		const result = await tryCatchAsync(() =>
			apiClient.GetConversationDemographics({
				queries: { conversation_id: selectedConversationId, limit: 100 }
			})
		);

		selectedConversationQuestions = result.err !== null ? [] : result.ok.records;
		if (result.err !== null) {
			notifications.send({
				priority: 'ERROR',
				message: 'Failed to load conversation demographics'
			});
		}

		loadingAssociations = false;
	}

	async function addAssociation() {
		if (!selectedConversationId || !associationSlugToAdd) return;
		const conversationId = selectedConversationId;
		const questionSlug = associationSlugToAdd;

		const result = await tryCatchAsync(() =>
			apiClient.CreateConversationDemographics({ conversationId, questionSlug })
		);

		if (result.err !== null) {
			notifications.send({
				priority: 'ERROR',
				message: 'Failed to link question to conversation'
			});
			return;
		}

		selectedConversationQuestions = [...selectedConversationQuestions, result.ok];
		associationSlugToAdd = undefined;
	}

	async function removeAssociation(questionSlug: string) {
		if (!selectedConversationId) return;
		const conversationId = selectedConversationId;

		const result = await tryCatchAsync(() =>
			apiClient.DeleteConversationDemographicsByQuestion(undefined, {
				params: { conversation_id: conversationId, question_slug: questionSlug }
			})
		);

		if (result.err !== null) {
			notifications.send({ priority: 'ERROR', message: 'Failed to unlink question' });
			return;
		}

		selectedConversationQuestions = selectedConversationQuestions.filter(
			(cq) => cq.questionSlug !== questionSlug
		);
	}

	// --- Test responses -------------------------------------------------
	let responseQuestionSlug = $state<string | undefined>(undefined);
	let responseUserId = $derived(data.user.id);
	let responseValue = $state('');
	let responses = $state<DemographicsResponse[]>([]);
	let loadingResponses = $state(false);

	async function loadResponses() {
		if (!responseQuestionSlug) return;
		loadingResponses = true;

		const result = await tryCatchAsync(() =>
			apiClient.GetDemographicsResponses({
				queries: { question_slug: responseQuestionSlug, limit: 100 }
			})
		);

		responses = result.err !== null ? [] : result.ok.records;
		if (result.err !== null) {
			notifications.send({ priority: 'ERROR', message: 'Failed to load responses' });
		}

		loadingResponses = false;
	}

	async function submitResponse(e: Event) {
		e.preventDefault();
		if (!responseQuestionSlug || !responseUserId) return;
		const questionSlug = responseQuestionSlug;
		const userId = responseUserId;

		const existing = responses.find(
			(r) => r.questionSlug === questionSlug && r.userId === userId
		);

		const result = existing
			? await tryCatchAsync(() =>
					apiClient.UpdateDemographicsResponse(
						{ value: responseValue },
						{ params: { question_slug: questionSlug, user_id: userId } }
					)
				)
			: await tryCatchAsync(() =>
					apiClient.CreateDemographicsResponse({
						questionSlug,
						userId,
						value: responseValue
					})
				);

		if (result.err !== null) {
			notifications.send({ priority: 'ERROR', message: 'Failed to submit response' });
			return;
		}

		responses = existing
			? responses.map((r) => (r.id === result.ok.id ? result.ok : r))
			: [...responses, result.ok];
		notifications.send({ priority: 'INFO', message: 'Response saved' });
	}

	async function deleteResponse(questionSlug: string, userId: string) {
		const result = await tryCatchAsync(() =>
			apiClient.DeleteDemographicsResponse(undefined, {
				params: { question_slug: questionSlug, user_id: userId }
			})
		);

		if (result.err !== null) {
			notifications.send({ priority: 'ERROR', message: 'Failed to delete response' });
			return;
		}

		responses = responses.filter(
			(r) => !(r.questionSlug === questionSlug && r.userId === userId)
		);
	}
</script>

<svelte:head>
	<title>Demographics (dev) - Comhairle Admin</title>
</svelte:head>

<PageHeader
	title="Demographics (dev)"
	description="Internal tool for setting up demographics questions, linking them to a conversation, and posting test responses."
/>

<div class="flex flex-col gap-6">
	<Card.Root>
		<Card.Header>
			<Card.Title>Questions</Card.Title>
			<Card.Description>Create and manage demographics questions.</Card.Description>
		</Card.Header>
		<Card.Content class="flex flex-col gap-6">
			<Table.Root>
				<Table.Header>
					<Table.Row>
						<Table.Head>Slug</Table.Head>
						<Table.Head>Display name</Table.Head>
						<Table.Head>Type</Table.Head>
						<Table.Head>Bucket config</Table.Head>
						<Table.Head></Table.Head>
					</Table.Row>
				</Table.Header>
				<Table.Body>
					{#each questions as question (question.slug)}
						<Table.Row>
							<Table.Cell>{question.slug}</Table.Cell>
							<Table.Cell>{question.displayName}</Table.Cell>
							<Table.Cell>{question.responseType}</Table.Cell>
							<Table.Cell class="max-w-xs truncate font-mono text-xs">
								{question.bucketConfig
									? JSON.stringify(question.bucketConfig)
									: '—'}
							</Table.Cell>
							<Table.Cell class="flex gap-2">
								<Button
									variant="outline"
									size="sm"
									onclick={() => startEditQuestion(question)}
								>
									Edit buckets
								</Button>
								<Button
									variant="destructive"
									size="sm"
									onclick={() => deleteQuestion(question.slug)}
								>
									Delete
								</Button>
							</Table.Cell>
						</Table.Row>
						{#if editingSlug === question.slug}
							<Table.Row>
								<Table.Cell colspan={5} class="bg-muted/30">
									<div class="flex flex-col gap-3 py-2">
										{#if question.responseType === 'number'}
											<Label>Numeric buckets</Label>
											{#each editNumericBuckets as bucket, i (i)}
												<div class="flex gap-2">
													<Input
														placeholder="min"
														bind:value={bucket.min}
														class="w-24"
													/>
													<Input
														placeholder="max"
														bind:value={bucket.max}
														class="w-24"
													/>
													<Input
														placeholder="label"
														bind:value={bucket.label}
														class="flex-1"
													/>
													<Button
														type="button"
														variant="ghost"
														size="sm"
														onclick={() =>
															(editNumericBuckets =
																editNumericBuckets.filter(
																	(_, j) => j !== i
																))}
													>
														Remove
													</Button>
												</div>
											{/each}
											<Button
												type="button"
												variant="outline"
												size="sm"
												onclick={addEditNumericBucket}
											>
												Add bucket
											</Button>
										{:else}
											<Label>Options</Label>
											{#each editChoiceOptions as option, i (i)}
												<div class="flex gap-2">
													<Input
														placeholder="value"
														bind:value={option.value}
														class="w-32"
													/>
													<Input
														placeholder="label"
														bind:value={option.label}
														class="flex-1"
													/>
													<Button
														type="button"
														variant="ghost"
														size="sm"
														onclick={() =>
															(editChoiceOptions =
																editChoiceOptions.filter(
																	(_, j) => j !== i
																))}
													>
														Remove
													</Button>
												</div>
											{/each}
											<Button
												type="button"
												variant="outline"
												size="sm"
												onclick={addEditChoiceOption}
											>
												Add option
											</Button>
										{/if}

										<div class="flex gap-2">
											<Button
												size="sm"
												disabled={savingEdit}
												onclick={() => saveEditQuestion(question)}
											>
												{savingEdit ? 'Saving…' : 'Save'}
											</Button>
											<Button
												variant="ghost"
												size="sm"
												disabled={savingEdit}
												onclick={cancelEditQuestion}
											>
												Cancel
											</Button>
										</div>
									</div>
								</Table.Cell>
							</Table.Row>
						{/if}
					{:else}
						<Table.Row>
							<Table.Cell colspan={5} class="text-muted-foreground"
								>No questions yet.</Table.Cell
							>
						</Table.Row>
					{/each}
				</Table.Body>
			</Table.Root>

			<form class="grid gap-4 md:grid-cols-2" onsubmit={createQuestion}>
				<div class="space-y-2">
					<Label for="new-slug">Slug</Label>
					<Input id="new-slug" bind:value={newSlug} placeholder="age" required />
				</div>
				<div class="space-y-2">
					<Label for="new-display-name">Display name</Label>
					<Input
						id="new-display-name"
						bind:value={newDisplayName}
						placeholder="Age"
						required
					/>
				</div>
				<div class="space-y-2 md:col-span-2">
					<Label>Response type</Label>
					<Select.Root
						type="single"
						value={newResponseType}
						onValueChange={(v) =>
							(newResponseType = v as DemographicsQuestionResponseType)}
					>
						<Select.Trigger>{newResponseType}</Select.Trigger>
						<Select.Content>
							<Select.Item value="number">number</Select.Item>
							<Select.Item value="string">string</Select.Item>
						</Select.Content>
					</Select.Root>
				</div>

				{#if newResponseType === 'number'}
					<div class="space-y-2 md:col-span-2">
						<Label>Numeric buckets (optional)</Label>
						{#each numericBuckets as bucket, i (i)}
							<div class="flex gap-2">
								<Input placeholder="min" bind:value={bucket.min} class="w-24" />
								<Input placeholder="max" bind:value={bucket.max} class="w-24" />
								<Input
									placeholder="label"
									bind:value={bucket.label}
									class="flex-1"
								/>
								<Button
									type="button"
									variant="ghost"
									size="sm"
									onclick={() =>
										(numericBuckets = numericBuckets.filter((_, j) => j !== i))}
								>
									Remove
								</Button>
							</div>
						{/each}
						<Button
							type="button"
							variant="outline"
							size="sm"
							onclick={addNumericBucket}
						>
							Add bucket
						</Button>
					</div>
				{:else}
					<div class="space-y-2 md:col-span-2">
						<Label>Options (optional)</Label>
						{#each choiceOptions as option, i (i)}
							<div class="flex gap-2">
								<Input placeholder="value" bind:value={option.value} class="w-32" />
								<Input
									placeholder="label"
									bind:value={option.label}
									class="flex-1"
								/>
								<Button
									type="button"
									variant="ghost"
									size="sm"
									onclick={() =>
										(choiceOptions = choiceOptions.filter((_, j) => j !== i))}
								>
									Remove
								</Button>
							</div>
						{/each}
						<Button type="button" variant="outline" size="sm" onclick={addChoiceOption}>
							Add option
						</Button>
					</div>
				{/if}

				<div class="md:col-span-2">
					<Button type="submit" disabled={creatingQuestion}>
						{creatingQuestion ? 'Creating…' : 'Create question'}
					</Button>
				</div>
			</form>
		</Card.Content>
	</Card.Root>

	<Card.Root>
		<Card.Header>
			<Card.Title>Conversation associations</Card.Title>
			<Card.Description>Link demographics questions to a conversation.</Card.Description>
		</Card.Header>
		<Card.Content class="flex flex-col gap-4">
			<div class="flex flex-wrap items-end gap-2">
				<div class="space-y-2">
					<Label>Conversation</Label>
					<Select.Root
						type="single"
						value={selectedConversationId}
						onValueChange={(v) => (selectedConversationId = v)}
					>
						<Select.Trigger class="w-64">
							{conversations.find((c) => c.id === selectedConversationId)?.title ??
								'Select a conversation'}
						</Select.Trigger>
						<Select.Content>
							{#each conversations as conversation (conversation.id)}
								<Select.Item value={conversation.id}
									>{conversation.title}</Select.Item
								>
							{/each}
						</Select.Content>
					</Select.Root>
				</div>
				<Button
					variant="outline"
					disabled={!selectedConversationId || loadingAssociations}
					onclick={loadAssociations}
				>
					Load associations
				</Button>
			</div>

			<Table.Root>
				<Table.Header>
					<Table.Row>
						<Table.Head>Question</Table.Head>
						<Table.Head></Table.Head>
					</Table.Row>
				</Table.Header>
				<Table.Body>
					{#each selectedConversationQuestions as association (association.questionSlug)}
						<Table.Row>
							<Table.Cell>{association.questionSlug}</Table.Cell>
							<Table.Cell>
								<Button
									variant="destructive"
									size="sm"
									onclick={() => removeAssociation(association.questionSlug)}
								>
									Unlink
								</Button>
							</Table.Cell>
						</Table.Row>
					{:else}
						<Table.Row>
							<Table.Cell colspan={2} class="text-muted-foreground">
								No questions linked yet.
							</Table.Cell>
						</Table.Row>
					{/each}
				</Table.Body>
			</Table.Root>

			<div class="flex flex-wrap items-end gap-2">
				<div class="space-y-2">
					<Label>Question to link</Label>
					<Select.Root
						type="single"
						value={associationSlugToAdd}
						onValueChange={(v) => (associationSlugToAdd = v)}
					>
						<Select.Trigger class="w-64">
							{associationSlugToAdd ?? 'Select a question'}
						</Select.Trigger>
						<Select.Content>
							{#each questions as question (question.slug)}
								<Select.Item value={question.slug}>{question.slug}</Select.Item>
							{/each}
						</Select.Content>
					</Select.Root>
				</div>
				<Button
					disabled={!selectedConversationId || !associationSlugToAdd}
					onclick={addAssociation}
				>
					Link question
				</Button>
			</div>
		</Card.Content>
	</Card.Root>

	<Card.Root>
		<Card.Header>
			<Card.Title>Test responses</Card.Title>
			<Card.Description
				>Post and inspect responses for a question, as if a user answered it.</Card.Description
			>
		</Card.Header>
		<Card.Content class="flex flex-col gap-4">
			<div class="flex flex-wrap items-end gap-2">
				<div class="space-y-2">
					<Label>Question</Label>
					<Select.Root
						type="single"
						value={responseQuestionSlug}
						onValueChange={(v) => (responseQuestionSlug = v)}
					>
						<Select.Trigger class="w-64">
							{responseQuestionSlug ?? 'Select a question'}
						</Select.Trigger>
						<Select.Content>
							{#each questions as question (question.slug)}
								<Select.Item value={question.slug}>{question.slug}</Select.Item>
							{/each}
						</Select.Content>
					</Select.Root>
				</div>
				<Button
					variant="outline"
					disabled={!responseQuestionSlug || loadingResponses}
					onclick={loadResponses}
				>
					Load responses
				</Button>
			</div>

			<Table.Root>
				<Table.Header>
					<Table.Row>
						<Table.Head>User ID</Table.Head>
						<Table.Head>Value</Table.Head>
						<Table.Head></Table.Head>
					</Table.Row>
				</Table.Header>
				<Table.Body>
					{#each responses as response (response.id)}
						<Table.Row>
							<Table.Cell class="font-mono text-xs">{response.userId}</Table.Cell>
							<Table.Cell>{response.value}</Table.Cell>
							<Table.Cell>
								<Button
									variant="destructive"
									size="sm"
									onclick={() =>
										response.userId &&
										deleteResponse(response.questionSlug, response.userId)}
								>
									Delete
								</Button>
							</Table.Cell>
						</Table.Row>
					{:else}
						<Table.Row>
							<Table.Cell colspan={3} class="text-muted-foreground">
								No responses loaded.
							</Table.Cell>
						</Table.Row>
					{/each}
				</Table.Body>
			</Table.Root>

			<form class="grid gap-4 md:grid-cols-3" onsubmit={submitResponse}>
				<div class="space-y-2">
					<Label for="response-user-id">User ID</Label>
					<Input id="response-user-id" bind:value={responseUserId} required />
				</div>
				<div class="space-y-2 md:col-span-2">
					<Label for="response-value">Value</Label>
					<Input id="response-value" bind:value={responseValue} required />
				</div>
				<div class="md:col-span-3">
					<Button type="submit" disabled={!responseQuestionSlug}>Submit response</Button>
				</div>
			</form>
		</Card.Content>
	</Card.Root>
</div>

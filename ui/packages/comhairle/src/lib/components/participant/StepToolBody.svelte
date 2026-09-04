<script module lang="ts">
	import type { ToolType } from '$lib/tool_meta';

	// Each tool's participant UI is loaded on demand, and imported by file rather than
	// through the tool's barrel. A static import of every barrel put all seven tools, plus the
	// manage and report components the barrels re-export, in this route's module preloads:
	// over 200 entries in the `link` header, past what the ingress will buffer, so a full-page
	// load of a step answered 502.
	const loaders = {
		learn: () => import('$lib/tools/learn/LearnUI.svelte'),
		polis: () => import('$lib/tools/polis/PolisEmbed.svelte'),
		heyform: () => import('$lib/tools/heyform/HeyFormEmbed.svelte'),
		stories: () => import('$lib/tools/lived_experince/LivedExperinceUI.svelte'),
		thinkingspace: () => import('$lib/tools/thinking_space/ThinkingSpaceEmbed.svelte'),
		elicitationbot: () => import('$lib/tools/elicitation_bot/ElicitationBotEmbed.svelte'),
		prioritization: () => import('$lib/tools/prioritization/PrioritizationUser.svelte')
	} satisfies Record<ToolType, () => Promise<unknown>>;

	type Tool<K extends ToolType> = Awaited<ReturnType<(typeof loaders)[K]>>['default'];

	const loaded = new Map<ToolType, unknown>();

	/**
	 * The tool's component, or the promise that resolves to it. A tool that has loaded once
	 * comes back synchronously, so re-keying on a step change remounts it without a blank
	 * frame while the same module is fetched again.
	 */
	function tool<K extends ToolType>(type: K): Tool<K> | Promise<Tool<K>> {
		const cached = loaded.get(type);
		if (cached) return cached as Tool<K>;
		return loaders[type]().then((module) => {
			loaded.set(type, module.default);
			return module.default as Tool<K>;
		});
	}
</script>

<script lang="ts">
	import type { ToolSequence } from '$lib/step-brief/toolSequence';
	import type { ComhairleDocument } from '@crownshy/api-client/api';
	import LearnArticleSkeleton from '$lib/tools/learn/LearnArticleSkeleton.svelte';
	import PolisEmbedSkeleton from '$lib/tools/polis/PolisEmbedSkeleton.svelte';
	import HeyFormEmbedSkeleton from '$lib/tools/heyform/HeyFormEmbedSkeleton.svelte';

	let {
		toolConfig,
		conversation,
		workflowStep,
		userId,
		availableDocuments = [],
		hasKnowledgeBaseDocs = false,
		page = 0,
		preview = false,
		permissionToShareWithOrganizers = null,
		onDone = () => {},
		onCanContinueChange = () => {},
		onSequenceChange = () => {}
	}: {
		/*
		 * These three are `any` because the two call sites are handed nominally different
		 * generated types for the same thing: the admin route's step and tool config come
		 * from a different zod path than the participant route's, and svelte-check reports
		 * them as "two different types with this name exist, but they are unrelated" (the
		 * Setup page already had that error before any of this).
		 *
		 * Note what this does to the error count: the tool switch carried around 23
		 * pre-existing type errors while it lived in the route, and moving it here silenced
		 * them rather than fixing them. Giving these real types is a separate piece of work
		 * on the generated DTOs, and it would put those errors back where they can be seen.
		 */
		/* eslint-disable @typescript-eslint/no-explicit-any */
		toolConfig: any;
		conversation: any;
		workflowStep: any;
		/* eslint-enable @typescript-eslint/no-explicit-any */
		/**
		 * Whose answers the tools read on mount. A participant's own id on the real route; an
		 * admin's on a participant view, where reads then scope to someone with no answers and
		 * the tool shows its empty first-time state (ADR-0030).
		 */
		userId: string;
		availableDocuments?: ComhairleDocument[];
		hasKnowledgeBaseDocs?: boolean;
		/** Which page within the tool to show, where the tool has pages. Learn only, today. */
		page?: number;
		preview?: boolean;
		permissionToShareWithOrganizers?: boolean | null;
		onDone?: () => void;
		onCanContinueChange?: (value: boolean) => void;
		onSequenceChange?: (next: ToolSequence) => void;
	} = $props();
</script>

<!--
	The step's tool, in the column every tool body shares. One definition, rendered both by
	the participant route and by an admin participant view, so a tool added here shows up in
	both and neither can quietly fall behind the other (ADR-0030).

	The pending branches show the tool's skeleton, where it has one, for the first load of a
	tool's module in a session. After that `tool()` returns synchronously and they never show.
-->
<div
	class="mx-auto flex min-h-full w-full max-w-5xl flex-col px-4 pb-[clamp(0.5rem,2vh,1.5rem)] md:px-6"
>
	{#if toolConfig.type === 'learn'}
		{#key workflowStep.id}
			{#await tool('learn')}
				<LearnArticleSkeleton />
			{:then LearnUI}
				<LearnUI
					pages={toolConfig.pages}
					{page}
					{onSequenceChange}
					{conversation}
					{availableDocuments}
					{hasKnowledgeBaseDocs}
				/>
			{/await}
		{/key}
	{:else if toolConfig?.type === 'polis'}
		{#key workflowStep.id}
			{#await tool('polis')}
				<PolisEmbedSkeleton />
			{:then PolisUI}
				<PolisUI
					user_id={userId}
					polis_id={toolConfig.poll_id}
					polis_url={toolConfig.server_url}
					requiredVotes={toolConfig.required_votes}
					workflowStepId={workflowStep.id}
					isPreview={preview}
					{onDone}
					{onCanContinueChange}
					{onSequenceChange}
					showRemainingStatementCount={toolConfig.show_remaining_statements}
				/>
			{/await}
		{/key}
	{:else if toolConfig.type === 'heyform'}
		{#key workflowStep.id}
			{#await tool('heyform')}
				<HeyFormEmbedSkeleton />
			{:then HeyFormUI}
				<HeyFormUI
					{userId}
					surveyId={toolConfig.survey_id}
					surveyURL={toolConfig.survey_url}
					serverURL={toolConfig.server_url}
					{onDone}
				/>
			{/await}
		{/key}
	{:else if toolConfig.type === 'stories'}
		{#key workflowStep.id}
			{#await tool('stories') then LivedExperienceUI}
				<LivedExperienceUI {onDone} {onSequenceChange} />
			{/await}
		{/key}
	{:else if toolConfig.type === 'thinkingspace'}
		{#key workflowStep.id}
			{#await tool('thinkingspace') then ThinkingSpaceUI}
				<ThinkingSpaceUI
					workflowStepId={workflowStep.id}
					workflowId={workflowStep.workflowId}
					conversationId={conversation.id}
					{userId}
					topic={toolConfig.topic}
					rootQuestions={toolConfig.root_questions}
					followUpRoundsCount={toolConfig.follow_up_rounds_count}
					requestUserSharePermission={workflowStep.requestUserSharePermission}
					initialPermissionToShareWithOrganizers={permissionToShareWithOrganizers}
					progressStatus={workflowStep.progressStatus}
					{onDone}
					{onCanContinueChange}
					{onSequenceChange}
				/>
			{/await}
		{/key}
	{:else if toolConfig.type === 'elicitationbot'}
		{#key workflowStep.id}
			{#await tool('elicitationbot') then ElicitationBotUI}
				<ElicitationBotUI
					conversationId={conversation.id}
					workflowId={workflowStep.workflowId}
					workflowStepId={workflowStep.id}
					{userId}
					topic={toolConfig.topic}
					{onDone}
					{onCanContinueChange}
				/>
			{/await}
		{/key}
	{:else if toolConfig.type === 'prioritization'}
		{#key workflowStep.id}
			{#await tool('prioritization') then PrioritizationUI}
				<PrioritizationUI
					{workflowStep}
					conversation={{
						primaryLocale: conversation.primaryLocale,
						isLive: conversation.isLive,
						supportedLanguages: conversation.supportedLanguages
					}}
					participantId={userId}
					{onDone}
					{onCanContinueChange}
					{onSequenceChange}
				/>
			{/await}
		{/key}
	{/if}
</div>

<script lang="ts">
	import type { Snippet } from 'svelte';
	import * as Polis from '$lib/tools/polis/index.js';
	import * as HeyForm from '$lib/tools/heyform/index.js';
	import * as Learn from '$lib/tools/learn/index.js';
	import * as LivedExperience from '$lib/tools/lived_experince/index.js';
	import * as ThinkingSpace from '$lib/tools/thinking_space/index.js';
	import * as ElicitationBot from '$lib/tools/elicitation_bot/index.js';
	import * as Prioritization from '$lib/tools/prioritization/index.js';
	import type { ToolSequence } from '$lib/step-brief/toolSequence';
	import type { ComhairleDocument } from '@crownshy/api-client/api';

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
		onSequenceChange = () => {},
		loading
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
		/** Rendered instead of the tool while the route is navigating to another step. */
		loading?: Snippet;
	} = $props();
</script>

<!--
	The step's tool, in the column every tool body shares. One definition, rendered both by
	the participant route and by an admin participant view, so a tool added here shows up in
	both and neither can quietly fall behind the other (ADR-0030).
-->
<div
	class="mx-auto flex min-h-full w-full max-w-5xl flex-col px-4 pb-[clamp(0.5rem,2vh,1.5rem)] md:px-6"
>
	{#if loading}
		{@render loading()}
	{:else if toolConfig.type === Learn.TOOL_NAME}
		{#key workflowStep.id}
			<Learn.UserUI
				pages={toolConfig.pages}
				{page}
				{onSequenceChange}
				{conversation}
				{availableDocuments}
				{hasKnowledgeBaseDocs}
			/>
		{/key}
	{:else if toolConfig?.type === Polis.TOOL_NAME}
		{#key workflowStep.id}
			<Polis.UserUI
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
		{/key}
	{:else if toolConfig.type === HeyForm.TOOL_NAME}
		{#key workflowStep.id}
			<HeyForm.UserUI
				{userId}
				surveyId={toolConfig.survey_id}
				surveyURL={toolConfig.survey_url}
				serverURL={toolConfig.server_url}
				{onDone}
			/>
		{/key}
	{:else if toolConfig.type === LivedExperience.TOOL_NAME}
		{#key workflowStep.id}
			<LivedExperience.UserUI {onDone} {onSequenceChange} />
		{/key}
	{:else if toolConfig.type === ThinkingSpace.TOOL_NAME}
		{#key workflowStep.id}
			<ThinkingSpace.UserUI
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
		{/key}
	{:else if toolConfig.type === ElicitationBot.TOOL_NAME}
		{#key workflowStep.id}
			<ElicitationBot.UserUI
				conversationId={conversation.id}
				workflowId={workflowStep.workflowId}
				workflowStepId={workflowStep.id}
				{userId}
				topic={toolConfig.topic}
				{onDone}
				{onCanContinueChange}
			/>
		{/key}
	{:else if toolConfig.type === Prioritization.TOOL_NAME}
		{#key workflowStep.id}
			<Prioritization.UserUI
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
		{/key}
	{/if}
</div>

<script lang="ts">
	import { Info, Plus } from 'lucide-svelte';
	import { Button } from '$lib/components/ui/button';
	import {
		canScopeToConversation,
		type EmailTypeView,
		type EmailTemplateRow
	} from '$lib/utils/emailTemplateGroups';
	import { EMAIL_TYPE_COPY } from './emailTypeCopy';
	import DefaultTemplateCard from './DefaultTemplateCard.svelte';
	import ConversationTemplatesTable from './ConversationTemplatesTable.svelte';
	import type { EmailTypeSchema, LocalizedConversationDto } from '@crownshy/api-client/api';
	import { notifications } from '$lib/notifications.svelte';
	import InboxPreviewDialog from './InboxPreviewDialog.svelte';
	import { renderTemplatePreview, type RenderedPreview } from './templatePreview';
	import { templateHref } from './templateHref';
	import ChooseConversationDialog from './ChooseConversationDialog.svelte';

	type Props = {
		view: EmailTypeView;
		schema: EmailTypeSchema;
		conversations: LocalizedConversationDto[];
	};
	let { view, schema, conversations }: Props = $props();

	let previewOpen = $state(false);
	let preview = $state<RenderedPreview | null>(null);
	let previewRow = $state<EmailTemplateRow | null>(null);
	let showFallbacks = $state(false);
	let choosingConversation = $state(false);

	async function openPreview(row: EmailTemplateRow) {
		const conversation = conversations.find((candidate) => candidate.id === row.conversationId);
		const rendered = await renderTemplatePreview(
			schema,
			row,
			conversation ? { title: conversation.title, imageUrl: conversation.imageUrl } : null
		);
		if (!rendered) {
			notifications.send({ priority: 'ERROR', message: 'The preview could not be loaded.' });
			return;
		}
		preview = rendered;
		previewRow = row;
		previewOpen = true;
	}

	let copy = $derived(EMAIL_TYPE_COPY[view.emailType]);
	let scopable = $derived(canScopeToConversation(view.emailType));
	let fallbackCount = $derived(view.usingBase.length);
</script>

<div data-tour="email-configs-list" class="flex max-w-5xl flex-col gap-7">
	<p class="text-muted-foreground text-base">{copy.sentWhen}</p>

	<DefaultTemplateCard
		row={view.base}
		emailType={view.emailType}
		onPreview={() => openPreview(view.base)}
	/>

	{#if scopable}
		{#if fallbackCount > 0}
			<div
				class="bg-accent text-foreground flex flex-col gap-2 rounded-lg px-4 py-3 text-base"
			>
				<p class="flex items-start gap-2.5">
					<Info class="text-primary mt-0.5 size-5 shrink-0" aria-hidden="true" />
					<span>
						<strong
							>{fallbackCount}
							{fallbackCount === 1 ? 'conversation' : 'conversations'}</strong
						>
						without their own version {fallbackCount === 1 ? 'sends' : 'send'} the default
						above.
						<button
							type="button"
							class="text-primary cursor-pointer font-medium underline-offset-4 hover:underline"
							aria-expanded={showFallbacks}
							aria-controls="fallback-conversations"
							onclick={() => (showFallbacks = !showFallbacks)}
						>
							{showFallbacks ? 'Hide' : 'See which'}
						</button>
					</span>
				</p>
				{#if showFallbacks}
					<ul
						id="fallback-conversations"
						class="text-muted-foreground ms-7 list-disc ps-5 text-base"
					>
						{#each view.usingBase as conversation (conversation.id)}
							<li>{conversation.title}</li>
						{/each}
					</ul>
				{/if}
			</div>
		{/if}

		<section aria-labelledby="own-heading" class="flex flex-col gap-2.5">
			<div class="flex flex-wrap items-end justify-between gap-3">
				<h2 id="own-heading" class="text-foreground text-lg font-semibold">
					Conversations with their own invitation versions ({view.conversationVersions
						.length})
				</h2>
				<Button
					type="button"
					variant="outline"
					size="sm"
					class="text-sm"
					onclick={() => (choosingConversation = true)}
				>
					<Plus class="size-4" aria-hidden="true" />
					Add for a conversation
				</Button>
			</div>
			{#if view.conversationVersions.length > 0}
				<ConversationTemplatesTable
					rows={view.conversationVersions}
					emailType={view.emailType}
					onPreview={openPreview}
				/>
			{:else}
				<p class="text-muted-foreground text-base">
					No conversation has its own version yet.
				</p>
			{/if}
		</section>
		<ChooseConversationDialog
			bind:open={choosingConversation}
			emailType={view.emailType}
			conversations={view.usingBase}
		/>
	{:else}
		<p class="text-muted-foreground text-base">
			Event emails use one version for all your events.
		</p>
	{/if}
	{#if preview && previewRow}
		{@const editHref = templateHref(previewRow, view.emailType)}
		<InboxPreviewDialog
			bind:open={previewOpen}
			subject={preview.subject}
			html={preview.html}
			variables={schema.variables}
		>
			{#snippet footer()}
				<Button href={editHref}>Edit this email</Button>
			{/snippet}
		</InboxPreviewDialog>
	{/if}
</div>

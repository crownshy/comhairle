<script lang="ts">
	import { resolve } from '$app/paths';
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

	type Props = {
		view: EmailTypeView;
		schema: EmailTypeSchema;
		conversations: LocalizedConversationDto[];
	};
	let { view, schema, conversations }: Props = $props();

	let previewOpen = $state(false);
	let preview = $state<RenderedPreview | null>(null);
	let previewRow = $state<EmailTemplateRow | null>(null);

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
	let newHref = $derived(
		`${resolve('/(admin)/admin/email-template-configs/new')}?type=${view.emailType}`
	);
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
			<p
				class="bg-accent text-foreground flex items-start gap-2.5 rounded-lg px-4 py-3 text-base"
			>
				<Info class="text-primary mt-0.5 size-5 shrink-0" aria-hidden="true" />
				<span>
					<strong
						>{fallbackCount}
						{fallbackCount === 1 ? 'conversation' : 'conversations'}</strong
					>
					without their own version {fallbackCount === 1 ? 'sends' : 'send'} the default above.
				</span>
			</p>
		{/if}

		<section aria-labelledby="own-heading" class="flex flex-col gap-2.5">
			<div class="flex flex-wrap items-end justify-between gap-3">
				<h2 id="own-heading" class="text-foreground text-lg font-semibold">
					Conversations with their own version ({view.conversationVersions.length})
				</h2>
				<Button href={newHref} variant="outline" size="sm" class="text-sm">
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

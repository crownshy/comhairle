<script lang="ts">
	import { resolve } from '$app/paths';
	import { Info, Plus } from 'lucide-svelte';
	import { Button } from '$lib/components/ui/button';
	import { canScopeToConversation, type EmailTypeView } from '$lib/utils/emailTemplateGroups';
	import { EMAIL_TYPE_COPY } from './emailTypeCopy';
	import DefaultTemplateCard from './DefaultTemplateCard.svelte';
	import ConversationTemplatesTable from './ConversationTemplatesTable.svelte';

	type Props = { view: EmailTypeView };
	let { view }: Props = $props();

	let copy = $derived(EMAIL_TYPE_COPY[view.emailType]);
	let scopable = $derived(canScopeToConversation(view.emailType));
	let fallbackCount = $derived(view.usingBase.length);
	let newHref = $derived(
		`${resolve('/(admin)/admin/email-template-configs/new')}?type=${view.emailType}`
	);
</script>

<div data-tour="email-configs-list" class="flex max-w-5xl flex-col gap-7">
	<p class="text-muted-foreground text-base">{copy.sentWhen}</p>

	<DefaultTemplateCard row={view.base} emailType={view.emailType} />

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
</div>

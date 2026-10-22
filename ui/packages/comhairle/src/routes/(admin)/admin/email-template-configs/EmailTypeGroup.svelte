<script lang="ts">
	import { resolve } from '$app/paths';
	import { Mail, Plus } from 'lucide-svelte';
	import { Button } from '$lib/components/ui/button';
	import { canScopeToConversation, type EmailTypeGroup } from '$lib/utils/emailTemplateGroups';
	import { EMAIL_TYPE_COPY } from './emailTypeCopy';
	import EmailTemplateRow from './EmailTemplateRow.svelte';

	type Props = {
		group: EmailTypeGroup;
		/** Hides the add action while the list is narrowed to one conversation. */
		showAddForConversation?: boolean;
	};

	let { group, showAddForConversation = true }: Props = $props();

	let copy = $derived(EMAIL_TYPE_COPY[group.emailType]);
	let headingId = $derived(`email-type-${group.emailType}`);
	let newHref = $derived(
		`${resolve('/(admin)/admin/email-template-configs/new')}?type=${group.emailType}`
	);
</script>

<section
	aria-labelledby={headingId}
	class="bg-card border-border overflow-hidden rounded-xl border"
>
	<div class="flex flex-wrap items-start justify-between gap-3 px-5 py-4">
		<div class="flex min-w-0 items-start gap-3.5">
			<div
				class="bg-accent flex size-10 shrink-0 items-center justify-center rounded-lg"
				aria-hidden="true"
			>
				<Mail class="text-primary size-5" />
			</div>
			<div class="flex min-w-0 flex-col gap-0.5">
				<h2 id={headingId} class="text-foreground text-lg font-semibold">{copy.name}</h2>
				<p class="text-muted-foreground text-base">{copy.sentWhen}</p>
			</div>
		</div>
		{#if showAddForConversation && canScopeToConversation(group.emailType)}
			<Button href={newHref} variant="outline" size="sm" class="text-sm">
				<Plus class="size-4" aria-hidden="true" />
				Customise for a conversation
			</Button>
		{/if}
	</div>
	<ul class="border-border divide-border divide-y border-t">
		{#each group.rows as row (row.key)}
			<EmailTemplateRow {row} emailType={group.emailType} />
		{/each}
	</ul>
</section>

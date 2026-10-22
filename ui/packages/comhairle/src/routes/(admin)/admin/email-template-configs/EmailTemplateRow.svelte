<script lang="ts">
	import { resolve } from '$app/paths';
	import { Badge } from '$lib/components/ui/badge';
	import { Button } from '$lib/components/ui/button';
	import { formatDateShort } from '$lib/utils';
	import type { EmailTemplateRow } from '$lib/utils/emailTemplateGroups';
	import type { EmailType } from '@crownshy/api-client/api';
	import { EMAIL_TYPE_COPY } from './emailTypeCopy';

	type Props = {
		row: EmailTemplateRow;
		emailType: EmailType;
	};

	let { row, emailType }: Props = $props();

	let copy = $derived(EMAIL_TYPE_COPY[emailType]);
	let scopeLabel = $derived(
		row.source === 'conversation'
			? (row.conversationTitle ?? 'Untitled conversation')
			: copy.allScopeLabel
	);
	let isCustom = $derived(row.source !== 'default');
	let editHref = $derived(
		row.configId
			? resolve('/(admin)/admin/email-template-configs/[email_config_id]', {
					email_config_id: row.configId
				})
			: `${resolve('/(admin)/admin/email-template-configs/new')}?type=${emailType}`
	);
</script>

<li
	class="grid grid-cols-1 gap-x-6 gap-y-2 px-5 py-4 md:grid-cols-[minmax(10rem,14rem)_minmax(0,1fr)_auto_auto] md:items-center md:pl-[4.625rem]"
>
	<div class="flex flex-col items-start gap-1">
		<span class="text-foreground text-base font-semibold">{scopeLabel}</span>
		{#if !isCustom}
			<Badge variant="secondary" class="text-sm">Comhairle default</Badge>
		{/if}
	</div>
	<div class="flex min-w-0 flex-col">
		<span class="text-muted-foreground text-sm font-medium">Subject</span>
		<span class="text-foreground truncate text-base" title={row.subject}>{row.subject}</span>
	</div>
	<span class="text-muted-foreground text-sm">
		{row.createdAt ? `Created ${formatDateShort(row.createdAt)}` : 'Not customised'}
	</span>
	<Button
		href={editHref}
		variant="link"
		class="justify-self-start px-0 text-base md:justify-self-end"
	>
		{isCustom ? 'Edit' : 'Customise'}
		<span class="sr-only">{copy.name}, {scopeLabel}</span>
	</Button>
</li>

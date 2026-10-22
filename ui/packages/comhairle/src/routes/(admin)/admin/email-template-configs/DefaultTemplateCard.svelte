<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { formatDateShort } from '$lib/utils';
	import type { EmailTemplateRow } from '$lib/utils/emailTemplateGroups';
	import type { EmailType } from '@crownshy/api-client/api';
	import { EMAIL_TYPE_COPY } from './emailTypeCopy';
	import { templateHref } from './templateHref';

	type Props = { row: EmailTemplateRow; emailType: EmailType };
	let { row, emailType }: Props = $props();

	let copy = $derived(EMAIL_TYPE_COPY[emailType]);
	let isCustom = $derived(row.source !== 'default');
	let heading = $derived(
		isCustom ? `Default for ${copy.allScopeLabel.toLowerCase()}` : "Comhairle's standard email"
	);
</script>

<section aria-labelledby="default-heading" class="flex flex-col gap-2.5">
	<h2 id="default-heading" class="text-foreground text-lg font-semibold">{heading}</h2>
	<div
		class="border-border flex flex-wrap items-center gap-x-6 gap-y-3 rounded-xl border px-5 py-4"
	>
		<div class="flex min-w-0 flex-[1_1_20rem] flex-col gap-1">
			<span class="text-muted-foreground text-sm font-medium">Subject</span>
			<span class="text-foreground truncate text-base" title={row.subject}>{row.subject}</span
			>
		</div>
		<span class="text-muted-foreground text-sm">
			{row.createdAt ? `Created ${formatDateShort(row.createdAt)}` : 'Not customised'}
		</span>
		<Button href={templateHref(row, emailType)} variant="outline" size="sm" class="text-sm">
			{isCustom ? 'Edit' : 'Customise'}
		</Button>
	</div>
</section>

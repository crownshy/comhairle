<script lang="ts">
	import { formatDateShort } from '$lib/utils';
	import type { EmailTemplateRow } from '$lib/utils/emailTemplateGroups';
	import type { EmailType } from '@crownshy/api-client/api';
	import { templateHref } from './templateHref';

	type Props = {
		rows: EmailTemplateRow[];
		emailType: EmailType;
		onPreview: (row: EmailTemplateRow) => void;
	};
	let { rows, emailType, onPreview }: Props = $props();

	const HEAD_CLASS = 'text-muted-foreground px-4 py-2.5 text-sm font-semibold';
</script>

<div class="border-border overflow-x-auto rounded-xl border">
	<table class="w-full min-w-[40rem] table-fixed text-base">
		<colgroup>
			<col class="w-[34%]" />
			<col class="w-[34%]" />
			<col class="w-[14%]" />
			<col class="w-[18%]" />
		</colgroup>
		<thead class="bg-admin-background text-left">
			<tr>
				<th scope="col" class={HEAD_CLASS}>Conversation</th>
				<th scope="col" class={HEAD_CLASS}>Subject</th>
				<th scope="col" class={HEAD_CLASS}>Created</th>
				<th scope="col" class={HEAD_CLASS}><span class="sr-only">Actions</span></th>
			</tr>
		</thead>
		<tbody>
			{#each rows as row (row.key)}
				<tr class="border-border border-t align-top">
					<td class="px-4 py-3.5">
						<span class="line-clamp-2 font-semibold" title={row.conversationTitle}>
							{row.conversationTitle}
						</span>
					</td>
					<td class="px-4 py-3.5">
						<span class="block truncate" title={row.subject}>{row.subject}</span>
					</td>
					<td class="text-muted-foreground px-4 py-3.5 text-sm">
						{row.createdAt ? formatDateShort(row.createdAt) : ''}
					</td>
					<td class="px-4 py-2.5 text-right whitespace-nowrap">
						<button
							type="button"
							onclick={() => onPreview(row)}
							class="text-primary inline-flex h-8 cursor-pointer items-center px-3 text-sm font-medium hover:underline"
						>
							Preview<span class="sr-only">: {row.conversationTitle}</span>
						</button>
						<a
							href={templateHref(row, emailType)}
							class="text-primary inline-flex h-8 items-center px-3 text-sm font-medium hover:underline"
						>
							Edit<span class="sr-only">: {row.conversationTitle}</span>
						</a>
					</td>
				</tr>
			{/each}
		</tbody>
	</table>
</div>

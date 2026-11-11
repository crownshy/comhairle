<script lang="ts">
	import ReportBody from '$lib/reports/ReportBody.svelte';
	import ExpandableContent from './ExpandableContent.svelte';
	import { REPORT_TEXT_CLASS } from '$lib/reports/styles';

	type Props = {
		content: string;
		conversationId: string;
	};

	let { content, conversationId }: Props = $props();
</script>

<!-- No frame: a page reads as the report itself, as it does in the editor. -->
<article>
	<ExpandableContent collapsedHeight={28}>
		<div class={REPORT_TEXT_CLASS}>
			<ReportBody {content} {conversationId} />
		</div>
	</ExpandableContent>
</article>

<script lang="ts">
	import ReportBody from '$lib/reports/ReportBody.svelte';
	import ExpandableContent from './ExpandableContent.svelte';
	import { REPORT_TEXT_CLASS } from './styles';

	type Props = {
		title: string;
		content: string;
		conversationId: string;
	};

	let { title, content, conversationId }: Props = $props();
</script>

<article class="bg-card border-accent rounded-t-lg rounded-b-3xl border px-4 py-6">
	<h2 class="text-muted-foreground text-2xl leading-8 font-semibold">{title}</h2>
	<div class="pt-2">
		<ExpandableContent collapsedHeight={28}>
			<div class={REPORT_TEXT_CLASS}>
				<ReportBody {content} {conversationId} />
			</div>
		</ExpandableContent>
	</div>
</article>

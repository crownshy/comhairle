<script lang="ts">
	let { polisReportId, polisUrl }: { polisReportId: string; polisUrl: string } = $props();
</script>

<iframe class="min-h-[700px] w-full border-none" src={`${polisUrl}/report/${polisReportId}`} />

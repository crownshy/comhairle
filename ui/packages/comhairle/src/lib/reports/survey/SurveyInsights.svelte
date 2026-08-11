<script lang="ts">
	import Doughnut from '$lib/components/Charts/Doughnut.svelte';
	import KdePlot from '$lib/components/Charts/KdePlot.svelte';
	import Responses from './Responses.svelte';
	import type { InsightQuestion } from '@crownshy/api-client/api';
	import SurveyBarChart from './SurveyBarChart.svelte';
	import { isHeyFormFieldKind } from '$lib/tools/heyform/utils';

	type ChartType = 'Bar' | 'Doughnut' | 'KdePlot' | 'Text';

	interface Props {
		data: InsightQuestion[];
	}

	let { data }: Props = $props();

	function getType(kind: string | null | undefined): ChartType | undefined {
		if (!kind || !isHeyFormFieldKind(kind)) {
			return undefined;
		}
		switch (kind) {
			case 'yes_no':
				return 'Doughnut';
			case 'opinion_scale':
			case 'picture_choice':
			case 'multiple_choice':
				return 'Bar';
			case 'number':
			case 'short_text':
			case 'long_text':
				return 'Text';
			case 'group':
			case 'welcome':
			case 'thank_you':
			case 'statement':
			case 'file_upload':
			case 'rating':
			case 'date':
			case 'date_range':
			case 'time':
			case 'input_table':
			case 'payment':
			case 'full_name':
			case 'address':
			case 'email':
			case 'url':
			case 'phone_number':
			case 'country_selector':
			case 'signature':
			case 'legal_terms':
			case 'submit_date':
			case 'hidden_fields':
			case 'variable':
			case 'hidden_checkbox':
			case 'custom_text':
			case 'custom_single':
			case 'custom_multiple':
			case 'custom_date':
			case 'custom_number':
			case 'custom_checkbox':
				return undefined;
		}
	}

	$inspect(data).with(console.log);
</script>

{#each data as section (section.id)}
	{@const type = getType(section.kind)}
	{#if type !== undefined}
		<div class="py-10">
			<h2 class="text-md font-bold">{section.title}</h2>
			{#if type === 'Bar'}
				<SurveyBarChart data={section.choices} x="label" y="count" />
			{/if}
			{#if type === 'Doughnut'}
				<Doughnut data={section.choices} key="label" value="count" />
			{/if}
			{#if type === 'KdePlot'}
				<p>KdePlot chart</p>
				<!-- <KdePlot data={section.chart.data} /> -->
			{/if}
			{#if type === 'Text'}
				<p>Text chart</p>
				<!-- <Responses data={section.chart.data} /> -->
			{/if}
		</div>
	{/if}
{/each}

<script lang="ts">
	import { page } from '$app/state';
	import FilterChips from '$lib/components/FilterChips.svelte';
	import {
		CONVERSATION_STATUS_FILTER_LABELS,
		CONVERSATION_STATUS_FILTERS,
		statusFilterHref,
		type ConversationStatusFilter
	} from './conversationFilters';

	let { active }: { active: ConversationStatusFilter } = $props();

	const options = CONVERSATION_STATUS_FILTERS.map((value) => ({
		value,
		label: CONVERSATION_STATUS_FILTER_LABELS[value]
	}));
</script>

<FilterChips
	{options}
	{active}
	hrefFor={(value) => statusFilterHref(page.url, value)}
	label="Filter conversations"
/>

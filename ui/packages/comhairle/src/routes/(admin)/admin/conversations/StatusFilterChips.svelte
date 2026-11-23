<script lang="ts">
	import FilterBadges from '$lib/components/FilterBadges.svelte';
	import {
		CONVERSATION_STATUS_FILTER_LABELS,
		CONVERSATION_STATUS_FILTERS
	} from './conversationFilters';

	const options = CONVERSATION_STATUS_FILTERS.map(
		(value) => [value, CONVERSATION_STATUS_FILTER_LABELS[value]] as const
	);
</script>

<FilterBadges param="filter" {options} label="Filter conversations" />

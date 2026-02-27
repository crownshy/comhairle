<script module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import Pager from './Pager.svelte';

	const { Story } = defineMeta({
		title: '$lib/pagination'
	});
</script>

<Story name="first page">
	<Pager url={new URL('https://example.org/pagination')} pageSize={6} count={13} />
</Story>

<Story name="middle page">
	<Pager url={new URL('https://example.org/pagination?page=2')} pageSize={6} count={13} />
</Story>

<Story name="last page">
	<Pager url={new URL('https://example.org/pagination?page=3')} pageSize={6} count={13} />
</Story>

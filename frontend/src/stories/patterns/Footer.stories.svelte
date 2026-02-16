<script module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import Footer from '$lib/components/Footer.svelte';
	import { Separator } from '$lib/components/ui/separator';

	const { Story } = defineMeta({
		title: 'Patterns/Footer',
		component: Footer,
		tags: ['autodocs']
	});
</script>

<Story name="Footer Component">
	<Footer />
</Story>


<script module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import ComhairleLogo from './ComhairleLogo.svelte';

	const { Story } = defineMeta({
		title: 'Components/ComhairleLogo',
		component: ComhairleLogo,
		tags: ['autodocs']
	});
</script>

<Story name="Default">
	<ComhairleLogo showText={false}/>
</Story>
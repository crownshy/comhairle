<script module>
	import ButtonHelper from './ButtonHelper.svelte';
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import { fn } from '@storybook/test';
	import '../app.css';

	// More on how to set up stories at: https://storybook.js.org/docs/writing-stories
	const { Story } = defineMeta({
		title: 'Example/Button',
		component: ButtonHelper,
		tags: ['autodocs'],
		argTypes: {
			variant: ['default', 'link'],
			size: { control: { type: 'select' }, options: ['default', 'sm', 'lg', 'icon'] }
		},
		args: {
			onclick: fn(),
			content: 'test'
		}
	});
</script>

<!-- More on writing stories with args: https://storybook.js.org/docs/writing-stories/args -->
<Story name="Default" args={{ variant: 'default' }} />

<Story name="Link" args={{ variant: 'link' }} />

<Story name="Destructive" args={{ variant: 'destructive' }} />

<Story name="Outline" args={{ variant: 'outline' }} />

<Story name="Secondary" args={{ variant: 'secondary' }} />

<Story name="Ghost" args={{ variant: 'ghost' }} />

<Story name="Icon" args={{ variant: 'default', size: 'icon', content: '⌨' }} />

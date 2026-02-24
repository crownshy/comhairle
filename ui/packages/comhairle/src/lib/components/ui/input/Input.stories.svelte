<script module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import { Input } from './index.js';

	const { Story } = defineMeta({
		title: 'UI/Input',
		component: Input,
		tags: ['autodocs'],
		argTypes: {
			type: {
				control: { type: 'select' },
				options: ['text', 'email', 'password', 'number', 'search']
			},
			placeholder: { control: 'text' },
			disabled: { control: 'boolean' }
		}
	});
</script>

<Story name="Default">
	<div class="w-[300px]">
		<Input placeholder="Enter text..." />
	</div>
</Story>


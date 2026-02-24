<script module>
	import ButtonHelper from './ButtonHelper.svelte';
	import { Button } from '$lib/components/ui/button';
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import { fn } from '@storybook/test';
	import '../app.css';

	const { Story } = defineMeta({
		title: 'UI/Button',
		component: ButtonHelper,
		tags: ['autodocs'],
		argTypes: {
			variant: {
				control: { type: 'select' },
				options: ['default', 'secondary', 'destructive', 'outline', 'ghost', 'link', 'nav']
			},
			size: { control: { type: 'select' }, options: ['default', 'sm', 'lg', 'icon'] }
		},
		args: {
			onclick: fn(),
			content: 'Button'
		}
	});
</script>

<Story name="Default" args={{ variant: 'default' }} />

<Story name="Secondary" args={{ variant: 'secondary' }} />

<Story name="Destructive" args={{ variant: 'destructive' }} />

<Story name="Outline" args={{ variant: 'outline' }} />

<Story name="Ghost" args={{ variant: 'ghost' }} />

<Story name="Link" args={{ variant: 'link' }} />

<Story name="Icon" args={{ variant: 'default', size: 'icon', content: '⌨' }} />

<Story name="Small" args={{ variant: 'default', size: 'sm', content: 'Small' }} />

<Story name="Large" args={{ variant: 'default', size: 'lg', content: 'Large' }} />

<Story name="All Variants">
	<div class="flex flex-wrap items-center gap-3">
		<Button variant="default">Default</Button>
		<Button variant="secondary">Secondary</Button>
		<Button variant="destructive">Destructive</Button>
		<Button variant="outline">Outline</Button>
		<Button variant="ghost">Ghost</Button>
		<Button variant="link">Link</Button>
	</div>
</Story>

<Story name="All Sizes">
	<div class="flex items-center gap-3">
		<Button variant="default" size="sm">Small</Button>
		<Button variant="default" size="default">Default</Button>
		<Button variant="default" size="lg">Large</Button>
		<Button variant="default" size="icon">
			<svg
				xmlns="http://www.w3.org/2000/svg"
				width="16"
				height="16"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2"
				stroke-linecap="round"
				stroke-linejoin="round"
			>
				<path d="M5 12h14" />
				<path d="M12 5v14" />
			</svg>
		</Button>
	</div>
</Story>

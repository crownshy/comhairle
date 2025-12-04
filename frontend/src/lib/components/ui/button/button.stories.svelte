<script module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import { Button } from './index.js';

	const { Story } = defineMeta({
		title: 'Components/Button',
		component: Button,
		tags: ['autodocs'],
		argTypes: {
			variant: {
				control: { type: 'select' },
				options: ['default', 'destructive', 'outline', 'secondary', 'ghost', 'link', 'nav']
			},
			size: {
				control: { type: 'select' },
				options: ['default', 'sm', 'lg', 'icon']
			}
		}
	});
</script>

<Story name="Default">
	<Button>Default Button</Button>
</Story>

<Story name="Secondary">
	<Button variant="secondary">Secondary Button</Button>
</Story>

<Story name="Destructive">
	<Button variant="destructive">Destructive Button</Button>
</Story>

<Story name="Outline">
	<Button variant="outline">Outline Button</Button>
</Story>

<Story name="Ghost">
	<Button variant="ghost">Ghost Button</Button>
</Story>

<Story name="Link">
	<Button variant="link">Link Button</Button>
</Story>

<Story name="All Variants">
	<div class="flex flex-wrap gap-4 p-8">
		<Button variant="default">Default</Button>
		<Button variant="secondary">Secondary</Button>
		<Button variant="destructive">Destructive</Button>
		<Button variant="outline">Outline</Button>
		<Button variant="ghost">Ghost</Button>
		<Button variant="link">Link</Button>
	</div>
</Story>

<Story name="All Sizes">
	<div class="flex flex-wrap items-center gap-4 p-8">
		<Button size="sm">Small</Button>
		<Button size="default">Default</Button>
		<Button size="lg">Large</Button>
		<Button size="icon">
			<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
				<path d="M5 12h14"></path>
				<path d="M12 5v14"></path>
			</svg>
		</Button>
	</div>
</Story>

<Story name="With Icons">
	<div class="flex flex-wrap gap-4 p-8">
		<Button>
			<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
				<path d="M5 12h14"></path>
				<path d="M12 5v14"></path>
			</svg>
			Add Item
		</Button>
		<Button variant="secondary">
			<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
				<path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path>
				<polyline points="7 10 12 15 17 10"></polyline>
				<line x1="12" x2="12" y1="15" y2="3"></line>
			</svg>
			Download
		</Button>
		<Button variant="destructive">
			<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
				<path d="M3 6h18"></path>
				<path d="M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6"></path>
				<path d="M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2"></path>
			</svg>
			Delete
		</Button>
	</div>
</Story>

<Story name="As Link">
	<div class="flex flex-wrap gap-4 p-8">
		<Button href="#">Default Link</Button>
		<Button variant="secondary" href="#">Secondary Link</Button>
		<Button variant="outline" href="#">Outline Link</Button>
	</div>
</Story>

<Story name="Disabled States">
	<div class="flex flex-wrap gap-4 p-8">
		<Button disabled>Disabled Default</Button>
		<Button variant="secondary" disabled>Disabled Secondary</Button>
		<Button variant="destructive" disabled>Disabled Destructive</Button>
		<Button variant="outline" disabled>Disabled Outline</Button>
	</div>
</Story>

<Story name="Design System Colors">
	<div class="flex flex-col gap-6 p-8">
		<h3 class="text-lg font-semibold">CS_Blue Palette Preview</h3>
		<div class="flex flex-wrap gap-2">
			<div class="w-16 h-16 rounded bg-cs-blue-50 flex items-center justify-center text-xs">50</div>
			<div class="w-16 h-16 rounded bg-cs-blue-100 flex items-center justify-center text-xs">100</div>
			<div class="w-16 h-16 rounded bg-cs-blue-200 flex items-center justify-center text-xs">200</div>
			<div class="w-16 h-16 rounded bg-cs-blue-300 flex items-center justify-center text-xs">300</div>
			<div class="w-16 h-16 rounded bg-cs-blue-400 flex items-center justify-center text-xs text-white">400</div>
			<div class="w-16 h-16 rounded bg-cs-blue-500 flex items-center justify-center text-xs text-white">500</div>
			<div class="w-16 h-16 rounded bg-cs-blue-600 flex items-center justify-center text-xs text-white">600</div>
			<div class="w-16 h-16 rounded bg-cs-blue-700 flex items-center justify-center text-xs text-white">700</div>
			<div class="w-16 h-16 rounded bg-cs-blue-800 flex items-center justify-center text-xs text-white">800</div>
			<div class="w-16 h-16 rounded bg-cs-blue-900 flex items-center justify-center text-xs text-white">900</div>
			<div class="w-16 h-16 rounded bg-cs-blue-950 flex items-center justify-center text-xs text-white">950</div>
		</div>
		
		<h3 class="text-lg font-semibold mt-4">Buttons with CS_Blue</h3>
		<div class="flex flex-wrap gap-4">
			<button class="px-4 py-2 rounded bg-cs-blue-500 text-white hover:bg-cs-blue-600">CS Blue 500</button>
			<button class="px-4 py-2 rounded bg-cs-blue-600 text-white hover:bg-cs-blue-700">CS Blue 600</button>
			<button class="px-4 py-2 rounded bg-cs-blue-700 text-white hover:bg-cs-blue-800">CS Blue 700</button>
		</div>
	</div>
</Story>

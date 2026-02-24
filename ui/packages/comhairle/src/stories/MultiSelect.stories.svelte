<script lang="ts" module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import MultiSelect from '$lib/components/ui/mutli-select/multi-select.svelte';
	import type { Option } from '$lib/components/ui/mutli-select/multi-select.svelte';

	const { Story } = defineMeta({
		title: 'Components/MultiSelect',
		component: MultiSelect,
		tags: ['autodocs'],
		argTypes: {
			placeholder: { control: 'text' },
			emptyMessage: { control: 'text' },
			disabled: { control: 'boolean' },
		},
	});

	const languages: Option[] = [
		{ value: 'en', label: 'English' },
		{ value: 'es', label: 'Spanish' },
		{ value: 'fr', label: 'French' },
		{ value: 'de', label: 'German' },
		{ value: 'it', label: 'Italian' },
		{ value: 'pt', label: 'Portuguese' },
		{ value: 'ja', label: 'Japanese' },
		{ value: 'ko', label: 'Korean' },
		{ value: 'zh', label: 'Chinese' },
		{ value: 'ar', label: 'Arabic' },
	];
</script>

<Story name="Default" args={{ defaultOptions: languages, placeholder: 'Select languages...' }}>
	{#snippet children(args)}
		<div class="w-full max-w-sm">
			<MultiSelect {...args} />
		</div>
	{/snippet}
</Story>

<script module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import FileInput from './FileInput.svelte';
	import { MB } from '$lib/utils/units';

	const { Story } = defineMeta({
		title: 'Components/FileInput',
		component: FileInput,
		tags: ['autodocs']
	});
</script>

<Story name="Default">
	<FileInput
		name="files"
		onfile={async () => {}}
		accept=".mp4,.mp3,.webm,.jpg,.jpeg,.png,.webp,.svg"
		maxSize={100 * MB}
	/>
</Story>
<Story name="Drag & drop file">
	<FileInput name="files" accept=".*" />
</Story>
<Story name="Error: Couldn't read file type">
	<FileInput name="files" accept=".*" />
</Story>
<Story name="Error: Unsupported file type">
	<FileInput name="files" accept=".*" />
</Story>
<Story name="Error: Max file size exceeded">
	<FileInput name="files" accept=".*" />
</Story>

<script module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import FileUpload from './FileUpload.svelte';
	import { MB } from '$lib/utils/units';

	const { Story } = defineMeta({
		title: 'Components/FileUpload',
		component: FileUpload,
		tags: ['autodocs']
	});
</script>

<Story name="Default">
	<FileUpload
		name="files"
		onfile={async () => {}}
		accept=".mp4,.mp3,.webm,.jpg,.jpeg,.png,.webp,.svg"
		maxSize={100 * MB}
	/>
</Story>
<Story name="Drag & drop file">
	<FileUpload name="files" accept=".*" />
</Story>
<Story name="Error: Couldn't read file type">
	<FileUpload name="files" accept=".*" />
</Story>
<Story name="Error: Unsupported file type">
	<FileUpload name="files" accept=".*" />
</Story>
<Story name="Error: Max file size exceeded">
	<FileUpload name="files" accept=".*" />
</Story>

<script module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import FileUpload from './FileUpload.svelte';

	const { Story } = defineMeta({
		title: 'Components/FileUpload',
		component: FileUpload,
		tags: ['autodocs']
	});
</script>

<Story name="Default">
	<FileUpload
		onfile={async () => {}}
		accept=".mp4,.mp3,.webm,.jpg,.jpeg,.png,.webp,.svg"
		maxSizeMB={100}
	/>
</Story>
<Story name="Drag & drop file">
	<FileUpload onfile={async () => {}} />
</Story>
<Story name="Error: Couldn't read file type">
	<FileUpload onfile={async () => {}} />
</Story>
<Story name="Error: Unsupported file type">
	<FileUpload onfile={async () => {}} />
</Story>
<Story name="Error: Max file size exceeded">
	<FileUpload onfile={async () => {}} />
</Story>

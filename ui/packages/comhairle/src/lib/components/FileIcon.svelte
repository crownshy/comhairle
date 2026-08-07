<script lang="ts">
	import {
		type Icon,
		File as FileIcon,
		FileVideoCamera,
		FileAudio,
		FileImage
	} from '@lucide/svelte';

	interface Props extends Icon {
		file: File;
	}

	let { file, ...props }: Props = $props();
	let type = $derived(file.type.match(/(.*)\//)?.[1]);
</script>

{#if type === undefined}
	<FileIcon {...props} />
{:else if type === 'image'}
	<FileImage {...props} />
{:else if type === 'video'}
	<FileVideoCamera {...props} />
{:else if type === 'audio'}
	<FileAudio {...props} />
{:else}
	<FileIcon {...props} />
{/if}

<script lang="ts">
	import FileIcon from '@lucide/svelte/icons/file';
	import FileVideoCamera from '@lucide/svelte/icons/file-video-camera';
	import FileAudio from '@lucide/svelte/icons/file-audio';
	import FileImage from '@lucide/svelte/icons/file-image';
	import type { IconProps } from '@lucide/svelte';

	interface Props extends IconProps {
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

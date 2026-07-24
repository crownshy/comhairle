<script lang="ts">
	import Button from '$lib/components/ui/button/button.svelte';
	import { Upload } from 'lucide-svelte';
	import type { ComponentProps } from 'svelte';
	import { m } from '$lib/paraglide/messages';
	import type { MediaDto } from '@crownshy/api-client/api';
	import * as Dialog from '$lib/components/ui/dialog';
	import FileUpload from '$lib/components/FileUpload.svelte';
	import MediaSchema from './schema';
	import { enhance } from '$app/forms';
	import { Input } from '$lib/components/EasyForm';

	interface Props extends Omit<ComponentProps<typeof Button>, 'onclick'> {
		clientSide?: boolean;
		oncomplete?: (media: MediaDto[]) => void;
	}
	const { clientSide, oncomplete, ...props }: Props = $props();

	let uploadForm: HTMLFormElement | null = $state(null);
</script>

<Dialog.Root>
	<Dialog.Trigger>
		<Button {...props} type="button">
			<Upload class="h-4 w-4" />
			{m.upload()}
		</Button>
	</Dialog.Trigger>
	<Dialog.Portal>
		<Dialog.Content class=" min-h-[50vh] min-w-138  rounded-2xl p-8">
			<form
				bind:this={uploadForm}
				method="POST"
				action="/admin/media-library?/upload"
				enctype="multipart/form-data"
				class="flex flex-col"
				use:enhance
			>
				<FileUpload {...MediaSchema.media} maxSizeMB={50} />
				<Input {...MediaSchema.name} label="Filename" type="text" />
				<Input {...MediaSchema.alt} label="Alt" type="text" />
				<Button class="mt-7 self-end" type="submit">Upload</Button>
			</form>
		</Dialog.Content>
	</Dialog.Portal>
</Dialog.Root>

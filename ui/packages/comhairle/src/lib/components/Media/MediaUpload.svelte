<script lang="ts">
	import { enhance } from '$app/forms';
	import Button from '$lib/components/ui/button/button.svelte';
	import { Upload } from 'lucide-svelte';
	import type { ComponentProps } from 'svelte';
	import Media from '$lib/interfaces/Media';
	import { notifications } from '$lib/notifications.svelte';

	interface Props extends Omit<ComponentProps<typeof Button>, 'onclick'> {
		clientSide?: boolean;
	}
	const { clientSide, ...props }: Props = $props();

	let fileInput: HTMLInputElement | undefined;
	let uploadForm: HTMLFormElement | undefined;
</script>

<form
	bind:this={uploadForm}
	method="POST"
	action="/admin/media-library?/upload"
	enctype="multipart/form-data"
	use:enhance
>
	<input
		bind:this={fileInput}
		type="file"
		name="media"
		accept="image/*"
		multiple
		class="hidden"
		aria-hidden="true"
		oninput={async (e) => {
			if (clientSide) {
				const { files } = e.target as HTMLInputElement;
				if (!files) return;
				const response = await Media.upload('/api/media', files);
				let count = 0;
				for (const res of response) {
					if (res.err !== null) {
						notifications.send({
							message: res.err.message,
							priority: 'ERROR'
						});
					} else {
						count++;
					}
				}
				if (count > 0) {
					notifications.send({
						message: `${count} files uploaded`,
						priority: 'SUCCESS'
					});
				}
				return;
			}
			uploadForm?.submit();
		}}
	/>
	<Button
		onclick={() => {
			fileInput?.click();
		}}
		{...props}
	>
		<Upload class="h-4 w-4" />Upload
	</Button>
</form>

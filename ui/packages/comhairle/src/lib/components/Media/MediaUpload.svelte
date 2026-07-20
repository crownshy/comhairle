<script lang="ts">
	import { enhance } from '$app/forms';
	import Button from '$lib/components/ui/button/button.svelte';
	import { Upload } from 'lucide-svelte';
	import type { ComponentProps } from 'svelte';
	import Media from '$lib/interfaces/Media';
	import { notifications } from '$lib/notifications.svelte';
	import Spinner from '$lib/components/ui/spinner/spinner.svelte';
	import { m } from '$lib/paraglide/messages';
	import type { MediaDto } from '@crownshy/api-client/api';
	import { tryCatchAsync } from '$lib/utils/errorHandling';

	interface Props extends Omit<ComponentProps<typeof Button>, 'onclick'> {
		clientSide?: boolean;
		oncomplete?: (media: MediaDto[]) => void;
	}
	const { clientSide, oncomplete, ...props }: Props = $props();

	const media = new Media();

	let fileInput: HTMLInputElement | undefined;
	let uploadForm: HTMLFormElement | undefined;

	let uploading = $state<boolean>(false);

	async function oninput(e: Event) {
		if (clientSide) {
			const rawFiles = (e.target as HTMLInputElement).files;
			if (!rawFiles) return;
			// FIX: Check why the erroring here is incorrect
			const response = await media.upload('/api/media', Media.sanitiseMulti(rawFiles));

			if (response.err !== null) {
				notifications.send({
					message: response.err.message,
					priority: 'ERROR'
				});
				return;
			}
			const result = await tryCatchAsync(() => response.ok.json());
			if (result.err !== null) {
				// NOTE: Data might be uploaded but wouldn't count as uploaded if the response can't be parsed, probably need to fix at some point
				return;
			}

			oncomplete?.(result.ok as MediaDto[]);
			return;
		}
		uploadForm?.submit();
	}
</script>

<form
	bind:this={uploadForm}
	method="POST"
	action="/admin/media-library?/upload"
	enctype="multipart/form-data"
	use:enhance={() => {
		uploading = true;
		return async ({ update }) => {
			await update();
			uploading = false;
		};
	}}
>
	<input
		bind:this={fileInput}
		type="file"
		name="media"
		multiple
		class="hidden"
		aria-hidden="true"
		{oninput}
	/>
	<Button
		disabled={uploading}
		onclick={() => {
			fileInput?.click();
		}}
		{...props}
	>
		{#if uploading}
			<Spinner />
		{:else}
			<Upload class="h-4 w-4" />
		{/if}
		{m.upload()}
	</Button>
</form>

<script lang="ts">
	import * as Dialog from '$lib/components/ui/dialog';
	import type { HTMLMediaElement } from '$lib/utils/types';
	import '$lib/components/Media/MediaBackground.css';
	import Button from '$lib/components/ui/button/button.svelte';
	import { Edit, Trash2 } from 'lucide-svelte';
	import * as AlertDialog from '$lib/components/ui/alert-dialog';
	import { apiClient } from '@crownshy/api-client/client';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import { notifications } from '$lib/notifications.svelte';
	import { invalidate } from '$app/navigation';
	import DetailsField from './DetailsField.svelte';

	interface Props {
		type: HTMLMediaElement | undefined;
		id: string;
		filename: string;
		name: string;
		alt: string;
		src: string;
		close: () => void;
	}

	const { id, type, filename, name: initialName, alt: initialAlt, src, close }: Props = $props();

	let editable = $state<boolean>(false);
	let deleteDialogOpen = $state<boolean>(false);

	async function deleteMedia() {
		const response = await tryCatchAsync(() =>
			apiClient.DeleteMedia(undefined, {
				params: {
					media_id: id
				}
			})
		);

		if (response.err !== null) {
			notifications.send({
				message: 'Could not delete media. Please try again later.',
				priority: 'ERROR'
			});
			return;
		}

		deleteDialogOpen = false;
		close();
		await invalidate('media-library:media');
	}

	async function updateMedia() {
		const response = await tryCatchAsync(() =>
			// FIX: Currently not uploading the new name and alt
			apiClient.UpdateMedia(
				{ name: initialName },
				{
					params: {
						media_id: id
					}
				}
			)
		);

		if (response.err !== null) {
			notifications.send({
				message: `Failed to update ${filename}. Please try again.`,
				priority: 'ERROR'
			});
			return;
		}
		notifications.send({
			message: `Updated ${filename} successfully`,
			priority: 'SUCCESS'
		});
		editable = false;
		await invalidate('media-library:media');
	}
</script>

{#snippet deleteMediaDialog()}
	<AlertDialog.Root bind:open={deleteDialogOpen}>
		<AlertDialog.Trigger>
			<Button
				variant="outline"
				size="sm"
				class="text-destructive"
				aria-label="Delete"
				title="Delete"
			>
				<Trash2 />
			</Button>
		</AlertDialog.Trigger>
		<AlertDialog.Portal>
			<AlertDialog.Content>
				Are you sure you want to delete this?
				<AlertDialog.Footer>
					<AlertDialog.Action onclick={deleteMedia}>Yes</AlertDialog.Action>
					<AlertDialog.Cancel>Cancel</AlertDialog.Cancel>
				</AlertDialog.Footer>
			</AlertDialog.Content>
		</AlertDialog.Portal>
	</AlertDialog.Root>
{/snippet}

<Dialog.Portal>
	<Dialog.Overlay>
		<Dialog.Content class="flex h-[80vh] min-w-[90vw] flex-col overflow-y-scroll">
			<Dialog.Header class="mr-10 flex h-8 flex-row items-center text-xl font-bold">
				<span class="grow">{filename}</span>
				{#if !editable}
					{@render deleteMediaDialog()}
					<Button
						class="ml-2"
						size="sm"
						onclick={() => (editable = true)}
						aria-label="Edit"
						title="Edit"><Edit /></Button
					>
				{/if}
			</Dialog.Header>
			<div class="grid-columns grid gap-7">
				<div
					class="chequered-background flex flex-col justify-center rounded p-1 lg:min-h-[50vh] lg:p-6"
				>
					{#if type === undefined}
						<p>Error reading file type</p>
					{:else if type === 'audio'}
						<audio {src} controls autoplay={false}></audio>
					{:else if type === 'video'}
						<video {src} controls autoplay={false} class="max-h-[60vh]">
							<track kind="captions" />
						</video>
					{:else}
						<img {src} alt="" class="max-h-[60vh] object-contain" />
					{/if}
				</div>
				<aside class="mr-0 flex w-full flex-col lg:mr-auto lg:w-9/10">
					<DetailsField label="Name" initialValue={initialName} {editable} field="name" />
					<DetailsField label="Alt" initialValue={initialAlt} {editable} field="alt" />
					<div class="mt-4 self-end">
						{#if editable}
							<Button variant="outline" onclick={() => (editable = false)}>
								Cancel
							</Button>
							<Button class="ml-2" onclick={updateMedia}>Update</Button>
						{/if}
					</div>
				</aside>
			</div>
		</Dialog.Content>
	</Dialog.Overlay>
</Dialog.Portal>

<style>
	.grid-columns {
		grid-template-columns: 2fr 1fr;

		@media (width <= 64rem) {
			grid-template-columns: 1fr;
		}
	}
</style>

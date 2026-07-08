<script lang="ts">
	import { enhance } from '$app/forms';
	import Button from '$lib/components/ui/button/button.svelte';
	import { SquarePen, Trash2, X } from 'lucide-svelte';
	import DeleteDialog from './DeleteDialog.svelte';
	import { MediaUpload } from '$lib/components/Media';
	import { m } from '$lib/paraglide/messages';
	import { notifications } from '$lib/notifications.svelte';
	import MediaItem from '$lib/components/Media/MediaItem.svelte';
	import MediaLibrary from '$lib/components/Media/MediaLibrary.svelte';
	import * as Dialog from '$lib/components/ui/dialog';
	import type { MediaDto } from '@crownshy/api-client/api';
	import DetailsDialog from './DetailsDialog.svelte';
	import { htmlFromMediaType } from '$lib/utils/types';

	let deleteForm: HTMLFormElement | undefined;

	let bulkEdit = $state<boolean>(false);
	let selected = $state([]);

	const { form, data } = $props();

	$effect(() => {
		if (form?.error) {
			notifications.send({
				message: form.error,
				priority: 'ERROR'
			});
		}
	});

	let details = $state<MediaDto | null>(null);
</script>

<svelte:head>
	<title>Media library - Comhairle Admin</title>
</svelte:head>

<div class="mx-auto w-4/5 p-10">
	<header class="flex flex-row items-baseline justify-between">
		<h1 class="text-4xl font-bold">Media library</h1>
		<div class="flex flex-row gap-4">
			{#if !bulkEdit}
				<MediaUpload />
				<Button
					variant="outline"
					onclick={() => {
						bulkEdit = true;
					}}
				>
					<SquarePen class="h-4 w-4" />{m.edit()}
				</Button>
			{:else}
				<DeleteDialog
					count={selected.length}
					onconfirm={() => {
						deleteForm?.submit();
					}}
				>
					<Button variant="destructive" disabled={selected.length === 0}>
						<Trash2 class="h-4 w-4" />{m.delete()}
					</Button>
				</DeleteDialog>
				<Button
					variant="outline"
					onclick={() => {
						bulkEdit = false;
					}}
				>
					<X class="h-4 w-4" />{m.cancel()}
				</Button>
			{/if}
		</div>
	</header>
	<div class="mt-5">
		<Dialog.Root
			onOpenChange={(open) => {
				if (!open) {
					details = null;
				}
			}}
		>
			<form method="POST" action="?/delete" use:enhance bind:this={deleteForm}>
				<MediaLibrary data={data.media ?? []}>
					{#snippet media(type, media)}
						{#if bulkEdit}
							<label>
								<input
									type="checkbox"
									name="media"
									value={media.id.toString()}
									class="accent-primary absolute top-2 left-2 z-2 h-4 cursor-pointer"
									bind:group={selected}
								/>
								<span>
									<MediaItem {type} {...media} alt="" />
								</span>
							</label>
						{:else}
							<Dialog.Trigger
								class="inline h-full w-full"
								onclick={() => {
									details = media;
								}}
							>
								<MediaItem {type} {...media} alt="" />
							</Dialog.Trigger>
						{/if}
					{/snippet}
				</MediaLibrary>
			</form>
			{#if details !== null}
				<DetailsDialog
					id={details.id}
					type={htmlFromMediaType(details.contentType)}
					filename={details.filename}
					src={details.url}
					alt=""
					close={() => (details = null)}
				/>
			{/if}
		</Dialog.Root>
	</div>
</div>

<style>
	input:checked + * {
		filter: opacity(50%);
	}
</style>

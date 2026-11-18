<script lang="ts">
	import Button from '$lib/components/ui/button/button.svelte';
	import SquarePen from '@lucide/svelte/icons/square-pen';
	import Trash2 from '@lucide/svelte/icons/trash-2';
	import X from '@lucide/svelte/icons/x';
	import DeleteDialog from './DeleteDialog.svelte';
	import { MediaUpload } from '$lib/components/Media';
	import { m } from '$lib/paraglide/messages';
	import MediaLibrary from '$lib/components/Media/MediaLibrary.svelte';
	import * as Dialog from '$lib/components/ui/dialog';
	import type { MediaDto } from '@crownshy/api-client/api';
	import DetailsDialog from './DetailsDialog.svelte';
	import { htmlFromMediaType, type HTMLMediaElement } from '$lib/utils/types';
	import { Badge } from '$lib/components/ui/badge';
	import { capitalise } from '$lib/utils/casingUtils';
	import SectionHeader from '$lib/components/SectionHeader.svelte';
	import TabContent from '$lib/components/TabContent.svelte';
	import { useTour } from '$lib/tours/useTour.svelte';
	import { mediaLibraryTour } from './tour';
	import { invalidate } from '$app/navigation';
	import { key } from '$lib/utils/invalidationKey';
	import { Form, type OnSubmit } from '$lib/components/EasyForm';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import { apiClient } from '@crownshy/api-client/client';
	import { notifications } from '$lib/notifications.svelte';

	useTour(() => mediaLibraryTour);

	const { data } = $props();

	let bulkEdit = $state<boolean>(false);
	let selected = $state.raw([]);
	let details = $state.raw<MediaDto | null>(null);
	let filter = $state<HTMLMediaElement | null>(null);
	let detailsDialogOpen = $state<boolean>(false);

	let items = $derived.by(() => {
		if (!data.media) {
			return [];
		}
		if (filter === null) {
			return data.media;
		}
		return data.media.filter((m) => htmlFromMediaType(m.contentType) === filter);
	});

	const deleteMedia: OnSubmit = async (formData) => {
		const media = formData.getAll('media') as string[];

		const results = await tryCatchAsync(async () => {
			for (const id of media) {
				await apiClient.DeleteMedia(undefined, { params: { media_id: id } });
			}
		});

		if (results.err !== null) {
			notifications.addFlash({
				message: 'Failed to delete some media, please try again',
				priority: 'ERROR'
			});
		}

		bulkEdit = false;
		selected = [];
		await invalidate(key('admin/media-library/media'));
	};
</script>

<svelte:head>
	<title>Media library - Comhairle Admin</title>
</svelte:head>

{#snippet filterButton(type: HTMLMediaElement)}
	<label class="cursor-pointer">
		<input
			type="radio"
			name="filter"
			class="hidden"
			value={type}
			onclick={(e) => {
				e.preventDefault();
				if (filter === type) {
					filter = null;
					return;
				}
				filter = type;
			}}
		/>
		<Badge variant={filter === type ? 'primary' : 'default'} class="px-3 py-1 text-xs">
			{capitalise(type)}
		</Badge>
	</label>
{/snippet}
<SectionHeader
	content={{
		eyebrow: 'Library',
		title: 'Media library',
		description: 'Images, audio and video you can use across your conversations.'
	}}
>
	{#snippet actions()}
		<div class="flex flex-col gap-4 md:flex-row">
			{#if !bulkEdit}
				<MediaUpload oncomplete={() => void invalidate(key('admin/media-library/media'))} />
				<Button variant="outline" onclick={() => (bulkEdit = true)} icon={SquarePen}>
					{m.edit()}
				</Button>
			{:else}
				<DeleteDialog count={selected.length}>
					<Button variant="destructive" disabled={selected.length === 0} icon={Trash2}>
						{m.delete()}
					</Button>
				</DeleteDialog>
				<Button
					variant="outline"
					onclick={() => {
						bulkEdit = false;
						selected = [];
					}}
					icon={X}
				>
					{m.cancel()}
				</Button>
			{/if}
		</div>
	{/snippet}
</SectionHeader>

<TabContent>
	<div data-tour="media-filters" class="flex flex-row gap-4">
		{@render filterButton('audio')}
		{@render filterButton('image')}
		{@render filterButton('video')}
	</div>
	<div class="mt-5">
		<Dialog.Root
			bind:open={detailsDialogOpen}
			onOpenChange={(open) => {
				if (!open) {
					details = null;
				}
			}}
		>
			{#if bulkEdit}
				<Form id="bulk-delete-media" onsubmit={deleteMedia}>
					<MediaLibrary media={items} mode="multi-select" bind:selected />
				</Form>
			{:else}
				<MediaLibrary
					media={items}
					mode="view"
					onDetailsOpen={(media) => {
						details = media;
						detailsDialogOpen = true;
					}}
				/>
			{/if}
			{#if details !== null}
				<DetailsDialog
					{...details}
					type={htmlFromMediaType(details.contentType)}
					close={() => (details = null)}
				/>
			{/if}
		</Dialog.Root>
	</div>
</TabContent>

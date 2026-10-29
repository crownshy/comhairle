<script lang="ts">
	import { m } from '$lib/paraglide/messages';
	import { SquarePen, Trash2, X } from '@lucide/svelte';
	import { MediaUpload } from '$lib/components/Media';
	import { Badge } from '$lib/components/ui/badge';
	import Button from '$lib/components/ui/button/button.svelte';
	import MediaLibrary from '$lib/components/Media/MediaLibrary.svelte';
	import DetailsDialog from './DetailsDialog.svelte';
	import SectionHeader from '$lib/components/SectionHeader.svelte';
	import TabContent from '$lib/components/TabContent.svelte';
	import Dialog from '$lib/components/Dialog.svelte';
	import { Form, Submit, type OnSubmit } from '$lib/components/EasyForm';
	import { htmlFromMediaType, type HTMLMediaElement } from '$lib/utils/types';
	import { capitalise } from '$lib/utils/casingUtils';
	import { useTour } from '$lib/tours/useTour.svelte';
	import { mediaLibraryTour } from './tour';
	import type { MediaDto } from '@crownshy/api-client/api';
	import { apiClient } from '@crownshy/api-client/client';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import { invalidate } from '$app/navigation';
	import { key } from '$lib/utils/invalidationKey';
	import { notifications } from '$lib/notifications.svelte';

	useTour(() => mediaLibraryTour);

	const { data } = $props();

	let bulkEdit = $state<boolean>(false);
	let selected = $state.raw([]);
	let details = $state.raw<MediaDto | undefined>(undefined);
	let filter = $state<HTMLMediaElement | null>(null);

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

{#snippet FilterButton(type: HTMLMediaElement)}
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

<Form onsubmit={deleteMedia} invalidate="admin/media">
	{#snippet children({ isSubmitting })}
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
						<MediaUpload
							oncomplete={() => void invalidate(key('admin/media-library/media'))}
						/>
						<Button
							variant="outline"
							icon={SquarePen}
							onclick={() => void (bulkEdit = true)}
						>
							{m.edit()}
						</Button>
					{:else}
						<Button
							variant="destructive"
							disabled={selected.length === 0}
							icon={Trash2}
							command="open-dialog:delete-media"
						>
							{m.delete()}
						</Button>
						<Button
							variant="outline"
							icon={X}
							onclick={() => {
								bulkEdit = false;
								selected = [];
							}}
						>
							{m.cancel()}
						</Button>
					{/if}
				</div>
			{/snippet}
		</SectionHeader>

		<Dialog id="delete-media" title="Are you sure?" --min-width="30vw">
			{#snippet PrimaryAction()}
				<Submit
					text={m.delete()}
					icon={Trash2}
					variant="destructive"
					{isSubmitting}
					command="close-dialog:delete-media"
				/>
			{/snippet}
			{#snippet SecondaryAction()}
				<Button
					variant="outline"
					command="close-dialog:delete-media"
					disabled={isSubmitting}
				>
					{m.cancel()}
				</Button>
			{/snippet}
		</Dialog>

		<TabContent>
			<div data-tour="media-filters" class="flex flex-row gap-4">
				{@render FilterButton('audio')}
				{@render FilterButton('image')}
				{@render FilterButton('video')}
			</div>
			{#if !bulkEdit}
				<MediaLibrary
					media={data.media}
					mode="view"
					onDetailsOpen={(media) => {
						details = media;
					}}
				/>
			{:else}
				<MediaLibrary media={data.media} mode="multi-select" bind:selected />
			{/if}
		</TabContent>
	{/snippet}
</Form>

<DetailsDialog
	id="media-details"
	media={details}
	type={htmlFromMediaType(details?.contentType ?? 'image/jpeg')}
/>

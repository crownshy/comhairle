<script lang="ts">
	import Button from '$lib/components/ui/button/button.svelte';
	import { SquarePen, Trash2, X } from 'lucide-svelte';
	import { MediaUpload } from '$lib/components/Media';
	import { m } from '$lib/paraglide/messages';
	import MediaItem from '$lib/components/Media/MediaItem.svelte';
	import MediaLibrary from '$lib/components/Media/MediaLibrary.svelte';
	import DetailsDialog from './DetailsDialog.svelte';
	import { htmlFromMediaType, type HTMLMediaElement } from '$lib/utils/types';
	import { Badge } from '$lib/components/ui/badge';
	import { capitalise } from '$lib/utils/casingUtils';
	import SectionHeader from '$lib/components/SectionHeader.svelte';
	import TabContent from '$lib/components/TabContent.svelte';
	import { useTour } from '$lib/tours/useTour.svelte';
	import { mediaLibraryTour } from './tour';
	import ErrorMessage from '$lib/components/ErrorMessage.svelte';
	import { Form, Submit, type HandleSubmission } from '$lib/components/EasyForm';
	import type { MediaDto } from '@crownshy/api-client/api';
	import { apiClient } from '@crownshy/api-client/client';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import { notifications } from '$lib/notifications.svelte';
	import { invalidate } from '$app/navigation';
	import { key } from '$lib/utils/invalidationKey';
	import PageSpinner from '$lib/components/PageSpinner.svelte';
	import Dialog from '$lib/components/Dialog.svelte';

	useTour(() => mediaLibraryTour);

	const { data } = $props();

	let bulkEdit = $state<boolean>(false);
	let selected = $state([]);
	let details = $state.raw<MediaDto | null>(null);
	let filter = $state<HTMLMediaElement | null>(null);

	const handleDelete: HandleSubmission = async (formData) => {
		const media = formData.getAll('media') as string[];

		const results = await tryCatchAsync(async () => {
			for (const id of media) {
				apiClient.DeleteMedia(undefined, { params: { media_id: id } });
			}
		});

		if (results.err !== null) {
			notifications.addFlash({
				message: 'Failed to delete some media, please try again',
				priority: 'ERROR'
			});
		}

		await invalidate(key('admin/media'));
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

<Form handleSubmission={handleDelete}>
	{#snippet children(isSubmitting)}
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
						<MediaUpload />
						<Button
							variant="outline"
							icon={SquarePen}
							onclick={() => (bulkEdit = true)}
						>
							{m.edit()}
						</Button>
					{:else}
						<Button
							variant="destructive"
							disabled={selected.length !== 0}
							icon={Trash2}
							dialogOpen="delete-media"
						>
							{m.delete()}
						</Button>
						<Button variant="outline" icon={X} onclick={() => (bulkEdit = false)}>
							{m.cancel()}
						</Button>
					{/if}
				</div>
			{/snippet}
		</SectionHeader>

		<Dialog id="delete-media" title="Are you sure?">
			{#snippet PrimaryAction()}
				<Submit text="Delete" icon={Trash2} variant="destructive" {isSubmitting} />
			{/snippet}
			{#snippet SecondaryAction({ close })}
				<Button variant="outline" onclick={close} disabled={isSubmitting}>Cancel</Button>
			{/snippet}
		</Dialog>

		<TabContent>
			{#await data.streamedMedia}
				<PageSpinner subject="media" />
			{:then media}
				{#if media.err !== null}
					<ErrorMessage>Could not load media, please try again</ErrorMessage>
				{:else}
					{@const items = (() => {
						if (filter === null) return media.ok;
						return media.ok.filter((m) => htmlFromMediaType(m.contentType) === filter);
					})()}
					<div data-tour="media-filters" class="flex flex-row gap-4">
						{@render FilterButton('audio')}
						{@render FilterButton('image')}
						{@render FilterButton('video')}
					</div>
					<MediaLibrary data={items}>
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
									<MediaItem {type} {...media} />
								</label>
							{:else}
								<button
									class="block aspect-video h-full w-full"
									onclick={() => (details = media)}
								>
									<MediaItem {type} {...media} />
								</button>
							{/if}
						{/snippet}
					</MediaLibrary>
				{/if}
			{/await}
		</TabContent>
	{/snippet}
</Form>

{#if details}
	<DetailsDialog
		{...details}
		type={htmlFromMediaType(details.contentType)}
		close={() => (details = null)}
	/>
{/if}

<style>
	input:checked + * {
		filter: opacity(50%);
	}
</style>

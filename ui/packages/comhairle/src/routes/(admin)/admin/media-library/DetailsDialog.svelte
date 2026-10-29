<script lang="ts">
	import '$lib/components/Media/MediaBackground.css';
	import Button from '$lib/components/ui/button/button.svelte';
	import { SquarePen, Trash2 } from '@lucide/svelte';
	import { apiClient } from '@crownshy/api-client/client';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import { notifications } from '$lib/notifications.svelte';
	import { invalidate } from '$app/navigation';
	import DetailsField from './DetailsField.svelte';
	import { Form, Submit, type OnSubmit } from '$lib/components/EasyForm';
	import type { MediaDto } from '@crownshy/api-client/api';
	import { capitalise } from '$lib/utils/casingUtils';
	import Media from '$lib/interfaces/Media.svelte';
	import Dialog from '$lib/components/Dialog.svelte';
	import { key } from '$lib/utils/invalidationKey';

	type Props = {
		id: string;
		media: MediaDto | undefined;
	};

	let { id, media }: Props = $props();

	let editable = $state<boolean>(false);
	let mediaType = $derived(
		media?.contentType ? Media.mediaTypeFromContentType(media.contentType) : undefined
	);

	async function deleteMedia() {
		if (!media) {
			return;
		}

		const response = await tryCatchAsync(() =>
			apiClient.DeleteMedia(undefined, {
				params: {
					media_id: media.id
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

		close();
		await invalidate(key('admin/media-library/media'));
	}

	const updateMedia: OnSubmit = async (formData) => {
		if (!media) {
			return;
		}
		const newName = (formData.get('name') ?? undefined) as string | undefined;
		const newAlt = (formData.get('alt') ?? undefined) as string | undefined;

		if (!newName || !newAlt) {
			return;
		}

		// If the values are the same then just cancel
		if (newName === media.name && newAlt === media.alt) {
			editable = false;
			return;
		}

		const response = await tryCatchAsync(() =>
			apiClient.UpdateMedia(
				{ name: newName, alt: newAlt },
				{
					params: {
						media_id: media.id
					}
				}
			)
		);

		if (response.err !== null) {
			notifications.send({
				message: `Failed to update ${media.filename}. Please try again.`,
				priority: 'ERROR'
			});
			return;
		}
		notifications.send({
			message: `Updated ${media.filename} successfully`,
			priority: 'SUCCESS'
		});
		editable = false;
		media.name = newName;
		media.alt = newAlt;
		await invalidate(key('admin/media-library/media'));
	};
</script>

<Dialog
	id="delete-this-media"
	title="Are you sure you want to delete this?"
	actions={{ secondary: { label: 'Cancel' } }}
>
	{#snippet PrimaryAction()}
		<Button
			command="close-dialog:delete-this-media,media-details"
			onclick={async () => {
				await deleteMedia();
			}}
			variant="destructive"
			icon={Trash2}
		>
			Delete
		</Button>
	{/snippet}
</Dialog>

{#snippet ReadOnlyField(label: string, value: string)}
	<div class="text-muted-foreground mb-1 text-sm font-semibold">{label}</div>
	<div class="mb-5">{value}</div>
{/snippet}

<Dialog {id} --min-width="90%" --max-width="90vw" title={media?.filename ?? ''}>
	{#snippet HeaderActions()}
		{#if !editable}
			<Button
				class="mr-2"
				size="sm"
				onclick={() => (editable = true)}
				aria-label="Edit"
				title="Edit"
				icon={SquarePen}
			/>
			<Button
				variant="outline"
				size="sm"
				class="text-destructive/90 hover:text-destructive"
				aria-label="Delete"
				title="Delete"
				command="open-dialog:delete-this-media"
				icon={Trash2}
			/>
		{/if}
	{/snippet}
	<div class="grid grid-cols-1 gap-7 lg:grid-cols-[2fr_1fr]">
		{#if media}
			<div
				class="chequered-background flex flex-col justify-center rounded p-1 lg:min-h-[50vh] lg:p-6"
			>
				{#if mediaType === 'audio'}
					<audio src={media.url} controls autoplay={false}></audio>
				{:else if mediaType === 'video'}
					<video src={media.url} controls autoplay={false} class="max-h-[60vh]">
						<track kind="captions" />
					</video>
				{:else}
					<img src={media.url} alt={media.alt} class="max-h-[60vh] object-contain" />
				{/if}
			</div>
			<aside class="mr-0 w-full lg:mr-auto lg:w-9/10">
				<div class="mt-4 self-end">
					{#if editable}
						<Button variant="outline" onclick={() => (editable = false)} type="button">
							Cancel
						</Button>
						<Submit class="ml-2" text="Update" />
					{/if}
				</div>
				<aside class="mr-0 w-full lg:mr-auto lg:w-9/10">
					<Form class="flex flex-col" onsubmit={updateMedia}>
						{#snippet children({ isSubmitting })}
							<DetailsField
								label="Name"
								initialValue={media.name}
								{editable}
								field="name"
								readOnlyField={ReadOnlyField}
							/>
							<DetailsField
								label="Alt"
								initialValue={media.alt}
								{editable}
								field="alt"
								readOnlyField={ReadOnlyField}
							/>
							<section class="grid grid-cols-2 {editable && 'mt-3'}">
								<div>
									{@render ReadOnlyField('Type', capitalise(mediaType ?? ''))}
								</div>
								<div>
									{@render ReadOnlyField(
										'Format',
										Media.getExtension(media.filename)
											?.toUpperCase()
											.slice(1) ?? ''
									)}
								</div>
								<div>
									{@render ReadOnlyField(
										'File size',
										Media.formatBytes(media.size)
									)}
								</div>
								<div>
									{@render ReadOnlyField(
										'Created at',
										new Date(media.createdAt).toLocaleString()
									)}
								</div>
							</section>

							<div class="mt-4 self-end">
								{#if editable}
									<Button
										variant="outline"
										onclick={() => (editable = false)}
										type="button"
									>
										Cancel
									</Button>
									<Submit class="ml-2" text="Update" {isSubmitting} />
								{/if}
							</div>
						{/snippet}
					</Form>
				</aside>
			</aside>
		{/if}
	</div>
</Dialog>

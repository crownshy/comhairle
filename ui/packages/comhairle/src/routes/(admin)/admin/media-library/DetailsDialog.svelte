<script lang="ts">
	import type { HTMLMediaElement } from '$lib/utils/types';
	import '$lib/components/Media/MediaBackground.css';
	import Button from '$lib/components/ui/button/button.svelte';
	import { Edit, Trash2 } from 'lucide-svelte';
	import { apiClient } from '@crownshy/api-client/client';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import { notifications } from '$lib/notifications.svelte';
	import { invalidate } from '$app/navigation';
	import DetailsField from './DetailsField.svelte';
	import { Form, Submit } from '$lib/components/EasyForm';
	import type { MediaDto } from '@crownshy/api-client/api';
	import { capitalise } from '$lib/utils/casingUtils';
	import Media from '$lib/interfaces/Media';
	import Dialog from '$lib/components/Dialog.svelte';
	import ErrorMessage from '$lib/components/ErrorMessage.svelte';
	import { key } from '$lib/utils/invalidationKey';

	interface Props extends MediaDto {
		type: HTMLMediaElement | undefined;
		close: () => void;
	}

	let {
		id,
		type,
		filename,
		name: initialName,
		alt: initialAlt,
		url: src,
		createdAt,
		close
	}: Props = $props();

	let editable = $state<boolean>(false);

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

		close();
		await invalidate(key('admin/media'));
	}

	async function updateMedia(formData: FormData) {
		const newName = (formData.get('name') ?? undefined) as string | undefined;
		const newAlt = (formData.get('alt') ?? undefined) as string | undefined;

		if (!newName || !newAlt) {
			return;
		}

		// If the values are the same then just cancel
		if (newName === initialName && newAlt === initialAlt) {
			editable = false;
			return;
		}

		const response = await tryCatchAsync(() =>
			apiClient.UpdateMedia(
				{ name: newName, alt: newAlt },
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
		initialName = newName;
		initialAlt = newAlt;
		await invalidate(key('admin/media'));
	}
</script>

<Dialog
	{id}
	title="Are you sure you want to delete this?"
	actions={{ secondary: { label: 'Cancel' } }}
>
	{#snippet PrimaryAction()}
		<Button
			onclick={async () => {
				await deleteMedia();
				close();
			}}
		>
			Yes
		</Button>
	{/snippet}
</Dialog>

{#snippet ReadOnlyField(label: string, value: string)}
	<div class="text-muted-foreground mb-1 text-sm font-semibold">{label}</div>
	<div class="mb-5">{value}</div>
{/snippet}

<Dialog id="media-details" class="flex min-w-[90vw] flex-col overflow-y-scroll" title={filename}>
	{#snippet HeaderActions()}
		{#if !editable}
			<Button
				class="mr-2"
				size="sm"
				onclick={() => (editable = true)}
				aria-label="Edit"
				title="Edit"><Edit /></Button
			>
			<Button
				variant="outline"
				size="sm"
				class="text-destructive/90 hover:text-destructive"
				aria-label="Delete"
				title="Delete"
				icon={Trash2}
			/>
		{/if}
	{/snippet}
	<div class="grid grid-cols-1 gap-7 lg:grid-cols-[2fr_1fr]">
		<div
			class="chequered-background flex flex-col justify-center rounded p-1 lg:min-h-[50vh] lg:p-6"
		>
			{#if type === undefined}
				<ErrorMessage>Error trying to read the file type</ErrorMessage>
			{:else if type === 'audio'}
				<audio {src} controls autoplay={false}></audio>
			{:else if type === 'video'}
				<video {src} controls autoplay={false} class="max-h-[60vh]">
					<track kind="captions" />
				</video>
			{:else}
				<img {src} alt={initialAlt} class="max-h-[60vh] object-contain" />
			{/if}
		</div>
		<aside class="mr-0 w-full lg:mr-auto lg:w-9/10">
			<Form class="flex flex-col" handleSubmission={updateMedia}>
				<DetailsField
					label="Name"
					initialValue={initialName}
					{editable}
					field="name"
					readOnlyField={ReadOnlyField}
				/>
				<DetailsField
					label="Alt"
					initialValue={initialAlt}
					{editable}
					field="alt"
					readOnlyField={ReadOnlyField}
				/>
				<section class="grid grid-cols-2">
					<div>
						{@render ReadOnlyField('Type', capitalise(type ?? ''))}
					</div>
					<div>
						{@render ReadOnlyField(
							'Format',
							Media.getExtension(filename)?.toUpperCase().slice(1) ?? ''
						)}
					</div>
					<div>
						{@render ReadOnlyField('Created at', new Date(createdAt).toLocaleString())}
					</div>
				</section>

				<div class="mt-4 self-end">
					{#if editable}
						<Button variant="outline" onclick={() => (editable = false)} type="button">
							Cancel
						</Button>
						<Submit class="ml-2" text="Update" />
					{/if}
				</div>
			</Form>
		</aside>
	</div>
</Dialog>

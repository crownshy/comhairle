<script lang="ts">
	import Button from '$lib/components/ui/button/button.svelte';
	import { Trash2 } from 'lucide-svelte';
	import { Upload } from 'lucide-svelte';
	import type { ComponentProps } from 'svelte';
	import { m } from '$lib/paraglide/messages';
	import type { MediaDto } from '@crownshy/api-client/api';
	import FileInput from '$lib/components/FileInput.svelte';
	import MediaSchema from '$lib/schemas/MediaSchema';
	import { Form, Input, Submit, type HandleSubmission } from '$lib/components/EasyForm';
	import Media from '$lib/interfaces/Media';
	import FileIcon from '$lib/components/FileIcon.svelte';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import { notifications } from '$lib/notifications.svelte';
	import Dialog from '$lib/components/Dialog.svelte';
	import { invalidate } from '$app/navigation';
	import { key } from '$lib/utils/invalidationKey';

	type Props = Omit<ComponentProps<typeof Button>, 'onclick'> & {
		clientSide?: boolean;
		oncomplete?: (media: MediaDto) => void;
	};
	const { oncomplete, ...props }: Props = $props();

	let dialog = $state<HTMLDialogElement | null>(null);
	let file = $state<File | null>(null);
	let fileInput = $state<FileInput | null>(null);
	let nameInput = $state<Input | null>(null);

	const media = new Media();

	const handleSubmission: HandleSubmission = async (formData) => {
		const response = await media.upload('/api/media', formData, {
			schema: MediaSchema
		});

		if (response.err !== null) {
			notifications.send({
				message: 'Failed to upload. Please try again',
				priority: 'ERROR'
			});
			return;
		}

		const mediaDto = await tryCatchAsync(() => response.ok.json());
		if (mediaDto.err !== null) {
			notifications.send({
				message: 'Failed to parse result. Please try refreshing',
				priority: 'ERROR'
			});
			return;
		}

		notifications.send({
			message: 'Successfully uploaded',
			priority: 'SUCCESS'
		});

		dialog?.close();
		oncomplete?.(mediaDto.ok as MediaDto);
		await invalidate(key('admin/media'));
	};
</script>

<Button {...props} type="button" icon={Upload} dialogOpen="upload-media">
	{m.upload()}
</Button>

<Dialog id="upload-media" ref={dialog} --max-width="35rem" --min-width="20rem">
	<Form enctype="multipart/form-data" class="flex flex-col" {handleSubmission}>
		<FileInput
			{...MediaSchema.media}
			bind:this={fileInput}
			onfile={(newFile) => {
				file = newFile;
				nameInput?.setValue(Media.getFilename(file.name));
			}}
			class={file !== null ? 'hidden' : ''}
		/>
		{#if file}
			<div
				class="flex w-full min-w-0 flex-row items-center justify-between rounded-lg p-3 text-xs font-medium shadow-xs"
			>
				<div class="flex basis-3/5 flex-row items-center gap-2">
					<FileIcon {file} />
					<span>{file.name}</span>
				</div>
				<div class="flex flex-row items-center justify-evenly gap-4">
					<span class="text-muted-foreground">
						{Media.getExtension(file.name)?.toUpperCase()}
					</span>
					<span class="text-muted-foreground">{Media.formatBytes(file.size)}</span>
					<Button
						title="Remove file"
						aria-label="Remove file"
						variant="ghost"
						size="sm"
						icon={Trash2}
						class="text-destructive hover:text-destructive/80 rounded-sm"
						onclick={() => {
							file = null;
							nameInput?.setValue('');
							nameInput?.setError('');
							fileInput?.clear();
						}}
					/>
				</div>
			</div>
		{/if}
		<Input {...MediaSchema.name} bind:this={nameInput} label="Filename" />
		<Input {...MediaSchema.alt} label="Alt" placeholder="e.g. Company logo" />
		<Submit class="mt-7 self-end" text="Upload" />
	</Form>
</Dialog>

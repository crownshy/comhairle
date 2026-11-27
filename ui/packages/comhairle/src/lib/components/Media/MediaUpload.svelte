<script lang="ts">
	import Button from '$lib/components/ui/button/button.svelte';
	import Trash2 from '@lucide/svelte/icons/trash-2';
	import Upload from '@lucide/svelte/icons/upload';
	import type { ComponentProps } from 'svelte';
	import { m } from '$lib/paraglide/messages';
	import type { MediaDto } from '@crownshy/api-client/api';
	import FileInput from '$lib/components/FileInput.svelte';
	import MediaSchema from '$lib/schemas/MediaSchema';
	import { Form, Input, Submit, type OnSubmit } from '$lib/components/EasyForm';
	import Media, { UploaderProgress } from '$lib/interfaces/Media.svelte';
	import FileIcon from '$lib/components/FileIcon.svelte';
	import StepIcon from './StepIcon.svelte';
	import ProgressBar from '../ProgressBar.svelte';
	import Dialog from '$lib/components/Dialog.svelte';
	import ErrorMessage from '../ErrorMessage.svelte';

	type Props = Omit<ComponentProps<typeof Button>, 'onclick'> & {
		oncomplete?: (media: MediaDto) => void;
	};

	const { oncomplete, ...props }: Props = $props();

	let file = $state<File | null>(null);
	let fileInput = $state<FileInput | null>(null);
	let nameInput = $state<Input | null>(null);
	let page = $state<number>(1);
	let RemoveFileButton = $state.raw<HTMLElement | null>(null);

	const mediaUploader = new Media.uploader();

	const onsubmit: OnSubmit = async (formData) => {
		const [media] = Media.normalise(formData.get(MediaSchema.media.name) as File | null);
		const name = formData.get(MediaSchema.name.name) as string | null;
		const alt = formData.get(MediaSchema.alt.name) as string | null;

		if (!media || !name || !alt) {
			return;
		}

		page++;

		const upload = await mediaUploader.upload({
			media,
			name,
			alt
		});

		if (upload.err !== null) {
			return;
		}

		oncomplete?.(upload.ok);
	};
</script>

{#snippet Step(step: keyof typeof UploaderProgress)}
	{@const diff = mediaUploader.step - UploaderProgress[step]}
	<div class="flex items-center gap-x-2">
		<StepIcon state={diff} error={diff === 0 && !!mediaUploader.error} />
		<span class="text-lg font-semibold">{step}</span>
	</div>
{/snippet}

<Button {...props} type="button" icon={Upload} command="open-dialog:upload-media">
	{m.upload()}
</Button>

<Dialog
	id="upload-media"
	onlyHardDismiss={page === 2}
	closeHidden={page === 2}
	--max-width="35rem"
	--min-width="20rem"
	class={page === 2 ? 'md:w-75' : 'md:w-138'}
>
	<section class="flex flex-col rounded-2xl p-5 duration-350">
		<section class="mb-5 flex flex-row justify-center gap-x-3">
			{#each { length: 2 }, index}
				<div
					data-slot="page {index + 1}"
					aria-hidden="true"
					class="size-2 rounded-full bg-slate-500/70 transition-colors duration-300"
					class:bg-primary!={index + 1 === page}
				></div>
			{/each}
		</section>
		{#if page === 1}
			<div class="flex flex-col items-center gap-y-2">
				<Form
					enctype="multipart/form-data"
					class="flex flex-col"
					{onsubmit}
					additionalReset={() => {
						RemoveFileButton?.click();
					}}
				>
					{#snippet children({ isSubmitting })}
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
									<span class="text-muted-foreground">
										{Media.formatBytes(file.size)}
									</span>
									<Button
										bind:ref={RemoveFileButton}
										title="Remove file"
										aria-label="Remove file"
										variant="ghost"
										size="sm"
										class="text-destructive hover:text-destructive/80 rounded-sm"
										icon={Trash2}
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
						<Input
							{...MediaSchema.name}
							bind:this={nameInput}
							label="Filename"
							type="text"
						/>
						<Input
							{...MediaSchema.alt}
							label="Alt"
							type="text"
							placeholder="e.g. Company logo"
						/>
						<Submit class="mt-7 self-end" text="Upload" {isSubmitting} />
					{/snippet}
				</Form>
			</div>
		{:else}
			<div class="flex flex-col gap-y-2 self-start">
				{@render Step('Creating')}
				{@render Step('Uploading')}
				{#if mediaUploader.step >= UploaderProgress.Uploading}
					<ProgressBar
						percentage={mediaUploader.percentage}
						error={!!mediaUploader.error}
					/>
				{/if}
				{#if mediaUploader.error}
					<ErrorMessage size="sm">{mediaUploader.error}</ErrorMessage>
				{/if}
				<Button
					type="button"
					class="mt-3 self-end"
					command="close-dialog:upload-media"
					onclick={() => void (page = 1)}
				>
					{mediaUploader.error || mediaUploader.step === UploaderProgress.Completed
						? 'Finish'
						: 'Close'}
				</Button>
			</div>
		{/if}
	</section>
</Dialog>

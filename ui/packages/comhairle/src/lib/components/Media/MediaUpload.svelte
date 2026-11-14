<script lang="ts">
	import Button from '$lib/components/ui/button/button.svelte';
	import { Trash2 } from 'lucide-svelte';
	import { Upload } from 'lucide-svelte';
	import type { ComponentProps } from 'svelte';
	import { m } from '$lib/paraglide/messages';
	import type { MediaDto } from '@crownshy/api-client/api';
	import * as Dialog from '$lib/components/ui/dialog';
	import FileInput from '$lib/components/FileInput.svelte';
	import MediaSchema from '$lib/schemas/MediaSchema';
	import { Form, Input, Submit } from '$lib/components/EasyForm';
	import Media, { UploaderProgress } from '$lib/interfaces/Media.svelte';
	import FileIcon from '$lib/components/FileIcon.svelte';
	import StepIcon, { StepProgress } from './StepIcon.svelte';
	import type { Values } from '$lib/types';

	interface Props extends Omit<ComponentProps<typeof Button>, 'onclick'> {
		oncomplete?: (media: MediaDto) => void;
	}
	const { oncomplete, ...props }: Props = $props();

	let uploadForm: HTMLFormElement | null = $state(null);
	let file = $state<File | null>(null);
	let fileInput = $state<FileInput | null>(null);
	let nameInput = $state<Input | null>(null);
	let open = $state<boolean>(false);
	let page = $state<number>(1);

	const mediaUploader = new Media.uploader();

	async function handleSubmission(formData: FormData) {
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
	}

	function calculateState(step: Values<typeof UploaderProgress>): number {
		// If we have an error
		if (mediaUploader.error) {
			if (mediaUploader.step > step) {
				// And it's a future step, set it to waiting
				return StepProgress.Waiting;
			} else if (mediaUploader.step === step) {
				// If it's the current step set it to error
				return StepProgress.Error;
			}
		}

		// Return the current step state, but don't let it go to error, return a minimum of waiting
		return Math.max(mediaUploader.step - step, StepProgress.Waiting);
	}
</script>

{#snippet Step(step: keyof typeof UploaderProgress)}
	<div class="flex items-center gap-x-2">
		<StepIcon state={calculateState(UploaderProgress[step])} />
		<span class="text-lg font-semibold">{step}</span>
	</div>
{/snippet}

<Button {...props} type="button" onclick={() => (open = true)}>
	<Upload class="h-4 w-4" />
	{m.upload()}
</Button>

<Dialog.Root
	bind:open
	onOpenChangeComplete={(open) => {
		if (!open) {
			file = null;
		}
	}}
>
	<Dialog.Portal>
		<Dialog.Content
			class="{page === 2 ? 'md:w-75' : 'md:w-138'} rounded-2xl p-8 duration-350"
			interactOutsideBehavior={page === 2 ? 'ignore' : 'close'}
			showCloseButton={page !== 2}
		>
			{#if page === 1}
				<div class="flex flex-col items-center gap-y-2">
					<Form
						bind:ref={uploadForm}
						enctype="multipart/form-data"
						class="flex flex-col"
						{handleSubmission}
					>
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
										title="Remove file"
										aria-label="Remove file"
										variant="ghost"
										size="sm"
										class="text-destructive hover:text-destructive/80 rounded-sm"
										onclick={() => {
											file = null;
											nameInput?.setValue('');
											nameInput?.setError('');
											fileInput?.clear();
										}}
									>
										<Trash2 />
									</Button>
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
						<Submit class="mt-7 self-end" text="Upload" />
					</Form>
				</div>
			{:else}
				<div class="flex flex-col gap-y-2 self-start">
					{@render Step('Creating')}
					{@render Step('Uploading')}
					{#if mediaUploader.step === UploaderProgress.Uploading}
						<span class="text-muted-foreground font-mono font-semibold">
							{mediaUploader.percentage}%
						</span>
					{/if}
				</div>
			{/if}
			<section class="flex flex-row justify-center gap-x-3">
				{#each { length: 2 }, index}
					<div
						data-slot="page {index + 1}"
						aria-hidden="true"
						class="size-2 rounded-full bg-slate-500/70 transition-colors duration-300"
						class:bg-primary!={index + 1 === page}
					></div>
				{/each}
				<Button onclick={() => void (page = 1)}>&lt;</Button>
				<Button onclick={() => void (page = 2)}>&gt;</Button>
			</section>
		</Dialog.Content>
	</Dialog.Portal>
</Dialog.Root>

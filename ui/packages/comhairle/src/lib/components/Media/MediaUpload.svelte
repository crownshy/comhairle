<script lang="ts">
	import Button from '$lib/components/ui/button/button.svelte';
	import { FileText, Trash2 } from 'lucide-svelte';
	import { Upload } from 'lucide-svelte';
	import type { ComponentProps } from 'svelte';
	import { m } from '$lib/paraglide/messages';
	import type { MediaDto } from '@crownshy/api-client/api';
	import * as Dialog from '$lib/components/ui/dialog';
	import FileInput from '$lib/components/FileInput.svelte';
	import MediaSchema from './schema';
	import { Form, Input, Submit } from '$lib/components/EasyForm';
	import Media from '$lib/interfaces/Media';

	interface Props extends Omit<ComponentProps<typeof Button>, 'onclick'> {
		clientSide?: boolean;
		oncomplete?: (media: MediaDto[]) => void;
	}
	const { clientSide, oncomplete, ...props }: Props = $props();

	let uploadForm: HTMLFormElement | null = $state(null);
	let file = $state<File | null>(null);
	let filename = $state<Input | null>(null);
	let open = $state<boolean>(false);
</script>

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
		<Dialog.Content class="min-w-138  rounded-2xl p-8">
			<Form
				bind:ref={uploadForm}
				method="POST"
				action="/admin/media-library?/upload"
				enctype="multipart/form-data"
				class="flex flex-col"
				onsubmit={() => (open = false)}
			>
				<FileInput
					{...MediaSchema.media}
					onfile={(newFile) => {
						file = newFile;
						filename?.setValue(Media.getFilename(file.name));
					}}
					class={file !== null ? 'hidden' : ''}
				/>
				{#if file}
					<div
						class="flex w-full min-w-0 flex-row items-center justify-between rounded-lg p-3 text-xs font-medium shadow-xs"
					>
						<div class="flex basis-3/5 flex-row items-center gap-2">
							<FileText />
							<span>{file.name}</span>
						</div>
						<div class="flex flex-row items-center justify-evenly gap-4">
							<span class="text-muted-foreground"
								>{Media.getExtension(file.name)?.toUpperCase()}</span
							>
							<span class="text-muted-foreground">{Media.formatBytes(file.size)}</span
							>
							<Button
								title="Remove file"
								aria-label="Remove file"
								variant="ghost"
								size="sm"
								class="text-destructive hover:text-destructive/80 rounded-sm"
								onclick={() => (file = null)}
							>
								<Trash2 />
							</Button>
						</div>
					</div>
				{/if}
				<Input {...MediaSchema.name} bind:this={filename} label="Filename" type="text" />
				<Input {...MediaSchema.alt} label="Alt" type="text" />
				<Submit class="mt-7 self-end" text="Upload" />
			</Form>
		</Dialog.Content>
	</Dialog.Portal>
</Dialog.Root>

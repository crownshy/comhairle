<script lang="ts">
	import Button from '$lib/components/ui/button/button.svelte';
	import { Upload } from 'lucide-svelte';
	import type { ComponentProps } from 'svelte';
	import Spinner from '$lib/components/ui/spinner/spinner.svelte';
	import { m } from '$lib/paraglide/messages';
	import type { MediaDto } from '@crownshy/api-client/api';
	import * as Dialog from '$lib/components/ui/dialog';
	import { Input } from '$lib/components/ui/input';
	import * as Form from '$lib/components/ui/form';
	import { defaults, fileProxy, superForm, type SuperValidated } from 'sveltekit-superforms';
	import { zod, zodClient } from 'sveltekit-superforms/adapters';
	import MediaSchema from './schema';
	import FileUpload from '../FileUpload.svelte';

	interface Props extends Omit<ComponentProps<typeof Button>, 'onclick'> {
		data?: { form?: SuperValidated<(typeof MediaSchema)['_output']> };
		clientSide?: boolean;
		oncomplete?: (media: MediaDto[]) => void;
	}
	const { data, clientSide, oncomplete, ...props }: Props = $props();

	let uploadForm: HTMLFormElement | undefined;

	const form = superForm(data?.form ?? defaults(zod(MediaSchema)), {
		validators: zodClient(MediaSchema),
		taintedMessage: false,
		validationMethod: 'onsubmit'
	});

	let { form: formData, enhance, submitting } = $derived(form);

	const file = fileProxy(form, 'media');
</script>

<Dialog.Root>
	<Dialog.Trigger>
		<Button {...props}>
			<Upload class="h-4 w-4" />
			{m.upload()}
		</Button>
	</Dialog.Trigger>
	<Dialog.Portal>
		<Dialog.Content class="min-h-[50vh] min-w-138">
			<FileUpload onfile={async () => {}} accept=".jpeg,.png,.mp4" maxSizeMB={1} multiple />
			<form
				bind:this={uploadForm}
				method="POST"
				action="/admin/media-library?/upload"
				enctype="multipart/form-data"
				use:enhance
			>
				<div class="flex flex-col gap-1 p-4">
					<Form.Field {form} name="media">
						<Form.Control>
							{#snippet children({ props })}
								<Form.Label
									class="text-sm font-semibold lg:w-50 lg:shrink-0 lg:pt-2"
									>Media</Form.Label
								>
								<div>
									<Input {...props} type="file" name="media" bind:files={$file} />
									<Form.FieldErrors />
								</div>
							{/snippet}
						</Form.Control>
					</Form.Field>
					<Form.Field {form} name="name">
						<Form.Control>
							{#snippet children({ props })}
								<Form.Label
									class="text-sm font-semibold lg:w-50 lg:shrink-0 lg:pt-2"
									>Filename</Form.Label
								>
								<div>
									<Input
										{...props}
										type="text"
										bind:value={$formData.name}
										defaultvalue={$file.item(0)?.name}
									/>
									<Form.FieldErrors />
								</div>
							{/snippet}
						</Form.Control>
					</Form.Field>
					<Form.Field {form} name="alt">
						<Form.Control>
							{#snippet children({ props })}
								<Form.Label
									class="text-sm font-semibold lg:w-50 lg:shrink-0 lg:pt-2"
									>Alt</Form.Label
								>
								<div>
									<Input {...props} type="text" bind:value={$formData.alt} />
									<Form.FieldErrors />
								</div>
							{/snippet}
						</Form.Control>
					</Form.Field>
					<Form.Button class="mt-7 self-end">Upload</Form.Button>
				</div>
			</form>
		</Dialog.Content>
	</Dialog.Portal>
</Dialog.Root>

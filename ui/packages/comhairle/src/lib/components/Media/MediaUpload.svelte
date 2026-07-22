<script lang="ts">
	import Button from '$lib/components/ui/button/button.svelte';
	import { Upload } from 'lucide-svelte';
	import type { ComponentProps } from 'svelte';
	import Media from '$lib/interfaces/Media';
	import { notifications } from '$lib/notifications.svelte';
	import Spinner from '$lib/components/ui/spinner/spinner.svelte';
	import { m } from '$lib/paraglide/messages';
	import type { MediaDto } from '@crownshy/api-client/api';
	import * as Dialog from '$lib/components/ui/dialog';
	import { Input } from '$lib/components/ui/input';
	import * as Form from '$lib/components/ui/form';
	import { fileProxy, superForm } from 'sveltekit-superforms';
	import { zodClient } from 'sveltekit-superforms/adapters';
	import MediaSchema from './schema';

	interface Props extends Omit<ComponentProps<typeof Button>, 'onclick'> {
		clientSide?: boolean;
		oncomplete?: (media: MediaDto[]) => void;
	}
	const { data, clientSide, oncomplete, ...props }: Props = $props();

	const media = new Media();

	let uploadForm: HTMLFormElement | undefined;

	let uploading = $state<boolean>(false);

	async function onSubmit(e: Event) {
		if (clientSide) {
			const result = await validateForm({ update: true });
			if (!result.valid) {
				return;
			}

			const rawFiles = (e.target as HTMLInputElement).files;
			if (!rawFiles) return;
			// FIX: Check why the erroring here is incorrect
			const response = await media.upload('/api/media', Media.sanitiseMulti(rawFiles));

			if (response.err !== null) {
				notifications.send({
					message: response.err.message,
					priority: 'ERROR'
				});
				return;
			}
			return;
		}
		uploadForm?.submit();
	}

	const form = superForm(
		data?.form ?? {
			media: [],
			name: '',
			alt: ''
		},
		{
			validators: zodClient(MediaSchema),
			taintedMessage: false,
			validationMethod: 'oninput'
		}
	);

	let { form: formData, enhance, validateForm, errors } = $derived(form);

	$effect(() => {
		console.log('form:', $formData);
		console.log('errors:', $errors);
	});

	const file = fileProxy(form, 'media');
</script>

<Dialog.Root>
	<Dialog.Trigger>
		<Button {...props}>
			{#if uploading}
				<Spinner />
			{:else}
				<Upload class="h-4 w-4" />
			{/if}
			{m.upload()}
		</Button>
	</Dialog.Trigger>
	<Dialog.Portal>
		<Dialog.Content class="min-h-[50vh] min-w-[50vw]">
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
									<Input {...props} type="text" bind:value={$formData.name} />
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

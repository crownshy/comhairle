<script lang="ts">
	import { FileText } from 'lucide-svelte';
	import { Button } from '$lib/components/ui/button';

	type Props = {
		onfile: (file: File) => Promise<void>;
		accept?: string;
		maxSizeMB?: number;
		multiple?: boolean;
	};

	const { onfile, accept, maxSizeMB, multiple = false }: Props = $props();

	let fileInput = $state<HTMLInputElement | null>(null);

	type State = 'idle' | 'dragging' | 'uploading' | 'error';
	let status = $state<State>('idle');

	const acceptedExtensions = $derived(accept?.split(',') ?? []);
	const maxSizeBytes = $derived(maxSizeMB ? maxSizeMB * 1_024 * 1_024 : undefined);

	let inputMessage = $derived.by(() => {
		let inputMessage = '';

		// Add allowed file types info
		if (accept) {
			const length = acceptedExtensions.length - 1;

			for (let i = 0; i <= length; i++) {
				inputMessage += acceptedExtensions[i].slice(1).toUpperCase();
				switch (length - i) {
					case 0:
						inputMessage += ' format';
						if (length > 1) inputMessage += 's';
						break;
					case 1:
						inputMessage += ' and ';
						break;
					default:
						inputMessage += ', ';
				}
			}
		}

		// Add max size info
		if (maxSizeMB) {
			if (inputMessage !== '') {
				inputMessage += ', ';
			}
			inputMessage += `up to ${maxSizeMB}MB`;
		}

		return inputMessage;
	});

	// TODO: Merge with interfaces/Media.ts

	function handleFiles(files: FileList | undefined | null) {
		fileInput?.setCustomValidity('');

		if (files && files.length > 0) {
			for (const file of files) {
				if (maxSizeBytes && file.size > maxSizeBytes) {
					status = 'error';
					fileInput?.setCustomValidity('Max file size exceeded');
					return;
				}

				const extension = file.name.match(/\..*/)?.[0];
				if (!extension) {
					status = 'error';
					fileInput?.setCustomValidity("Couldn't recognise file type");
					return;
				}
				if (!acceptedExtensions.includes(extension)) {
					status = 'error';
					fileInput?.setCustomValidity('File type not supported');
					return;
				}

				status = 'idle';
				onfile(file);
			}
		}
	}
</script>

<div
	role="button"
	tabindex="0"
	class="border-input dark:bg-input/30 flex w-full cursor-pointer flex-col items-center gap-4 rounded-xl border bg-gray-50 p-8 py-5 transition-colors"
	class:bg-gray-100={status === 'dragging'}
	class:border-primary={status === 'dragging'}
	ondrop={(event) => {
		event.preventDefault();
		status = 'idle';
		handleFiles(event.dataTransfer?.files);
	}}
	ondragover={(event) => {
		event.preventDefault();
		status = 'dragging';
	}}
	ondragleave={(event) => {
		event.preventDefault();
		status = 'idle';
	}}
	onkeydown={(event) => {
		if (event.key !== 'Enter') {
			return;
		}
		fileInput?.click();
	}}
>
	<div class="h-8 w-8 text-gray-400">
		<FileText class="h-full w-full" />
	</div>
	<div class="flex flex-col items-center gap-2">
		<div class="text-foreground text-center text-base font-medium">
			{status === 'dragging' ? 'Drop your files here' : 'Drag and drop your files'}
		</div>
		{#if inputMessage}
			<div class="text-muted-foreground text-center text-sm">
				{inputMessage}
			</div>
		{/if}
		{#if status === 'error' && fileInput?.validity.valid === false}
			<div class="text-destructive text-center text-sm">
				{fileInput?.validationMessage}
			</div>
		{/if}
	</div>
	<Button variant="outline" onclick={() => fileInput?.click()} disabled={status === 'uploading'}>
		{status === 'uploading' ? 'Uploading...' : 'Select file'}
	</Button>
	<input
		bind:this={fileInput}
		type="file"
		{accept}
		{multiple}
		class="hidden"
		onchange={(event) => {
			handleFiles((event.target as HTMLInputElement).files);
		}}
	/>
</div>

<style>
	div[role='button']:has(input:user-invalid) {
		border-color: oklch(from var(--color-destructive) l c h / 0.8);
	}
</style>

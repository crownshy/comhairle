<script lang="ts">
	import { FileText } from 'lucide-svelte';
	import { Button } from '$lib/components/ui/button';
	import type { FileAttr } from '$lib/components/EasyForm';
	import Media from '$lib/interfaces/Media';

	interface Props extends FileAttr {
		disabled?: boolean;
		onfile?: (file: File) => Promise<unknown> | unknown;
		multiple?: boolean;
		class?: string;
	}

	const {
		name,
		disabled = false,
		onfile,
		required,
		accept,
		maxSize,
		multiple = false,
		class: className
	}: Props = $props();

	let input = $state<HTMLInputElement | null>(null);

	type State = 'idle' | 'dragging' | 'uploading' | 'error';
	let status = $state<State>('idle');

	const acceptedExtensions = $derived(accept?.split(',') ?? []);
	const plural = $derived(multiple ? 'files' : 'file');

	let inputMessage = $derived.by(() => {
		let inputMessage = '';

		// Generate text for allowed file types
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
		if (maxSize) {
			if (inputMessage !== '') {
				inputMessage += ', ';
			}
			inputMessage += `up to ${Media.formatBytes(maxSize)}`;
		}

		return inputMessage;
	});

	// TODO: Merge with interfaces/Media.ts

	let errorMessage = $state<string>('');

	function setError(message: string) {
		status = message ? 'error' : 'idle';
		input?.setCustomValidity(message);
		errorMessage = message;
	}

	export function clear() {
		if (input) {
			input.value = '';
		}
	}

	function handleFiles(files: FileList | undefined | null) {
		if (disabled) return;
		setError('');

		if (files && files.length > 0) {
			for (const file of files) {
				const extension = Media.getExtension(file.name);
				if (!extension) {
					setError("Couldn't recognise file type");
					return;
				}
				if (!acceptedExtensions.includes(extension)) {
					setError('File type not supported');
					return;
				}

				if (maxSize && file.size > maxSize) {
					setError('Max file size exceeded');
					return;
				}

				status = 'idle';
				onfile?.(file);
			}
		}
	}
</script>

<div
	role="button"
	tabindex={disabled ? -1 : 0}
	aria-disabled={disabled}
	class="border-input dark:bg-input/30 flex w-full flex-col items-center gap-4 rounded-xl border bg-gray-50 p-8 py-5 transition-colors {status ===
		'error' && 'border-destructive!'} {className}"
	class:cursor-pointer={!disabled}
	class:cursor-default={disabled}
	class:opacity-50={disabled}
	class:bg-gray-100={!disabled && status === 'dragging'}
	class:border-primary={!disabled && status === 'dragging'}
	ondrop={(event) => {
		event.preventDefault();
		if (disabled) return;
		status = 'idle';
		handleFiles(event.dataTransfer?.files);
	}}
	ondragover={(event) => {
		event.preventDefault();
		if (disabled) return;
		status = 'dragging';
	}}
	ondragleave={(event) => {
		event.preventDefault();
		status = 'idle';
	}}
	onkeydown={(event) => {
		if (disabled || event.key !== 'Enter') {
			return;
		}
		input?.click();
	}}
>
	<div class="h-8 w-8 text-gray-400">
		<FileText class="h-full w-full" />
	</div>
	<div class="flex flex-col items-center gap-2">
		<div class="text-foreground text-center text-base font-medium">
			{status === 'dragging' ? `Drop your ${plural} here` : `Drag and drop your ${plural}`}
		</div>
		{#if inputMessage}
			<div class="text-muted-foreground text-center text-sm">
				{inputMessage}
			</div>
		{/if}
		{#if status === 'error'}
			<div class="text-destructive text-center text-sm">
				{errorMessage}
			</div>
		{/if}
	</div>
	<Button
		variant="outline"
		onclick={() => input?.click()}
		disabled={disabled || status === 'uploading'}
	>
		{status === 'uploading' ? 'Uploading...' : `Select ${plural}`}
	</Button>
	<input
		bind:this={input}
		type="file"
		{disabled}
		{name}
		{required}
		{accept}
		{multiple}
		class="hidden"
		oninvalid={() => {
			if (input?.validity.valid === false) {
				setError(input.validationMessage);
			}
		}}
		onchange={(event) => {
			handleFiles((event.target as HTMLInputElement).files);
		}}
	/>
</div>

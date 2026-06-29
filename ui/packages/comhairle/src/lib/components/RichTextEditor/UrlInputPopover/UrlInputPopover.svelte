<script lang="ts">
	import { type Snippet } from 'svelte';
	import * as Popover from '$lib/components/ui/popover';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import {
		DEFAULT_ALLOWED_DOMAINS,
		validateIframeUrl,
		validateUrl
	} from '$lib/utils/urlValidation';
	import { capitalise } from '$lib/utils/string';

	type Props = {
		type: 'audio' | 'image' | 'video' | 'link';
		onSubmit: (value: string) => void;
		children: Snippet;
		label?: string;
		placeholder?: string;
		buttonText?: string;
	};

	let {
		label,
		placeholder,
		buttonText = 'Insert', // TODO: consider translations
		type,
		onSubmit,
		children
	}: Props = $props();

	let open = $state<boolean>(false);
	let inputValue = $state('');
	let errorMessage = $state<string | null>(null);

	$effect(() => {
		if (!open) {
			inputValue = '';
			errorMessage = null;
		}
	});

	function validate(url: string) {
		switch (type) {
			case 'video':
				return validateIframeUrl(url, DEFAULT_ALLOWED_DOMAINS);
			case 'audio':
			case 'image':
			case 'link':
				return validateUrl(url);
		}
	}

	function getPlaceholder(type: Props['type']): string {
		switch (type) {
			case 'video':
				return 'https://youtube.com/embed/...';
			case 'audio':
				return 'https://example.com/audio.mp3';
			case 'image':
				return 'https://example.com/image.jpg';
			case 'link':
				return 'https://example.com';
		}
	}

	function handleSubmit() {
		const trimmed = inputValue.trim();

		if (!trimmed) {
			errorMessage = 'URL cannot be empty';
			return;
		}

		const validURL = validate(trimmed);
		if (validURL === null) {
			errorMessage = `Please enter a valid ${type} URL`;
			return;
		}

		onSubmit(validURL);
		open = false;
	}

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Enter') {
			e.preventDefault();
			handleSubmit();
		}
	}

	function handleCancel() {
		open = false;
	}
</script>

<Popover.Root bind:open>
	<Popover.Trigger>
		{@render children()}
	</Popover.Trigger>
	<Popover.Content class="w-80" side="bottom" align="start">
		<div class="space-y-4">
			<div class="space-y-2">
				<Label for="url-input">{label ?? `Insert ${capitalise(type)}`}</Label>
				<Input
					id="url-input"
					bind:value={inputValue}
					type="url"
					placeholder={placeholder ?? getPlaceholder(type)}
					onkeydown={handleKeydown}
					aria-invalid={!!errorMessage}
					aria-describedby={errorMessage ? 'error-message' : undefined}
				/>
				{#if errorMessage}
					<p id="error-message" class="text-destructive text-sm" role="alert">
						{errorMessage}
					</p>
				{/if}
			</div>
			<div class="flex justify-end gap-2">
				<Button variant="outline" size="sm" onclick={handleCancel}>Cancel</Button>
				<Button size="sm" onclick={handleSubmit}>{buttonText}</Button>
			</div>
		</div>
	</Popover.Content>
</Popover.Root>

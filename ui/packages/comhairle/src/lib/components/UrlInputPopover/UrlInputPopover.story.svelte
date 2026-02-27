<script lang="ts">
	import UrlInputPopover from './UrlInputPopover.svelte';
	import { validateUrl, validateIframeUrl, DEFAULT_ALLOWED_DOMAINS } from '$lib/utils/urlValidation';

	type ValidationType = 'url' | 'image' | 'video';

	type Props = {
		label: string;
		placeholder?: string;
		buttonText?: string;
		buttonStyle?: string;
		buttonLabel?: string;
		validationType?: ValidationType;
		open?: boolean;
	};

	let {
		label,
		placeholder = 'https://example.com',
		buttonText = 'Insert',
		buttonStyle = 'px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600',
		buttonLabel = 'Click me',
		validationType = 'url',
		open = false
	}: Props = $props();

	let isOpen = $state(open);

	function validateFn(url: string): string | null {
		switch (validationType) {
			case 'video':
				if (!validateIframeUrl(url, DEFAULT_ALLOWED_DOMAINS)) {
					return 'Please enter a valid video URL';
				}
				break;
			case 'image':
				if (!validateUrl(url)) {
					return 'Please enter a valid HTTPS image URL';
				}
				break;
			case 'url':
			default:
				if (!validateUrl(url)) {
					return 'Please enter a valid HTTPS URL';
				}
				break;
		}
		return null;
	}

	function handleSubmit(url: string) {
		console.log('Submitted URL:', url);
	}
</script>

<div class="p-8">
	<UrlInputPopover
		bind:open={isOpen}
		{label}
		{placeholder}
		{buttonText}
		onSubmit={handleSubmit}
		onOpenChange={(open) => (isOpen = open)}
		validateFn={validateFn}
	>
		<button type="button" class={buttonStyle}>
			{buttonLabel}
		</button>
	</UrlInputPopover>
</div>

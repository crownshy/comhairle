<script lang="ts">
	import { type Snippet } from 'svelte';
	import * as Popover from '$lib/components/ui/popover';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import { Checkbox } from '$lib/components/ui/checkbox';
	import {
		DEFAULT_ALLOWED_DOMAINS,
		validateIframeUrl,
		validateUrl
	} from '$lib/utils/urlValidation';
	import { capitalise } from '$lib/utils/casingUtils';
	import { MediaUpload } from '$lib/components/Media';
	import MediaLibraryDialog from '../../Media/MediaLibraryDialog.svelte';
	import { m } from '$lib/paraglide/messages';
	import type { MediaDto } from '@crownshy/api-client/api';

	type OnSubmit = {
		from: 'library' | 'external-url';
		url: string;
		/** Image alt text. An empty string means the author marked the image decorative. */
		alt?: string;
		/** Accessible title for an embedded video. */
		title?: string | null;
		/** WebVTT captions file URL for a video. */
		captions?: string | null;
	};
	type Props = {
		type: 'audio' | 'image' | 'video' | 'link';
		onSubmit: (value: OnSubmit) => void;
		children: Snippet;
		label?: string;
		placeholder?: string;
		allowLocalSelection?: boolean;
		buttonText?: string;
	};

	let {
		label,
		placeholder,
		type,
		onSubmit,
		children,
		allowLocalSelection,
		buttonText = 'Insert' // TODO: consider translations
	}: Props = $props();

	const id = $props.id();

	let open = $state<boolean>(false);
	let inputValue = $state('');
	let errorMessage = $state<string | null>(null);
	let altText = $state('');
	let decorative = $state(false);
	let videoTitle = $state('');
	let captionsUrl = $state('');

	function resetFields() {
		inputValue = '';
		errorMessage = null;
		altText = '';
		decorative = false;
		videoTitle = '';
		captionsUrl = '';
	}

	/** Reads the optional video fields, or an error message when the captions URL is invalid. */
	function readVideoDetails(): { title: string | null; captions: string | null } | string {
		const captions = captionsUrl.trim();
		const validCaptions = captions ? validateUrl(captions) : null;
		if (captions && validCaptions === null) return m.rich_text_captions_url_invalid();
		return { title: videoTitle.trim() || null, captions: validCaptions };
	}

	function submitFromLibrary(media: MediaDto) {
		if (type === 'video') {
			const details = readVideoDetails();
			if (typeof details === 'string') {
				errorMessage = details;
				return;
			}
			onSubmit({ from: 'library', url: media.url, ...details });
		} else {
			onSubmit({ from: 'library', url: media.url, alt: media.alt });
		}
		open = false;
	}

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

		if (type === 'image') {
			const alt = altText.trim();
			if (!alt && !decorative) {
				errorMessage = m.rich_text_image_alt_required();
				return;
			}
			onSubmit({ from: 'external-url', url: validURL, alt: decorative ? '' : alt });
		} else if (type === 'video') {
			const details = readVideoDetails();
			if (typeof details === 'string') {
				errorMessage = details;
				return;
			}
			onSubmit({ from: 'external-url', url: validURL, ...details });
		} else {
			onSubmit({ from: 'external-url', url: validURL });
		}
		open = false;
	}

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Enter') {
			e.preventDefault();
			handleSubmit();
		}
	}
</script>

<Popover.Root
	bind:open
	onOpenChange={(isOpen) => {
		if (isOpen) resetFields();
	}}
>
	<Popover.Trigger>
		{@render children()}
	</Popover.Trigger>
	<Popover.Content class="w-80" side="bottom" align="start">
		<div class="space-y-4">
			{#if type === 'video'}
				<!-- First, so the details also apply to a video picked from the library below. -->
				<div class="space-y-2">
					<Label for="{id}-video-title">{m.rich_text_video_title_label()}</Label>
					<Input id="{id}-video-title" bind:value={videoTitle} type="text" />
					<Label for="{id}-captions-url">{m.rich_text_captions_url_label()}</Label>
					<Input
						id="{id}-captions-url"
						bind:value={captionsUrl}
						type="url"
						placeholder="https://example.com/captions.vtt"
					/>
				</div>
				<hr class="mx-2 my-4" />
			{/if}
			{#if allowLocalSelection}
				<span class="mb-2 block text-sm leading-none font-medium">
					{`Insert ${capitalise(type)} From Files`}
				</span>
				<div class="flex flex-row gap-2">
					<MediaLibraryDialog onconfirm={submitFromLibrary} />
					<MediaUpload size="sm" oncomplete={submitFromLibrary} />
				</div>
				<hr class="mx-2 my-4" />
			{/if}
			<div class="space-y-2">
				<Label for="{id}-url"
					>{label ??
						`Insert ${capitalise(type)}${allowLocalSelection ? ' From URL' : ''}`}</Label
				>
				<Input
					id="{id}-url"
					bind:value={inputValue}
					type="url"
					placeholder={placeholder ?? getPlaceholder(type)}
					onkeydown={handleKeydown}
					aria-invalid={!!errorMessage}
					aria-describedby={errorMessage ? `${id}-error` : undefined}
				/>
				{#if type === 'image'}
					<Label for="{id}-alt">{m.rich_text_image_alt_label()}</Label>
					<Input
						id="{id}-alt"
						bind:value={altText}
						type="text"
						disabled={decorative}
						placeholder={m.rich_text_image_alt_placeholder()}
						onkeydown={handleKeydown}
					/>
					<div class="flex items-center gap-2">
						<Checkbox id="{id}-decorative" bind:checked={decorative} />
						<Label for="{id}-decorative">{m.rich_text_image_decorative()}</Label>
					</div>
				{/if}
				{#if errorMessage}
					<p id="{id}-error" class="text-destructive text-sm" role="alert">
						{errorMessage}
					</p>
				{/if}
			</div>
			<div class="flex justify-end gap-2">
				<Button variant="outline" size="sm" onclick={() => void (open = false)}>
					Cancel
				</Button>
				<Button size="sm" onclick={handleSubmit}>{buttonText}</Button>
			</div>
		</div>
	</Popover.Content>
</Popover.Root>

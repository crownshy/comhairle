<script lang="ts">
	import * as Dialog from '$lib/components/ui/dialog';
	import Button from '$lib/components/ui/button/button.svelte';
	import { SquareArrowOutUpLeft } from 'lucide-svelte';
	import type { Snippet } from 'svelte';
	import MediaLibrary from '$lib/components/Media/MediaLibrary.svelte';

	interface Props {
		children: Snippet;
	}

	const { children }: Props = $props();
</script>

<Dialog.Root>
	<Dialog.Trigger>
		{@render children()}
	</Dialog.Trigger>
	<Dialog.Portal>
		<Dialog.Content>
			<Dialog.Title>
				<div class="flex flex-row items-center gap-1">
					<span>Media library</span>
					<Button
						href="/admin/media-library"
						variant="ghost"
						title="Go to media library"
						aria-label="Go to media library"
					>
						<SquareArrowOutUpLeft />
					</Button>
				</div>
				<MediaLibrary {data} rowHeight={15}>
					{#snippet media(type, media)}
						<!-- <MediaItem {type} src={media.src} alt="" /> -->
					{/snippet}
				</MediaLibrary>
			</Dialog.Title>
		</Dialog.Content>
	</Dialog.Portal>
</Dialog.Root>

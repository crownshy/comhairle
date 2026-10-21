<script lang="ts" module>
	let cache: MediaDto[] | null = null;

	export function addToCache(media: MediaDto): void {
		if (cache !== null) {
			cache.push(media);
		}
	}
</script>

<script lang="ts">
	import * as Dialog from '$lib/components/ui/dialog';
	import Button from '$lib/components/ui/button/button.svelte';
	import { Images, SquareArrowOutUpLeft, X } from 'lucide-svelte';
	import MediaLibrary from '$lib/components/Media/MediaLibrary.svelte';
	import MediaItem from '$lib/components/Media/MediaItem.svelte';
	import type { MediaDto } from '@crownshy/api-client/api';
	import { tryCatchAsync, type Result } from '$lib/utils/errorHandling';
	import Spinner from '$lib/components/ui/spinner/spinner.svelte';
	import { apiClient } from '@crownshy/api-client/client';

	interface Props {
		onconfirm: (media: MediaDto) => void;
	}

	const { onconfirm }: Props = $props();

	async function getData(): Promise<Result<'ok', MediaDto[], string>> {
		const request = await tryCatchAsync(() => apiClient.ListMedia());
		if (request.err !== null) {
			return {
				ok: null,
				err: request.err
			};
		}
		const media = request.ok.records as MediaDto[];
		cache = media;
		return { ok: media, err: null };
	}

	let dataRequest = $state.raw<ReturnType<typeof getData> | null>(null);

	let selectedMedia = $state<MediaDto | null>(null);
	let open = $state(false);
</script>

<Dialog.Root bind:open>
	<Dialog.Trigger class="flex justify-start">
		<Button
			size="sm"
			aria-label="Open media library"
			onclick={() => {
				if (cache !== null) {
					dataRequest = Promise.resolve({ ok: cache, err: null });
					return;
				}
				dataRequest = getData();
			}}><Images class="h-4 w-4" />Media library</Button
		>
	</Dialog.Trigger>
	<Dialog.Portal>
		<Dialog.Content
			showCloseButton={false}
			class="max-h-[80vh] overflow-y-auto pt-0 sm:max-w-3xl"
		>
			<div
				class="bg-background sticky top-0 z-10 -mx-6 mb-5 flex flex-row items-center justify-between px-6 pt-6 pb-3"
			>
				<div class="flex flex-row items-center gap-1">
					<Dialog.Title>Media library</Dialog.Title>
					<Button
						href="/admin/media-library"
						variant="ghost"
						size="sm"
						title="Go to media library"
						aria-label="Go to media library"
					>
						<SquareArrowOutUpLeft />
					</Button>
				</div>
				<div class="flex items-center gap-2">
					<Button
						disabled={!selectedMedia}
						onclick={() => {
							if (!selectedMedia) return;
							onconfirm(selectedMedia);
							open = false;
						}}>Select</Button
					>
					<Dialog.Close
						class="focus-visible:outline-ring rounded-sm p-2 opacity-70 hover:opacity-100 focus-visible:outline-2"
						aria-label="Close media library"
					>
						<X class="h-4 w-4" />
					</Dialog.Close>
				</div>
			</div>

			{#if dataRequest !== null}
				{#await dataRequest}
					<Spinner />
				{:then data}
					{#if data.err !== null}
						<p>{data.err}</p>
					{:else}
						<MediaLibrary data={data.ok}>
							{#snippet media(type, media)}
								<label>
									<input
										type="radio"
										name="selected"
										value={media}
										bind:group={selectedMedia}
										class="hidden"
									/>
									<MediaItem
										{type}
										{...media}
										alt=""
										--selected={selectedMedia?.url === media.url &&
											'var(--ring)'}
									/>
								</label>
							{/snippet}
						</MediaLibrary>
					{/if}
				{/await}
			{/if}
		</Dialog.Content>
	</Dialog.Portal>
</Dialog.Root>

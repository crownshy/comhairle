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
	import { Images, SquareArrowOutUpLeft } from 'lucide-svelte';
	import MediaLibrary from '$lib/components/Media/MediaLibrary.svelte';
	import MediaItem from '$lib/components/Media/MediaItem.svelte';
	import type { MediaDto } from '@crownshy/api-client/api';
	import { tryCatchAsync, type Result } from '$lib/utils/errorHandling';
	import Spinner from '$lib/components/ui/spinner/spinner.svelte';
	import { apiClient } from '@crownshy/api-client/client';

	interface Props {
		onconfirm: (url: string) => void;
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

	let url = $state(null);
</script>

<Dialog.Root>
	<Dialog.Trigger>
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
		<Dialog.Content class="sm:max-w-3xl">
			<Dialog.Title>
				<div class="mr-8 mb-5 flex flex-row items-center justify-between">
					<div class="flex flex-row items-center gap-1">
						<span>Media library</span>
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
					<Button
						disabled={!url}
						onclick={() => {
							if (!url) return;
							onconfirm(url);
						}}>Insert</Button
					>
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
											value={media.url}
											bind:group={url}
											class="hidden"
										/>
										<MediaItem
											{type}
											{...media}
											alt=""
											--selected={url === media.url && 'var(--ring)'}
										/>
									</label>
								{/snippet}
							</MediaLibrary>
						{/if}
					{/await}
				{/if}
			</Dialog.Title>
		</Dialog.Content>
	</Dialog.Portal>
</Dialog.Root>

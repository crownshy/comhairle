<script lang="ts" module>
	let cache: MediaDto[] | null = null;
</script>

<script lang="ts">
	import * as Dialog from '$lib/components/ui/dialog';
	import Button from '$lib/components/ui/button/button.svelte';
	import { Images, SquareArrowOutUpLeft } from 'lucide-svelte';
	import MediaLibrary from '$lib/components/Media/MediaLibrary.svelte';
	import MediaItem from '$lib/components/Media/MediaItem.svelte';
	import type { MediaDto } from '@crownshy/api-client/api';
	import { tryCatchAsync, tryFetch, type FetchErr, type Result } from '$lib/utils/errorHandling';
	import Spinner from '$lib/components/ui/spinner/spinner.svelte';

	async function getData(): Promise<Result<'ok', MediaDto[], FetchErr>> {
		const request = await tryFetch('/api/media');
		// FIX:
		if (request.err !== null) {
			return {
				ok: null,
				err: {
					id: 'NETWORK_ERROR',
					message: 'Network error. Please check your internet connection.'
				}
			};
		}
		const json = await tryCatchAsync(() => request.ok.json());
		if (json.err !== null) {
			// FIX:
			return {
				ok: null,
				err: {
					id: 'NETWORK_ERROR',
					message: 'Network error. Please check your internet connection.'
				}
			};
		}
		const media = json.ok.records as MediaDto[];
		cache = media;
		return { ok: media, err: null };
	}

	let dataRequest = $state.raw<ReturnType<typeof getData> | null>(null);

	interface Props {
		onconfirm: (url: string) => void;
	}

	const { onconfirm }: Props = $props();

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
				<div class="mr-8 flex flex-row items-center justify-between">
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
							<p>{data.err.message}</p>
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
											src={media.url}
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

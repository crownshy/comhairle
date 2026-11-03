<script lang="ts" module>
	let cache: MediaDto[] | null = null;

	export function addToCache(media: MediaDto): void {
		if (cache !== null) {
			cache.push(media);
		}
	}
</script>

<script lang="ts">
	import Button from '$lib/components/ui/button/button.svelte';
	import MediaLibrary from '$lib/components/Media/MediaLibrary.svelte';
	import Spinner from '$lib/components/ui/spinner/spinner.svelte';
	import ErrorMessage from '$lib/components/ErrorMessage.svelte';
	import Dialog from '$lib/components/Dialog.svelte';
	import type { MediaDto } from '@crownshy/api-client/api';
	import { Images, SquareArrowOutUpLeft } from 'lucide-svelte';
	import { tryCatchAsync, type Result } from '$lib/utils/errorHandling';
	import { apiClient } from '@crownshy/api-client/client';

	type Props = {
		onconfirm: (media: MediaDto) => void;
	};

	const { onconfirm }: Props = $props();

	async function getData(): Promise<Result<'ok', MediaDto[], string>> {
		const request = await tryCatchAsync(() => apiClient.ListMedia());
		if (request.err !== null) {
			return {
				ok: null,
				err: request.err as string
			};
		}
		const media = request.ok.records as MediaDto[];
		cache = media;
		return { ok: media, err: null };
	}

	let dataRequest = $state.raw<ReturnType<typeof getData> | null>(null);
	let selected = $state.raw<MediaDto | null>(null);
</script>

<Button
	size="sm"
	aria-label="Open media library"
	icon={Images}
	command="open-dialog:media-library"
	onclick={() => {
		if (cache !== null) {
			dataRequest = Promise.resolve({ ok: cache, err: null });
			return;
		}
		dataRequest = getData();
	}}
>
	Media library
</Button>

<Dialog
	id="media-library"
	--max-height="80vh"
	--min-width="40vw"
	class="pt-0 sm:max-w-3xl"
	title="Media library"
>
	{#snippet HeaderActions()}
		<Button
			href="/admin/media-library"
			variant="ghost"
			size="sm"
			title="Go to media library"
			aria-label="Go to media library"
			icon={SquareArrowOutUpLeft}
		/>
		<Button
			disabled={!selected}
			onclick={() => {
				if (!selected) return;
				onconfirm(selected);
			}}
			command="close-dialog:media-library"
		>
			Select
		</Button>
	{/snippet}

	{#if dataRequest !== null}
		{#await dataRequest}
			<Spinner />
		{:then data}
			{#if data.err !== null}
				<ErrorMessage>{data.err}</ErrorMessage>
			{:else}
				<MediaLibrary media={data.ok} mode="single-select" bind:selected />
			{/if}
		{/await}
	{/if}
</Dialog>

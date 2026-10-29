<script lang="ts">
	import Button from '$lib/components/ui/button/button.svelte';
	import MediaLibrary from '$lib/components/Media/MediaLibrary.svelte';
	import Spinner from '$lib/components/ui/spinner/spinner.svelte';
	import Dialog from '$lib/components/Dialog.svelte';
	import type { MediaDto } from '@crownshy/api-client/api';
	import { Images, SquareArrowOutUpLeft } from '@lucide/svelte';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import { apiClient } from '@crownshy/api-client/client';

	type Props = {
		onconfirm: (media: MediaDto) => void;
	};

	const { onconfirm }: Props = $props();

	const getData = () => tryCatchAsync(() => apiClient.ListMedia().then((res) => res.records));

	let dataRequest = $state.raw<ReturnType<typeof getData> | null>(null);
	let selected = $state<MediaDto['id'] | null>(null);
</script>

<Button
	size="sm"
	aria-label="Open media library"
	icon={Images}
	command="open-dialog:media-library"
	onclick={() => void (dataRequest = getData())}
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
			onclick={async () => {
				if (!selected || !dataRequest) return;
				const media = (await dataRequest).ok?.find((m) => m.id === selected);
				if (!media) return;
				onconfirm(media);
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
			<MediaLibrary media={data} mode="single-select" bind:selected />
		{/await}
	{/if}
</Dialog>

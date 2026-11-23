<script lang="ts">
	import * as Dialog from '$lib/components/ui/dialog';
	import Button from '$lib/components/ui/button/button.svelte';
	import Images from '@lucide/svelte/icons/images';
	import SquareArrowOutUpLeft from '@lucide/svelte/icons/square-arrow-out-up-left';
	import X from '@lucide/svelte/icons/x';
	import MediaLibrary from '$lib/components/Media/MediaLibrary.svelte';
	import type { MediaDto } from '@crownshy/api-client/api';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import Spinner from '$lib/components/ui/spinner/spinner.svelte';
	import { apiClient } from '@crownshy/api-client/client';
	import ErrorMessage from '../ErrorMessage.svelte';

	type Props = {
		onconfirm: (media: MediaDto) => void;
	};

	const { onconfirm }: Props = $props();

	const getData = () => tryCatchAsync(() => apiClient.ListMedia().then((res) => res.records));

	let dataRequest = $state.raw<ReturnType<typeof getData> | null>(null);
	let selected = $state<MediaDto['id'] | null>(null);
	let open = $state(false);
</script>

<Dialog.Root
	bind:open
	onOpenChange={(open) => {
		if (!open) {
			selected = null;
		}
	}}
>
	<Dialog.Trigger class="flex justify-start">
		<Button
			size="sm"
			aria-label="Open media library"
			icon={Images}
			onclick={() => {
				dataRequest = getData();
			}}
		>
			Media library
		</Button>
	</Dialog.Trigger>
	<Dialog.Portal>
		<Dialog.Content
			showCloseButton={false}
			class="max-h-[80vh] overflow-y-auto pt-0 sm:max-w-3xl"
		>
			<div class="bg-background sticky top-0 z-10 -mx-6 px-6 pt-6 pb-3">
				<div class=" flex flex-row items-center justify-between">
					<div class="flex flex-row items-center gap-1">
						<Dialog.Title>Media library</Dialog.Title>
						<Button
							href="/admin/media-library"
							variant="ghost"
							size="sm"
							title="Go to media library"
							aria-label="Go to media library"
							icon={SquareArrowOutUpLeft}
						/>
					</div>
					<div class="flex items-center gap-2">
						<Button
							disabled={!selected}
							onclick={async () => {
								if (!selected || !dataRequest) return;
								const media = (await dataRequest).ok?.find(
									(m) => m.id === selected
								);
								if (!media) return;
								onconfirm(media);
								open = false;
							}}
						>
							Select
						</Button>
						<Dialog.Close
							class="focus-visible:outline-ring rounded-sm p-2 opacity-70 hover:opacity-100 focus-visible:outline-2"
							aria-label="Close media library"
						>
							<X class="h-4 w-4" />
						</Dialog.Close>
					</div>
				</div>
			</div>

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
		</Dialog.Content>
	</Dialog.Portal>
</Dialog.Root>

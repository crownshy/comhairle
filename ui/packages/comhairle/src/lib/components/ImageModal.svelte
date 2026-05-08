<script lang="ts">
	import ImagePlaceholder from './ui/images/ImagePlaceholder.svelte';
	import * as Tabs from '$lib/components/ui/tabs';
	import {
		Dialog,
		DialogContent,
		DialogHeader,
		DialogTitle,
		DialogFooter
	} from '$lib/components/ui/dialog';
	import { X } from 'lucide-svelte';
	import FileUpload from './KnowledgeBase/FileUpload.svelte';
	import type { MediaDto } from '@crownshy/api-client/api';
	import { formatDateShort } from '$lib/utils';

	type Props = {
		assets: MediaDto[];
		onSelectExistingAsset: (asset: MediaDto) => void;
		onUpload: (file: File) => void;
	};

	let open = $state(false);

	let { assets, onUpload, onSelectExistingAsset }: Props = $props();
	let activeTab = $state('assets');

	async function handleUploadImage(file: File) {
		try {
			await onUpload(file);

			activeTab = 'assets';
		} catch (e) {
			console.error(e);
		}
	}

	async function handleSelectExistingAsset(asset: MediaDto) {
		try {
			await onSelectExistingAsset(asset);

			open = false;
		} catch (e) {
			console.error(e);
		}
	}
</script>

<div></div>

<Dialog {open}>
	<button type="button" class="relative w-full" onclick={() => (open = true)}>
		<ImagePlaceholder class="relative rounded-lg" />
		<span class="absolute top-1/2 left-1/2 -translate-1/2">Select or upload image</span>
	</button>

	<DialogContent showCloseButton={false} class="w-[80vw] max-w-[unset] sm:max-w-[unset]">
		<button
			type="button"
			class="absolute top-4 right-4"
			aria-label="Close modal window"
			onclick={() => (open = false)}><X /></button
		>
		<DialogHeader>
			<DialogTitle>Select or upload an image</DialogTitle>
		</DialogHeader>

		<Tabs.Root bind:value={activeTab}>
			<div class="flex justify-center">
				<div class="bg-sidebar mb-4 flex shrink-0 flex-row gap-0.5 rounded-xl p-1">
					<Tabs.Trigger
						value="assets"
						class="text-sidebar-foreground data-[state=active]:text-foreground border-none"
						>Assets</Tabs.Trigger
					>
					<Tabs.Trigger
						value="upload"
						class="text-sidebar-foreground data-[state=active]:text-foreground border-none"
						>Upload</Tabs.Trigger
					>
				</div>
			</div>
			<Tabs.Content value="assets">
				{#if !!assets.length}
					<div class="grid grid-cols-2 gap-4">
						{#each assets as asset (asset.id)}
							<button
								type="button"
								onclick={() => handleSelectExistingAsset(asset)}
								class="bg-sidebar-accent outline-primary flex justify-between rounded-lg p-4 transition-all duration-300 ease-in-out hover:outline"
							>
								<span>{asset.filename}</span>
								<span>Created: {formatDateShort(asset.createdAt)}</span>
							</button>
						{/each}
					</div>
				{:else}
					<div class="flex justify-center">
						<p>You have no existing image assets</p>
					</div>
				{/if}
			</Tabs.Content>
			<Tabs.Content value="upload">
				<FileUpload onUpload={handleUploadImage} accept=".jpeg,.jpg,.png" />
			</Tabs.Content>
		</Tabs.Root>

		<DialogFooter></DialogFooter>
	</DialogContent>
</Dialog>

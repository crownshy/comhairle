<script lang="ts">
	import { enhance } from '$app/forms';
	import Button from '$lib/components/ui/button/button.svelte';
	import { SquarePen, Trash2, Upload, X } from 'lucide-svelte';
	import climateUk from '$lib/assets/climageuk.jpg';
	import vtaiwan from '$lib/assets/vtaiwan.jpg';
	import seattleUSA from '$lib/assets/seattle_usa.jpg';
	import wavesLogoLg from '$lib/assets/waves-logo-lg.png';
	import comhairleLogo from '$lib/assets/comhairle_logo.png';
	import placeholderConvo from '$lib/assets/placeholder_convo.png';
	import comhairleFullLogo from '$lib/assets/comhairle_full_logo.svg';
	import DeleteDialog from './DeleteDialog.svelte';

	let fileInput: HTMLInputElement | undefined;
	let form: HTMLFormElement | undefined;

	let bulkEdit = $state<boolean>(false);

	const images: { id: number; src: string }[] = [
		{ id: 1, src: climateUk },
		{ id: 2, src: vtaiwan },
		{ id: 3, src: seattleUSA },
		{ id: 4, src: wavesLogoLg },
		{ id: 5, src: comhairleLogo },
		{ id: 6, src: placeholderConvo },
		{ id: 7, src: comhairleFullLogo }
	];

	const ROW_HEIGHT = 30;

	let deleteDialog: DeleteDialog | undefined;
</script>

<svelte:head>
	<title>Media library - Comhairle Admin</title>
</svelte:head>

<div class="mx-auto w-4/5 p-10">
	<header class="flex flex-row items-baseline justify-between">
		<h1 class="text-4xl font-bold">Media library</h1>

		<form
			bind:this={form}
			method="POST"
			action="?/upload"
			enctype="multipart/form-data"
			use:enhance
		>
			<input
				bind:this={fileInput}
				type="file"
				name="images"
				accept="image/*"
				multiple
				class="hidden"
				aria-hidden="true"
				oninput={() => {
					form?.submit();
				}}
			/>
			<div class="flex flex-row gap-4">
				{#if !bulkEdit}
					<Button
						onclick={() => {
							fileInput?.click();
						}}
					>
						<Upload class="h-4 w-4" />Upload
					</Button>
					<Button
						variant="outline"
						onclick={() => {
							bulkEdit = true;
						}}
					>
						<SquarePen class="h-4 w-4" />Edit
					</Button>
				{:else}
					<Button
						variant="destructive"
						onclick={() => {
							deleteDialog?.open();
						}}
					>
						<Trash2 class="h-4 w-4" />Delete
					</Button>
					<Button
						variant="outline"
						onclick={() => {
							bulkEdit = false;
						}}
					>
						<X class="h-4 w-4" />Cancel
					</Button>
				{/if}
			</div>
		</form>
	</header>
	<main class="mt-5">
		{#snippet libraryImage(src: string, alt: string)}
			<img {src} {alt} class="h-full w-full cursor-pointer overflow-hidden object-cover" />
		{/snippet}
		<ul class="flex flex-wrap gap-2">
			{#each images as image (image.id)}
				<li
					class={`relative h-[${ROW_HEIGHT}vh] hover:border-primary grow overflow-hidden rounded-sm border border-transparent`}
				>
					{#if bulkEdit}
						<input
							type="checkbox"
							class="accent-primary absolute top-2 left-2 z-2 h-4 cursor-pointer"
						/>
						<button
							class="h-full w-full"
							aria-label="toggle checkbox"
							onclick={(e: MouseEvent) => {
								const img = e.target as HTMLImageElement;
								const checkbox = img.parentElement
									?.previousElementSibling as HTMLInputElement;
								if (checkbox) {
									checkbox.checked = !checkbox.checked;
								}
							}}
						>
							{@render libraryImage(image.src, 'temp')}
						</button>
					{:else}
						{@render libraryImage(image.src, 'temp')}
					{/if}
				</li>
			{/each}
		</ul>
	</main>
	<DeleteDialog bind:this={deleteDialog} />
</div>

<style>
	input:checked + * {
		filter: opacity(50%);
	}
</style>

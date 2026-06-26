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
	import ErrorDialog from './ErrorDialog.svelte';

	let fileInput: HTMLInputElement | undefined;
	let uploadForm: HTMLFormElement | undefined;
	let deleteForm: HTMLFormElement | undefined;

	let bulkEdit = $state<boolean>(false);
	let selected = $state([]);

	let deleteDialogOpen = $state<boolean>(false);

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

	const { form } = $props();
</script>

<svelte:head>
	<title>Media library - Comhairle Admin</title>
</svelte:head>

<div class="mx-auto w-4/5 p-10">
	<header class="flex flex-row items-baseline justify-between">
		<h1 class="text-4xl font-bold">Media library</h1>

		<form
			bind:this={uploadForm}
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
					uploadForm?.submit();
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
						disabled={selected.length === 0}
						onclick={() => {
							deleteDialogOpen = true;
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
		<form method="POST" action="?/delete" use:enhance bind:this={deleteForm}>
			{#snippet libraryImage(src: string, alt: string)}
				<img
					{src}
					{alt}
					class="h-full w-full cursor-pointer overflow-hidden object-cover"
				/>
			{/snippet}
			<ul class="flex flex-wrap gap-2">
				{#each images as image (image.id)}
					<li class={`relative h-[${ROW_HEIGHT}vh] grow overflow-hidden rounded-sm`}>
						{#if bulkEdit}
							<label>
								<input
									type="checkbox"
									name="items"
									value={image.id.toString()}
									class="accent-primary absolute top-2 left-2 z-2 h-4 cursor-pointer"
									bind:group={selected}
								/>
								{@render libraryImage(image.src, 'temp')}
							</label>
						{:else}
							{@render libraryImage(image.src, 'temp')}
						{/if}
					</li>
				{/each}
			</ul>
			<ErrorDialog open={!!form?.error} message={form?.error ?? ''} />
			<DeleteDialog
				open={deleteDialogOpen}
				count={selected.length}
				onconfirm={() => {
					deleteForm?.submit();
				}}
			/>
		</form>
	</main>
</div>

<style>
	input:checked + * {
		filter: opacity(50%);
	}
	img {
		transition: all 300ms ease-out;
	}
	img:is(:focus, :hover) {
		filter: brightness(85%);
		transform: scale(1.02);
	}
</style>

<script lang="ts">
	import { enhance } from '$app/forms';
	import Button from '$lib/components/ui/button/button.svelte';
	import { SquarePen, Trash2, X } from 'lucide-svelte';
	import DeleteDialog from './DeleteDialog.svelte';
	import { flip } from 'svelte/animate';
	import { MediaUpload } from '$lib/components/Media';
	import { m } from '$lib/paraglide/messages';
	import { notifications } from '$lib/notifications.svelte';
	import MediaItem from '$lib/components/Media/MediaItem.svelte';

	let deleteForm: HTMLFormElement | undefined;

	let bulkEdit = $state<boolean>(false);
	let selected = $state([]);

	const ROW_HEIGHT = 30;

	const { form, data } = $props();

	$effect(() => {
		if (form?.failures) {
			form.failures.forEach((message) => {
				notifications.send({
					message,
					priority: 'ERROR'
				});
			});
		}
	});
</script>

<svelte:head>
	<title>Media library - Comhairle Admin</title>
</svelte:head>

<div class="mx-auto w-4/5 p-10">
	<header class="flex flex-row items-baseline justify-between">
		<h1 class="text-4xl font-bold">Media library</h1>
		<div class="flex flex-row gap-4">
			{#if !bulkEdit}
				<MediaUpload />
				<Button
					variant="outline"
					onclick={() => {
						bulkEdit = true;
					}}
				>
					<SquarePen class="h-4 w-4" />{m.edit()}
				</Button>
			{:else}
				<DeleteDialog
					count={selected.length}
					onconfirm={() => {
						deleteForm?.submit();
					}}
				>
					<Button variant="destructive" disabled={selected.length === 0}>
						<Trash2 class="h-4 w-4" />{m.delete()}
					</Button>
				</DeleteDialog>
				<Button
					variant="outline"
					onclick={() => {
						bulkEdit = false;
					}}
				>
					<X class="h-4 w-4" />{m.cancel()}
				</Button>
			{/if}
		</div>
	</header>
	<main class="mt-5">
		<form method="POST" action="?/delete" use:enhance bind:this={deleteForm}>
			<ul class="flex flex-wrap gap-2">
				{#each data.media as item (item.id)}
					<li
						class={`relative h-[${ROW_HEIGHT}vh] grow overflow-hidden rounded-sm`}
						animate:flip={{ duration: 100 }}
					>
						{#if bulkEdit}
							<label>
								<input
									type="checkbox"
									name="media"
									value={item.id.toString()}
									class="accent-primary absolute top-2 left-2 z-2 h-4 cursor-pointer"
									bind:group={selected}
								/>
								<span>
									<MediaItem
										type={item.contentType.split('/')[0]}
										src={item.src}
										alt="temp"
									/>
								</span>
							</label>
						{:else}
							<MediaItem
								type={item.contentType.split('/')[0]}
								src={item.src}
								alt="temp"
							/>
						{/if}
					</li>
				{/each}
			</ul>
		</form>
	</main>
</div>

<style>
	input:checked + * {
		filter: opacity(50%);
	}

	li {
		transition: all 300ms ease-out;

		&:is(:focus, :hover) {
			filter: brightness(85%);
			transform: scale(1.01);
		}
	}
</style>

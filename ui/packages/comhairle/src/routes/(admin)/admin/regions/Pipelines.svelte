<script lang="ts">
	import { onMount } from 'svelte';
	import { invalidate } from '$app/navigation';
	import { Plus, RefreshCw, Trash2 } from 'lucide-svelte';
	import { Button } from '$lib/components/ui/button';
	import { guardUnsavedChanges } from '$lib/utils/unsavedChangesGuard.svelte';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import { key } from '$lib/utils/invalidationKey';
	import PipelineForm from './PipelineForm.svelte';
	import DeletePipelineDialog from './DeletePipelineDialog.svelte';
	import PipelinePreview from './PipelinePreview.svelte';
	import { createPipelineEditor } from './pipelineEditor.svelte';

	const editor = createPipelineEditor();
	let refreshError = $state('');
	let deleteDialogOpen = $state(false);
	guardUnsavedChanges(() => editor.dirty || editor.busy);
	onMount(() => {
		editor.load();
	});

	function canLeaveDraft() {
		return (
			!editor.busy && (!editor.dirty || window.confirm('Discard unsaved pipeline changes?'))
		);
	}

	function date(value: string | null | undefined) {
		return value ? new Date(value).toLocaleString() : 'Not scheduled';
	}

	async function publish() {
		if (!window.confirm('Publish the previewed changes and activate scheduled refreshes?'))
			return;
		refreshError = '';
		await editor.activate();
		if (editor.error) return;
		const result = await tryCatchAsync(() => invalidate(key('admin/regions')));
		if (result.err !== null) {
			refreshError =
				'Pipeline published, but the area list could not refresh. Reload the page.';
		}
	}

	async function remove(deleteAreas: boolean) {
		deleteDialogOpen = false;
		refreshError = '';
		await editor.remove(deleteAreas);
		if (editor.error) return;
		const result = await tryCatchAsync(() => invalidate(key('admin/regions')));
		if (result.err !== null) {
			refreshError =
				'Pipeline deleted, but the area list could not refresh. Reload the page.';
		}
	}
</script>

<section class="flex min-h-0 flex-1 flex-col" aria-label="Region area pipelines">
	<header class="flex flex-wrap items-center justify-between gap-4 border-b px-5 py-4 md:px-8">
		<div>
			<h2 class="text-xl font-semibold">Region area pipelines</h2>
			<p class="text-muted-foreground">
				Import and refresh source-managed Regions and geographic areas.
			</p>
		</div>
		<div class="flex gap-2">
			<Button
				variant="outline"
				disabled={editor.busy || editor.dirty}
				onclick={() => editor.load()}><RefreshCw class="size-4" />Refresh status</Button
			>
			<Button
				disabled={editor.busy}
				onclick={() => {
					if (canLeaveDraft()) editor.create();
				}}><Plus class="size-4" />New pipeline</Button
			>
		</div>
	</header>
	{#if editor.error}<p role="alert" class="text-destructive border-b p-4">{editor.error}</p>{/if}
	{#if refreshError}<p role="alert" class="text-destructive border-b p-4">{refreshError}</p>{/if}
	{#if editor.notice}<p role="status" class="border-b p-4">{editor.notice}</p>{/if}
	{#if editor.busy}<p role="status" class="border-b p-4">Working on pipeline...</p>{/if}
	<div
		class="grid min-h-0 flex-1 grid-cols-1 overflow-auto lg:grid-cols-[minmax(240px,300px)_minmax(0,1fr)]"
	>
		<aside
			class="border-b lg:overflow-auto lg:border-r lg:border-b-0"
			aria-label="Pipeline list"
		>
			{#each editor.pipelines as pipeline (pipeline.id)}
				<button
					type="button"
					disabled={editor.busy}
					class="hover:bg-muted grid w-full gap-1 border-b p-4 text-left disabled:opacity-50"
					class:bg-muted={editor.selected?.id === pipeline.id}
					aria-pressed={editor.selected?.id === pipeline.id}
					onclick={() => {
						if (canLeaveDraft()) editor.select(pipeline);
					}}
				>
					<strong>{pipeline.draft_definition.name}</strong>
					<span class="text-muted-foreground capitalize"
						>{pipeline.status} · {pipeline.run_status.replaceAll('_', ' ')}</span
					>
					{#if pipeline.last_error}<span class="text-destructive"
							>Last refresh failed</span
						>{/if}
				</button>
			{:else}
				{#if !editor.busy}<p class="text-muted-foreground p-4">
						No pipelines loaded. Create one or refresh the list.
					</p>{/if}
			{/each}
		</aside>
		<div class="min-w-0 space-y-6 p-5 md:p-8 lg:overflow-auto">
			{#if editor.editing}
				{#if editor.selected}
					{@const pipeline = editor.selected}
					<section class="grid gap-3 rounded border p-4" aria-label="Pipeline status">
						<h3 class="text-xl font-semibold capitalize">{pipeline.status} pipeline</h3>
						<p class="capitalize">
							Run status: {pipeline.run_status.replaceAll('_', ' ')}
						</p>
						<p>
							Last success: {pipeline.last_success_at
								? date(pipeline.last_success_at)
								: 'Never'}
						</p>
						<p>Next refresh: {date(pipeline.next_run_at)}</p>
						{#if pipeline.run_started_at}<p>
								Run started: {date(pipeline.run_started_at)}
							</p>{/if}
						{#if pipeline.last_error}<p
								role="alert"
								class="text-destructive break-words"
							>
								{pipeline.last_error}
							</p>{/if}
						{#if pipeline.status === 'staged'}<p>
								The previous active configuration continues to refresh until this
								draft is published.
							</p>{/if}
						{#if pipeline.active_definition}
							<details>
								<summary class="cursor-pointer">Active configuration</summary>
								<pre class="mt-3 max-h-64 overflow-auto text-base">{JSON.stringify(
										pipeline.active_definition,
										null,
										2
									)}</pre>
							</details>
						{/if}
					</section>
				{/if}
				<PipelineForm
					bind:definition={editor.definition}
					bind:rulesText={editor.rulesText}
					bind:fixedTagsText={editor.fixedTagsText}
					busy={editor.busy}
					dirty={editor.dirty}
					onsave={() => editor.save()}
					onupload={(file) => editor.upload(file)}
				/>
				{#if editor.selected}
					<section class="grid gap-4 border-t pt-6" aria-label="Publish pipeline">
						<h3 class="text-xl font-semibold">Preview and publish</h3>
						{#if editor.dirty}<p>
								Save the draft before generating a preview or publishing.
							</p>{/if}
						<div>
							<Button
								variant="outline"
								disabled={editor.busy || editor.dirty}
								onclick={() => editor.generatePreview()}>Generate preview</Button
							>
						</div>
						{#if editor.preview && !editor.dirty}<PipelinePreview
								preview={editor.preview}
							/>{/if}
						<div class="flex flex-wrap gap-3">
							<Button disabled={!editor.canActivate} onclick={publish}
								>Publish and activate</Button
							>
							{#if editor.selected.status === 'active' || editor.selected.status === 'staged'}
								<Button
									variant="outline"
									disabled={editor.busy || editor.dirty}
									onclick={() => {
										if (
											window.confirm(
												'Stop scheduled refreshes? Existing Regions and areas will be retained.'
											)
										)
											editor.deactivate();
									}}>Deactivate</Button
								>
							{/if}
							<Button
								variant="outline"
								class="text-destructive"
								disabled={editor.busy || editor.dirty}
								onclick={() => (deleteDialogOpen = true)}
								><Trash2 class="size-4" />Delete pipeline</Button
							>
						</div>
					</section>
					{#if deleteDialogOpen}
						<DeletePipelineDialog
							name={editor.selected.draft_definition.name}
							onConfirm={remove}
							onClose={() => (deleteDialogOpen = false)}
						/>
					{/if}
				{/if}
			{:else}
				<p class="text-muted-foreground">Select a pipeline or create a new draft.</p>
			{/if}
		</div>
	</div>
</section>

<script lang="ts">
	import { onMount } from 'svelte';
	import { tryFetch } from '$lib/utils/errorHandling';
	import { HttpStatus } from '$lib/utils/constants';

	type Props = { src: string };

	let { src }: Props = $props();

	let text = $state<string | null>(null);
	let failed = $state(false);
	// A 404 means the source is gone (e.g. the learning material was re-synced and
	// this answer's document was replaced), not a transient load failure - worth a
	// clearer, less alarming message than a generic error.
	let gone = $state(false);

	// Rendered as escaped text (not {@html}) so an uploaded text file can never
	// inject markup. Synced learn content is markdown, which reads fine this way.
	onMount(async () => {
		const res = await tryFetch(src);
		if (res.err !== null) {
			// A 404 means the source is gone (re-synced), not a transient failure.
			if (res.err.id === 'HTTP_ERROR' && res.err.status === HttpStatus.NotFound) {
				gone = true;
				return;
			}
			console.error(res.err);
			failed = true;
			return;
		}
		text = await res.ok.text();
	});
</script>

<div class="h-full w-full overflow-auto bg-white p-6">
	{#if gone}
		<div class="mx-auto max-w-3xl">
			<p class="text-foreground text-base font-medium">This source is no longer available</p>
			<p class="text-muted-foreground mt-1 text-base">
				The learning material was updated after this answer was written, so the passage it
				cited can't be shown.
			</p>
		</div>
	{:else if failed}
		<p class="text-destructive text-base">Failed to load document.</p>
	{:else if text === null}
		<p class="text-muted-foreground text-base">Loading...</p>
	{:else}
		<div class="mx-auto max-w-3xl text-base leading-relaxed break-words whitespace-pre-wrap">
			{text}
		</div>
	{/if}
</div>

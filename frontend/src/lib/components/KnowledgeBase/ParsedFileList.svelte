<script lang="ts">
	import type { ComhairleDocument } from '$lib/api/api';
	import ParsedFileBadge from './ParsedFileBadge.svelte';

	type Props = {
		documents: ComhairleDocument[];
	};

	let { documents }: Props = $props();
</script>

<h2 class="mb-3 font-bold">Uploaded files</h2>
<ul class="flex flex-col gap-3">
	{#each documents as document (document.id)}
		<li>
			<ParsedFileBadge {document} />
		</li>
	{/each}
</ul>

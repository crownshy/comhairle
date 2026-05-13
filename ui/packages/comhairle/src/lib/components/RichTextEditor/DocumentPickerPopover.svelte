<script lang="ts">
	import * as Popover from '$lib/components/ui/popover';
	import { Button } from '$lib/components/ui/button';
	import { FileText, LoaderCircle } from 'lucide-svelte';
	import type { ComhairleDocument } from '@crownshy/api-client/api';

	type Props = {
		open: boolean;
		documents: ComhairleDocument[];
		onSelect: (documentId: string) => void;
		onOpenChange: (open: boolean) => void;
		children: any;
	};

	let { open = $bindable(false), documents, onSelect, onOpenChange, children }: Props = $props();

	function handleSelect(docId: string) {
		onSelect(docId);
		onOpenChange(false);
	}

	function formatSize(bytes: number): string {
		if (bytes < 1024) return `${bytes} B`;
		if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
		return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
	}
</script>

<Popover.Root bind:open {onOpenChange}>
	<Popover.Trigger>
		{@render children()}
	</Popover.Trigger>
	<Popover.Content class="max-h-72 w-80 overflow-y-auto" side="bottom" align="start">
		{#if documents.length === 0}
			<p class="text-muted-foreground p-2 text-center text-sm">
				No parsed documents available.
			</p>
		{:else}
			<ul class="flex flex-col gap-1">
				{#each documents as doc (doc.id)}
					<li>
						<button
							type="button"
							class="flex w-full items-center gap-2 rounded-md px-3 py-2 text-left text-sm transition-colors hover:bg-gray-100"
							onclick={() => handleSelect(doc.id)}
						>
							<FileText class="text-muted-foreground h-4 w-4 shrink-0" />
							<div class="flex min-w-0 flex-col">
								<span class="truncate font-medium">{doc.name}</span>
								<span class="text-muted-foreground text-xs"
									>{formatSize(doc.size)}</span
								>
							</div>
						</button>
					</li>
				{/each}
			</ul>
		{/if}
	</Popover.Content>
</Popover.Root>

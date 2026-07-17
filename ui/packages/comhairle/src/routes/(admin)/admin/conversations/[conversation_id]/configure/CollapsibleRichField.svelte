<script lang="ts">
	import type { Snippet } from 'svelte';
	import { Plus, Pencil } from 'lucide-svelte';
	import ContentRenderer from '$lib/components/RichTextEditor/ContentRenderer/ContentRenderer.svelte';

	type Props = {
		/** Field name, used in the empty-state call to action ("Add privacy policy"). */
		label: string;
		/** Current primary-locale value, for the preview and the empty check. */
		content: string | null | undefined;
		/** Whether this field is expanded. Controlled by the parent so only one is open at a time. */
		open: boolean;
		/** Request to expand (true) or collapse (false) this field. */
		onOpenChange: (open: boolean) => void;
		/** The editor to reveal when expanded (a TranslatableField and its errors). */
		children: Snippet;
	};

	let { label, content, open, onOpenChange, children }: Props = $props();

	// Rich text counts as empty when it's null/blank or just an empty paragraph, so a
	// never-set field shows the "Add …" call to action rather than an empty preview card.
	let isEmpty = $derived.by(() => {
		const raw = (content ?? '').trim();
		if (!raw) return true;
		const withoutTags = raw
			.replace(/<[^>]*>/g, '')
			.replace(/&nbsp;/g, '')
			.trim();
		return withoutTags.length === 0;
	});
</script>

{#if open}
	<div class="flex flex-col gap-2">
		{@render children()}
		<div>
			<button
				type="button"
				class="text-muted-foreground hover:text-foreground text-sm font-medium"
				onclick={() => onOpenChange(false)}
			>
				Done
			</button>
		</div>
	</div>
{:else if isEmpty}
	<button
		type="button"
		onclick={() => onOpenChange(true)}
		class="bg-card border-border text-muted-foreground hover:border-foreground/40 hover:text-foreground flex h-11 w-full items-center gap-2 rounded-lg border border-dashed px-4 text-sm"
	>
		<Plus class="size-4" />
		Add {label.toLowerCase()}
	</button>
{:else}
	<button
		type="button"
		onclick={() => onOpenChange(true)}
		class="bg-card border-border hover:border-primary/60 flex w-full flex-col gap-2 rounded-lg border p-4 text-left"
	>
		<div class="text-foreground line-clamp-3 w-full text-sm">
			<ContentRenderer content={content ?? ''} minimal />
		</div>
		<span class="text-primary inline-flex items-center gap-1 text-sm font-medium">
			<Pencil class="size-3.5" />
			Edit
		</span>
	</button>
{/if}

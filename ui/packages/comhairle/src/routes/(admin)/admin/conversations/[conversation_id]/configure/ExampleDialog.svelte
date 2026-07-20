<script lang="ts">
	import * as Dialog from '$lib/components/ui/dialog';

	type Props = {
		/** Whether the dialog is open (two-way). */
		open: boolean;
		/** Field name, shown as the dialog title. */
		title: string;
		/** Static example image path (e.g. /examples/privacy-policy.png), or null. */
		src: string | null;
	};

	let { open = $bindable(), title, src }: Props = $props();

	// Writable-derived: recomputes to `false` whenever `src` changes (so switching fields
	// clears a prior error), but the img `onerror` handler can still flip it to `true`.
	let failed = $derived.by(() => {
		void src;
		return false;
	});
</script>

<Dialog.Root bind:open>
	<Dialog.Content class="max-w-3xl">
		<Dialog.Header>
			<Dialog.Title>{title}</Dialog.Title>
			<Dialog.Description>Where this appears for participants.</Dialog.Description>
		</Dialog.Header>
		{#if src && !failed}
			<img
				{src}
				alt={`Example of ${title}`}
				class="border-border w-full rounded-lg border"
				onerror={() => (failed = true)}
			/>
		{:else}
			<div
				class="border-border text-muted-foreground flex h-48 items-center justify-center rounded-lg border border-dashed text-sm"
			>
				Example coming soon.
			</div>
		{/if}
	</Dialog.Content>
</Dialog.Root>

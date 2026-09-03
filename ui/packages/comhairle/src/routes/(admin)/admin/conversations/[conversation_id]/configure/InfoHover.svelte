<script lang="ts">
	import * as HoverCard from '$lib/components/ui/hover-card';
	import * as Dialog from '$lib/components/ui/dialog';
	import { Info, Image } from '@lucide/svelte';

	interface Props {
		info: string;
		example?: {
			title: string;
			src: string;
		};
	}

	let { info, example }: Props = $props();

	let open = $state<boolean>(false);

	// Writable-derived: recomputes to `false` whenever `src` changes (so switching fields
	// clears a prior error), but the img `onerror` handler can still flip it to `true`.
	let failed = $derived.by(() => {
		void example;
		return false;
	});
</script>

<HoverCard.Root openDelay={150} closeDelay={100}>
	<HoverCard.Trigger
		class="text-muted-foreground hover:text-foreground inline-flex cursor-help"
		aria-label="More information"
	>
		<Info class="size-4" />
	</HoverCard.Trigger>
	<HoverCard.Content class="w-72 text-sm" side="top" sideOffset={6}>
		<p>{info}</p>
		{#if example}
			<button
				type="button"
				onclick={() => void (open = true)}
				class="text-primary mt-3 inline-flex items-center gap-1 text-sm font-medium hover:underline"
			>
				<Image class="size-3.5" />
				See example
			</button>
		{/if}
	</HoverCard.Content>
</HoverCard.Root>

{#if example}
	<Dialog.Root bind:open>
		<Dialog.Content class="max-w-3xl">
			<Dialog.Header>
				<Dialog.Title>{example.title}</Dialog.Title>
				<Dialog.Description>Where this appears for participants.</Dialog.Description>
			</Dialog.Header>
			{#if example.src && !failed}
				<img
					src={example.src}
					alt={`Example of ${example.title}`}
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
{/if}

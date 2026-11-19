<script lang="ts">
	import * as Dialog from '$lib/components/ui/dialog';
	import ContentRenderer from '$lib/components/RichTextEditor/ContentRenderer/ContentRenderer.svelte';
	import { m } from '$lib/paraglide/messages';

	type Props = {
		stepName: string;
		/** The admin's Data protocol text for this step (rich-text HTML). */
		text: string;
		open: boolean;
	};

	let { stepName, text, open = $bindable(false) }: Props = $props();
</script>

<Dialog.Root bind:open>
	<Dialog.Content class="max-h-[calc(100dvh-2rem)] gap-4 overflow-y-auto p-6 sm:max-w-lg">
		<Dialog.Header class="gap-1 text-start">
			<p class="text-muted-foreground text-sm font-medium">{stepName}</p>
			<Dialog.Title class="text-xl">{m.data_protocol_dialog_heading()}</Dialog.Title>
		</Dialog.Header>
		<div class="border-border border-t pt-4">
			<ContentRenderer content={text} />
		</div>
	</Dialog.Content>
</Dialog.Root>

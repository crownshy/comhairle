<script lang="ts">
	import ShieldCheck from '@lucide/svelte/icons/shield-check';
	import type { ComhairleDocument } from '@crownshy/api-client/api';
	import * as Dialog from '$lib/components/ui/dialog';
	import StepDataProtocol from '$lib/components/StepDataProtocol.svelte';
	import { useSupportDrawer } from '$lib/components/supportDrawerContext.svelte';
	import { m } from '$lib/paraglide/messages';

	type Props = {
		stepName: string;
		/** The admin's Data protocol text, or blank for the tool default (ADR-0050). */
		text: string | null | undefined;
		toolType: string | undefined;
		availableDocuments?: ComhairleDocument[];
		conversationId?: string;
	};

	let { stepName, text, toolType, availableDocuments, conversationId }: Props = $props();

	const supportDrawer = useSupportDrawer();
	let open = $state(false);

	function openFaq() {
		open = false;
		supportDrawer.openOn('faqs');
	}
</script>

<Dialog.Root bind:open>
	<Dialog.Trigger
		class="text-muted-foreground hover:text-foreground flex items-center gap-1.5 text-sm underline underline-offset-4 sm:text-base"
	>
		<ShieldCheck class="size-4 shrink-0" aria-hidden="true" />
		<span class="sm:hidden">{m.data_protocol_link_short()}</span>
		<span class="max-sm:hidden">{m.data_protocol_link()}</span>
	</Dialog.Trigger>
	<Dialog.Content class="max-h-[calc(100dvh-2rem)] gap-4 overflow-y-auto p-6 sm:max-w-lg">
		<Dialog.Header class="gap-1 text-start">
			<p class="text-muted-foreground text-sm font-medium">{stepName}</p>
			<Dialog.Title class="text-xl">{m.data_protocol_dialog_heading()}</Dialog.Title>
		</Dialog.Header>
		<div class="border-border border-t pt-4">
			<StepDataProtocol
				{text}
				{toolType}
				onOpenFaq={openFaq}
				{availableDocuments}
				{conversationId}
			/>
		</div>
	</Dialog.Content>
</Dialog.Root>

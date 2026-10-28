<script lang="ts">
	import * as Dialog from '$lib/components/ui/dialog';
	import * as Tabs from '$lib/components/ui/tabs';
	import TranslationEditor from '$lib/components/Translation/TranslationEditor.svelte';
	import type { AutosaveSource } from '$lib/components/Translation/translationUtils';
	import type { Locale } from '$lib/paraglide/runtime';

	type Props = {
		open: boolean;
		summarySource: AutosaveSource;
		bodySource: AutosaveSource;
		primaryLocale: Locale;
		supportedLanguages: Locale[];
		conversationId: string;
		disabled: boolean;
		/** The dialog can edit the main language too, so the page reloads its editors after. */
		onClosed: () => void;
	};
	let {
		open = $bindable(),
		summarySource,
		bodySource,
		primaryLocale,
		supportedLanguages,
		conversationId,
		disabled,
		onClosed
	}: Props = $props();

	let tab = $state('summary');

	// Commit any pending debounced edit before the dialog goes away, as TranslatableField does.
	async function close() {
		await Promise.all([summarySource.flush(), bodySource.flush()]);
		open = false;
		onClosed();
	}
</script>

<Dialog.Root
	{open}
	onOpenChange={(isOpen) => {
		if (!isOpen) close();
	}}
>
	<Dialog.Content class="h-[90vh] min-w-[80vw] grid-rows-[auto_1fr] rounded-xl p-12 pt-5">
		<Dialog.Header>
			<Dialog.Title class="text-muted-foreground text-xl font-semibold"
				>Translations</Dialog.Title
			>
		</Dialog.Header>
		<Tabs.Root bind:value={tab} class="min-h-0">
			<Tabs.List>
				<Tabs.Trigger value="summary" class="text-base">Summary</Tabs.Trigger>
				<Tabs.Trigger value="body" class="text-base">Report</Tabs.Trigger>
			</Tabs.List>
			<div class="max-h-[calc(90vh-180px)] overflow-y-auto pt-4">
				<Tabs.Content value="summary">
					<TranslationEditor
						source={summarySource}
						{primaryLocale}
						{supportedLanguages}
						editorType="rich"
						{conversationId}
						{disabled}
					/>
				</Tabs.Content>
				<Tabs.Content value="body">
					<TranslationEditor
						source={bodySource}
						{primaryLocale}
						{supportedLanguages}
						editorType="rich"
						{conversationId}
						{disabled}
					/>
				</Tabs.Content>
			</div>
		</Tabs.Root>
	</Dialog.Content>
</Dialog.Root>

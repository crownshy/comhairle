<script lang="ts">
	import * as Dialog from '$lib/components/ui/dialog';
	import { Button } from '$lib/components/ui/button';
	import ContentRenderer from '$lib/components/RichTextEditor/ContentRenderer/ContentRenderer.svelte';
	import ComhairlePrivacyPolicy from './ComhairlePrivacyPolicy.svelte';

	type Props = {
		privacyPolicy?: string | null;
		open: boolean;
		onAccept: () => void;
	};

	let { privacyPolicy, open = $bindable(), onAccept }: Props = $props();

	function handleAccept() {
		open = false;
		onAccept();
	}
</script>

<Dialog.Root bind:open>
	<Dialog.Content
		showCloseButton={false}
		escapeKeydownBehavior="ignore"
		interactOutsideBehavior="ignore"
		class="flex max-h-[85vh] flex-col gap-0 p-0 sm:max-w-lg"
	>
		<Dialog.Header class="border-b px-6 pt-6 pb-4">
			<Dialog.Title>Privacy Policy</Dialog.Title>
			<Dialog.Description>
				Please review the privacy policy before joining this conversation.
			</Dialog.Description>
		</Dialog.Header>

		<div class="overflow-y-auto px-6 py-4">
			{#if privacyPolicy}
				<ContentRenderer content={privacyPolicy} />
			{:else}
				<ComhairlePrivacyPolicy
					class="[&_h1]:text-primary [&_h2]:text-primary flex flex-col gap-4 [&_h1,&_h2,&_h3,&_h4,&_h5,&_h6]:font-bold [&_ul]:list-inside [&_ul]:list-[square]!"
				/>
			{/if}
		</div>

		<Dialog.Footer class="border-t px-6 pt-4 pb-6">
			<Button class="w-full" onclick={handleAccept}>Continue</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>

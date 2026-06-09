<script lang="ts">
	import * as Dialog from '$lib/components/ui/dialog';
	import { Button } from '$lib/components/ui/button';

	type Props = {
		open: boolean;
		onOpenChange: (open: boolean) => void;
		onShare: () => void;
		onKeepPrivate: () => void;
	};

	let { open, onOpenChange, onShare, onKeepPrivate }: Props = $props();
</script>

<Dialog.Root {open} {onOpenChange}>
	<Dialog.Content class="sm:max-w-md">
		<Dialog.Header>
			<Dialog.Title>Share your thinking?</Dialog.Title>
			<Dialog.Description>
				We hope you have found this experience useful to refine your thoughts on the topic.
				If you are willing to share these responses with the organizers, it will help guide
				future conversations. You can opt in or out of this at any time.
			</Dialog.Description>
		</Dialog.Header>
		<Dialog.Footer class="gap-2 sm:gap-2">
			<Button variant="outline" onclick={onKeepPrivate}>Keep private</Button>
			<Button onclick={onShare}>Share with organizers</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>

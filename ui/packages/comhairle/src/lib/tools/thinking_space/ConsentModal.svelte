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
				We hope this has been a useful space to reflect and clarify your thoughts. If you're
				willing to share your responses with the organizing team, it will help them
				understand what delegates are bringing to the assembly and shape how the sessions
				are designed. You can change your mind at any time.
			</Dialog.Description>
		</Dialog.Header>
		<Dialog.Footer class="gap-2 sm:gap-2">
			<Button variant="outline" onclick={onKeepPrivate}>Keep private</Button>
			<Button onclick={onShare}>Share with organizers</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>

<script lang="ts">
	import * as m from '$lib/paraglide/messages';
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
			<Dialog.Title>{m.thinking_space_share_thinking()}</Dialog.Title>
			<Dialog.Description>{m.thinking_space_share_desc()}</Dialog.Description>
		</Dialog.Header>
		<Dialog.Footer class="gap-2 sm:gap-2">
			<Button variant="outline" onclick={onKeepPrivate}>{m.keep_private()}</Button>
			<Button onclick={onShare}>{m.share_with_org()}</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>

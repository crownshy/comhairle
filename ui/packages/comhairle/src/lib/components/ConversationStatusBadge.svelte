<script lang="ts">
	import type { ConversationStatus } from '$lib/interfaces/Conversation';
	import { cn } from '$lib/utils';
	import { capitalise } from '$lib/utils/casingUtils';

	type Props = { status: ConversationStatus; label?: string; class?: string };
	let { status, label, class: className = '' }: Props = $props();

	// FIX: Consolidate with ConversationStatus.svelte
	// One colour per status, shown as a dot so the meaning doesn't rely on colour alone.
	const DOT: Record<ConversationStatus, string> = {
		live: 'bg-green-600',
		draft: 'bg-muted-foreground',
		completed: 'bg-primary'
	};
</script>

<!-- The one status style used on every card and row, on the image or on a plain background. -->
<span
	class={cn(
		'bg-background/90 text-foreground flex w-fit items-center gap-2 rounded-full border px-3 py-1 text-sm font-semibold',
		className
	)}
>
	<span class={cn('size-2.5 rounded-full', DOT[status])} aria-hidden="true"></span>
	{label ?? capitalise(status)}
</span>

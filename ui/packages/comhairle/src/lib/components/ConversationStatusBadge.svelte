<script lang="ts">
	import type { ConversationStatus } from '$lib/interfaces/Conversation';
	import { cn } from '$lib/utils';
	import { capitalise } from '$lib/utils/casingUtils';

	type Props = {
		status: ConversationStatus;
		label?: string;
		floating?: boolean;
		class?: string;
	};

	let { status, label, floating, class: classes }: Props = $props();

	function DOT_STYLES(status: ConversationStatus): string {
		switch (status) {
			case 'live':
				return 'bg-emerald-400';
			case 'draft':
				return 'bg-orange-400';
			case 'completed':
				return 'bg-sidebar-foreground/40';
		}
	}
</script>

<span
	class={cn(
		floating &&
			'bg-background/90 absolute top-2 left-2 rounded-full border-transparent px-2 py-1 shadow-md backdrop-blur',
		'text-foreground/90 flex items-center gap-x-2 text-sm',
		classes
	)}
>
	<span class={cn('size-2 shrink-0 rounded-full', DOT_STYLES(status))} aria-hidden="true"></span>
	{label ?? capitalise(status)}
</span>

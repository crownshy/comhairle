<script lang="ts">
	import ChevronUp from '@lucide/svelte/icons/chevron-up';
	import { cn } from '$lib/utils';

	type Props = {
		title: string;
		expanded: boolean;
		onToggle: () => void;
	};

	let { title, expanded, onToggle }: Props = $props();
</script>

<!-- The grab handle alone didn't read as "this opens"; the chevron points the way it will move. -->
<button
	type="button"
	class="flex h-16 w-full shrink-0 cursor-pointer flex-col items-center justify-center gap-2"
	onclick={onToggle}
	aria-expanded={expanded}
>
	<div class="bg-foreground/20 h-1 w-10 rounded-full" aria-hidden="true"></div>
	<span class="text-foreground flex items-center gap-1.5 text-sm font-medium">
		{title}
		<ChevronUp
			class={cn(
				'h-4 w-4 transition-transform duration-200 motion-reduce:transition-none',
				expanded && 'rotate-180'
			)}
			aria-hidden="true"
		/>
	</span>
</button>

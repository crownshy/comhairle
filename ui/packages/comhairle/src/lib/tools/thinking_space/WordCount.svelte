<script lang="ts">
	import { m } from '$lib/paraglide/messages';
	import { cn } from '$lib/utils';

	type Props = {
		id: string;
		count: number;
		max: number;
		class?: string;
	};

	let { id, count, max, class: className }: Props = $props();
</script>

<span {id} class={cn('text-base', count > max && 'text-destructive font-semibold', className)}>
	{m.thinking_space_word_count({ count, max })}
</span>

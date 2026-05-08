<script lang="ts">
	import { cn } from '$lib/utils';
	import { Image } from 'lucide-svelte';

	type Props = {
		class?: string;
	};

	const { class: className }: Props = $props();
</script>

<div class={cn('bg-primary/10 absolute inset-0 flex items-center justify-center', className)}>
	<Image
		class="text-primary/30 group-hover:text-primary/50 h-32 w-32 transition-all duration-300 group-hover:scale-110"
	/>
</div>

<script lang="ts">
	import type { LayoutProps } from './$types';

	let { children }: LayoutProps = $props();
</script>

<div
	class="bg-muted flex w-full flex-col justify-between gap-11 border-b-black px-4 py-6 sm:px-8 md:px-16 md:py-8"
>
	{@render children()}
</div>

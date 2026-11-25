<script lang="ts">
	import type { Snippet } from 'svelte';

	type Props = {
		/**
		 * `alert` interrupts the screen reader and is announced as soon as it mounts, so it
		 * can sit inside an `{#if}`. `status` waits for a pause and is for save states; keep
		 * it mounted and toggle what's inside, since a status region that mounts with its text
		 * already in it is often skipped.
		 */
		kind: 'alert' | 'status';
		class?: string;
		children?: Snippet;
	};

	let { kind, class: className, children }: Props = $props();
</script>

<div role={kind} class={className}>
	{@render children?.()}
</div>

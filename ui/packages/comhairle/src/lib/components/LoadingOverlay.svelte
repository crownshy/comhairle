<script lang="ts">
	import { untrack } from 'svelte';
	import { Portal } from 'bits-ui';
	import { Spinner } from '$lib/components/ui/spinner';

	/**
	 * A full-screen, screen-level loading overlay for slow, deliberate actions (creating
	 * a conversation, adding a workflow step) where a disabled button alone is too subtle.
	 *
	 * Anti-flicker: once shown it stays for at least {@link minVisibleMs}, so an action that
	 * finishes almost instantly can't blink the overlay on and off.
	 *
	 * Portaled to `<body>` so no ancestor stacking context (e.g. the sidebar) can trap it,
	 * and above dialogs/toasts so it reads as a single clear "working…" state.
	 */
	type Props = {
		/** Whether the underlying action is in progress. */
		open: boolean;
		/** Short status message shown next to the spinner, e.g. `"Adding step…"`. */
		message: string;
		/** Once shown, keep it visible at least this long (ms) so it never flickers. */
		minVisibleMs?: number;
	};
	let { open, message, minVisibleMs = 500 }: Props = $props();

	let visible = $state(false);
	// Plain (non-reactive) so reading it inside the effect doesn't re-trigger the effect.
	let shownAt = 0;

	$effect(() => {
		// Track only `open` (and `minVisibleMs`). `visible` is read via `untrack` so setting
		// it here never re-runs this effect and resets the hide timer.
		if (open) {
			visible = true;
			shownAt = Date.now();
			return;
		}

		if (!untrack(() => visible)) return; // never showed: nothing to hide
		const remaining = Math.max(0, minVisibleMs - (Date.now() - shownAt));
		const hideTimer = setTimeout(() => (visible = false), remaining);
		return () => clearTimeout(hideTimer);
	});
</script>

{#if visible}
	<Portal>
		<div
			class="bg-background/70 fixed inset-0 z-100 flex items-center justify-center backdrop-blur-sm"
			role="status"
			aria-live="polite"
		>
			<div
				class="bg-card border-border flex items-center gap-3 rounded-xl border px-5 py-4 shadow-lg"
			>
				<Spinner class="text-primary size-5" />
				<span class="text-foreground text-base font-medium">{message}</span>
			</div>
		</div>
	</Portal>
{/if}

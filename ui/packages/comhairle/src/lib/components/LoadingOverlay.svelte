<script lang="ts">
	import { Portal } from 'bits-ui';
	import { Spinner } from '$lib/components/ui/spinner';

	/**
	 * A full-screen, screen-level loading overlay for slow, deliberate actions (creating
	 * a conversation, adding a workflow step) where a disabled button alone is too subtle.
	 *
	 * Portaled to `<body>` so no ancestor stacking context (e.g. the sidebar) can trap it,
	 * and above dialogs/toasts so it reads as a single clear "working…" state.
	 */
	type Props = {
		/** Whether the overlay is shown. */
		open: boolean;
		/** Short status message shown next to the spinner, e.g. `"Adding step…"`. */
		message: string;
	};
	let { open, message }: Props = $props();
</script>

{#if open}
	<Portal>
		<div
			class="bg-background/70 fixed inset-0 z-[100] flex items-center justify-center backdrop-blur-sm"
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

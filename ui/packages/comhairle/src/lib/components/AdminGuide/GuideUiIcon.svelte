<script lang="ts">
	import { Check, Pencil, X } from '@lucide/svelte';
	import type { GuideIconName } from '$lib/admin_guide_text';

	let { name }: { name: GuideIconName } = $props();

	// Same icons and colours as the action buttons in
	// lib/tools/polis/polis-moderation/StatementModerationRow.svelte, shown in their
	// hover state so they read as buttons. Not interactive: it's a picture of the button.
	const ICONS = {
		accept: { icon: Check, label: 'Accept (tick)', tone: 'text-primary bg-primary/15' },
		reject: { icon: X, label: 'Reject (cross)', tone: 'text-destructive bg-destructive/15' },
		split: {
			icon: Pencil,
			label: 'Split or reword (pencil)',
			tone: 'text-muted-foreground bg-muted'
		}
	} as const;

	let item = $derived(ICONS[name]);
</script>

<span
	role="img"
	aria-label={item.label}
	title={item.label}
	class="{item.tone} mx-0.5 inline-flex size-7 items-center justify-center rounded-full align-middle"
>
	<item.icon class="size-4" strokeWidth={2.5} aria-hidden="true" />
</span>

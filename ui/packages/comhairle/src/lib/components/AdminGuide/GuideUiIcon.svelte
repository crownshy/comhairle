<script lang="ts">
	import {
		ArrowUpRight,
		Briefcase,
		Check,
		EllipsisVertical,
		PanelLeftClose,
		PanelLeftOpen,
		Pencil,
		X
	} from '@lucide/svelte';
	import { cn } from '$lib/utils';
	import { buttonVariants } from '$lib/components/ui/button';
	import * as m from '$lib/paraglide/messages';
	import type { GuideIconName } from '$lib/admin_guide_text';

	let { name }: { name: GuideIconName } = $props();

	// Pictures of real buttons, not interactive. Keep them in step with:
	// - accept/reject/split: lib/tools/polis/polis-moderation/StatementModerationRow.svelte
	//   (shown in their hover state so they read as buttons)
	// - collapse/expand: lib/components/AdminNav.svelte (on the sidebar's own background)
	// - actions: a step's actions menu trigger (MoreVertical) in
	//   routes/(admin)/admin/conversations/[conversation_id]/design/+page.svelte, in its hover state
	// - login/signup: the submit buttons in routes/(public)/auth/login and auth/signup,
	//   using the same button styles and translated labels
	// - workspace: the Workspace button in lib/components/NavBar.svelte, on the navbar's blue
	// - preview: the Preview button in routes/(admin)/admin/conversations/[conversation_id]/+layout.svelte
	//   (same classes, but h-8 instead of h-10 so it fits in a line of text)
	const ICONS = {
		accept: {
			icon: Check,
			label: 'Accept (tick)',
			tone: 'text-primary bg-primary/15 rounded-full',
			stroke: 2.5
		},
		reject: {
			icon: X,
			label: 'Reject (cross)',
			tone: 'text-destructive bg-destructive/15 rounded-full',
			stroke: 2.5
		},
		split: {
			icon: Pencil,
			label: 'Split or reword (pencil)',
			tone: 'text-muted-foreground bg-muted rounded-full',
			stroke: 2.5
		},
		collapse: {
			icon: PanelLeftClose,
			label: 'Collapse sidebar',
			tone: 'bg-sidebar text-sidebar-foreground/70 rounded-md',
			stroke: 2
		},
		expand: {
			icon: PanelLeftOpen,
			label: 'Expand sidebar',
			tone: 'bg-sidebar text-sidebar-foreground/70 rounded-md',
			stroke: 2
		},
		actions: {
			icon: EllipsisVertical,
			label: 'Step actions menu',
			tone: 'text-foreground bg-card rounded-md border',
			stroke: 2
		}
	} as const;

	const BUTTONS = {
		login: {
			label: () => m.login(),
			class: buttonVariants({ variant: 'default', size: 'sm' })
		},
		signup: {
			label: () => m.sign_up(),
			class: buttonVariants({ variant: 'default', size: 'sm' })
		},
		preview: {
			label: () => 'Preview',
			class: cn(
				buttonVariants({ variant: 'secondary' }),
				'bg-primary/20 text-foreground inline-flex h-8 rounded-full px-4 text-sm'
			),
			trailingIcon: ArrowUpRight
		},
		workspace: {
			label: () => m.workspace(),
			class: 'bg-primary text-primary-foreground inline-flex h-7 items-center gap-1.5 rounded-full px-3 text-sm',
			leadingIcon: Briefcase
		}
	} as const;
</script>

{#if name in BUTTONS}
	{@const button = BUTTONS[name as keyof typeof BUTTONS]}
	{@const label = button.label()}
	<span role="img" aria-label={`${label} button`} class="{button.class} mx-0.5 align-middle"
		>{#if 'leadingIcon' in button}<button.leadingIcon
				class="size-3.5"
				aria-hidden="true"
			/>{/if}{label}{#if 'trailingIcon' in button}<button.trailingIcon
				class="size-4"
				aria-hidden="true"
			/>{/if}</span
	>
{:else}
	{@const item = ICONS[name as keyof typeof ICONS]}
	<span
		role="img"
		aria-label={item.label}
		title={item.label}
		class="{item.tone} mx-0.5 inline-flex size-7 items-center justify-center align-middle"
	>
		<item.icon class="size-4" strokeWidth={item.stroke} aria-hidden="true" />
	</span>
{/if}

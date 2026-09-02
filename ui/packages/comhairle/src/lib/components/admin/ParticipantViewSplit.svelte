<script lang="ts">
	import type { Snippet } from 'svelte';
	import { Smartphone, Monitor, Maximize2, X } from 'lucide-svelte';
	import { Button } from '$lib/components/ui/button';
	import * as Dialog from '$lib/components/ui/dialog';
	import { screenScale, type Device } from './participantView';

	let {
		editor,
		screens,
		description = 'What a participant sees. Updates as you type.'
	}: {
		editor: Snippet;
		/** The screens to show, rendered once for the dock and once for the expanded view. */
		screens: Snippet<[{ device: Device; scale: number }]>;
		description?: string;
	} = $props();

	let device = $state<Device>('phone');
	let expanded = $state(false);

	// Measured rather than assumed: a desktop screen is fitted to the width it actually has,
	// which differs between the dock and the expanded dialog.
	let dockWidth = $state(0);
	let expandedWidth = $state(0);

	let dockScale = $derived(screenScale({ device, available: dockWidth, expanded: false }));
	let expandedScale = $derived(screenScale({ device, available: expandedWidth, expanded: true }));
</script>

{#snippet deviceToggle()}
	<div class="flex items-center gap-1">
		<Button
			variant={device === 'phone' ? 'secondary' : 'ghost'}
			size="sm"
			aria-pressed={device === 'phone'}
			onclick={() => (device = 'phone')}
		>
			<Smartphone class="size-4" />
			Phone
		</Button>
		<Button
			variant={device === 'desktop' ? 'secondary' : 'ghost'}
			size="sm"
			aria-pressed={device === 'desktop'}
			onclick={() => (device = 'desktop')}
		>
			<Monitor class="size-4" />
			Desktop
		</Button>
	</div>
{/snippet}

<!-- The dock needs the editor column plus a 420px panel plus the admin sidebar, which does
     not fit under about 1440px. Below that the panel is hidden and the expand button in the
     editor column is the only way in. -->
<div class="flex items-start gap-6">
	<div class="min-w-0 flex-1">
		<div class="mb-4 flex justify-end min-[1440px]:hidden">
			<Button variant="outline" size="sm" onclick={() => (expanded = true)}>
				<Maximize2 class="size-4" />
				Participant view
			</Button>
		</div>
		{@render editor()}
	</div>

	<aside
		class="bg-card sticky top-4 hidden max-h-[calc(100dvh-6rem)] w-[420px] shrink-0 flex-col rounded-lg border min-[1440px]:flex"
	>
		<div class="flex flex-col gap-2 border-b px-4 py-3">
			<div class="flex items-center justify-between gap-2">
				<h2 class="text-base font-medium">Participant view</h2>
				<Button
					variant="ghost"
					size="sm"
					aria-label="Expand participant view"
					onclick={() => (expanded = true)}
				>
					<Maximize2 class="size-4" />
				</Button>
			</div>
			<p class="text-muted-foreground text-sm">{description}</p>
			{@render deviceToggle()}
		</div>

		<div
			bind:clientWidth={dockWidth}
			class="flex min-h-0 flex-wrap content-start justify-center gap-4 overflow-auto p-4"
		>
			{@render screens({ device, scale: dockScale })}
		</div>
	</aside>
</div>

<Dialog.Root bind:open={expanded}>
	<Dialog.Content
		showCloseButton={false}
		class="bg-admin-background top-0 left-0 grid h-dvh w-screen max-w-none translate-x-0 translate-y-0 grid-rows-[auto_minmax(0,1fr)] gap-0 rounded-none border-0 p-0 sm:max-w-none"
	>
		<Dialog.Header
			class="bg-background flex shrink-0 flex-row flex-wrap items-center justify-between gap-3 border-b px-6 py-3"
		>
			<div class="flex min-w-0 flex-col gap-0.5">
				<Dialog.Title class="text-lg">Participant view</Dialog.Title>
				<Dialog.Description class="text-sm">{description}</Dialog.Description>
			</div>
			<div class="flex items-center gap-1">
				{@render deviceToggle()}
				<Button variant="ghost" size="sm" onclick={() => (expanded = false)}>
					<X class="size-4" />
					Close
				</Button>
			</div>
		</Dialog.Header>

		<div
			bind:clientWidth={expandedWidth}
			class="flex min-h-0 flex-wrap content-start justify-center gap-6 overflow-auto p-6"
		>
			{@render screens({ device, scale: expandedScale })}
		</div>
	</Dialog.Content>
</Dialog.Root>

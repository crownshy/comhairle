<script lang="ts">
	import type { Snippet } from 'svelte';
	import { Smartphone, Monitor, X } from 'lucide-svelte';
	import { Button } from '$lib/components/ui/button';
	import * as Dialog from '$lib/components/ui/dialog';
	import { screenScale, type Device } from './participantView';

	let {
		screens,
		description
	}: {
		screens: Snippet<[{ device: Device; scale: number }]>;
		description: string;
	} = $props();

	let open = $state(false);
	let device = $state<Device>('phone');

	// Measured rather than assumed: how much depth the overlay leaves under its header is
	// what decides how big a screen can be, and that changes with the window.
	let availableHeight = $state(0);
	let scale = $derived(screenScale({ device, availableHeight }));
</script>

<!-- Summoned, not docked: the view is something to reach for rather than a panel competing
     with the form on every visit. Desktop only, since the overlay is the whole window. -->
<Button
	variant="default"
	class="fixed right-6 bottom-6 z-40 hidden h-12 rounded-full px-5 text-base font-bold shadow-lg lg:inline-flex"
	onclick={() => (open = true)}
>
	Participant view
</Button>

<Dialog.Root bind:open>
	<!-- Full screen rather than a drawer: a participant screen is worth seeing at life size,
	     and a surface with several of them (a brief's slides, the landing page's viewports)
	     needs the whole window to show them side by side rather than two at a time. -->
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
				<Button variant="ghost" size="sm" onclick={() => (open = false)}>
					<X class="size-4" />
					Close
				</Button>
			</div>
		</Dialog.Header>

		<!-- The measured element is the row itself, not its padded parent: `clientHeight`
		     includes padding, so measuring the parent would size every screen against depth it
		     does not have. `w-max` lets the row centre while it fits and scroll once it does
		     not, which `justify-center` alone would break by clipping the first screen. -->
		<div class="min-h-0 overflow-auto p-6">
			<div bind:clientHeight={availableHeight} class="mx-auto flex h-full w-max gap-6">
				{@render screens({ device, scale })}
			</div>
		</div>
	</Dialog.Content>
</Dialog.Root>

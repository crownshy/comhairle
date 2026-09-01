<script lang="ts">
	import { Smartphone, Monitor, RefreshCw, X, Play } from 'lucide-svelte';
	import { Button } from '$lib/components/ui/button';
	import * as Dialog from '$lib/components/ui/dialog';
	import { workflow_step_url } from '$lib/urls';
	import { postStepPreview } from '$lib/step-brief/livePreview';
	import { stepPreviewDraft } from '$lib/step-brief/previewDraft.svelte';

	let {
		conversationId,
		workflowId,
		stepId
	}: {
		conversationId: string;
		workflowId: string;
		stepId: string;
	} = $props();

	let open = $state(false);
	let device = $state<'phone' | 'desktop'>('phone');
	let frame = $state<HTMLIFrameElement>();
	let ready = $state(false);
	/** Bumped to force a reload, which is the only way to pick up saved tool config. */
	let reloadKey = $state(0);

	let src = $derived(workflow_step_url(conversationId, workflowId, stepId, true));

	// Undefined on every sub-tab but Configure, and on Configure until the editor reports its
	// first value. The frame then shows the saved description, which is what it loaded with.
	let draft = $derived(stepPreviewDraft.for(stepId));

	// Debounced so a fast typist does not post on every keystroke. The frame keeps the last
	// value it was given, so a dropped intermediate costs nothing.
	$effect(() => {
		const description = draft;
		if (!ready || description === undefined) return;

		const timer = setTimeout(() => {
			postStepPreview(frame?.contentWindow, stepId, description);
		}, 250);
		return () => clearTimeout(timer);
	});

	function onLoad() {
		ready = true;
		if (draft !== undefined) postStepPreview(frame?.contentWindow, stepId, draft);
	}
</script>

<Button variant="ghost" size="sm" class="text-primary" onclick={() => (open = true)}>
	<Play class="size-4" />
	Preview step
</Button>

<!-- Full screen rather than inline: the phone is only useful at its real size, and at that
	size it crowds out the tab it was opened from (ADR-0017). -->
<Dialog.Root
	bind:open
	onOpenChange={(isOpen) => {
		if (!isOpen) ready = false;
	}}
>
	<Dialog.Content
		showCloseButton={false}
		class="bg-admin-background top-0 left-0 grid h-dvh w-screen max-w-none translate-x-0 translate-y-0 grid-rows-[auto_minmax(0,1fr)] gap-0 rounded-none border-0 p-0 sm:max-w-none"
	>
		<Dialog.Header
			class="bg-background flex shrink-0 flex-row flex-wrap items-center justify-between gap-3 border-b px-6 py-3"
		>
			<div class="flex min-w-0 flex-col gap-0.5">
				<Dialog.Title class="text-lg">Participant preview</Dialog.Title>
				<Dialog.Description class="text-sm">
					Slide breaks appear as you type; other settings need a save and a reload.
				</Dialog.Description>
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
				<Button
					variant="ghost"
					size="sm"
					aria-label="Reload preview"
					onclick={() => {
						ready = false;
						reloadKey += 1;
					}}
				>
					<RefreshCw class="size-4" />
				</Button>
				<Button variant="ghost" size="sm" onclick={() => (open = false)}>
					<X class="size-4" />
					Close
				</Button>
			</div>
		</Dialog.Header>

		<div class="flex min-h-0 justify-center overflow-auto p-6">
			{#key reloadKey}
				<iframe
					bind:this={frame}
					{src}
					onload={onLoad}
					title="Participant preview"
					class="bg-background border shadow-lg {device === 'phone'
						? 'h-full max-h-[860px] w-[390px] shrink-0 rounded-[28px]'
						: 'h-full w-full max-w-[1400px] rounded-lg'}"
				></iframe>
			{/key}
		</div>
	</Dialog.Content>
</Dialog.Root>

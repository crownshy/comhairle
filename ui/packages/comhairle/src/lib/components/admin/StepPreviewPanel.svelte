<script lang="ts">
	import { Smartphone, Monitor, RefreshCw } from 'lucide-svelte';
	import { Button } from '$lib/components/ui/button';
	import { workflow_step_url } from '$lib/urls';
	import { postStepPreview } from '$lib/step-brief/livePreview';

	let {
		conversationId,
		workflowId,
		stepId,
		description
	}: {
		conversationId: string;
		workflowId: string;
		stepId: string;
		/** The editor's unsaved value. Pushed into the frame as it changes. */
		description: string;
	} = $props();

	let device = $state<'phone' | 'desktop'>('phone');
	let frame = $state<HTMLIFrameElement>();
	let ready = $state(false);
	/** Bumped to force a reload, which is the only way to pick up saved tool config. */
	let reloadKey = $state(0);

	let src = $derived(workflow_step_url(conversationId, workflowId, stepId, true));

	// Debounced so a fast typist does not post on every keystroke. The frame keeps the last
	// value it was given, so a dropped intermediate costs nothing.
	$effect(() => {
		const draft = description;
		if (!ready) return;

		const timer = setTimeout(() => {
			postStepPreview(frame?.contentWindow, stepId, draft);
		}, 250);
		return () => clearTimeout(timer);
	});

	function onLoad() {
		ready = true;
		postStepPreview(frame?.contentWindow, stepId, description);
	}
</script>

<div class="flex flex-col gap-3">
	<div class="flex items-center justify-between gap-2">
		<span class="text-lg font-semibold">Participant preview</span>
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
		</div>
	</div>

	<p class="text-muted-foreground text-sm">
		Type <code class="bg-muted rounded px-1">---</code> in the description to start a new slide. Breaks
		appear here as you type; other settings need a save and a reload.
	</p>

	<div
		class="border-border bg-muted/40 flex justify-center overflow-hidden rounded-2xl border p-4"
	>
		{#key reloadKey}
			<iframe
				bind:this={frame}
				{src}
				onload={onLoad}
				title="Participant preview"
				class="bg-background rounded-[28px] border shadow-lg transition-[width,height] {device ===
				'phone'
					? 'h-[720px] w-[390px]'
					: 'h-[720px] w-full max-w-[1100px] rounded-lg'}"
			></iframe>
		{/key}
	</div>
</div>

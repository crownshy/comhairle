<script lang="ts">
	import type { Snippet } from 'svelte';
	import { AppWindow, Smartphone } from 'lucide-svelte';
	import { Button } from '$lib/components/ui/button';

	type Props = {
		children: Snippet;
		step?: number;
		steps?: number;
		showNext?: boolean;
		onNext?: () => void;
		nextDisabled?: boolean;
		nextLabel?: string;
		/** Shown at the bottom, above Next (for example a consent checkbox) */
		footer?: Snippet;
	};

	let {
		children,
		step = 1,
		steps = 5,
		showNext = true,
		onNext,
		nextDisabled = false,
		nextLabel = 'Next',
		footer
	}: Props = $props();

	let device = $state<'phone' | 'desktop'>('phone');
</script>

<div class="bg-foreground flex h-full min-h-0 flex-col items-center gap-6 rounded-3xl p-5">
	<div class="flex gap-3">
		<Button
			variant="secondary"
			size="icon"
			aria-label="Phone preview"
			aria-pressed={device === 'phone'}
			class={device === 'phone' ? '' : 'opacity-60'}
			onclick={() => (device = 'phone')}
		>
			<Smartphone class="size-4" />
		</Button>
		<Button
			variant="secondary"
			size="icon"
			aria-label="Desktop preview"
			aria-pressed={device === 'desktop'}
			class={device === 'desktop' ? '' : 'opacity-60'}
			onclick={() => (device = 'desktop')}
		>
			<AppWindow class="size-4" />
		</Button>
	</div>

	<div
		class="bg-card flex min-h-0 w-full flex-1 flex-col overflow-y-auto rounded-3xl p-5 {device ===
		'phone'
			? 'max-w-sm'
			: 'max-w-2xl'}"
	>
		<div class="flex gap-2" aria-hidden="true">
			{#each Array(steps) as _, index (index)}
				<span
					class="h-1.5 rounded-full {index + 1 === step
						? 'bg-primary flex-[6]'
						: 'bg-primary/15 flex-1'}"
				></span>
			{/each}
		</div>
		<div class="mt-8 flex flex-1 flex-col gap-4">
			{@render children()}
		</div>
		{#if showNext || footer}
			<div class="border-border mt-6 flex flex-col gap-4 border-t pt-4">
				{@render footer?.()}
				{#if showNext}
					<Button class="w-full" disabled={nextDisabled} onclick={onNext}
						>{nextLabel}</Button
					>
				{/if}
			</div>
		{/if}
	</div>
</div>

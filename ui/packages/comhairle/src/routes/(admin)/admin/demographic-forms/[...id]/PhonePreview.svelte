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
		/** Short caption shown under the device toggle */
		note?: string;
	};

	let {
		children,
		step = 1,
		steps = 5,
		showNext = true,
		onNext,
		nextDisabled = false,
		nextLabel = 'Next',
		footer,
		note
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
	{#if note}
		<p class="text-background/70 -mt-3 text-center text-sm">{note}</p>
	{/if}

	<!-- Device frame: a phone bezel with a camera island, or a browser window. -->
	<div
		class="relative flex min-h-0 flex-1 flex-col border border-white/20 bg-neutral-950 shadow-lg {device ===
		'phone'
			? 'aspect-[9/19.5] max-w-full rounded-[2.75rem] p-2.5'
			: 'w-full max-w-2xl overflow-hidden rounded-xl'}"
	>
		{#if device === 'phone'}
			<span
				class="absolute top-5 left-1/2 z-10 h-5 w-24 -translate-x-1/2 rounded-full bg-neutral-950"
				aria-hidden="true"
			></span>
		{:else}
			<div
				class="flex shrink-0 items-center gap-3 bg-neutral-800 px-4 py-2.5"
				aria-hidden="true"
			>
				<div class="flex gap-1.5">
					<span class="size-2.5 rounded-full bg-white/30"></span>
					<span class="size-2.5 rounded-full bg-white/30"></span>
					<span class="size-2.5 rounded-full bg-white/30"></span>
				</div>
				<span class="h-5 flex-1 rounded-md bg-neutral-950/70"></span>
			</div>
		{/if}
		<div
			class="bg-card flex min-h-0 flex-1 flex-col overflow-y-auto p-5 {device === 'phone'
				? 'rounded-[2.1rem] pt-12'
				: ''}"
		>
			<!-- A phone screen is narrower than the panel, so the content is scaled down to keep text realistic. -->
			<div
				class="flex min-h-full flex-1 flex-col"
				style:zoom={device === 'phone' ? 0.78 : 0.85}
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
			{#if device === 'phone'}
				<span
					class="bg-foreground/25 mx-auto mt-4 h-1 w-28 shrink-0 rounded-full"
					aria-hidden="true"
				></span>
			{/if}
		</div>
	</div>
</div>

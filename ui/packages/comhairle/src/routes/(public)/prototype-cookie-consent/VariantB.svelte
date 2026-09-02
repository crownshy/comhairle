<script lang="ts">
	// PROTOTYPE - Variant B: "Swipe deck".
	// Reuses the participant step-brief idiom (full screen, one thing per page, a cue at
	// the end of every page) so consent is the same interaction as the rest of the product.
	import { fly } from 'svelte/transition';
	import { necessary, optional } from './cookieFacts';

	let { onDone }: { onDone: () => void } = $props();

	const pages = [
		{
			kind: 'intro' as const,
			eyebrow: 'Before you start',
			heading: 'Three things get stored. Here they are.',
			body: 'Takes about fifteen seconds. You get a say on the last one.'
		},
		...necessary.map((fact) => ({
			kind: 'fact' as const,
			eyebrow: 'Needed to work',
			heading: fact.what,
			body: fact.plain,
			name: fact.name,
			lasts: fact.lasts
		})),
		{
			kind: 'choice' as const,
			eyebrow: 'Your call',
			heading: optional.what,
			body: optional.plain,
			name: optional.name,
			lasts: optional.lasts
		}
	];

	const dots = pages.map((_page, i) => i);

	let index = $state(0);
	let current = $derived(pages[index]);
	let isLast = $derived(index === pages.length - 1);
	let analytics = $state(true);

	let startX = 0;
	function onpointerdown(event: PointerEvent) {
		startX = event.clientX;
	}
	function onpointerup(event: PointerEvent) {
		const dx = event.clientX - startX;
		if (dx < -60) index = Math.min(pages.length - 1, index + 1);
		if (dx > 60) index = Math.max(0, index - 1);
	}
</script>

<div
	class="bg-background fixed inset-0 z-50 flex flex-col"
	role="dialog"
	aria-modal="true"
	aria-label="What gets stored"
	tabindex="-1"
	{onpointerdown}
	{onpointerup}
>
	<div class="mx-auto flex w-full max-w-2xl shrink-0 items-center gap-2 px-6 pt-8">
		{#each dots as i (i)}
			<span
				class="h-1.5 flex-1 rounded-full transition-colors duration-200 {i <= index
					? 'bg-primary'
					: 'bg-muted'}"
			></span>
		{/each}
	</div>

	<div class="flex min-h-0 flex-1 flex-col justify-center overflow-y-auto px-6 py-8">
		<div class="mx-auto w-full max-w-2xl">
			{#key index}
				<div in:fly={{ x: 24, duration: 220 }}>
					<p class="text-primary text-base font-semibold tracking-wide uppercase">
						{current.eyebrow}
					</p>
					<h2
						class="text-foreground mt-3 text-3xl leading-tight font-bold sm:text-5xl sm:leading-tight"
					>
						{current.heading}
					</h2>
					<p class="text-muted-foreground mt-5 text-lg leading-7 sm:text-xl sm:leading-8">
						{current.body}
					</p>

					{#if current.kind !== 'intro'}
						<p class="text-subtle-foreground mt-6 font-mono text-base">
							{current.name} &middot; {current.lasts.toLowerCase()}
						</p>
					{/if}

					{#if current.kind === 'choice'}
						<div class="mt-8 flex gap-3">
							<button
								type="button"
								class="flex-1 rounded-2xl border-2 px-4 py-5 text-left text-lg font-semibold transition-colors {analytics
									? 'border-primary bg-accent text-accent-foreground'
									: 'border-border text-muted-foreground'}"
								onclick={() => (analytics = true)}
								aria-pressed={analytics}
							>
								Count me
							</button>
							<button
								type="button"
								class="flex-1 rounded-2xl border-2 px-4 py-5 text-left text-lg font-semibold transition-colors {!analytics
									? 'border-primary bg-accent text-accent-foreground'
									: 'border-border text-muted-foreground'}"
								onclick={() => (analytics = false)}
								aria-pressed={!analytics}
							>
								Leave me out
							</button>
						</div>
					{/if}
				</div>
			{/key}
		</div>
	</div>

	<div class="mx-auto w-full max-w-2xl shrink-0 px-6 pb-8">
		<button
			type="button"
			class="bg-primary text-primary-foreground w-full rounded-full px-6 py-4 text-lg font-semibold"
			onclick={() => (isLast ? onDone() : (index += 1))}
		>
			{isLast ? 'Save and start' : 'Next'}
		</button>
		{#if !isLast}
			<button
				type="button"
				class="text-muted-foreground mt-3 w-full py-2 text-base underline"
				onclick={onDone}
			>
				Skip, keep everything on
			</button>
		{/if}
	</div>
</div>

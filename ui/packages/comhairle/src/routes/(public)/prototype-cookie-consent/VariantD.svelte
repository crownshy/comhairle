<script lang="ts">
	// PROTOTYPE - Variant D: "The jar".
	// Direct manipulation instead of toggles. Each thing stored is an object you can
	// pick up. The two essentials are bolted down; the optional one you can tip out.
	import { flip } from 'svelte/animate';
	import { fly, scale } from 'svelte/transition';
	import { Lock } from 'lucide-svelte';
	import { necessary, optional } from './cookieFacts';

	let { onDone }: { onDone: () => void } = $props();

	type Disc = {
		name: string;
		short: string;
		what: string;
		plain: string;
		locked: boolean;
		tint: string;
	};

	const discs: Disc[] = [
		{ ...necessary[0], short: 'Sign-in', locked: true, tint: 'var(--chart-2)' },
		{ ...necessary[1], short: 'Language', locked: true, tint: 'var(--chart-1)' },
		{ ...optional, short: 'Head count', locked: false, tint: 'var(--chart-4)' }
	];

	let out = $state<string[]>([]);
	let inJar = $derived(discs.filter((d) => !out.includes(d.name)));
	let tippedOut = $derived(discs.filter((d) => out.includes(d.name)));
	let selected = $state<string | null>(null);
	let detail = $derived(discs.find((d) => d.name === selected) ?? null);

	function tap(disc: Disc) {
		if (selected === disc.name && !disc.locked) {
			out = out.includes(disc.name)
				? out.filter((n) => n !== disc.name)
				: [...out, disc.name];
			return;
		}
		selected = disc.name;
	}
</script>

<div
	class="bg-background/95 fixed inset-0 z-50 flex flex-col items-center justify-center overflow-y-auto p-6 backdrop-blur-sm"
	role="dialog"
	aria-modal="true"
	aria-label="What this site keeps"
>
	<div class="flex w-full max-w-md flex-col items-center gap-6">
		<h2 class="text-foreground text-center text-3xl leading-tight font-bold">
			Three things we keep. Tip out what you do not want.
		</h2>

		<!-- The jar -->
		<div
			class="border-border bg-card relative flex min-h-44 w-full flex-wrap items-center justify-center gap-4 rounded-t-xl rounded-b-[3rem] border-4 p-6"
		>
			<span class="text-subtle-foreground absolute top-2 left-1/2 -translate-x-1/2 text-base">
				Kept
			</span>
			{#each inJar as disc (disc.name)}
				<button
					type="button"
					animate:flip={{ duration: 320 }}
					in:scale={{ duration: 220, start: 0.6 }}
					class="relative flex size-28 shrink-0 flex-col items-center justify-center rounded-full text-center text-base leading-tight font-semibold text-white shadow-lg transition-transform {selected ===
					disc.name
						? 'ring-ring scale-110 ring-4'
						: ''}"
					style="background: {disc.tint}"
					onclick={() => tap(disc)}
					aria-pressed={selected === disc.name}
				>
					{#if disc.locked}
						<Lock class="mb-1 size-4 opacity-80" />
					{/if}
					<span class="px-2">{disc.short}</span>
				</button>
			{/each}
		</div>

		<!-- Detail for the tapped disc -->
		<div class="min-h-24 w-full">
			{#if detail}
				{#key detail.name}
					<div class="bg-muted rounded-2xl p-4" in:fly={{ y: 8, duration: 180 }}>
						<p class="text-foreground text-base font-semibold">{detail.what}</p>
						<p class="text-subtle-foreground font-mono text-base">{detail.name}</p>
						<p class="text-muted-foreground mt-1 text-base leading-6">{detail.plain}</p>
						<p class="text-subtle-foreground mt-2 text-base">
							{detail.locked
								? 'Bolted down. Removing it would just log you out.'
								: out.includes(detail.name)
									? 'Tap again to put it back.'
									: 'Tap again to tip it out.'}
						</p>
					</div>
				{/key}
			{:else}
				<p class="text-muted-foreground text-center text-base leading-6">
					Tap one to see what it does.
				</p>
			{/if}
		</div>

		<!-- Out tray -->
		{#if tippedOut.length}
			<div
				class="border-border flex w-full flex-wrap items-center justify-center gap-4 rounded-2xl border-2 border-dashed p-4"
				in:fly={{ y: 12, duration: 200 }}
			>
				<span class="text-subtle-foreground w-full text-center text-base">Tipped out</span>
				{#each tippedOut as disc (disc.name)}
					<button
						type="button"
						animate:flip={{ duration: 320 }}
						class="border-border text-muted-foreground flex size-24 shrink-0 items-center justify-center rounded-full border-2 border-dashed px-2 text-center text-base leading-tight"
						onclick={() => tap(disc)}
					>
						{disc.short}
					</button>
				{/each}
			</div>
		{/if}

		<button
			type="button"
			class="bg-primary text-primary-foreground w-full rounded-full px-6 py-4 text-lg font-semibold"
			onclick={onDone}
		>
			{tippedOut.length ? `Keep ${inJar.length}, bin ${tippedOut.length}` : 'Keep all three'}
		</button>
		<a class="text-muted-foreground text-base underline" href="/rights/cookies">Full policy</a>
	</div>
</div>

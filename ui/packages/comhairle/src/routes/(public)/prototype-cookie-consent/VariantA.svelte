<script lang="ts">
	// PROTOTYPE - Variant A: "Nothing to decide".
	// The bet: the friendliest thing you can do with two strictly necessary cookies is
	// not turn them into a decision. No overlay, no page block, no false choice.
	import { fly } from 'svelte/transition';
	import { necessary } from './cookieFacts';
	import { ChevronDown } from 'lucide-svelte';

	let { onDone }: { onDone: () => void } = $props();

	let open = $state(false);
</script>

<div
	class="fixed inset-x-0 bottom-0 z-50 p-3 sm:p-4"
	in:fly={{ y: 40, duration: 320, delay: 400 }}
	out:fly={{ y: 40, duration: 180 }}
>
	<div
		class="bg-card text-card-foreground border-border mx-auto w-full max-w-2xl rounded-2xl border shadow-lg"
	>
		<div class="flex flex-col gap-3 p-4 sm:flex-row sm:items-center sm:gap-4 sm:p-5">
			<p class="text-base leading-6 sm:flex-1">
				Two cookies. One keeps you signed in, one remembers your language. That is the lot,
				and neither one follows you anywhere.
			</p>
			<button
				type="button"
				class="bg-primary text-primary-foreground shrink-0 rounded-full px-6 py-3 text-base font-semibold"
				onclick={onDone}
			>
				Grand
			</button>
		</div>

		<div class="border-border border-t px-4 sm:px-5">
			<button
				type="button"
				class="text-muted-foreground flex w-full items-center justify-between py-3 text-base"
				onclick={() => (open = !open)}
				aria-expanded={open}
			>
				<span>Show me the two</span>
				<ChevronDown
					class="size-5 transition-transform duration-200 {open ? 'rotate-180' : ''}"
				/>
			</button>

			{#if open}
				<div class="pb-4" transition:fly={{ y: -8, duration: 160 }}>
					<dl class="flex flex-col gap-3">
						{#each necessary as fact (fact.name)}
							<div class="bg-muted rounded-xl p-3">
								<dt class="text-foreground font-mono text-base">{fact.name}</dt>
								<dd class="text-muted-foreground mt-1 text-base leading-6">
									{fact.plain}
									<span class="block">Lasts: {fact.lasts.toLowerCase()}</span>
								</dd>
							</div>
						{/each}
					</dl>
					<p class="text-muted-foreground mt-3 text-base leading-6">
						No "reject" button because there is nothing here to reject. Turning these
						off would just log you out.
						<a class="text-primary underline" href="/rights/cookies">Full policy</a>
					</p>
				</div>
			{/if}
		</div>
	</div>
</div>

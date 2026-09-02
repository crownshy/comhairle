<script lang="ts">
	// PROTOTYPE - Variant C: "Ask me anything".
	// Consent as a short thread. The site opens, you reply with a chip, it answers.
	// Reading more is a reply rather than a link out to a policy page.
	import { fly } from 'svelte/transition';
	import { necessary, optional } from './cookieFacts';

	let { onDone }: { onDone: () => void } = $props();

	type Bubble = { from: 'site' | 'you'; text: string };

	let thread = $state<Bubble[]>([
		{ from: 'site', text: 'Quick one before you start.' },
		{
			from: 'site',
			text: 'This site stores two small things on your device, and counts visits without storing anything.'
		}
	]);

	const answers: Record<string, string[]> = {
		'What are the two?': necessary.map(
			(f) => `${f.name}: ${f.plain} Lasts ${f.lasts.toLowerCase()}.`
		),
		'Who sees this?': [
			'Nobody outside the team running this consultation. Nothing is sold, nothing goes to advertisers.'
		],
		'Can I opt out of the counting?': [
			`${optional.plain} You can still switch it off and nothing else changes.`
		]
	};

	let scroller = $state<HTMLElement | null>(null);
	$effect(() => {
		const count = thread.length;
		if (count) scroller?.scrollTo({ top: scroller.scrollHeight, behavior: 'smooth' });
	});

	let asked = $state<string[]>([]);
	let remaining = $derived(Object.keys(answers).filter((q) => !asked.includes(q)));

	function ask(question: string) {
		asked = [...asked, question];
		thread = [...thread, { from: 'you', text: question }];
		for (const reply of answers[question]) {
			thread = [...thread, { from: 'site', text: reply }];
		}
	}
</script>

<div
	class="bg-background/95 fixed inset-0 z-50 flex items-end justify-center backdrop-blur-sm sm:items-center"
	role="dialog"
	aria-modal="true"
	aria-label="About cookies"
>
	<div
		class="bg-card flex max-h-[92dvh] w-full max-w-lg flex-col rounded-t-3xl shadow-2xl sm:rounded-3xl"
		in:fly={{ y: 60, duration: 320 }}
	>
		<div bind:this={scroller} class="flex min-h-0 flex-1 flex-col gap-3 overflow-y-auto p-5">
			{#each thread as bubble, i (i)}
				<div
					class="flex {bubble.from === 'you' ? 'justify-end' : 'justify-start'}"
					in:fly={{ y: 12, duration: 200 }}
				>
					<p
						class="max-w-[85%] rounded-3xl px-4 py-3 text-base leading-6 {bubble.from ===
						'you'
							? 'bg-primary text-primary-foreground rounded-br-md'
							: 'bg-muted text-foreground rounded-bl-md'}"
					>
						{bubble.text}
					</p>
				</div>
			{/each}
		</div>

		<div class="border-border flex flex-col gap-3 border-t p-5">
			{#if remaining.length}
				<div class="flex flex-wrap gap-2">
					{#each remaining as question (question)}
						<button
							type="button"
							class="border-border text-foreground rounded-full border px-4 py-2 text-base"
							onclick={() => ask(question)}
						>
							{question}
						</button>
					{/each}
				</div>
			{/if}

			<div class="flex gap-2">
				<button
					type="button"
					class="border-border text-foreground flex-1 rounded-full border px-4 py-3 text-base font-semibold"
					onclick={onDone}
				>
					Essentials only
				</button>
				<button
					type="button"
					class="bg-primary text-primary-foreground flex-1 rounded-full px-4 py-3 text-base font-semibold"
					onclick={onDone}
				>
					All good
				</button>
			</div>
		</div>
	</div>
</div>

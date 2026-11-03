<script lang="ts">
	import { Check, Copy } from 'lucide-svelte';
	import { copy } from 'svelte-copy';

	type Props = {
		variables: string[];
	};

	let { variables }: Props = $props();

	const COPIED_FOR_MS = 2000;
	let copiedVariable = $state<string | null>(null);
	let resetTimer: ReturnType<typeof setTimeout> | undefined;

	function placeholder(variable: string) {
		return `{{${variable}}}`;
	}

	// INFO toasts are off app-wide, so the chip itself shows that the copy worked.
	function showCopied(variable: string) {
		copiedVariable = variable;
		clearTimeout(resetTimer);
		resetTimer = setTimeout(() => (copiedVariable = null), COPIED_FOR_MS);
	}
</script>

<section
	aria-labelledby="email-variables-heading"
	class="bg-admin-background border-border flex flex-col gap-2 rounded-xl border px-4 py-3"
>
	<h2 id="email-variables-heading" class="text-foreground text-base font-semibold">
		Personalise with variables
	</h2>
	<p class="text-muted-foreground text-sm">
		Copy one and paste it into any field. It is replaced with the real value when the email is
		sent.
	</p>
	<ul class="flex flex-wrap gap-2">
		{#each variables as variable (variable)}
			<li>
				<button
					type="button"
					use:copy={{ text: placeholder(variable), onCopy: () => showCopied(variable) }}
					class="border-primary/40 bg-accent text-accent-foreground hover:bg-primary/10 inline-flex cursor-pointer items-center gap-1.5 rounded-md border border-dashed px-2.5 py-1 font-mono text-sm"
				>
					<span class="sr-only">Copy</span>
					{placeholder(variable)}
					{#if copiedVariable === variable}
						<Check class="size-4" aria-hidden="true" />
						<span class="sr-only">Copied</span>
					{:else}
						<Copy class="size-4" aria-hidden="true" />
					{/if}
				</button>
			</li>
		{/each}
	</ul>
</section>

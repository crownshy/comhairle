<script lang="ts" module>
	export type PreviewMode = 'sample' | 'variables';
</script>

<script lang="ts">
	import { Mail } from 'lucide-svelte';
	import { cn } from '$lib/utils';
	import { Button } from '$lib/components/ui/button';
	import { Skeleton } from '$lib/components/ui/skeleton';
	import InboxPreviewDialog from './InboxPreviewDialog.svelte';

	type Props = {
		subject: string;
		/** The rendered email from the preview endpoint, empty while the first one loads. */
		html: string;
		failed: boolean;
		/** The schema's variables, named in the note about example values. */
		variables: string[];
		/** Variables shown with their real value rather than an example. */
		knownVariables: string[];
		/** Example values from the API, or the placeholders highlighted. */
		mode: PreviewMode;
		onModeChange: (mode: PreviewMode) => void;
	};

	let {
		subject,
		html,
		failed,
		variables,
		knownVariables,
		mode,
		onModeChange
	}: Props = $props();

	let hasExampleValues = $derived(
		variables.some((variable) => !knownVariables.includes(variable))
	);

	const MODES: { value: PreviewMode; label: string }[] = [
		{ value: 'sample', label: 'Sample values' },
		{ value: 'variables', label: 'Variables' }
	];

	let inboxOpen = $state(false);
</script>

<section aria-labelledby="email-preview-heading" class="flex flex-col gap-3">
	<div class="flex flex-wrap items-baseline justify-between gap-2">
		<h2 id="email-preview-heading" class="text-foreground text-base font-semibold">Preview</h2>
		{#if variables.length > 0}
			<div
				role="group"
				aria-label="Show variables as"
				class="border-border inline-flex rounded-full border p-0.5"
			>
				{#each MODES as option (option.value)}
					<button
						type="button"
						aria-pressed={mode === option.value}
						onclick={() => onModeChange(option.value)}
						class={cn(
							'cursor-pointer rounded-full px-3 py-1 text-sm font-medium transition-colors',
							mode === option.value
								? 'bg-primary text-primary-foreground'
								: 'text-foreground hover:bg-muted'
						)}
					>
						{option.label}
					</button>
				{/each}
			</div>
		{/if}
		<Button
			type="button"
			variant="outline"
			size="sm"
			class="text-sm"
			disabled={!html}
			onclick={() => (inboxOpen = true)}
		>
			<Mail class="size-4" aria-hidden="true" />
			See it in an inbox
		</Button>
	</div>
	<div class="border-border bg-muted overflow-hidden rounded-xl border">
		<div class="bg-card border-border flex flex-col gap-0.5 border-b px-4 py-3">
			<span class="text-foreground truncate text-base font-semibold">
				{subject || 'No subject'}
			</span>
			<span class="text-muted-foreground text-sm">To: participant</span>
		</div>
		{#if failed}
			<p role="status" class="text-muted-foreground px-4 py-10 text-base">
				The preview could not be loaded. Your changes are not affected.
			</p>
		{:else if html}
			<!-- Empty sandbox: no scripts, and no access to the admin page. -->
			<iframe title="Email preview" srcdoc={html} sandbox="" class="h-[60vh] w-full bg-white"
			></iframe>
		{:else}
			<Skeleton class="m-4 h-[calc(60vh-2rem)] rounded-lg" />
		{/if}
	</div>
	<p class="text-muted-foreground text-sm">
		Updates as you type.
		{#if mode === 'sample' && hasExampleValues}
			Variables show example values here. Participants see the real ones.
		{:else if mode === 'variables' && variables.length > 0}
			Highlighted variables are filled in with real values when the email is sent.
		{/if}
	</p>
</section>

<InboxPreviewDialog bind:open={inboxOpen} {subject} {html} {variables} />

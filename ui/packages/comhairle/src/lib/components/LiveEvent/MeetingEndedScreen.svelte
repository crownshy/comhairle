<script lang="ts">
	import Button from '$lib/components/ui/button/button.svelte';
	import { CheckCircle } from 'lucide-svelte';

	interface Props {
		title: string;
		conversationUrl?: string;
		isModerator?: boolean;
		onResetCall?: () => void;
	}

	let { title, conversationUrl, isModerator = false, onResetCall }: Props = $props();
</script>

<div class="bg-background flex min-h-dvh w-full items-center justify-center">
	<div class="flex max-w-lg flex-col items-center gap-12 px-6 text-center">
		<!-- Success icon -->
		<div class="bg-primary/10 flex h-20 w-20 items-center justify-center rounded-full">
			<CheckCircle class="text-primary h-10 w-10" />
		</div>

		<!-- Heading -->
		<div class="flex flex-col items-center gap-4">
			<h1 class="text-foreground text-4xl leading-10 font-bold">
				Thank you for participating!
			</h1>
			<p class="text-muted-foreground text-lg leading-7">
				The meeting <span class="text-foreground font-medium">{title}</span> has ended.
			</p>
		</div>

		<!-- Next steps -->
		<div class="bg-card border-border w-full rounded-xl border p-6 text-left">
			<h2 class="text-foreground mb-3 text-base font-semibold">What happens next?</h2>
			<ul class="text-muted-foreground flex flex-col gap-2 text-sm leading-6">
				<li class="flex items-start gap-2">
					<span class="text-primary mt-1 text-sm">1.</span>
					<span>A summary and any transcripts will be available shortly.</span>
				</li>
				<li class="flex items-start gap-2">
					<span class="text-primary mt-1 text-sm">2.</span>
					<span>You may receive follow-up materials from the facilitator.</span>
				</li>
				<li class="flex items-start gap-2">
					<span class="text-primary mt-1 text-sm">3.</span>
					<span>Check the conversation page for updates and next steps.</span>
				</li>
			</ul>
		</div>

		<!-- CTA -->
		<div class="flex flex-col items-center gap-3">
			{#if conversationUrl}
				<Button
					variant="primaryDark"
					class="h-12 px-8 text-base font-semibold"
					onclick={() => (window.location.href = conversationUrl)}
				>
					Back to conversation
				</Button>
			{/if}
			{#if isModerator && onResetCall}
				<Button
					variant="outline"
					class="h-10 px-6 text-sm font-medium"
					onclick={onResetCall}
				>
					Reset call
				</Button>
			{/if}
		</div>
	</div>
</div>

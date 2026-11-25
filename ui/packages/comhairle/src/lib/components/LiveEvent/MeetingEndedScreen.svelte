<script lang="ts">
	import Button from '$lib/components/ui/button/button.svelte';
	import { m } from '$lib/paraglide/messages';
	import { focusOnScreenChange } from '$lib/utils/focusOnScreenChange';
	import CheckCircle from '@lucide/svelte/icons/circle-check-big';

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
			<h1
				tabindex="-1"
				{@attach focusOnScreenChange}
				class="text-foreground text-4xl leading-10 font-bold outline-none"
			>
				{m.live_event_ended_title()}
			</h1>
			<p class="text-muted-foreground text-lg leading-7">
				{m.live_event_ended_body({ title })}
			</p>
		</div>

		<!-- Next steps -->
		<div class="bg-card border-border w-full rounded-xl border p-6 text-left">
			<h2 class="text-foreground mb-3 text-base font-semibold">
				{m.live_event_ended_next_title()}
			</h2>
			<ul class="text-muted-foreground flex flex-col gap-2 text-base leading-6">
				<li class="flex items-start gap-2">
					<span class="text-primary">1.</span>
					<span>{m.live_event_ended_next_summary()}</span>
				</li>
				<li class="flex items-start gap-2">
					<span class="text-primary">2.</span>
					<span>{m.live_event_ended_next_follow_up()}</span>
				</li>
				<li class="flex items-start gap-2">
					<span class="text-primary">3.</span>
					<span>{m.live_event_ended_next_updates()}</span>
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
					{m.events_back_to_conversation()}
				</Button>
			{/if}
			{#if isModerator && onResetCall}
				<Button
					variant="outline"
					class="h-10 px-6 text-sm font-medium"
					onclick={onResetCall}
				>
					{m.live_event_reset_call()}
				</Button>
			{/if}
		</div>
	</div>
</div>

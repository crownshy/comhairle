<!--
	@component Shown before Polis has formed opinion groups: the question, a join QR code,
	live counts and how many more voters are needed.
-->
<script lang="ts">
	import JoinQrCode from './JoinQrCode.svelte';

	type Props = {
		question: string;
		/** Should be an open invite link, so scanning does not hit a sign-up page. */
		joinUrl: string;
		participants: number;
		votes: number;
		/** Countdown copy, e.g. "4 more voters". Omit once nothing is pending. */
		unlockLabel?: string | null;
	};

	let { question, joinUrl, participants, votes, unlockLabel = null }: Props = $props();
</script>

<!-- overflow-hidden: a centred flex column that outgrows its box spills up over the content above. -->
<div
	class="flex min-h-[85vh] flex-col items-center justify-center gap-6 overflow-hidden text-center lg:h-full lg:min-h-0 lg:gap-10"
>
	<div class="flex max-w-4xl flex-col gap-2 lg:gap-4">
		<p class="text-muted-foreground text-base font-medium tracking-wide uppercase lg:text-xl">
			Join the conversation
		</p>
		<h1
			class="text-foreground text-2xl leading-tight font-bold text-balance sm:text-4xl lg:text-5xl"
		>
			{question}
		</h1>
	</div>

	<div class="shrink-0 rounded-2xl bg-white p-3 lg:p-6">
		<JoinQrCode value={joinUrl} resolution={1024} class="size-36 sm:size-48 lg:size-64" />
	</div>

	<div class="flex flex-wrap items-baseline justify-center gap-x-6 gap-y-2 lg:gap-x-10">
		<!-- No "of N" total: anyone with the link can join, so there is no fixed roster. -->
		<p class="text-foreground text-2xl font-bold tabular-nums lg:text-4xl">
			{participants}
			<span class="text-muted-foreground text-base font-medium lg:text-2xl">here</span>
		</p>
		<p class="text-foreground text-2xl font-bold tabular-nums lg:text-4xl">
			{votes}
			<span class="text-muted-foreground text-base font-medium lg:text-2xl">
				{votes === 1 ? 'vote' : 'votes'}
			</span>
		</p>
	</div>

	{#if unlockLabel}
		<p class="text-muted-foreground text-base font-medium lg:text-2xl">
			{unlockLabel} and the room takes shape
		</p>
	{/if}
</div>

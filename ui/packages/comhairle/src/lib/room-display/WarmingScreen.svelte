<!--
	@component What the room sees before Polis has clustered: the question, a QR code,
	and a live count.

	This is not a loading state. For the first stretch of a session the honest state of
	the data is "nothing yet", and the most useful thing the screen can do is recruit.
	It is also what the room stares at for the first half hour, so it gets the whole
	display rather than a spinner in the corner of the real one.

	The countdown is the incentive: the threshold is real (Polis genuinely cannot
	cluster yet), so telling the room how far off it is turns waiting into a shared
	goal rather than dead air.

	Dumb: takes the copy, the join URL and the numbers. It does not know about stages
	or drivers.
-->
<script lang="ts">
	import QrCode from 'svelte-qrcode';

	type Props = {
		/** The Polis question the room is answering. */
		question: string;
		/** Where the QR code points. An open invite, so scanning hits no signup wall. */
		joinUrl: string;
		participants: number;
		votes: number;
		/** Countdown copy, e.g. "4 more voters". Omit once nothing is pending. */
		unlockLabel?: string | null;
	};

	let { question, joinUrl, participants, votes, unlockLabel = null }: Props = $props();
</script>

<!--
	Clipped, not overflowing: a centred flex column that outgrows its box spills in both
	directions and prints over whatever sits above it. Sizes step up with the viewport so
	the same screen works on a preview pane and a projector.
-->
<div
	class="flex h-full min-h-0 flex-col items-center justify-center gap-6 overflow-hidden text-center lg:gap-10"
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

	<!--
		Sized for a room, not a desk. The QR is the one thing on this screen someone has
		to act on from eight metres away, so it gets the most pixels.
	-->
	<div class="shrink-0 rounded-2xl bg-white p-3 lg:p-6">
		<QrCode
			value={joinUrl}
			size="1024"
			padding={null}
			errorCorrection="M"
			className="size-36 sm:size-48 lg:size-64"
		/>
	</div>

	<div class="flex flex-wrap items-baseline justify-center gap-x-6 gap-y-2 lg:gap-x-10">
		<!--
			No expected total: a denominator implies a fixed roster, which stops being true
			the moment the QR code leaves the room.
		-->
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

<script lang="ts">
	import * as Card from '$lib/components/ui/card';

	type Props = {
		joined: number;
		target: number;
		/** Where an even pace from launch to closing would put you today. */
		evenPaceToday: number;
		closingDate: Date;
		daysLeft: number;
		projectedAtClose: number;
		editTargetHref: string;
		/** AI mode only: the written pace and forecast under the bar. */
		showForecast: boolean;
	};

	let {
		joined,
		target,
		evenPaceToday,
		closingDate,
		daysLeft,
		projectedAtClose,
		editTargetHref,
		showForecast
	}: Props = $props();

	let joinedPercent = $derived(Math.min(100, Math.round((joined / target) * 100)));
	let pacePercent = $derived(Math.min(100, Math.round((evenPaceToday / target) * 100)));
	let shortfall = $derived(Math.max(0, target - projectedAtClose));
	let closingLabel = $derived(
		closingDate.toLocaleDateString('en-GB', { day: 'numeric', month: 'short' })
	);
</script>

<Card.Root>
	<Card.Content class="flex flex-col gap-4">
		<div class="flex flex-wrap items-baseline justify-between gap-3">
			<p class="flex flex-wrap items-baseline gap-2">
				<span class="text-4xl font-bold">{joined}</span>
				<span class="text-muted-foreground text-base">
					of {target} participants · closes {closingLabel} ({daysLeft} days left)
				</span>
			</p>
			<a href={editTargetHref} class="text-primary text-base font-semibold hover:underline"
				>Edit target</a
			>
		</div>
		<div
			class="bg-muted relative h-3.5 rounded-full"
			role="img"
			aria-label="{joined} joined of a target of {target}. On an even pace you would have {evenPaceToday} by today."
		>
			<div
				class="bg-primary absolute inset-y-0 left-0 rounded-full"
				style:width="{joinedPercent}%"
			></div>
			<div
				class="bg-foreground absolute -top-1.5 h-6 w-0.5"
				style:left="{pacePercent}%"
			></div>
		</div>
		<div class="text-muted-foreground flex flex-wrap justify-between gap-3 text-sm">
			<span>Joined so far</span>
			<span>Line: where you'd be on an even pace ({evenPaceToday})</span>
		</div>
		{#if showForecast}
			<p class="text-base leading-relaxed">
				Between recruitment pushes you get about 5 new people a day.
				{#if shortfall > 0}
					<strong
						>Without another push you'll close with about {projectedAtClose}, {shortfall}
						short.</strong
					>
				{:else}
					<strong>At this pace you'll reach your target before closing.</strong>
				{/if}
			</p>
		{/if}
	</Card.Content>
</Card.Root>

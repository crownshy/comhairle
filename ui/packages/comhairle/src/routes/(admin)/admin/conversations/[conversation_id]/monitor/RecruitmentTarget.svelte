<script lang="ts">
	import * as Card from '$lib/components/ui/card';
	import EditTargetDialog from './EditTargetDialog.svelte';

	type Props = {
		joined: number;
		/** People who finished every step. */
		completed: number;
		target: number;
		/** Where an even pace from launch to closing would put you today. */
		evenPaceToday: number;
		closingDate: Date;
		daysLeft: number;
		onTargetChange: (target: number) => void;
	};

	let { joined, completed, target, evenPaceToday, closingDate, daysLeft, onTargetChange }: Props =
		$props();

	function percentOfTarget(value: number): number {
		return Math.min(100, Math.round((value / target) * 100));
	}

	let joinedPercent = $derived(percentOfTarget(joined));
	let completedPercent = $derived(percentOfTarget(completed));
	let pacePercent = $derived(percentOfTarget(evenPaceToday));
	let closingLabel = $derived(
		closingDate.toLocaleDateString('en-GB', { day: 'numeric', month: 'short' })
	);
</script>

<!-- Joined and completed against the same target, so the gap between them is visible. -->
<Card.Root>
	<Card.Header class="flex flex-wrap items-center justify-between gap-3">
		<div class="flex flex-col gap-1">
			<Card.Title>Recruitment target</Card.Title>
			<Card.Description>
				{target} participants · closes {closingLabel} ({daysLeft} days left)
			</Card.Description>
		</div>
		<EditTargetDialog {target} {joined} onSave={onTargetChange} />
	</Card.Header>
	<Card.Content class="flex flex-col gap-4">
		<div class="flex flex-wrap gap-x-10 gap-y-3">
			<p class="flex items-baseline gap-2">
				<span class="text-4xl font-bold">{completed}</span>
				<span class="text-muted-foreground text-base"
					>completed ({completedPercent}% of target)</span
				>
			</p>
			<p class="flex items-baseline gap-2">
				<span class="text-4xl font-bold">{joined}</span>
				<span class="text-muted-foreground text-base"
					>joined ({joinedPercent}% of target)</span
				>
			</p>
		</div>
		<div
			class="bg-muted relative h-3.5 rounded-full"
			role="img"
			aria-label="{joined} joined and {completed} completed, of a target of {target}. On an even pace you would have {evenPaceToday} joined by today."
		>
			<div
				class="absolute inset-y-0 left-0 z-10 rounded-full bg-green-600"
				style:width="{completedPercent}%"
			></div>
			<div
				class="bg-primary absolute inset-y-0 left-0 rounded-full"
				style:width="{joinedPercent}%"
			></div>

			<div
				class="bg-foreground absolute -top-1.5 z-20 h-6 w-0.5"
				style:left="{pacePercent}%"
			></div>
		</div>
		<ul
			class="text-muted-foreground flex flex-wrap gap-x-5 gap-y-1 text-sm"
			aria-label="Legend"
		>
			<li class="inline-flex items-center gap-2">
				<span class="size-3 rounded-sm bg-green-600"></span>Completed
			</li>
			<li class="inline-flex items-center gap-2">
				<span class="bg-primary size-3 rounded-sm"></span>Joined
			</li>
			<li class="inline-flex items-center gap-2">
				<span class="bg-foreground h-3.5 w-0.5"></span>Where you'd be on an even pace ({evenPaceToday})
			</li>
		</ul>
	</Card.Content>
</Card.Root>

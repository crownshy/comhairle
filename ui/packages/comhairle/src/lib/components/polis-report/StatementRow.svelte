<script lang="ts">
	import type { ReportComment, ReportGroup } from '$lib/tools/polis/reportTypes';
	import { computeGroupVotePercents, totalVotes } from '$lib/tools/polis/report';
	import { User } from '@lucide/svelte';
	import GroupCircle from './GroupCircle.svelte';
	import RowAccentStripe from './RowAccentStripe.svelte';
	import ThemePicker from './ThemePicker.svelte';

	type Variant = 'consensus' | 'difference' | 'uncertainty' | 'neutral';

	interface Props {
		/** Accepted for call-site convenience; the "#" cell shows the Polis tid. */
		index?: number;
		comment: ReportComment;
		groups: ReportGroup[];
		variant?: Variant;
		/** Drop passes from the agree% denominator (agrees/(agrees+disagrees)). */
		excludePasses?: boolean;
		picker?: {
			availableThemes: string[];
			onAddTheme: (theme: string) => void | Promise<void>;
			onRemoveTheme: (theme: string) => void | Promise<void>;
			disabled?: boolean;
		};
	}

	let { comment, groups, variant = 'neutral', excludePasses = false, picker }: Props = $props();

	const groupPcts = $derived(computeGroupVotePercents(comment, groups, { excludePasses }));
	const count = $derived(totalVotes(comment));

	// Per-variant metric shown in its own column (matches the Figma tables):
	//   consensus  → lowest group agree% (MIN AGREE)
	//   difference → max−min group agree% spread (DIFFERENCE, in pp)
	//   otherwise  → total votes (COUNT)
	const agreedPcts = $derived(groupPcts.map((g) => g.agreed));
	const minAgree = $derived(agreedPcts.length ? Math.min(...agreedPcts) : 0);
	const spread = $derived(
		agreedPcts.length ? Math.max(...agreedPcts) - Math.min(...agreedPcts) : 0
	);

	const stripeClass = $derived(
		variant === 'consensus'
			? 'bg-primary'
			: variant === 'difference'
				? 'bg-destructive'
				: 'bg-transparent'
	);

	// Seed/host-authored statements (is_seed) label as "You" (the host viewing
	// this admin report authored them). Everyone else is "Participant" — the
	// report payload has no author pid yet, so we can't show the real participant
	// id. Seed author is assumed to be pid 0.
	const isHostAuthored = $derived(!!comment.is_seed);
</script>

<!-- col-span-full + grid-cols-subgrid: the row spans every column of the owning
     grid in StatementSection and adopts its exact tracks, so cells line up with the
     header without this component knowing the column widths. -->
<div
	class="border-border group hover:bg-muted/40 relative col-span-full grid grid-cols-subgrid items-start border-b py-6 pr-4 pl-5 transition-colors duration-150"
>
	<RowAccentStripe accent={stripeClass} />

	<!-- Polis statement id -->
	<div class="text-muted-foreground pt-1 text-center text-sm tabular-nums">
		{comment.tid}
	</div>

	<!-- Statement text + theme tags -->
	<div class="min-w-0">
		<p class="text-foreground text-lg leading-6 font-medium">{comment.text}</p>
		<div class="mt-3">
			{#if picker}
				<ThemePicker
					themes={comment.topics ?? []}
					availableThemes={picker.availableThemes}
					disabled={picker.disabled}
					onAddTheme={picker.onAddTheme}
					onRemoveTheme={picker.onRemoveTheme}
				/>
			{:else}
				<div class="flex flex-wrap items-center gap-1.5">
					{#each comment.topics ?? [] as topic (topic)}
						<span
							class="bg-muted text-foreground/80 inline-flex items-center rounded px-2 py-0.5 text-sm font-medium"
						>
							{topic}
						</span>
					{/each}
				</div>
			{/if}
		</div>
	</div>

	<!-- Author -->
	<div class="pt-1 text-right">
		{#if isHostAuthored}
			<span
				class="bg-primary text-primary-foreground inline-flex items-center gap-1 rounded px-2 py-0.5 text-sm font-medium"
			>
				<User class="size-3.5" />You
			</span>
		{:else}
			<span
				class="bg-muted text-muted-foreground inline-flex items-center gap-1 rounded px-2 py-0.5 text-sm font-medium"
			>
				<User class="size-3.5" />Participant
			</span>
		{/if}
	</div>

	<!-- Per-variant metric -->
	<div class="pt-1 text-center font-bold">
		{#if variant === 'consensus'}
			<span class="text-primary">{Math.round(minAgree)}%</span>
		{:else if variant === 'difference'}
			<span class="text-destructive">{Math.round(spread)}pp</span>
		{:else}
			<span class="text-foreground">{count}</span>
		{/if}
	</div>

	<!-- Per-group agree rings. Arc reflects the toggled agree% (g.agreed); the
	     tooltip uses raw group_votes counts for the honest full breakdown. -->
	<div class="flex items-center gap-3 self-start pt-0.5">
		{#each groupPcts as g (g.group_id)}
			{@const gv = comment.group_votes.find((v) => v.group_id === g.group_id)}
			<GroupCircle
				label={g.label}
				agreePct={g.agreed}
				agrees={gv?.agrees ?? 0}
				disagrees={gv?.disagrees ?? 0}
				passes={gv?.passes ?? 0}
				showLabel={false}
			/>
		{/each}
	</div>
</div>

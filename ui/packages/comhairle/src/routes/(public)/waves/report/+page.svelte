<script lang="ts">
	import type { PageProps } from './$types';
	import type { ReportComment } from '$lib/types/report';
	import { Badge } from '$lib/components/ui/badge';
	import StatementCard from '$lib/components/report/StatementCard.svelte';
	import BeeswarmChart from '$lib/components/report/BeeswarmChart.svelte';
	import ReportSection from '$lib/components/report/ReportSection.svelte';
	import AiCallout from '$lib/components/report/AiCallout.svelte';
	import ReportNav from '$lib/components/report/ReportNav.svelte';
	import Down from '@lucide/svelte/icons/download';
	import {
		getEngagementStats,
		getConsensusStatements,
		getDivisiveStatements,
		getSignificantComments
	} from '$lib/types/report';
	import { Button } from '$lib/components/ui/button';

	let { data }: PageProps = $props();

	const stats = $derived(getEngagementStats(data.reportData));
	const consensusStatements = $derived(getConsensusStatements(data.reportData, 5));
	const divisiveStatements = $derived(getDivisiveStatements(data.reportData, 5));
	const significantComments = $derived(getSignificantComments(data.reportData));
	const totalParticipants = $derived(data.reportData.participants.length);

	let selectedTid = $state<number | null>(null);

	const activeComment = $derived.by(() => {
		const tid = selectedTid ?? null;
		if (tid == null) return null;
		return significantComments.find((c) => c.tid === tid) ?? null;
	});

	function handleHoverComment(comment: ReportComment | null) {
		if (comment) selectedTid = comment.tid;
	}

	let engagementOpen = $state(true);
	let whoParticipatingOpen = $state(true);
	let agreementOpen = $state(true);
	let differencesOpen = $state(true);
	let groupsOpen = $state(true);
	let deeperLookOpen = $state(true);
</script>

{#snippet numberedStatements(statements: ReportComment[])}
	<div class="flex flex-col gap-6">
		{#each statements as comment, i (comment.tid)}
			<div class="flex items-start gap-6">
				<span class="text-xl font-semibold text-gray-400/50">#{i + 1}</span>
				<div class="flex-1">
					<StatementCard {comment} groups={data.reportData.groups} {totalParticipants} />
				</div>
			</div>
		{/each}
	</div>
{/snippet}

{#snippet statCard(label: string, value: string | number)}
	<div
		class="bg-card ring-border h-44 w-72 content-center items-center rounded-xl p-6 text-center shadow-[0px_2px_4px_0px_rgba(0,0,0,0.10)] ring-1"
	>
		<p class="text-card-foreground text-3xl leading-9 font-semibold">{value}</p>
		<p class="text-muted-foreground mt-1.5 text-lg leading-7 font-semibold">{label}</p>
	</div>
{/snippet}

<ReportNav />
<div class="flex flex-col items-center overflow-hidden py-20">
	<!-- Header -->
	<header
		id="dive-in"
		class="bg-card mb-16 flex w-full max-w-[1200px] flex-col items-center gap-4 rounded-xl px-24 py-12 text-center"
	>
		<Badge
			class="bg-primary/10 text-muted-foreground rounded-3xl px-4 py-2 text-lg font-semibold"
		>
			Interim Report
		</Badge>
		<h1 class="text-foreground text-5xl leading-[52px] font-bold">
			South Staffordshire Local Plan Public Engagement
		</h1>
		<p class="text-muted-foreground text-sm">
			After 1 week of engagement (Draft report created on 14th April '26)
		</p>

		<div
			class="text-card-foreground mt-6 w-full max-w-4xl text-left text-3xl leading-8 font-semibold"
		>
			How do people get involved?
		</div>
		<div class="w-full max-w-4xl text-left">
			<p class="report-body">
				Participants (anyone who lives in, works in or visits the district) can participate
				online <a
					href="https://waves.comhairle.scot/conversations/shaping-south-staffordshire/invite/0e6fbb72-49c5-42b1-bcf1-79f56d58fc62"
					class="text-primary hover:underline"
					rel="noopener noreferrer"
					target="_blank">here</a
				>.
			</p>
			<ol class="report-body mt-2 list-inside list-decimal pl-1">
				<li>They can learn about the topic.</li>
				<li>
					Then take part in an interactive discussion which asked the question
					<em
						>"Imagine your ideal South Staffordshire in 2045. What do you hope to see in
						the future?"</em
					>
					<br />
					Participants voted on others' statements and submitted their own views for others
					to vote on. This report shares the results so far from that discussion.
				</li>
				<li>
					Participants also have the option to register for a prize draw of &pound;1000
					and register interest to be part of a citizens panel on the local plan. By
					filling in a survey they shared demographic information that informs the "Who is
					participating?" section of this report.
				</li>
			</ol>
		</div>
	</header>

	<!-- Engagement so far, at a glance -->
	<ReportSection
		id="engagement"
		title="Engagement so far, at a glance"
		bind:open={engagementOpen}
	>
		<div class="-mx-24 flex flex-wrap items-center justify-center gap-10 px-4">
			{@render statCard(
				'Participants have taken part so far',
				stats.totalParticipants.toLocaleString()
			)}
			{@render statCard(
				'Of those were identified as forming opinion clusters',
				data.reportData.participants
					.filter((p) => p.group_id != null)
					.length.toLocaleString()
			)}
			{@render statCard('Votes have been cast', stats.totalVotes.toLocaleString())}
			{@render statCard(
				'Statements have been submitted',
				stats.totalStatements.toLocaleString()
			)}
			{@render statCard(
				'Approximate votes per participant',
				'~' + Math.round(stats.totalVotes / Math.max(stats.totalParticipants, 1))
			)}
		</div>
	</ReportSection>

	<div class="mt-10"></div>

	<!-- Who is participating? -->
	<ReportSection
		id="who-participating"
		title="Who is participating?"
		bind:open={whoParticipatingOpen}
	>
		<div class="max-w-4xl pt-2 pb-6">
			<p class="report-body">
				So far demographic information was voluntarily provided by 178 of {stats.totalParticipants.toLocaleString()}
				participants. This represents half of the total consultation participants and so the following
				figures should be treated as a rough indication.
			</p>
		</div>

		<div class="max-w-4xl space-y-4">
			<p class="report-body">
				<strong>Age:</strong> The participant base skews significantly older, with 78% aged 51+
				(46% are 65+, 32% are 51–64). Only 7% are under 35 years old.
			</p>
			<p class="report-body">
				<strong>Location:</strong> The vast majority (87%) live within the South Staffordshire
				Council area, with strong representation from Codsall (16%), Wombourne (14%), Brewood
				&amp; Coven (8%), and Penkridge (8%).
			</p>
			<p class="report-body">
				<strong>Housing:</strong> Most participants are homeowners (87% combined), with 70% owning
				outright and 17% with a mortgage. Only 11% rent (7% social housing, 4% private).
			</p>
			<p class="report-body">
				<strong>Ethnicity:</strong> The sample is predominantly White British (92%), with smaller
				representation from Asian (2%), White Other (2%), and other ethnic groups (3%).
			</p>
			<p class="report-body">
				<strong>Gender:</strong> Slightly more men (57%) than women (41%), with 2% preferring
				not to say.
			</p>
			<p class="report-body">
				<strong>Civic Engagement:</strong> Participants are relatively engaged citizens — 88%
				have voted, 70% have signed petitions, 57% have contacted officials, and 39% have previously
				engaged in council consultations.
			</p>
		</div>
	</ReportSection>

	<div class="mt-10"></div>

	<!-- Areas of Agreement -->
	<ReportSection id="agreement" title="Areas of agreement" bind:open={agreementOpen}>
		<div class="max-w-4xl pt-2 pb-6">
			<p class="report-body">
				So far, there is a great deal of agreement across the conversation, particularly on
				infrastructure, environmental protection, and community character.
			</p>
		</div>

		<p class="report-subheading">There is strong overall support for:</p>

		<AiCallout class="w-full max-w-4xl">
			<ul class="report-body list-disc space-y-2 pl-5">
				<li>
					<strong>Rural character protection:</strong> South Staffordshire's rural character
					is exactly what makes it desirable, and it must be protected.
				</li>
				<li>
					<strong>Brownfield first:</strong> New housing developments should be built on brownfield
					sites first. Green belt sites should be a last resort, with local consent after full
					consultation.
				</li>
				<li>
					<strong>Infrastructure requirements:</strong> New developments should only be allowed
					with suitable infrastructure including schools, shops, public transport, and parking.
				</li>
				<li>
					<strong>Community input:</strong> The views and feedback from a community nearby any
					development should be seen to be taken into account.
				</li>
				<li>
					<strong>Green spaces:</strong> A key priority is making sure everyone in the community
					has access to nature and green spaces.
				</li>
				<li>
					<strong>Flood zone protection:</strong> New developments should not be built on flood
					zones. Planning permission should only be given with this enhanced in law.
				</li>
				<li>
					<strong>Road conditions:</strong> The state of the roads needs to be addressed. Potholes
					are causing accidents and damage.
				</li>
			</ul>
		</AiCallout>

		<p class="report-body">
			These areas represent shared priorities across much of the conversation, even where
			participants differ on more contentious issues.
		</p>

		<p class="report-subheading">Examples of statements where there is agreement</p>

		<p class="report-body">
			Showing top {consensusStatements.length} consensus statements:
		</p>

		{@render numberedStatements(consensusStatements)}
	</ReportSection>

	<div class="mt-10"></div>

	<!-- Where there are differences -->
	<ReportSection title="Where there are differences" bind:open={differencesOpen}>
		<div class="max-w-4xl pt-2 pb-6">
			<p class="report-body">
				The interactive discussion element (called Pol.is) on Comhairle looks at patterns in
				how people vote on statements and groups similar responses together. This is done
				anonymously, no one is identified.
			</p>
			<p class="report-body mt-2">
				This helps make sense of a large conversation, so we can see the range of opinions
				and find areas of agreement.
			</p>
			<p class="report-body mt-2">
				So far, after 2 of the 4 weeks of this engagement the platform has identified
				{data.reportData.groups.length} distinctive opinion clusters.
			</p>
		</div>

		<AiCallout class="w-full max-w-4xl">
			<p class="report-body mb-3">
				The main differences emerge around the pace and location of development, green belt
				protection, and the relationship between housing growth and infrastructure.
			</p>
			<ul class="report-body list-disc space-y-2 pl-5">
				<li>
					<strong>Green belt designation:</strong> Group A expresses strong concern about any
					re-designation of green belt becoming "grey belt," while Group B is more open to reassessing
					green belt in the context of the housing crisis.
				</li>
				<li>
					<strong>Location of development:</strong> Group A strongly supports keeping new developments
					around large towns and leaving villages as villages. Group B is more divided on this
					question, with some supporting development if linked to specific improvements.
				</li>
				<li>
					<strong>No new housing stance:</strong> Group A shows stronger support for moratoriums
					on new housing until infrastructure improves. Group B is more willing to accept development
					alongside improvements.
				</li>
				<li>
					<strong>Scale and distribution:</strong> There is division between those favouring
					smaller, sensitive developments on brownfield land only, and those supporting broader
					growth with infrastructure investment.
				</li>
				<li>
					<strong>Perception of urgency:</strong> Participants also differ in how urgently they
					see the housing shortage versus environmental protection, which shapes their support
					for different measures.
				</li>
			</ul>
			<p class="report-body mt-3">
				Overall, the divide is less about whether development is needed, and more about
				where, how much, and under what conditions change should happen.
			</p>
		</AiCallout>

		{@render numberedStatements(divisiveStatements)}
	</ReportSection>

	<div class="mt-10"></div>

	<!-- Emerging opinion groups -->
	<ReportSection id="groups" title="Emerging opinion groups after 2 weeks" bind:open={groupsOpen}>
		{#each data.reportData.groups as group (group.group_id)}
			{@const label = String.fromCharCode(65 + group.group_id)}
			{@const repComments = group.representative_comments
				.map((rc) => data.reportData.comments.find((c) => c.tid === rc.tid))
				.filter((c) => c != null)}

			<h3 class="text-foreground text-3xl leading-9 font-semibold">
				Group {label} ({group.members.length} participants)
				{#if group.group_id === 0}
					<span class="text-muted-foreground text-xl font-normal">
						"Supportive of growth where it improves local infrastructure"
					</span>
				{:else if group.group_id === 1}
					<span class="text-muted-foreground text-xl font-normal">
						"Protection-focused, cautious on development"
					</span>
				{/if}
			</h3>

			{#if group.group_id === 0}
				<AiCallout class="w-full max-w-[960px]">
					<p class="report-body mb-2">This group:</p>
					<ul class="report-body list-disc space-y-1 pl-5">
						<li>
							Is more willing to accept new development if tied to infrastructure
							improvements.
						</li>
						<li>Is open to reassessing green belt in the context of housing needs.</li>
						<li>
							Favours development that brings funding for schools, health services,
							and transport.
						</li>
					</ul>
				</AiCallout>
			{:else if group.group_id === 1}
				<AiCallout class="w-full max-w-[960px]">
					<p class="report-body mb-2">This group expresses:</p>
					<ul class="report-body list-disc space-y-1 pl-5">
						<li>
							Strong concern about green belt erosion and "grey belt" re-designation.
						</li>
						<li>
							Preference for keeping villages as villages, not expanding into the
							countryside.
						</li>
						<li>
							Support for moratoriums on new housing until infrastructure is improved.
						</li>
						<li>
							Higher concern about traffic, road conditions, and village character
							loss.
						</li>
					</ul>
				</AiCallout>
			{:else}
				<AiCallout class="w-full max-w-[960px]">
					<p class="report-body">Group description will be generated by LLM analysis.</p>
				</AiCallout>
			{/if}

			<p class="report-body">Key statements this group agrees with:</p>

			{#each repComments as comment (comment.tid)}
				<StatementCard {comment} groups={data.reportData.groups} {totalParticipants} />
			{/each}
		{/each}
	</ReportSection>

	<div class="mt-10"></div>

	<!-- Taking a deeper look — Beeswarm -->
	<ReportSection
		title="Taking a deeper look at the results"
		bind:open={deeperLookOpen}
		id="deep-dive"
	>
		<div class="max-w-4xl pt-2 pb-6">
			<p class="report-body">
				Hover over or click on a dot to see the statement. Dots on the left represent
				consensus statements with higher bridging strength; dots on the right represent
				divisive statements with lower bridging strength.
			</p>
		</div>

		<div class="overflow-x-auto">
			<BeeswarmChart
				comments={significantComments}
				height={240}
				dotRadius={5}
				{selectedTid}
				onHoverComment={handleHoverComment}
			/>
		</div>

		{#if activeComment}
			<StatementCard
				comment={activeComment}
				groups={data.reportData.groups}
				{totalParticipants}
			/>
		{/if}
	</ReportSection>
</div>

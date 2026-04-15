<script lang="ts">
	import type { PageProps } from './$types';
	import type { ReportComment } from '$lib/types/report';
	import { Badge } from '$lib/components/ui/badge';
	import StatementCard from '$lib/components/report/StatementCard.svelte';
	import BeeswarmChart from '$lib/components/report/BeeswarmChart.svelte';
	import ReportSection from '$lib/components/report/ReportSection.svelte';
	import AiCallout from '$lib/components/report/AiCallout.svelte';
	import ReportNav from '$lib/components/report/ReportNav.svelte';
	import {
		getEngagementStats,
		getConsensusStatements,
		getDivisiveStatements,
		getSignificantComments
	} from '$lib/utils/report';

	let { data }: PageProps = $props();

	const stats = $derived(getEngagementStats(data.reportData));
	const consensusStatements = $derived(getConsensusStatements(data.reportData, 5));
	const divisiveStatements = $derived(getDivisiveStatements(data.reportData, 5));
	const significantComments = $derived(getSignificantComments(data.reportData));
	const totalParticipants = $derived(data.reportData.participants.length);

	let selectedTid = $state<number | null>(
		significantComments.length > 0
			? significantComments[Math.floor(Math.random() * significantComments.length)].tid
			: null
	);

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
			<div class="flex items-start gap-3 md:gap-6">
				<span class="text-base font-semibold text-gray-400/50 md:text-xl">#{i + 1}</span>
				<div class="flex-1">
					<StatementCard {comment} groups={data.reportData.groups} {totalParticipants} />
				</div>
			</div>
		{/each}
	</div>
{/snippet}

{#snippet statCard(label: string, value: string | number)}
	<div
		class="bg-card ring-border content-center items-center rounded-xl p-5 text-center shadow-[0px_2px_4px_0px_rgba(0,0,0,0.10)] ring-1 md:h-44 md:p-6"
	>
		<p class="text-card-foreground text-2xl leading-8 font-semibold md:text-3xl md:leading-9">
			{value}
		</p>
		<p
			class="text-muted-foreground mt-1 text-base leading-6 font-semibold md:mt-1.5 md:text-lg md:leading-7"
		>
			{label}
		</p>
	</div>
{/snippet}

<ReportNav
	sections={[
		{ id: 'engagement', label: 'Dive In' },
		{ id: 'agreement', label: 'Areas of agreement' },
		{ id: 'groups', label: 'Emerging opinion groups' },
		{ id: 'deep-dive', label: 'Deep Dive' }
	]}
/>
<div class="flex flex-col items-center overflow-hidden pb-20">
	<!-- Header -->
	<header
		id="dive-in"
		class="bg-card mb-8 flex w-full flex-col items-center gap-4 rounded-xl px-5 pt-10 pb-8 text-center md:mb-16 md:px-24 md:pt-24 md:pb-12"
	>
		<Badge
			class="bg-primary/10 text-muted-foreground rounded-3xl px-4 py-2 text-lg font-semibold"
		>
			Interim Report
		</Badge>
		<h1
			class="text-foreground max-w-4xl text-2xl leading-8 font-bold md:text-5xl md:leading-[52px]"
		>
			South Staffordshire Local Plan Public Engagement
		</h1>
		<p class="max-w-4xl">After 1 week of engagement (Draft report created on 14th April '26)</p>

		<!-- Executive Summary -->
		<div class="mt-6 flex w-full max-w-4xl flex-col gap-4 text-left md:mt-10 md:gap-6">
			<h2
				class="text-card-foreground text-2xl leading-8 font-semibold md:text-4xl md:leading-[48px]"
			>
				Executive Summary
			</h2>

			<div class="">
				<p class="text-card-foreground text-lg leading-6">
					This Interim report (at the half way stage before it closes on 5th May) shares
					what people in South Staffordshire want for their area by 2045. So far, based on
					thousands of votes from hundreds of residents, we can see what is emerging in
					terms of agreement and where opinions differ.
				</p>
			</div>

			<div class="pt-2">
				<p class="text-card-foreground text-lg leading-6">
					<strong>What everyone agrees on:</strong> From the residents who have taken part so
					far, they want to protect the countryside and the unique character of our villages.
					There is strong support for building new homes on old industrial sites (brownfield)
					first, rather than on greenbelt land. Everyone also agrees that new housing must come
					with better roads, schools, and green spaces.
				</p>
			</div>

			<div class="pt-2">
				<p class="text-card-foreground text-lg leading-6">
					<strong>Where opinions differ:</strong> While everyone wants a better future, people
					are divided on how to get there. The engagement platform identifies two main opinion
					groups. One group is very cautious, concerned that new development will ruin the countryside
					and overwhelm our roads. Another group is more open to new homes, but only if it brings
					clear improvements to local services like hospitals and transport.
				</p>
			</div>

			<div class="pt-2">
				<p class="text-card-foreground text-lg leading-6">
					<strong>What happens next:</strong> This report helps us understand these different
					views. When this part of the engagement is finished in early May it will guide the
					next steps. A residents panel will come together and look at evidence including the
					final report of this engagement. They will discuss how to create a Local Plan that
					provides the homes we need while keeping South Staffordshire the place people love
					to live.
				</p>
			</div>

			<div class="pt-2">
				<p class="text-card-foreground text-lg leading-6">
					The analysis and summaries of this report are generated by secure, privacy-first
					Lumo AI. All data is processed with full GDPR compliance and zero-access
					encryption.
				</p>
			</div>

			<h3 class="text-primary text-xl leading-6 font-semibold">
				How do people get involved?
			</h3>

			<div class="pt-2">
				<p class="text-card-foreground text-lg leading-6">
					Participants (anyone who lives in, works in or visits the district) can
					participate online <a
						href="https://waves.comhairle.scot/conversations/shaping-south-staffordshire/invite/0e6fbb72-49c5-42b1-bcf1-79f56d58fc62"
						class="text-primary hover:underline"
						rel="noopener noreferrer"
						target="_blank">here</a
					>.
				</p>
				<ol
					class="mt-4 flex list-inside list-decimal flex-col gap-3 pl-1 text-lg leading-6"
				>
					<li>They can learn about the topic.</li>
					<li>
						Then take part in an interactive discussion which asked the question
						"Imagine your ideal South Staffordshire in 2045. What do you hope to see in
						the future?" Participants voted on others' statements and submitted their
						own views for others to vote on. This report shares the results so far from
						that discussion.
					</li>
					<li>
						Participants also have the option to register for a prize draw of £1000 and
						register interest to be part of a citizens panel on the local plan. By
						filling in a survey they shared demographic information that informs the
						"Who is participating?" section of this report.
					</li>
				</ol>
			</div>
		</div>
	</header>

	<!-- Engagement so far, at a glance -->
	<ReportSection
		id="engagement"
		title="Engagement so far, at a glance"
		bind:open={engagementOpen}
	>
		<div class="grid grid-cols-1 gap-4 sm:grid-cols-2 md:gap-8 lg:grid-cols-3">
			<!-- Row 1 -->
			{@render statCard(
				'Participants have taken part',
				stats.totalParticipants.toLocaleString()
			)}
			{@render statCard('Votes cast', stats.totalVotes.toLocaleString())}
			{@render statCard('Statements submitted', stats.totalStatements.toLocaleString())}

			<!-- Row 2 -->
			{@render statCard(
				'Votes per participants on average',
				Math.round((stats.totalVotes / Math.max(stats.totalParticipants, 1)) * 100) / 100
			)}
			{@render statCard('Opinion groups were identified', data.reportData.groups.length)}
		</div>
	</ReportSection>

	<div class="mt-10"></div>

	<!-- Who is participating? -->
	<ReportSection
		id="who-participating"
		title="Who is participating?"
		bind:open={whoParticipatingOpen}
	>
		<div class="text-card-foreground text-lg leading-6">
			<span class="italic">
				So far demographic information was voluntarily provided by
				<span class="font-bold"
					>178 of {stats.totalParticipants.toLocaleString()} participants.</span
				>
				This represents half of the total consultation participants and so the following figures
				should be treated as a rough indication.
			</span>
			<br /><br />
			<strong>Age:</strong> The participant base skews significantly older, with
			<strong>78% aged 51+</strong> (46% are 65+, 32% are 51-64). Only 7% are under 35 years
			old.
			<br /><br />
			<strong>Location:</strong> The vast majority (87%) live within the South Staffordshire
			Council area, with strong representation from Codsall (16%), Wombourne (14%), Brewood &
			Coven (8%), and Penkridge (8%).
			<br /><br />
			<strong>Housing:</strong> Most participants are
			<strong>homeowners (87% combined)</strong>, with 70% owning outright and 17% with a
			mortgage. Only 11% rent (7% social housing, 4% private).
			<br /><br />
			<strong>Ethnicity:</strong> The sample is predominantly
			<strong>White British (92%),</strong> with smaller representation from Asian (2%), White
			Other (2%), and other ethnic groups (3%).
			<br /><br />
			<strong>Gender:</strong> Slightly more men (57%) than women (41%), with 2% preferring
			not to say.
			<br /><br />
			<strong>Civic Engagement:</strong> Participants are relatively engaged citizens — 88% have
			voted, 70% have signed petitions, 57% have contacted officials, and 39% have previously engaged
			in council consultations.
		</div>
	</ReportSection>

	<div class="mt-10"></div>

	<!-- Areas of Agreement -->
	<ReportSection id="agreement" title="Areas of agreement" bind:open={agreementOpen}>
		<div class="text-card-foreground text-lg leading-6">
			<p>
				So far, there is a great deal of agreement across the conversation, particularly on
				infrastructure, environmental protection, and community character.
			</p>
			<p class="mt-4">There is strong overall support for:</p>
			<ul class="mt-4 flex list-disc flex-col gap-2 pl-6">
				<li>
					Rural character protection: South Staffordshire's rural character is exactly
					what makes it desirable, and it must be protected.
				</li>
				<li>
					Brownfield first: New housing developments should be built on brownfield sites
					first. Green belt sites should be a last resort, with local consent after full
					consultation.
				</li>
				<li>
					Infrastructure requirements: New developments should only be allowed with
					suitable infrastructure including schools, shops, public transport, and parking.
				</li>
				<li>
					Community input: The views and feedback from a community nearby any development
					should be seen to be taken into account.
				</li>
				<li>
					Green spaces: A key priority is making sure everyone in the community has access
					to nature and green spaces.
				</li>
				<li>
					Flood zone protection: New developments should not be built on flood zones.
					Planning permission should only be given with this enhanced in law.
				</li>
				<li>
					Road conditions: The state of the roads needs to be addressed. Potholes are
					causing accidents and damage.
				</li>
			</ul>
			<p class="mt-4">
				These areas represent shared priorities across much of the conversation, even where
				participants differ on more contentious issues
			</p>
		</div>

		<div class="text-card-foreground mt-6 text-lg leading-6 font-bold">
			Examples of statements of where there is agreement
		</div>

		{@render numberedStatements(consensusStatements)}
	</ReportSection>

	<div class="mt-10"></div>

	<!-- Where there are differences -->
	<ReportSection id="differences" title="Where there are differences" bind:open={differencesOpen}>
		<div class="text-card-foreground text-lg leading-6">
			<p>
				The Interactive discussion element (called Pol.is) on Comhairle looks at patterns in
				how people vote on statements and groups similar responses together. This is done
				anonymously, no one is identified.
			</p>
			<p class="mt-4">
				This helps make sense of a large conversation, so we can see the range of opinions
				and find areas of agreement.
			</p>
			<p class="mt-4">
				So far, after 2 of the 4 weeks of this engagement the platform has identified {data
					.reportData.groups.length} distinctive opinion clusters. The main differences emerge
				around the pace and location of development, green belt protection, and the relationship
				between housing growth and infrastructure.
			</p>
			<p class="mt-4">
				<strong>Green belt designation:</strong>
				Group A expresses strong concern about any re-designation of green belt becoming "grey
				belt," while Group B is more open to reassessing green belt in the context of the housing
				crisis.
			</p>
			<p class="mt-4">
				<strong>Location of development:</strong>
				Group A strongly supports keeping new developments around large towns and leaving villages
				as villages. Group B is more divided on this question, with some supporting development
				if linked to specific improvements.
			</p>
			<p class="mt-4">
				<strong>No new housing stance:</strong>
				Group A shows stronger support for moratoriums on new housing until infrastructure improves.
				Group B is more willing to accept development alongside improvements.
			</p>
			<p class="mt-4">
				<strong>Scale and distribution:</strong>
				There is division between those favouring smaller, sensitive developments on brownfield
				land only, and those supporting broader growth with infrastructure investment.
			</p>
			<p class="mt-4">
				<strong>Perception of urgency:</strong>
				Participants also differ in how urgently they see the housing shortage versus environmental
				protection, which shapes their support for different measures.
			</p>
			<p class="mt-4">
				Overall, the divide is less about whether development is needed, and more about
				where, how much, and under what conditions change should happen.
			</p>
		</div>
	</ReportSection>

	<div class="mt-10"></div>

	<!-- Emerging opinion groups -->
	<ReportSection id="groups" title="Emerging opinion groups after 2 weeks" bind:open={groupsOpen}>
		{#each data.reportData.groups as group (group.group_id)}
			{@const label = String.fromCharCode(65 + group.group_id)}
			{@const repComments = group.representative_comments
				.map((rc) => data.reportData.comments.find((c) => c.tid === rc.tid))
				.filter((c) => c != null)}

			<div
				class="text-foreground w-full max-w-[960px] justify-start text-xl leading-7 font-semibold md:text-3xl md:leading-9"
			>
				Group {label} ({group.members.length} participants):
				{#if group.group_id === 0}
					"Supportive of growth where it improves local infrastructure"
				{:else if group.group_id === 1}
					"Protection-focused, cautious on development"
				{/if}
			</div>

			<div class="justify-start self-stretch">
				{#if group.group_id === 0}
					<p class="text-card-foreground text-lg leading-6 font-normal">This group:</p>
					<ul
						class="text-card-foreground mt-2 list-disc pl-6 text-lg leading-6 font-normal"
					>
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
				{:else if group.group_id === 1}
					<p class="text-card-foreground text-lg leading-6 font-normal">
						This group expresses:
					</p>
					<ul
						class="text-card-foreground mt-2 list-disc pl-6 text-lg leading-6 font-normal"
					>
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
				{:else}
					<span class="text-card-foreground text-lg leading-6 font-normal">
						Group description will be generated by LLM analysis.
					</span>
				{/if}
			</div>

			<div
				class="text-card-foreground justify-start self-stretch text-lg leading-6 font-bold"
			>
				Key Statements this group agrees with:
			</div>

			{#each repComments as comment, i (comment.tid)}
				<div
					class="inline-flex w-full max-w-[984px] items-start justify-start gap-3 md:gap-6"
				>
					<div class="justify-start text-xl leading-6 font-semibold text-gray-400/50">
						#{i + 1}
					</div>
					<div class="flex-1">
						<StatementCard
							{comment}
							groups={data.reportData.groups}
							{totalParticipants}
						/>
					</div>
				</div>
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
			<p class="report-body">Hover over or click on a dot to see the statement.</p>
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

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
		getSignificantComments,
		getAgreementStatements
	} from '$lib/utils/report';

	let { data }: PageProps = $props();

	const stats = $derived(getEngagementStats(data.reportData));
	const consensusStatements = $derived(getConsensusStatements(data.reportData, 5));
	const agreementStatements = $derived(getAgreementStatements(data.reportData, 5));
	const divisiveStatements = $derived(getDivisiveStatements(data.reportData, 5));
	const significantComments = $derived(getSignificantComments(data.reportData));
	const totalParticipants = $derived(stats.totalParticipants);

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
			Conversation update: Shaping South Staffordshire
		</h1>
		<p class="max-w-4xl">
			<span class="font-bold">What we're hearing so far</span>
			<br />
			<span class="italic">Created on 22nd April 2026</span>
		</p>
		<p class="max-w-4xl italic">This stage closes at midnight 4th May</p>

		<!-- Executive Summary -->
		<div class="mt-6 flex w-full max-w-4xl flex-col gap-4 text-left md:mt-10 md:gap-6">
			<h2
				class="text-card-foreground text-2xl leading-8 font-semibold md:text-4xl md:leading-[48px]"
			>
				What we're hearing so far
			</h2>

			<div class="">
				<p class="text-card-foreground text-lg leading-6">
					This update shares what people in South Staffordshire want for their area by
					2045. We created this update mid-way through the <strong>first stage</strong> of the
					public engagement. We are listening to your views to help shape the district’s future
					Local Plan.
				</p>
			</div>

			<div class="">
				<p class="text-card-foreground text-lg leading-6 font-bold">
					This report reflects what you as residents are actually saying. We are sharing
					your words back to you to provide an honest and transparent update on what we’re
					hearing. The council can't commit to making all of your thoughts and ideas
					happen. Where we can’t, we will explain why.
				</p>
			</div>

			<div class="">
				<p class="text-card-foreground text-lg leading-6">
					So far, based on over 21,000 votes from 420 participants, we can see what the
					community agrees on and where opinions differ.
				</p>
			</div>

			<div class="pt-2">
				<p class="text-card-foreground text-lg leading-6">
					<strong>Areas of agreement:</strong>
					Almost all participants want to protect the countryside and the unique character of
					our villages. There is strong support for building new homes on old industrial sites
					(brownfield) first, rather than on greenbelt land. Most agree that new housing must
					come with better roads, schools, and green spaces.
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
				<p class="text-card-foreground text-lg leading-6 italic">
					The analysis and summaries of this report are generated by secure, privacy-first
					Lumo AI. All data is processed with full GDPR compliance and zero-access
					encryption.
				</p>
			</div>

			<h3 class="text-xl leading-6 font-semibold">How do people get involved?</h3>

			<div class="pt-2">
				<p class="text-card-foreground text-lg leading-6">
					Anyone who lives in, works in or visits South Staffordshire can vote on what
					other people are saying and share their own thoughts. Click <a
						href="https://waves.comhairle.scot/conversations/shaping-south-staffordshire/invite/0e6fbb72-49c5-42b1-bcf1-79f56d58fc62"
						class="text-primary hover:underline"
						rel="noopener noreferrer"
						target="_blank">here</a
					> to join the conversation.
				</p>
			</div>
		</div>
	</header>

	<!-- Engagement so far, at a glance -->
	<ReportSection
		id="engagement"
		title="Engagement so far, at a glance"
		bind:open={engagementOpen}
	>
		<!-- TODO: all stat cards below are hardcoded — replace with real data once available:
			 stats.totalParticipants.toLocaleString()
			 stats.totalVotes.toLocaleString()
			 stats.totalStatements.toLocaleString()
			 Math.round((stats.totalVotes / Math.max(stats.totalParticipants, 1)) * 100) / 100
			 data.reportData.groups.length
		-->
		<div class="grid grid-cols-1 gap-4 sm:grid-cols-2 md:gap-8 lg:grid-cols-3">
			<!-- Row 1 -->
			{@render statCard('Participants have taken part', '425')}
			{@render statCard('Votes cast', '22,036')}
			{@render statCard('Statements submitted', '318')}

			<!-- Row 2 -->
			{@render statCard('Votes per participants on average', '51.85')}
			{@render statCard('Opinion groups were identified', '2')}
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
					<strong>Rural character protection:</strong> South Staffordshire's rural character
					is exactly what makes it desirable, and it must be protected.
				</li>
				<li>
					<strong>Brownfield first:</strong> New housing developments should be built on brownfield
					sites first. Green belt sites should be a last resort, with local consent after full
					consultation.
				</li>
				<li>
					<strong>Infrastructure first:</strong> New developments should only be allowed with
					suitable infrastructure including schools, shops, public transport, and parking.
				</li>
				<li>
					<strong>Listening to locals:</strong> The views of the community nearby any development
					must be taken into account.
				</li>
				<li>
					<strong>Green spaces:</strong> Everyone should have access to nature and green spaces.
				</li>
				<li>
					<strong>Safety:</strong> New homes should not be built on flood zones, and our roads
					need fixing to stop potholes and accidents.
				</li>
			</ul>
		</div>

		<div class="text-card-foreground mt-6 text-lg leading-6 font-bold">
			Examples of statements of where there is agreement
		</div>

		{@render numberedStatements(agreementStatements)}
	</ReportSection>

	<div class="mt-10"></div>

	<!-- Where there are differences -->
	<ReportSection id="differences" title="Where there are differences" bind:open={differencesOpen}>
		<div class="text-card-foreground text-lg leading-6">
			<p>
				The platform groups people anonymously based on how they voted. This helps us see
				the range of opinions. So far, after 2 weeks of participation the platform has
				identified 2 distinctive opinion clusters.
			</p>
			<p class="mt-4">
				The main difference we’re seeing are about <strong>how fast</strong> we build,
				<strong>where</strong>
				we build, and <strong>how much</strong> we protect the greenbelt.
			</p>
			<ul class="mt-4 list-disc space-y-3 pl-6">
				<li>
					<strong>Green belt:</strong>
					One group is concerned about any change to greenbelt land ("grey belt"), while the
					other is more open to reviewing it if it helps ease the housing crisis.
				</li>
				<li>
					<strong>Location:</strong>
					One group wants to keep villages as villages and build only around large towns. The
					other is more flexible if it brings improvements to infrastructure.
				</li>
				<li>
					<strong>Timing:</strong>
					One group says "no new homes until roads and schools are fixed." The other says "we
					can accept new homes, but we have to make sure we get the improvements at the same
					time."
				</li>
			</ul>
		</div>
	</ReportSection>

	<div class="mt-10"></div>

	<!-- Emerging opinion groups -->
	<ReportSection
		id="groups"
		title="Opinion groups as they stand so far (after 2 weeks)"
		bind:open={groupsOpen}
	>
		{#each data.reportData.groups as group (group.group_id)}
			{@const label = String.fromCharCode(65 + group.group_id)}
			{@const repComments = group.representative_comments
				.map((rc) => data.reportData.comments.find((c) => c.tid === rc.tid))
				.filter((c) => c != null)}

			<div
				class="text-foreground w-full max-w-[960px] justify-start text-xl leading-7 font-semibold md:text-3xl md:leading-9"
			>
				<!-- TODO: use real number {group.total_members} once data is verified -->
				Group {label} ({group.group_id === 0 ? 158 : 138} participants):
				{#if group.group_id === 0}
					"Supportive of growth where it improves local infrastructure"
				{:else if group.group_id === 1}
					"Protection first"
				{/if}
			</div>

			<div class="justify-start self-stretch">
				{#if group.group_id === 0}
					<p class="text-card-foreground text-lg leading-6 font-normal">
						This group is more willing to accept new development if it brings clear
						benefits.
					</p>
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
						This group is very cautious about new building.
					</p>
					<ul
						class="text-card-foreground mt-2 list-disc pl-6 text-lg leading-6 font-normal"
					>
						<li>They are strongly against changing greenbelt land.</li>
						<li>
							They want to keep villages as villages and stop expansion into the
							countryside.
						</li>
						<li>
							They believe we should not build new homes until existing roads and
							infrastructure are fixed.
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
				Statements and voting that reveal Group {group.group_id === 0 ? 'A' : 'B'}'s
				distinct views:
			</div>

			{#each repComments as comment, i (comment.tid)}
				<div
					class="inline-flex w-full max-w-[984px] items-start justify-start gap-3 md:gap-6"
				>
					<div
						class="text-muted-foreground justify-start text-xl leading-6 font-semibold"
					>
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

	<!-- Beeswarm -->
	<ReportSection
		title="Taking a deeper look at the results..."
		bind:open={deeperLookOpen}
		id="deep-dive"
	>
		<div class="max-w-4xl pt-2 pb-6">
			<p class="report-body">
				Hover over or click on a dot to see the statement and the voting it has received so
				far.
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

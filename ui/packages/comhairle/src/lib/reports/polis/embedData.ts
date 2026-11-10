import { apiClient } from '@crownshy/api-client/client';
import { tryCatchAsync } from '$lib/utils/errorHandling';
import type {
	PolisPublicGroup,
	PolisPublicKeyStats,
	PolisPublicOpinionGroup,
	PolisPublicStatement
} from '@crownshy/api-client/api';
import type { GroupSize, RepresentedGroup, VotedStatement } from '$lib/tools/polis/reportTypes';
import type { PolisEmbeddableComponentType } from './embeddableComponents';

/** What one Polis section block draws, from its public report-data endpoint (ADR-0061). */
export type PolisEmbedData =
	| { componentType: 'polis-key-stats'; stats: PolisPublicKeyStats }
	| {
			componentType: 'polis-area-consensus' | 'polis-area-disagreement';
			statements: (VotedStatement & { is_seed: boolean })[];
			groups: GroupSize[];
			/** The sensemaker's summary of this side, once generated. */
			summary?: string;
	  }
	| {
			componentType: 'polis-consensus-continuum';
			statements: (VotedStatement & { divisiveness: number })[];
			groups: GroupSize[];
	  }
	| {
			componentType: 'polis-opinion-groups';
			statements: VotedStatement[];
			groups: RepresentedGroup[];
			/** The sensemaker's name and summary per group, keyed by `group_id`. */
			groupSummaries: Record<number, { name: string; summary: string }>;
	  }
	| {
			componentType: 'polis-themes';
			themes: PolisEmbedTheme[];
			groups: GroupSize[];
	  };

export type PolisEmbedTheme = {
	title: string;
	description: string;
	contributions: number;
	statements: VotedStatement[];
};

/**
 * Each section block's endpoints. The preview ones need a facilitator who can update the
 * conversation; the public ones answer 404 until the published report embeds the block.
 */
const ENDPOINTS = {
	public: {
		keyStats: apiClient.PolisGetPublicKeyStats,
		consensus: apiClient.PolisGetPublicConsensus,
		disagreement: apiClient.PolisGetPublicDisagreement,
		consensusContinuum: apiClient.PolisGetPublicConsensusContinuum,
		opinionGroups: apiClient.PolisGetPublicOpinionGroups,
		themes: apiClient.PolisGetPublicThemes
	},
	preview: {
		keyStats: apiClient.PolisPreviewKeyStats,
		consensus: apiClient.PolisPreviewConsensus,
		disagreement: apiClient.PolisPreviewDisagreement,
		consensusContinuum: apiClient.PolisPreviewConsensusContinuum,
		opinionGroups: apiClient.PolisPreviewOpinionGroups,
		themes: apiClient.PolisPreviewThemes
	}
};

/**
 * Load one section block's data. A preview falls back to the public endpoint, so someone
 * opening a preview link without facilitator rights still sees a published report's charts.
 * Any failure throws, and the caller shows the embed's unavailable state.
 */
export async function loadPolisEmbedData(
	workflowStepId: string,
	componentType: PolisEmbeddableComponentType,
	preview = false
): Promise<PolisEmbedData> {
	if (preview) {
		const previewed = await tryCatchAsync(() =>
			loadFrom(ENDPOINTS.preview, workflowStepId, componentType)
		);
		if (previewed.err === null) return previewed.ok;
	}
	return loadFrom(ENDPOINTS.public, workflowStepId, componentType);
}

async function loadFrom(
	endpoints: (typeof ENDPOINTS)['public'],
	workflowStepId: string,
	componentType: PolisEmbeddableComponentType
): Promise<PolisEmbedData> {
	const request = { queries: { workflow_step_id: workflowStepId } };
	switch (componentType) {
		case 'polis-key-stats':
			return { componentType, stats: await endpoints.keyStats(request) };
		case 'polis-area-consensus': {
			const ranked = await endpoints.consensus(request);
			return {
				componentType,
				statements: ranked.statements.map((s) => ({
					...toVotedStatement(s),
					is_seed: s.isSeed
				})),
				groups: ranked.groups.map(toGroupSize),
				summary: ranked.summary ?? undefined
			};
		}
		case 'polis-area-disagreement': {
			const ranked = await endpoints.disagreement(request);
			return {
				componentType,
				statements: ranked.statements.map((s) => ({
					...toVotedStatement(s),
					is_seed: s.isSeed
				})),
				groups: ranked.groups.map(toGroupSize),
				summary: ranked.summary ?? undefined
			};
		}
		case 'polis-consensus-continuum': {
			const continuum = await endpoints.consensusContinuum(request);
			return {
				componentType,
				statements: continuum.statements.map((s) => ({
					...toVotedStatement(s),
					divisiveness: s.divisiveness
				})),
				groups: continuum.groups.map(toGroupSize)
			};
		}
		case 'polis-opinion-groups': {
			const opinionGroups = await endpoints.opinionGroups(request);
			return {
				componentType,
				statements: opinionGroups.statements.map(toVotedStatement),
				groups: opinionGroups.groups.map(toRepresentedGroup),
				groupSummaries: toGroupSummaries(opinionGroups.groups)
			};
		}
		case 'polis-themes': {
			const themes = await endpoints.themes(request);
			return {
				componentType,
				themes: themes.themes.map((theme) => ({
					title: theme.title,
					description: theme.description,
					contributions: theme.statements.length,
					statements: theme.statements.map(toVotedStatement)
				})),
				groups: themes.groups.map(toGroupSize)
			};
		}
	}
}

/** The groups the sensemaker has named and summarised, in the shape `OpinionGroups` reads. */
export function toGroupSummaries(
	groups: PolisPublicOpinionGroup[]
): Record<number, { name: string; summary: string }> {
	const summaries: Record<number, { name: string; summary: string }> = {};
	for (const group of groups) {
		if (!group.name || !group.description) continue;
		summaries[group.groupId] = { name: group.name, summary: group.description };
	}
	return summaries;
}

/**
 * The report components share their shapes with the admin Insights tab, which reads Polis's
 * snake_case report and the statement's themes as `topics`.
 */
export function toVotedStatement(statement: PolisPublicStatement): VotedStatement {
	return {
		tid: statement.tid,
		text: statement.text,
		topics: statement.themes,
		overall_votes: statement.overallVotes,
		group_votes: statement.groupVotes.map(({ groupId, agrees, disagrees, passes }) => ({
			group_id: groupId,
			agrees,
			disagrees,
			passes
		}))
	};
}

function toGroupSize(group: PolisPublicGroup): GroupSize {
	return { group_id: group.groupId, total_members: group.totalMembers };
}

function toRepresentedGroup(group: PolisPublicOpinionGroup): RepresentedGroup {
	return {
		...toGroupSize(group),
		representative_comments: group.representativeComments
	};
}

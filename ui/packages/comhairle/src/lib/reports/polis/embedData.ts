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
		opinionGroups: apiClient.PolisGetPublicOpinionGroups
	},
	preview: {
		keyStats: apiClient.PolisPreviewKeyStats,
		consensus: apiClient.PolisPreviewConsensus,
		disagreement: apiClient.PolisPreviewDisagreement,
		consensusContinuum: apiClient.PolisPreviewConsensusContinuum,
		opinionGroups: apiClient.PolisPreviewOpinionGroups
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
				groups: ranked.groups.map(toGroupSize)
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
				groups: ranked.groups.map(toGroupSize)
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
				groups: opinionGroups.groups.map(toRepresentedGroup)
			};
		}
	}
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

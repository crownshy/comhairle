import { apiClient } from '@crownshy/api-client/client';
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
 * Load one section block's data. The endpoints answer 404 until the published report embeds
 * the block, and that throws like any other failure so the caller shows the embed's
 * unavailable state.
 */
export async function loadPolisEmbedData(
	workflowStepId: string,
	componentType: PolisEmbeddableComponentType
): Promise<PolisEmbedData> {
	const request = { queries: { workflow_step_id: workflowStepId } };
	switch (componentType) {
		case 'polis-key-stats':
			return { componentType, stats: await apiClient.PolisGetPublicKeyStats(request) };
		case 'polis-area-consensus': {
			const ranked = await apiClient.PolisGetPublicConsensus(request);
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
			const ranked = await apiClient.PolisGetPublicDisagreement(request);
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
			const continuum = await apiClient.PolisGetPublicConsensusContinuum(request);
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
			const opinionGroups = await apiClient.PolisGetPublicOpinionGroups(request);
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

import type { ComponentProps } from 'svelte';
import { apiClient } from '@crownshy/api-client/client';
import { tryCatchAsync } from '$lib/utils/errorHandling';
import type {
	PolisPublicGroup,
	PolisPublicKeyStats,
	PolisPublicOpinionGroup,
	PolisPublicRankedStatements,
	PolisPublicStatement
} from '@crownshy/api-client/api';
import type { GroupSize, RepresentedGroup, VotedStatement } from '$lib/tools/polis/reportTypes';
import type AreaOfConsensus from './AreaOfConsensus.svelte';
import type ConsensusContinuum from './ConsensusContinuum.svelte';
import type OpinionGroups from './OpinionGroups.svelte';
import type { PolisEmbeddableComponentType } from './embeddableComponents';

type AreaProps = ComponentProps<typeof AreaOfConsensus>;
type ContinuumProps = ComponentProps<typeof ConsensusContinuum>;
type OpinionGroupsProps = ComponentProps<typeof OpinionGroups>;

/** What one Polis section block draws, from its public report-data endpoint (ADR-0061). */
export type PolisEmbedData =
	| { componentType: 'polis-key-stats'; stats: PolisPublicKeyStats }
	| {
			componentType: 'polis-area-consensus' | 'polis-area-disagreement';
			statements: AreaProps['comments'];
			groups: AreaProps['groups'];
	  }
	| {
			componentType: 'polis-consensus-continuum';
			statements: ContinuumProps['comments'];
			groups: ContinuumProps['groups'];
	  }
	| {
			componentType: 'polis-opinion-groups';
			statements: OpinionGroupsProps['comments'];
			groups: OpinionGroupsProps['groups'];
	  };

/**
 * Each section block's endpoints. The preview ones need a facilitator who can update the
 * conversation; the public ones answer 404 until the published report embeds the block.
 */
const ENDPOINTS = {
	public: {
		'polis-key-stats': apiClient.PolisGetPublicKeyStats,
		'polis-area-consensus': apiClient.PolisGetPublicConsensus,
		'polis-area-disagreement': apiClient.PolisGetPublicDisagreement,
		'polis-consensus-continuum': apiClient.PolisGetPublicConsensusContinuum,
		'polis-opinion-groups': apiClient.PolisGetPublicOpinionGroups
	},
	preview: {
		'polis-key-stats': apiClient.PolisPreviewKeyStats,
		'polis-area-consensus': apiClient.PolisPreviewConsensus,
		'polis-area-disagreement': apiClient.PolisPreviewDisagreement,
		'polis-consensus-continuum': apiClient.PolisPreviewConsensusContinuum,
		'polis-opinion-groups': apiClient.PolisPreviewOpinionGroups
	}
} satisfies Record<'public' | 'preview', Record<PolisEmbeddableComponentType, unknown>>;

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
			return { componentType, stats: await endpoints[componentType](request) };
		case 'polis-area-consensus':
		case 'polis-area-disagreement':
			return {
				componentType,
				...toRankedStatements(await endpoints[componentType](request))
			};
		case 'polis-consensus-continuum': {
			const continuum = await endpoints[componentType](request);
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
			const opinionGroups = await endpoints[componentType](request);
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
		tid: statement.statementId,
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

function toRankedStatements(ranked: PolisPublicRankedStatements) {
	return {
		statements: ranked.statements.map((statement) => ({
			...toVotedStatement(statement),
			is_seed: statement.isSeed
		})),
		groups: ranked.groups.map(toGroupSize)
	};
}

function toGroupSize(group: PolisPublicGroup): GroupSize {
	return { group_id: group.groupId, total_members: group.totalMembers };
}

function toRepresentedGroup(group: PolisPublicOpinionGroup): RepresentedGroup {
	return {
		...toGroupSize(group),
		representative_comments: group.representativeComments.map(({ statementId }) => ({
			tid: statementId
		}))
	};
}

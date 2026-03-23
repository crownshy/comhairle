export type RevisitPolicy = 'always' | 'before_ending' | 'never';

const REVISIT_POLICY_BY_TOOL_TYPE: Record<string, RevisitPolicy> = {
	learn: 'always',
	polis: 'before_ending',
	elicitationbot: 'never',
	heyform: 'never'
};

const DEFAULT_POLICY: RevisitPolicy = 'never';

export function canRevisitStep(toolType: string, workflowEnded: boolean): boolean {
	const policy = REVISIT_POLICY_BY_TOOL_TYPE[toolType] ?? DEFAULT_POLICY;

	switch (policy) {
		case 'always':
			return true;
		case 'before_ending':
			return !workflowEnded;
		case 'never':
			return false;
	}
}

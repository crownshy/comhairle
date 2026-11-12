/** The sensemaker's name and summary for an opinion group, as `OpinionGroupCard` shows it. */
export type GroupSummary = { name: string; summary: string };

export type GroupSummaries = Record<number, GroupSummary>;

/** A group as the stored summary or the public report describes it. */
type DescribedGroup = {
	groupId: number | string;
	name?: string | null;
	description?: string | null;
};

/** Groups with both a name and a description, keyed by group id. */
export function toGroupSummaries(groups: DescribedGroup[]): GroupSummaries {
	const summaries: GroupSummaries = {};
	for (const { groupId, name, description } of groups) {
		if (!name || !description) continue;
		summaries[Number(groupId)] = { name, summary: description };
	}
	return summaries;
}

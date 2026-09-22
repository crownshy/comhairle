// PROTOTYPE - throwaway. Practice statements and a fake crowd, shared by every variant so
// the numbers a tester sees line up whichever variant they get.

export type Audience = 'young' | 'policy';
export type Vote = 'agree' | 'disagree' | 'pass';

type Copy = {
	// Index 0 and 2 split the crowd along the same line, index 1 is common ground. That is
	// the shape that makes the Polis payoff visible in three votes: two groups, one thing
	// they both agree on.
	practice: [string, string, string];
	groupNames: [string, string];
	pattern: string;
	real: string[];
};

export const COPY: Record<Audience, Copy> = {
	young: {
		practice: [
			'Pineapple belongs on pizza.',
			'Everyone deserves a lie-in at the weekend.',
			'Cats are better than dogs.'
		],
		groupNames: ['Team pineapple and cats', 'Team no pineapple and dogs'],
		pattern:
			'People who like pineapple on pizza mostly prefer cats. Nobody told us that. The votes did.',
		real: [
			'Bus travel should be free for everyone under 25.',
			'Schools should teach more about money and bills.',
			'There is not enough for young people to do in my area.'
		]
	},
	policy: {
		practice: [
			'Meetings should never run longer than 30 minutes.',
			'Public services should be easy to use online.',
			'Working from home is better than working in an office.'
		],
		groupNames: ['Short meetings, work from home', 'Longer meetings, office days'],
		pattern:
			'People who want short meetings mostly prefer working from home. Nobody asked that directly. The votes showed it.',
		real: [
			'Local councils should publish how consultation responses changed a decision.',
			'Rural bus routes should be protected even when passenger numbers are low.',
			'Community groups should get multi-year funding, not annual grants.'
		]
	}
};

export type Person = {
	id: number;
	group: 0 | 1;
	votes: [Vote, Vote, Vote];
	startX: number;
	startY: number;
	jitterX: number;
	jitterY: number;
};

function seeded(seed: number) {
	let a = seed;
	return () => {
		a |= 0;
		a = (a + 0x6d2b79f5) | 0;
		let t = Math.imul(a ^ (a >>> 15), 1 | a);
		t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t;
		return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
	};
}

function makeCrowd(size: number): Person[] {
	const rand = seeded(2026);
	return Array.from({ length: size }, (_, id) => {
		const group: 0 | 1 = id % 7 < 4 ? 0 : 1;
		const vote = (kind: 'split' | 'common'): Vote => {
			const r = rand();
			if (rand() < 0.05) return 'pass';
			const agrees = kind === 'common' ? r < 0.9 : group === 0 ? r < 0.86 : r < 0.2;
			return agrees ? 'agree' : 'disagree';
		};
		return {
			id,
			group,
			votes: [vote('split'), vote('common'), vote('split')],
			startX: 8 + rand() * 84,
			startY: 8 + rand() * 54,
			jitterX: (rand() - 0.5) * 16,
			jitterY: (rand() - 0.5) * 14
		};
	});
}

export const CROWD = makeCrowd(42);

export function tally(index: number) {
	const counts = { agree: 0, disagree: 0, pass: 0 };
	for (const p of CROWD) counts[p.votes[index]]++;
	const pct = (n: number) => Math.round((n / CROWD.length) * 100);
	return { agree: pct(counts.agree), disagree: pct(counts.disagree), pass: pct(counts.pass) };
}

/** Share of the crowd who voted the same way as `vote` on statement `index`. */
export function sameAs(index: number, vote: Vote) {
	return tally(index)[vote];
}

const score = (v: Vote | undefined) => (v === 'agree' ? -1 : v === 'disagree' ? 1 : 0);

/**
 * Where a dot sits on a 100 x 70 map once `known` of its votes are counted. Before any
 * votes it is scattered; the split statements pull it left or right, the common one pulls
 * almost everyone into the same band.
 */
export function place(
	votes: (Vote | undefined)[],
	known: number,
	start: { x: number; y: number },
	jitter: { x: number; y: number }
) {
	if (known === 0) return start;
	const split = [votes[0], known >= 3 ? votes[2] : undefined].filter(
		(v): v is Vote => v !== undefined
	);
	const lean = split.reduce((sum, v) => sum + score(v), 0) / split.length;
	const x = 50 + lean * 28 + jitter.x;
	const y = known >= 2 ? 35 + score(votes[1]) * 16 + jitter.y : start.y * 0.5 + 17 + jitter.y;
	return { x: Math.min(95, Math.max(5, x)), y: Math.min(65, Math.max(5, y)) };
}

/** 0 for the group that agrees with the split statements, 1 for the one that does not. */
export function closestGroup(votes: Vote[]): 0 | 1 {
	return score(votes[0]) + score(votes[2]) <= 0 ? 0 : 1;
}

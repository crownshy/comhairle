// The test scenario lives in the URL (?user=1&data=empty&org=waves), so any
// scenario can be shared as a link and survives a reload.
import type { Dataset } from './mockData';
import type { ThemeName } from '$lib/types/theme';

export type Organisation = {
	id: string;
	/** Shown on the page, e.g. above the pitch. */
	name: string;
	/** Chip label in the scenario panel. */
	label: string;
	/** The real theme from the codebase: its colours (comhairle-themes.css) and its logo. */
	theme: ThemeName;
};

export const ORGANISATIONS: Organisation[] = [
	{ id: 'comhairle', name: 'Comhairle', label: 'Comhairle', theme: 'comhairle' },
	{
		id: 'scot-gov',
		name: 'Scottish Government',
		label: 'Scottish Government',
		theme: 'scot-gov'
	},
	{
		id: 'waves',
		name: 'South Staffordshire Council',
		label: 'Waves',
		theme: 'waves'
	}
];

export const DATASETS: { value: Dataset; label: string }[] = [
	{ value: 'empty', label: 'None' },
	{ value: 'normal', label: 'A few' },
	{ value: 'many', label: '15' }
];

export type Scenario = { loggedIn: boolean; dataset: Dataset; org: Organisation };

const SCENARIO_KEYS = ['user', 'data', 'org'];

export function readScenario(url: URL): Scenario {
	const data = url.searchParams.get('data');
	const dataset = DATASETS.find((d) => d.value === data)?.value ?? 'normal';
	return {
		loggedIn: url.searchParams.get('user') === '1',
		dataset,
		org: ORGANISATIONS.find((o) => o.id === url.searchParams.get('org')) ?? ORGANISATIONS[0]
	};
}

type ScenarioPatch = { loggedIn?: boolean; dataset?: Dataset; orgId?: string };

/** Only the scenario part of the query string, with some values changed. */
export function scenarioQuery(url: URL, patch: ScenarioPatch = {}): string {
	const current = readScenario(url);
	const params = new URLSearchParams();
	if (patch.loggedIn ?? current.loggedIn) params.set('user', '1');
	const dataset = patch.dataset ?? current.dataset;
	if (dataset !== 'normal') params.set('data', dataset);
	params.set('org', patch.orgId ?? current.org.id);
	return `?${params}`;
}

/** The scenario params as hidden-form-field pairs, so a GET search form keeps them. */
export function scenarioFields(url: URL): [string, string][] {
	return [...url.searchParams.entries()].filter(([key]) => SCENARIO_KEYS.includes(key));
}

/** The current URL with list params changed. Changing a filter goes back to page 1. */
export function withListParams(url: URL, patch: Record<string, string | null>): string {
	const next = new URL(url);
	next.searchParams.delete('page');
	for (const [key, value] of Object.entries(patch)) {
		if (value === null) next.searchParams.delete(key);
		else next.searchParams.set(key, value);
	}
	return `${next.pathname}${next.search}`;
}

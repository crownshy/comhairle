// The test scenario lives in the URL (?user=1&data=empty&org=riverside), so any
// scenario can be shared as a link and survives a reload.
import type { Dataset } from './mockData';
import type { ThemeName } from '$lib/types/theme';

export type Organisation = {
	id: string;
	name: string;
	initials: string;
	/** Brand colour that replaces the theme primary. null keeps the running theme's own. */
	brand: string | null;
};

/** The organisation of the running theme: its real logo and colours, nothing overridden. */
export const THEME_ORG_ID = 'theme';

export const ORGANISATIONS: Organisation[] = [
	{ id: THEME_ORG_ID, name: 'Current theme', initials: '', brand: null },
	{ id: 'riverside', name: 'Riverside Council', initials: 'RC', brand: '#2b62ba' },
	{ id: 'health', name: 'Northfield Health Board', initials: 'NH', brand: '#0f6e5c' },
	{ id: 'youth', name: 'Youth Assembly', initials: 'YA', brand: '#7a2e8e' }
];

export const DATASETS: { value: Dataset; label: string }[] = [
	{ value: 'normal', label: 'A few' },
	{ value: 'empty', label: 'None' },
	{ value: 'many', label: '150+' }
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

export function brandStyle(org: Organisation): string {
	if (!org.brand) return '';
	return `--primary: ${org.brand}; --color-brand: ${org.brand}; --primary-foreground: #ffffff; --ring: ${org.brand};`;
}

const THEME_ORG_NAMES: Record<ThemeName, string> = {
	comhairle: 'Comhairle',
	'scot-gov': 'Scottish Government',
	waves: 'Waves'
};

/** The name to show for an organisation. The theme org takes the running theme's name. */
export function orgDisplayName(org: Organisation, themeName: ThemeName): string {
	return org.id === THEME_ORG_ID ? THEME_ORG_NAMES[themeName] : org.name;
}

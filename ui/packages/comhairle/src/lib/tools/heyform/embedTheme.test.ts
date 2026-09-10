import { describe, expect, it } from 'vitest';
import { readEmbedTheme } from './embedTheme';

/** Stands in for getComputedStyle: an unset custom property resolves to the empty string. */
function resolver(tokens: Record<string, string>) {
	return (token: string) => tokens[token] ?? '';
}

const LIGHT = {
	'--card': '#ffffff',
	'--foreground': '#171f0f',
	'--primary': '#3d5a1e',
	'--primary-foreground': '#ffffff'
};

const DARK = {
	'--card': '#171f0f',
	'--foreground': '#e8eddf',
	'--primary': '#7cb342',
	'--primary-foreground': '#0f1509'
};

describe('readEmbedTheme', () => {
	it('maps comhairle tokens onto the fields the fork accepts', () => {
		expect(readEmbedTheme(resolver(LIGHT))).toEqual({
			backgroundColor: '#ffffff',
			questionTextColor: '#171f0f',
			answerTextColor: '#171f0f',
			buttonBackground: '#3d5a1e',
			buttonTextColor: '#ffffff'
		});
	});

	it('follows the mode', () => {
		const dark = readEmbedTheme(resolver(DARK));

		expect(dark.backgroundColor).toBe('#171f0f');
		expect(dark.questionTextColor).toBe('#e8eddf');
		expect(dark.buttonBackground).toBe('#7cb342');
	});

	it('accepts shorthand hex', () => {
		expect(readEmbedTheme(resolver({ ...LIGHT, '--card': '#fff' })).backgroundColor).toBe(
			'#fff'
		);
	});

	it('drops a token carrying alpha, which the fork would misread', () => {
		const theme = readEmbedTheme(resolver({ ...LIGHT, '--card': '#ffffff19' }));

		expect(theme.backgroundColor).toBeUndefined();
		expect(theme.questionTextColor).toBe('#171f0f');
	});

	it('drops anything that is not hex rather than passing it into a style tag', () => {
		const theme = readEmbedTheme(
			resolver({
				...LIGHT,
				'--card': 'oklch(0.98 0 0)',
				'--foreground': 'color-mix(in oklch, black, white 20%)',
				'--primary': 'red; } body { display: none } .x {'
			})
		);

		expect(theme).toEqual({ buttonTextColor: '#ffffff' });
	});

	it('returns nothing when the tokens are missing entirely', () => {
		expect(readEmbedTheme(resolver({}))).toEqual({});
	});
});

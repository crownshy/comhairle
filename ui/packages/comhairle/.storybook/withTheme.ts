import type { Decorator } from '@storybook/sveltekit';

/**
 * Storybook decorator that applies theme and mode to the document
 * based on toolbar selections
 */
export const withTheme: Decorator = (story, context) => {
	const { theme, mode } = context.globals;

	if (typeof document !== 'undefined') {
		const html = document.documentElement;

		if (theme === 'comhairle') {
			html.removeAttribute('data-theme');
		} else {
			html.setAttribute('data-theme', theme);
		}

		if (mode === 'dark') {
			html.classList.add('dark');
		} else {
			html.classList.remove('dark');
		}
	}

	return story();
};

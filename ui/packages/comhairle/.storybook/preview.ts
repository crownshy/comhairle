import type { Preview } from '@storybook/sveltekit';
import '../src/app.css';
import '../src/lib/styles/comhairle-themes.css';
import { withTheme } from './withTheme';

const preview: Preview = {
	decorators: [withTheme],
	parameters: {
		controls: {
			matchers: {
				color: /(background|color)$/i,
				date: /Date$/i
			}
		}
	},
	globalTypes: {
		theme: {
			description: 'Global theme for components',
			defaultValue: 'comhairle',
			toolbar: {
				title: 'Theme',
				icon: 'paintbrush',
				items: [
					{ value: 'comhairle', title: 'Comhairle' },
					{ value: 'scotgov', title: 'Scottish Gov' }
				],
				dynamicTitle: true
			}
		},
		mode: {
			description: 'Theme mode',
			defaultValue: 'light',
			toolbar: {
				title: 'Mode',
				icon: 'circlehollow',
				items: [
					{ value: 'light', title: 'Light', icon: 'sun' },
					{ value: 'dark', title: 'Dark', icon: 'moon' }
				],
				dynamicTitle: true
			}
		}
	}
};

export default preview;

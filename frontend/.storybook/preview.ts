import type { Preview } from '@storybook/sveltekit';
import { withThemeByClassName } from '@storybook/addon-themes';

// Import your app's global CSS (includes design tokens)
import '../src/app.css';

const preview: Preview = {
  parameters: {
    controls: {
      matchers: {
        color: /(background|color)$/i,
        date: /Date$/i,
      },
    },
  },
  decorators: [
    withThemeByClassName({
      themes: {
        'Figma': 'figma',
        'Scottish Government': 'scot-gov',
        'Comhairle': 'comhairle',
      },
      defaultTheme: 'Comhairle',
    }),
  ],
};

export default preview;
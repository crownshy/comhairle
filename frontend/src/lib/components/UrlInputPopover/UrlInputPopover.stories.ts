import type { Meta, StoryObj } from '@storybook/svelte';
import UrlInputPopoverStory from './UrlInputPopover.story.svelte';

const meta = {
	title: 'Components/UrlInputPopover',
	component: UrlInputPopoverStory,
	tags: ['autodocs'],
	argTypes: {
		open: {
			control: 'boolean',
			description: 'Controls whether the popover is initially open'
		},
		label: {
			control: 'text',
			description: 'Label shown above the input field'
		},
		placeholder: {
			control: 'text',
			description: 'Placeholder text in the input field'
		},
		buttonText: {
			control: 'text',
			description: 'Text shown on the submit button'
		},
		buttonStyle: {
			control: 'text',
			description: 'CSS classes for the trigger button'
		},
		buttonLabel: {
			control: 'text',
			description: 'Text shown on the trigger button'
		},
		validationType: {
			control: 'select',
			options: ['url', 'image', 'video'],
			description: 'Type of URL validation to apply'
		}
	},
	parameters: {
		docs: {
			description: {
				component: `
**URL Input Popover**

A reusable popover component for collecting and validating URLs in toolbars and other UI contexts.

**Features:**
- Built on top of bits-ui Popover
- Inline validation with error messages
- Keyboard navigation (Enter to submit, Escape to cancel)
- Auto-focus on open
- Accessible with proper ARIA attributes

**Usage:**
Wrap the trigger element (typically a button) with this component. The popover will appear below the trigger when opened.

**Validation Types:**
- \`url\` - Validates standard HTTPS URLs (default)
- \`image\` - Validates HTTPS image URLs  
- \`video\` - Validates YouTube/Vimeo embed URLs with domain whitelist
				`
			}
		}
	}
} satisfies Meta<any>;

export default meta;
type Story = StoryObj<typeof meta>;

export const LinkValidation: Story = {
	args: {
		label: 'Insert Link',
		placeholder: 'https://example.com',
		buttonText: 'Insert',
		buttonStyle: 'px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600',
		buttonLabel: 'Add Link',
		validationType: 'url'
	}
};

export const ImageValidation: Story = {
	args: {
		label: 'Insert Image',
		placeholder: 'https://example.com/image.jpg',
		buttonText: 'Insert Image',
		buttonStyle: 'px-4 py-2 bg-green-500 text-white rounded hover:bg-green-600',
		buttonLabel: 'Add Image',
		validationType: 'image'
	}
};

export const VideoValidation: Story = {
	args: {
		label: 'Insert Video',
		placeholder: 'https://youtube.com/embed/...',
		buttonText: 'Insert Video',
		buttonStyle: 'px-4 py-2 bg-purple-500 text-white rounded hover:bg-purple-600',
		buttonLabel: 'Add Video',
		validationType: 'video'
	}
};

import type { Meta, StoryObj } from '@storybook/svelte';
import RichTextEditor from '$lib/components/RichTextEditor/RichTextEditor.svelte';

const meta = {
	title: 'Editors/RichTextEditor (TipTap)',
	component: RichTextEditor as any,
	tags: ['autodocs'],
	argTypes: {
		value: {
			control: 'text',
			description: 'Content as Markdown or ProseMirror JSON',
		},
		placeholder: {
			control: 'text',
			description: 'Placeholder text when editor is empty'
		},
		editable: {
			control: 'boolean',
			description: 'Whether the editor is editable'
		},
		onUpdate: {
			action: 'content updated',
			description: 'Callback fired when content changes (receives JSON string)'
		}
	},
	parameters: {
		docs: {
			description: {
				component: `
**Rich Text Editor using TipTap**

A full-featured WYSIWYG editor with markdown support, built with TipTap.

**Features:**
- Auto-detects Markdown vs JSON format
- Mobile-responsive with collapsible toolbar
- Supports: Bold, Italic, Strikethrough, Underline, Code
- Lists, Headings, Blockquotes, Text Alignment
- Links, Images, Video embeds (YouTube, Vimeo, etc.)
- Saves content as ProseMirror JSON for better structure preservation

**Security:**
- Video embeds are validated against whitelist of trusted domains
- XSS protection on iframe URLs
				`
			}
		}
	}
} satisfies Meta<any>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
	args: {
		value: '',
		placeholder: 'Start typing...',
		editable: true
	}
};

export const WithMarkdownContent: Story = {
	args: {
		value: `# Welcome to the Rich Text Editor

This is a **bold** statement and this is *italic*.

## Features Include

- Bullet lists
- **Bold text**
- *Italic text*
- ~~Strikethrough~~

### Numbered Lists

1. First item
2. Second item
3. Third item

> Blockquotes look great too!

You can also add links and images.`,
		placeholder: 'Start typing...',
		editable: true
	}
};

// ✅ NEW: Example with JSON format
export const WithJSONContent: Story = {
	args: {
		value: JSON.stringify({
			"type": "doc",
			"content": [
				{
					"type": "heading",
					"attrs": { "level": 1 },
					"content": [{ "type": "text", "text": "Hello from JSON!" }]
				},
				{
					"type": "paragraph",
					"content": [
						{ "type": "text", "text": "This content is loaded from " },
						{ "type": "text", "marks": [{ "type": "bold" }], "text": "ProseMirror JSON" },
						{ "type": "text", "text": " format." }
					]
				}
			]
		}),
		editable: true
	}
};

export const WithLinks: Story = {
	args: {
		value: `# Useful Resources

Check out these helpful links:

- [Climate Action Toolkit](https://example.com/toolkit)
- [Community Guidelines](https://example.com/guidelines)
- [Report an Issue](https://example.com/report)

You can add links by clicking the 🔗 button in the toolbar.`,
		editable: true
	}
};

export const WithVideo: Story = {
	args: {
		value: `# Community Engagement Video

Watch this introduction to our participatory democracy platform:

<iframe src="https://www.youtube.com/embed/dQw4w9WgXcQ" frameborder="0" allowfullscreen></iframe>

## How to Add Videos

1. Click the 🎥 button in the toolbar
2. Enter a video URL (YouTube, Vimeo, etc.)
3. The video will be embedded responsively

**Supported platforms:**
- YouTube
- Vimeo
- (See whitelist in iframe.ts)


**Security:** Only HTTPS URLs from whitelisted domains are allowed.`,
		editable: true
	}
};

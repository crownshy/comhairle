import type { Meta, StoryObj } from '@storybook/svelte';
import ContentRenderer from '$lib/components/RichTextEditor/ContentRenderer/ContentRenderer.svelte';

const meta = {
	title: 'Components/ContentRenderer',
	component: ContentRenderer as any,
	tags: ['autodocs'],
	argTypes: {
		content: {
			control: 'text',
			description: 'Content as Markdown or ProseMirror JSON string',
		},
		class: {
			control: 'text',
			description: 'Additional CSS classes'
		}
	},
	parameters: {
		docs: {
			description: {
				component: `
**Read-Only Content Renderer**

A read-only renderer for rich text content using TipTap. Automatically detects and renders both Markdown and ProseMirror JSON formats.

**Features:**
- Auto-detects Markdown vs JSON format
- Full markdown support (headings, lists, links, images)
- Video/iframe embedding (YouTube, Vimeo)
- XSS protection on embedded content
- Consistent rendering with RichTextEditor
- Clean, prose-styled output
- Links open in new tabs with security attributes

**Format Support:**
- **Markdown:** Traditional markdown syntax
- **JSON:** ProseMirror document structure (saved from RichTextEditor)

**Security:**
- Only HTTPS video embeds allowed
- Domain whitelist for iframes
- Sandbox and referrer policies enforced
				`
			}
		}
	}
} satisfies Meta<any>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
	args: {
		content: `# Welcome

This is a simple markdown renderer example.

You can use **bold** and *italic* text.`
	}
};

// ✨ NEW: Add JSON format example
export const WithJSONContent: Story = {
	args: {
		content: JSON.stringify({
			"type": "doc",
			"content": [
				{
					"type": "heading",
					"attrs": { "level": 1 },
					"content": [{ "type": "text", "text": "Content from JSON" }]
				},
				{
					"type": "paragraph",
					"content": [
						{ "type": "text", "text": "This content is loaded from " },
						{ "type": "text", "marks": [{ "type": "bold" }], "text": "ProseMirror JSON" },
						{ "type": "text", "text": " format, which is how RichTextEditor saves content." }
					]
				},
				{
					"type": "bulletList",
					"content": [
						{
							"type": "listItem",
							"content": [
								{
									"type": "paragraph",
									"content": [{ "type": "text", "text": "Preserves formatting perfectly" }]
								}
							]
						},
						{
							"type": "listItem",
							"content": [
								{
									"type": "paragraph",
									"content": [{ "type": "text", "text": "More reliable than markdown" }]
								}
							]
						}
					]
				}
			]
		})
	}
};

export const CompleteExample: Story = {
	args: {
		content: `# Participatory Democracy Guide

Welcome to our community engagement platform! This guide will help you understand how to participate effectively.

## What is Participatory Democracy?

Participatory democracy is a process that emphasizes **broad participation** in the direction and operation of political systems. It strives to create opportunities for all members of a community to meaningfully contribute to decision-making.

### Key Principles

1. **Transparency** - All processes are open and clear
2. **Inclusivity** - Everyone's voice matters
3. **Deliberation** - Thoughtful discussion leads to better decisions

## Getting Started

To begin participating, follow these steps:

- Read the community guidelines
- Join ongoing conversations
- Share your perspectives respectfully
- Vote on proposals that matter to you

> "The only way to make democracy work is to make it everybody's work." - Community proverb

### Resources

For more information, visit:
- [Community Guidelines](https://example.com/guidelines)
- [How to Contribute](https://example.com/contribute)
- [FAQ](https://example.com/faq)

## Code of Conduct

Remember to always:
- Be respectful and kind
- Listen actively to others
- Assume good intentions
- Focus on ideas, not personalities

---

*Thank you for being part of our community!*`
	}
};

export const WithLists: Story = {
	args: {
		content: `# Meeting Agenda

## Unordered List

- Review last week's decisions
- Discuss new proposals
  - Budget allocation
  - Community events
  - Infrastructure improvements
- Set next meeting date

## Ordered List

1. Call to order
2. Review minutes
3. Old business
4. New business
5. Public comments
6. Adjournment`
	}
};

export const WithQuotes: Story = {
	args: {
		content: `# Community Feedback

## Recent Comments

> This platform has transformed how we engage with local government. I feel heard for the first time in years.
> 
> — Maria, Community Member

> The deliberation process helped us find common ground on issues where we initially disagreed strongly.
> 
> — James, Neighborhood Representative

> Being able to vote on proposals directly gives me confidence that my voice matters.
> 
> — Sarah, Local Resident`
	}
};

export const WithImages: Story = {
	args: {
		content: `# Project Showcase

Our community has accomplished amazing things together.

![Community Garden](https://images.unsplash.com/photo-1530836369250-ef72a3f5cda8?w=800&h=400&fit=crop)

*The new community garden, built with volunteer labor*

## Next Steps

We're planning more collaborative projects. Stay tuned!`
	}
};

export const WithVideo: Story = {
	args: {
		content: `# Town Hall Recording

Watch our latest community meeting:

<iframe src="https://www.youtube.com/embed/dQw4w9WgXcQ" frameborder="0" allowfullscreen></iframe>

## Discussion Topics

- Budget review
- New community center proposal  
- Parks and recreation updates

*All town halls are recorded and available for viewing*

## Supported Video Platforms

- **YouTube** - Paste embed URLs
- **Vimeo** - Paste player URLs

**Security Note:** Only HTTPS URLs from whitelisted domains are allowed.`
	}
};

export const BlockedIframe: Story = {
	args: {
		content: `# Security Test

		//TODO: add blocked iframe

This iframe should be blocked due to untrusted domain:

<iframe src="https://untrusted-domain.com/embed" frameborder="0"></iframe>

You should see a placeholder message instead of the iframe.`
	}
};

export const WithCode: Story = {
	args: {
		content: `# Technical Documentation

## Inline Code

To install the CLI tool, run \`npm install -g comhairle-cli\` in your terminal.

## Configuration

Here's a sample configuration:

\`\`\`json
{
  "name": "My Community",
  "language": "en",
  "features": ["voting", "discussion", "proposals"]
}
\`\`\`

The \`features\` array determines which tools are available.`
	}
};


export const CustomClass: Story = {
	args: {
		content: `# Styled Content

This renderer accepts custom classes for styling.`,
		class: 'p-8 bg-gray-50 rounded-lg border border-gray-200'
	}
};

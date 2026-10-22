import { describe, it, expect } from 'vitest';
import type { EmailTypeSchema } from '@crownshy/api-client/api';
import {
	applyPreviewConversation,
	buildSlotPayload,
	fillKnownVariables,
	fillSlotVariables,
	initialSlotValues,
	markVariables,
	revealVariables,
	richTextToHtml
} from './emailTemplateForm';

const SCHEMA = {
	email_type: 'event_reminder',
	default_subject: 'Upcoming event reminder',
	template: 'event_reminder.html',
	variables: ['event_name'],
	slots: [
		{
			key: 'heading',
			label: 'Heading',
			hint: '',
			default_content: 'Reminder!',
			content_type: 'plain_text'
		},
		{
			key: 'body',
			label: 'Body',
			hint: '',
			default_content: '<p>Starting soon.</p>',
			content_type: 'rich_text'
		}
	]
} as EmailTypeSchema;

describe('richTextToHtml', () => {
	it('passes HTML that was never edited through unchanged', () => {
		expect(richTextToHtml('<p>Hello <strong>there</strong></p>')).toBe(
			'<p>Hello <strong>there</strong></p>'
		);
	});
});

describe('initialSlotValues', () => {
	it('uses the schema defaults for a new version', () => {
		expect(initialSlotValues(SCHEMA)).toEqual({
			heading: 'Reminder!',
			body: '<p>Starting soon.</p>'
		});
	});

	it('prefers saved content, falling back per slot', () => {
		expect(initialSlotValues(SCHEMA, { type: 'event_reminder', heading: 'Hi' })).toEqual({
			heading: 'Hi',
			body: '<p>Starting soon.</p>'
		});
	});
});

describe('buildSlotPayload', () => {
	it('keeps untouched rich text instead of emptying it', () => {
		expect(buildSlotPayload(SCHEMA, initialSlotValues(SCHEMA))).toEqual({
			heading: 'Reminder!',
			body: '<p>Starting soon.</p>'
		});
	});

	it('only sends slots the schema defines', () => {
		expect(
			buildSlotPayload(SCHEMA, { heading: 'Hi', body: '<p>x</p>', stray: 'nope' })
		).toEqual({ heading: 'Hi', body: '<p>x</p>' });
	});
});

describe('markVariables and revealVariables', () => {
	it('round-trips a variable into a highlighted placeholder', () => {
		const marked = markVariables({ intro: '<p>Join {{ conversation_title }} today</p>' });

		expect(marked.intro).toBe('<p>Join [[variable:conversation_title]] today</p>');
		expect(revealVariables(marked.intro)).toContain('>{{conversation_title}}</mark>');
	});

	it('leaves text without variables alone', () => {
		expect(markVariables({ heading: 'Hello' })).toEqual({ heading: 'Hello' });
		expect(revealVariables('<p>Hello</p>')).toBe('<p>Hello</p>');
	});
});

describe('fillKnownVariables', () => {
	it('fills in known variables and leaves the rest', () => {
		expect(
			fillKnownVariables('{{conversation_title}} at {{ event_time }}', {
				conversation_title: 'Demo conversation'
			})
		).toBe('Demo conversation at {{ event_time }}');
	});

	it('escapes values for rich text', () => {
		expect(fillKnownVariables('<p>{{title}}</p>', { title: 'Parks & <paths>' }, true)).toBe(
			'<p>Parks &amp; &lt;paths&gt;</p>'
		);
	});
});

describe('fillSlotVariables', () => {
	it('escapes rich text slots but not plain text ones', () => {
		const filled = fillSlotVariables(
			SCHEMA,
			{ heading: 'Hi {{event_name}}', body: '<p>{{event_name}}</p>' },
			{ event_name: 'Q&A' }
		);

		expect(filled).toEqual({ heading: 'Hi Q&A', body: '<p>Q&amp;A</p>' });
	});
});

describe('applyPreviewConversation', () => {
	const EMAIL =
		'<img class="hero" data-preview="conversation-image" src="https://x.org/sample.png" alt="">' +
		'<h1 class="invite-title" data-preview="conversation-title" style="font-size: 32px">Sample</h1>';

	it('swaps in the chosen conversation title and banner', () => {
		const html = applyPreviewConversation(EMAIL, {
			title: 'Parks & paths',
			imageUrl: 'https://cdn.example.org/banner.jpg'
		});

		expect(html).toContain('style="font-size: 32px">Parks &amp; paths</h1>');
		expect(html).toContain('src="https://cdn.example.org/banner.jpg"');
		expect(html).not.toContain('sample.png');
	});

	it('keeps the example banner when the image is not https', () => {
		const html = applyPreviewConversation(EMAIL, {
			title: 'X',
			imageUrl: 'javascript:alert(1)'
		});

		expect(html).toContain('sample.png');
	});

	it('leaves the email alone with no conversation chosen', () => {
		expect(applyPreviewConversation(EMAIL, null)).toBe(EMAIL);
	});
});

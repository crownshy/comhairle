import { describe, expect, it, vi } from 'vitest';
import { render } from 'svelte/server';
import type { AudioRecordingDto } from '@crownshy/api-client/api';
import AgendaEditor from './AgendaEditor.svelte';
import FacilitatorRoleList from './FacilitatorRoleList.svelte';
import EventBreakoutRooms from './EventBreakoutRooms.svelte';
import EventRecordings from './EventRecordings.svelte';
import type { AgendaItemData } from './agenda-types';

vi.mock('@crownshy/api-client/client', () => ({ apiClient: {} }));
vi.mock('$app/navigation', () => ({ invalidate: vi.fn() }));

const agenda: AgendaItemData[] = [
	{ id: 'standard', type: 'standard', title: 'Welcome' },
	{
		id: 'breakout',
		type: 'breakout',
		title: 'Discussion',
		duration: 10,
		groupSize: 4,
		assignmentMode: 'balanced',
		balanceBy: ['Age'],
		prompts: [{ title: 'Priorities', instructions: 'Discuss local priorities.' }]
	}
];

const attendees = [
	{ id: 'attendance', userId: 'user', email: 'reader@example.com', role: 'facilitator' }
];

const recording: AudioRecordingDto = {
	id: '00000000-0000-4000-8000-000000000001',
	eventId: '00000000-0000-4000-8000-000000000002',
	name: 'Main room',
	fileExtension: 'mp3',
	s3KeyPrefix: 'recordings/main',
	status: 'transcription_failed',
	createdAt: '2026-10-05T10:00:00Z',
	updatedAt: '2026-10-05T10:00:00Z'
};

describe('Event read-only controls', () => {
	it('shows agenda details without add, remove, drag, or rich text editing controls', () => {
		const { body } = render(AgendaEditor, {
			props: { items: agenda, onUpdate: vi.fn(), editable: false }
		});

		expect(body).toContain('Welcome');
		expect(body).toContain('Priorities');
		expect(body).toContain('Discuss local priorities.');
		expect(body).not.toContain('<button');
		expect(body).not.toContain('draggable="true"');
		expect(body).not.toContain('contenteditable="true"');
		const fields = body.match(/<(?:input|select)\b[^>]*>/g) ?? [];
		expect(fields.length).toBeGreaterThan(0);
		expect(fields.every((field) => /\bdisabled\b/.test(field))).toBe(true);
	});

	it('keeps agenda creation available to editors', () => {
		const { body } = render(AgendaEditor, {
			props: { items: [], onUpdate: vi.fn(), editable: true }
		});

		expect(body).toContain('Add agenda item');
		expect(body).toContain('Add breakout session');
	});

	it('shows the current facilitator role without role buttons for readers', () => {
		const { body } = render(FacilitatorRoleList, {
			props: { attendees, pendingInvites: [], onSetRole: vi.fn(), editable: false }
		});

		expect(body).toContain('reader@example.com');
		expect(body).toContain('Facilitator');
		expect(body).not.toContain('<button');
	});

	it('keeps role assignment controls available to editors', () => {
		const { body } = render(FacilitatorRoleList, {
			props: { attendees, pendingInvites: [], onSetRole: vi.fn(), editable: true }
		});

		expect(body.match(/<button\b/g)).toHaveLength(3);
	});

	it('does not render breakout assignment or room editing controls for readers', () => {
		const { body } = render(EventBreakoutRooms, {
			props: {
				conversation_id: 'conversation',
				event_id: 'event',
				attendees: [],
				editable: false
			}
		});

		expect(body).toContain('Breakout rooms');
		expect(body).not.toContain('Auto-assign');
		expect(body).not.toContain('Edit rooms');
		expect(body).not.toContain('<button');
	});

	it('shows recording status and refresh without upload, retry, or deletion controls', () => {
		const { body } = render(EventRecordings, {
			props: {
				conversation_id: 'conversation',
				event_id: 'event',
				recordings: [recording],
				editable: false
			}
		});

		expect(body).toContain('Main room');
		expect(body).toContain('Transcription failed');
		expect(body).toContain('Refresh');
		expect(body).not.toContain('Retry');
		expect(body).not.toContain('Delete recording');
		expect(body).not.toContain('Upload');
		expect(body).not.toContain('type="file"');
	});

	it('keeps recording upload, retry, and deletion available to editors', () => {
		const { body } = render(EventRecordings, {
			props: {
				conversation_id: 'conversation',
				event_id: 'event',
				recordings: [recording],
				editable: true
			}
		});

		expect(body).toContain('Retry');
		expect(body).toContain('Delete recording');
		expect(body).toContain('Upload');
		expect(body).toContain('type="file"');
	});
});

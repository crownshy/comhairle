import { describe, expect, it, vi } from 'vitest';
import { render } from 'svelte/server';
import type { AudioRecordingDto } from '@crownshy/api-client/api';
import AgendaEditor from './AgendaEditor.svelte';
import FacilitatorRoleList from './FacilitatorRoleList.svelte';
import EventBreakoutRooms from './EventBreakoutRooms.svelte';
import EventRecordings from './EventRecordings.svelte';
import type { AgendaDraftItem } from '$lib/utils/eventAgenda';

vi.mock('@crownshy/api-client/client', () => ({ apiClient: {} }));
vi.mock('$app/navigation', () => ({ invalidate: vi.fn() }));

const agenda: AgendaDraftItem[] = [
	{ id: 'plenary', type: 'plenary', title: 'Welcome', durationMinutes: 10 },
	{
		id: 'breakout',
		type: 'breakout',
		prompt: 'Priorities',
		instructions: 'Discuss local priorities.',
		durationMinutes: 20,
		maxPerRoom: 4
	}
];

const agendaProps = {
	startTime: '2026-11-20T14:00:00Z',
	onChange: vi.fn(),
	saveStatus: 'idle' as const
};

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
	it('shows the run of show without add, reorder, or delete controls', () => {
		const { body } = render(AgendaEditor, {
			props: { ...agendaProps, items: agenda, editable: false }
		});

		expect(body).toContain('Welcome');
		expect(body).toContain('Priorities');
		expect(body).toContain('30 min');
		expect(body).not.toContain('Drag to reorder');
		expect(body).not.toContain('Options for');
		expect(body).not.toContain('Agenda item');
		expect(body).not.toContain('contenteditable="true"');
	});

	it('keeps agenda editing available to editors', () => {
		const { body } = render(AgendaEditor, {
			props: { ...agendaProps, items: agenda, editable: true }
		});

		expect(body).toContain('Drag to reorder Welcome');
		expect(body).toContain('Options for Priorities');
		expect(body).toContain('Agenda item');
		expect(body).toContain('Breakout session');
	});

	it('explains an empty agenda', () => {
		const { body } = render(AgendaEditor, {
			props: { ...agendaProps, items: [], editable: true }
		});

		expect(body).toContain('No agenda yet');
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

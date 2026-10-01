import { describe, expect, it, vi } from 'vitest';
import type { UserActions } from '@crownshy/api-client/api';
import {
	canPerformAction,
	createPermissions,
	loadUserActions,
	SYSTEM_RESOURCE_ID
} from './permissions';

const editor: UserActions = {
	resourceType: 'conversation',
	resourceId: '00000000-0000-0000-0000-000000000001',
	actions: ['conversation_read', 'conversation_update']
};

describe('permissions', () => {
	it('fails closed when actions are missing', () => {
		expect(canPerformAction(null, 'conversation_update')).toBe(false);
		expect(canPerformAction(undefined, 'conversation_read')).toBe(false);
		expect(createPermissions(() => []).can('system', 'conversation_create')).toBe(false);
	});

	it('uses returned actions rather than role names', () => {
		expect(canPerformAction(editor, 'conversation_update')).toBe(true);
		expect(canPerformAction(editor, 'conversation_launch')).toBe(false);
		expect(canPerformAction(editor, 'conversation_delete')).toBe(false);
	});

	it('requires export permission independently of edit permission', () => {
		const exporter: UserActions = {
			...editor,
			actions: ['conversation_read', 'conversation_export']
		};
		const editorPermissions = createPermissions(() => [editor]);
		const exporterPermissions = createPermissions(() => [exporter]);

		expect(
			editorPermissions.can('conversation', 'conversation_export', editor.resourceId)
		).toBe(false);
		expect(
			exporterPermissions.can('conversation', 'conversation_export', exporter.resourceId)
		).toBe(true);
		expect(
			exporterPermissions.can('conversation', 'conversation_update', exporter.resourceId)
		).toBe(false);
		expect(
			exporterPermissions.can('conversation', 'conversation_export', SYSTEM_RESOURCE_ID)
		).toBe(false);
	});

	it('does not reuse permissions from a different resource or resource type', () => {
		const permissions = createPermissions(() => [editor]);
		expect(permissions.can('conversation', 'conversation_update', editor.resourceId)).toBe(
			true
		);
		expect(permissions.can('conversation', 'conversation_update', SYSTEM_RESOURCE_ID)).toBe(
			false
		);
		expect(permissions.can('organization', 'organization_update', editor.resourceId)).toBe(
			false
		);
	});

	it('reads updated resources instead of capturing stale actions', () => {
		let resources: UserActions[] = [editor];
		const permissions = createPermissions(() => resources);
		expect(permissions.can('conversation', 'conversation_update', editor.resourceId)).toBe(
			true
		);
		resources = [];
		expect(permissions.can('conversation', 'conversation_update', editor.resourceId)).toBe(
			false
		);
	});

	it('loads the requested resource', async () => {
		const api = { GetUserActions: vi.fn().mockResolvedValue(editor) };
		expect(await loadUserActions(api, 'conversation', editor.resourceId)).toEqual(editor);
		expect(api.GetUserActions).toHaveBeenCalledWith({
			params: { resource_type: 'conversation', resource_id: editor.resourceId }
		});
	});

	it('fails closed when loading fails or returns another resource', async () => {
		const api = { GetUserActions: vi.fn().mockRejectedValue(new Error('Forbidden')) };
		expect(await loadUserActions(api, 'system')).toBeNull();
		api.GetUserActions.mockResolvedValue(editor);
		expect(await loadUserActions(api, 'system')).toBeNull();
	});
});

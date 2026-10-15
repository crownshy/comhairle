import { describe, expect, it, vi } from 'vitest';
import { latestConversations, loadAdminConversations } from './adminConversations';

const conversation = (id: string, createdAt: string) => ({ id, createdAt });
const ids = (conversations: { id: string }[]) => conversations.map(({ id }) => id);

describe('latestConversations', () => {
	it('merges owned and shared, newest first, up to the count', () => {
		const owned = [
			conversation('owned-new', '2026-10-03'),
			conversation('owned-old', '2026-01-01')
		];
		const shared = [
			conversation('shared-mid', '2026-06-01'),
			conversation('shared-newest', '2026-10-10')
		];

		expect(ids(latestConversations({ owned, shared }, 3))).toEqual([
			'shared-newest',
			'owned-new',
			'shared-mid'
		]);
	});
});

describe('loadAdminConversations', () => {
	const page = (records: { id: string }[]) => Promise.resolve({ records, total: records.length });

	function apiReturning(owned: { id: string }[], permitted: { id: string }[]) {
		return {
			GetOwnedConversations: vi.fn(() => page(owned)),
			GetPermittedConversations: vi.fn(() => page(permitted))
		};
	}

	it('drops conversations you own from the shared list', async () => {
		const api = apiReturning([{ id: 'mine' }], [{ id: 'mine' }, { id: 'theirs' }]);

		// @ts-expect-error the stub only returns the fields this function reads
		const result = await loadAdminConversations(api);

		expect(ids(result.owned)).toEqual(['mine']);
		expect(ids(result.shared)).toEqual(['theirs']);
	});

	it('passes the limit to both endpoints', async () => {
		const api = apiReturning([], []);

		// @ts-expect-error the stub only returns the fields this function reads
		await loadAdminConversations(api, 5);

		expect(api.GetOwnedConversations).toHaveBeenCalledWith({ queries: { limit: 5 } });
		expect(api.GetPermittedConversations).toHaveBeenCalledWith({ queries: { limit: 5 } });
	});

	it('treats a failed endpoint as empty', async () => {
		const api = {
			GetOwnedConversations: vi.fn(() => Promise.reject(new Error('down'))),
			GetPermittedConversations: vi.fn(() => page([{ id: 'theirs' }]))
		};

		// @ts-expect-error the stub only returns the fields this function reads
		const result = await loadAdminConversations(api);

		expect(result.owned).toEqual([]);
		expect(ids(result.shared)).toEqual(['theirs']);
	});
});

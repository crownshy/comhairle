// @vitest-environment jsdom
import { afterEach, describe, expect, it, vi } from 'vitest';
import { flushSync, mount, unmount } from 'svelte';
import QuestionBankTab from './QuestionBankTab.svelte';
import { blankQuestion, type SavedQuestion } from './demographicModel';

let component: ReturnType<typeof mount> | undefined;

afterEach(async () => {
	if (component) await unmount(component);
	component = undefined;
	document.body.replaceChildren();
});

function renderBank(canEdit = true) {
	const questions: SavedQuestion[] = [
		{ ...blankQuestion(), id: 'default', text: 'Default age question', isDefault: true },
		{ ...blankQuestion(), id: 'owned', text: 'Organisation age question' }
	];
	const onEdit = vi.fn();
	const onDuplicate = vi.fn();
	component = mount(QuestionBankTab, {
		target: document.body,
		props: { questions, canEdit, onEdit, onDuplicate }
	});
	flushSync();
	return { onEdit, onDuplicate };
}

describe('question bank default actions', () => {
	it('replaces Edit with a Default marker and keeps duplication available', () => {
		const { onEdit, onDuplicate } = renderBank();
		const row = document.querySelector('[data-row-id="default"]');
		const buttons = [...(row?.querySelectorAll('button') ?? [])];

		expect(row?.textContent).toContain('Default');
		expect(buttons.some((button) => button.textContent?.trim() === 'Edit')).toBe(false);
		const duplicate = row?.querySelector<HTMLButtonElement>(
			'button[aria-label="Duplicate Default age question"]'
		);
		expect(duplicate).not.toBeNull();
		duplicate?.click();
		expect(onDuplicate).toHaveBeenCalledWith('default');
		expect(onEdit).not.toHaveBeenCalled();
	});

	it('keeps Edit and Duplicate actions for organisation-owned questions', () => {
		const { onEdit, onDuplicate } = renderBank();
		const row = document.querySelector('[data-row-id="owned"]');
		const edit = [...(row?.querySelectorAll('button') ?? [])].find(
			(button) => button.textContent?.trim() === 'Edit'
		);
		expect(edit).toBeDefined();
		edit?.click();
		row?.querySelector<HTMLButtonElement>(
			'button[aria-label="Duplicate Organisation age question"]'
		)?.click();
		expect(onEdit).toHaveBeenCalledWith('owned');
		expect(onDuplicate).toHaveBeenCalledWith('owned');
	});

	it('does not expose editing or duplication for read-only users', () => {
		renderBank(false);
		expect(document.querySelector('[data-row-id="default"] button')).toBeNull();
		expect(document.querySelector('[data-row-id="owned"] button')).toBeNull();
	});
});

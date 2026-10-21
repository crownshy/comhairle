import { generateValue, testWithRefresh } from '..';
import type { Page } from '../types';
import type { Refs } from './types';
import { expect, type Locator } from '@playwright/test';
import Design from '../navigation/Design';

type MenuItemName = 'Delete' | 'Move up' | 'Move down' | 'Rename';
const MenuItem = (page: Page, name: MenuItemName) => page.getByRole('menuitem', { name });
const MenuButton = (card: Locator) => card.getByRole('button', { name: 'Step actions' });
const NO_STEPS = 'No steps yet. Add your first';

type DraggableListItem<T> = {
	id: T;
	name: string;
};

type DraggableListItemComponent = {
	reorder: (movement: 'up' | 'down') => Promise<void>;
	menu: () => {
		rename: (newName?: string) => Promise<void>;
		delete: () => Promise<void>;
		reorder: (movement: 'up' | 'down') => Promise<void>;
	};
};

async function openBoard(page: Page) {
	const board = page.getByRole('heading', { name: 'Process steps' });
	if (!(await board.isVisible())) {
		await page
			.getByRole('list', { name: 'Conversation sections' })
			.getByRole('link', { name: /^Process design/ })
			.click();
	}
	// Steps stream in after the page renders, so wait for the board to settle either way.
	await expect(Design.stepCards(page).first().or(page.getByText(NO_STEPS))).toBeVisible();
}

async function deleteAllSteps(page: Page) {
	await openBoard(page);
	const cards = Design.stepCards(page);
	for (let remaining = await cards.count(); remaining > 0; remaining--) {
		// Arriving from a step page refreshes the board once more, which can close a menu
		// that opened too early, so retry opening it.
		await expect(async () => {
			await page.keyboard.press('Escape');
			await MenuButton(cards.first()).click();
			await MenuItem(page, 'Delete').click({ timeout: 2000 });
		}).toPass();
		await expect(cards).toHaveCount(remaining - 1);
	}
}

/**
 * Tracks the steps a test adds, in board order, and checks the board and the workflow step
 * strip against it. The strip renders from the server, so it only changes once a save lands.
 */
const DraggableList = async <const T extends string>(refs: Refs) => {
	const { page } = refs;
	const list: DraggableListItem<T>[] = [];

	await deleteAllSteps(page);
	refs.cleanup?.(() => deleteAllSteps(page));

	async function expectOrder() {
		const names = list.map((item) => item.name);
		const cards = Design.stepCards(page);
		await expect(cards).toHaveCount(names.length);
		for (const [index, name] of names.entries()) {
			await expect(cards.nth(index).getByRole('link', { name, exact: true })).toBeVisible();
		}
		await expect(Design.stepTabs(page)).toHaveText(['Design', ...names]);
	}

	async function add(
		id: T,
		from: Parameters<typeof Design.addStep>[1],
		type: Parameters<typeof Design.addStep>[2]
	) {
		const name = await Design.addStep(page, from, type);
		list.push({ id, name });
		await expectOrder();
	}

	function indexOf(id: T) {
		const index = list.findIndex((item) => item.id === id);
		if (index < 0) {
			throw new Error(`Cannot find item "${id}" in ${JSON.stringify(list)}`);
		}
		return index;
	}

	async function move(id: T, movement: 'up' | 'down', click: (card: Locator) => Promise<void>) {
		const from = indexOf(id);
		const to = movement === 'up' ? from - 1 : from + 1;
		await click(Design.stepCards(page).nth(from));
		[list[from], list[to]] = [list[to], list[from]];
		await expectOrder();
	}

	async function get(id: T): Promise<DraggableListItemComponent> {
		indexOf(id);
		const card = () => Design.stepCards(page).nth(indexOf(id));

		return {
			reorder: (movement) =>
				move(id, movement, (c) =>
					c
						.getByRole('button', {
							name: movement === 'up' ? 'Move step up' : 'Move step down'
						})
						.click()
				),
			menu: () => ({
				rename: async (newName = generateValue()) => {
					await MenuButton(card()).click();
					await MenuItem(page, 'Rename').click();
					const nameField = card().getByRole('textbox');
					await nameField.fill(newName);
					// The field commits on blur too, so if a board refresh has already blurred it,
					// the name is saved and this Enter is a no-op.
					await page.keyboard.press('Enter');
					list[indexOf(id)].name = newName;
					await expectOrder();
				},
				delete: async () => {
					await MenuButton(card()).click();
					await MenuItem(page, 'Delete').click();
					list.splice(indexOf(id), 1);
					await expectOrder();
				},
				reorder: (movement) =>
					move(id, movement, async (c) => {
						await MenuButton(c).click();
						await MenuItem(page, movement === 'up' ? 'Move up' : 'Move down').click();
					})
			})
		};
	}

	async function expected() {
		if (list.length === 0) {
			await expect(page.getByText(NO_STEPS)).toBeVisible();
			return;
		}
		await expectOrder();
	}

	return {
		add,
		expect: () => testWithRefresh(page, expected),
		get
	};
};

export default DraggableList;

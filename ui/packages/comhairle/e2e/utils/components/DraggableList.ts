import { generateValue, sleep, testWithRefresh } from '..';
import type { Page } from '../types';
import type { Refs } from './types';
import { expect, type Locator } from '@playwright/test';
import Textboxes from './inputs/Textboxes';
import Design from '../navigation/Design';

const MoveUpBtn = (locator: Locator) => locator.getByLabel('Move step up');
const MoveDownBtn = (locator: Locator) => locator.getByLabel('Move step down');
const Menu = (page: Page) => page.getByRole('button', { name: 'Step actions' });

type MenuItemName = 'Delete' | 'Move up' | 'Move down' | 'Rename' | 'Learn more';
const MenuItem = (page: Page, name: MenuItemName) => page.getByRole('menuitem', { name });

type DraggableListItem<T> = {
	id: T;
	name: string;
	position: number;
};

type DraggableListItemComponent<T> = DraggableListItem<T> & {
	reorder: (movement: 'up' | 'down') => Promise<void>;
	menu: () => {
		rename: (newName?: string) => Promise<void>;
		delete: () => Promise<void>;
		reorder: (movement: 'up' | 'down') => Promise<void>;
	};
};

const DraggableList = async <const T extends string>(refs: Refs) => {
	const list: DraggableListItem<T>[] = [];

	refs.cleanup(async () => {
		const count = await Menu(refs.page).count();
		for (let i = 0; i < count; i++) {
			const MenuBtn = Menu(refs.page).first();
			if (!(await MenuBtn.isVisible())) {
				continue;
			}
			await MenuBtn.click();
			const DeleteBtn = MenuItem(refs.page, 'Delete');
			if (!(await DeleteBtn.isVisible())) {
				continue;
			}
			await DeleteBtn.click();
			await sleep(0.5);
		}
	});

	type params = Parameters<typeof Design.addStep>;
	async function add(id: T, from: params[1], type: params[2]) {
		const stepName = await Design.addStep(refs.page, from, type);
		list.push({ id, name: stepName, position: list.length + 1 });
	}

	async function reorder(
		locator: Locator,
		listItem: (typeof list)[number],
		movement: 'up' | 'down'
	) {
		switch (movement) {
			case 'up': {
				await MoveUpBtn(locator).click();
				const targetPosition = listItem.position - 1;
				const targetItem = list.find((l) => l.position === targetPosition);
				if (!targetItem) {
					return;
				}
				listItem.position -= 1;
				targetItem.position += 1;
				break;
			}
			case 'down': {
				await MoveDownBtn(locator).click();
				const targetPosition = listItem.position + 1;
				const targetItem = list.find((l) => l.position === targetPosition);
				if (!targetItem) {
					return;
				}
				listItem.position += 1;
				targetItem.position -= 1;
				break;
			}
		}
		await sleep(1);
	}

	async function get(id: T): Promise<DraggableListItemComponent<T>> {
		const itemIndex = list.findIndex((l) => l.id === id);
		const listItem = list[itemIndex];
		if (!listItem) {
			throw new Error(`Cannot find item! List: ${list}, id: ${id}`);
		}

		const locator = refs.page.getByRole('listitem').filter({
			hasText: `${listItem.position} ${listItem.name}`
		});
		if (!(await locator.isVisible())) {
			throw new Error(`Locator not found: "${listItem.position} ${listItem.name}"`);
		}

		return {
			...listItem,
			reorder: (movement) => reorder(locator, listItem, movement),
			menu: () => {
				const MenuBtn = Menu(refs.page).nth(listItem.position - 1);

				return {
					rename: async (newName = generateValue()) => {
						await MenuBtn.click();
						await MenuItem(refs.page, 'Rename').click();
						const NewNameTextbox = refs.page.getByRole('textbox');
						listItem.name = newName;

						await Textboxes.write(NewNameTextbox, newName);
						await NewNameTextbox.press('Enter');
						await sleep(1);
					},
					delete: async () => {
						await MenuBtn.click();
						await MenuItem(refs.page, 'Delete').click();
						list.splice(itemIndex, 1);
					},
					reorder: (movement) => reorder(locator, listItem, movement)
				};
			}
		};
	}

	async function expected() {
		if (list.length === 0) {
			expect(refs.page.getByText('No steps yet. Add your first')).toBeVisible();
			return;
		}
		for (const item of list) {
			const count = await refs.page
				.getByRole('listitem')
				.filter({ hasText: item.name })
				.count();
			expect(count).toBe(2);
		}
	}

	return {
		add,
		expect: () => testWithRefresh(refs.page, () => expected()),
		get
	};
};

export default DraggableList;

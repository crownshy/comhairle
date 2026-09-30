import { generateValue, sleep, testWithRefresh } from '..';
import type { Page } from '../types';
import type { Refs } from './types';
import { expect, type Locator } from '@playwright/test';

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
	expect: {
		toBeAbleToMoveUpwards: (bool: boolean) => Promise<void>;
		toBeAbleToMoveDownwards: (bool: boolean) => Promise<void>;
		toNotHaveMovementButtons: () => Promise<void>;
	};
	reorder: (movement: 'up' | 'down') => Promise<void>;
	menu: () => {
		rename: (newName?: string) => Promise<void>;
		delete: () => Promise<void>;
		reorder: (movement: 'up' | 'down') => Promise<void>;
		expect: {
			toBeAbleToMoveUpwards: (bool: boolean) => Promise<void>;
			toBeAbleToMoveDownwards: (bool: boolean) => Promise<void>;
		};
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

	// "tab" = Add button in the tab bar
	// "new" = Add button that shows up when there are no steps currently
	// "additional" = Add button that shows up after steps have already been added
	async function add(
		id: T,
		from: 'tab' | 'new' | 'additional',
		type:
			| 'Learn'
			| 'Poll'
			| 'Survey'
			| 'Individual view exploration'
			| 'Prioritisation'
			| 'Elicitation'
			| 'Lived experience'
	) {
		let name;
		let stepName;
		switch (type) {
			case 'Learn':
				name = 'Topic onboarding Present';
				stepName = 'New Learn Step';
				break;
			case 'Poll':
				name = 'Participant-led poll Show';
				stepName = 'New Polis Step';
				break;
			case 'Survey':
				name = 'Survey Ask participants a';
				stepName = 'New Survey Step';
				break;
			case 'Individual view exploration':
				name = 'Individual view exploration';
				stepName = 'Thinking Space';
				break;
			case 'Prioritisation':
				name = 'Proposal prioritisation';
				stepName = 'Rate the proposals';
				break;
			case 'Elicitation':
				name = 'Elicitation Bot Help';
				stepName = 'What do you think?';
				break;
			case 'Lived experience':
				name = 'Lived Experience Let users';
				stepName = 'New Lived Experience Step';
				break;
			default:
				throw new Error(`Incorrect type: ${type}`);
		}

		let AddBtn;
		switch (from) {
			case 'tab':
				AddBtn = refs.page
					.getByRole('navigation', { name: 'Workflow steps' })
					.getByRole('button', { name: 'Add step' });
				break;
			case 'new':
			case 'additional':
				AddBtn = refs.page.getByRole('button', { name: 'Add step' }).nth(1);
				break;
			default:
				throw new Error(`Incorrect from: ${from}`);
		}

		await AddBtn.click();
		await refs.page.getByRole('button', { name }).click();
		await refs.page.getByRole('button', { name: '+ Add this step' }).click();
		await sleep(1.5);

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
			expect: {
				toBeAbleToMoveUpwards: async (bool) =>
					bool
						? await expect(MoveUpBtn(locator)).not.toBeDisabled()
						: await expect(MoveUpBtn(locator)).toBeDisabled(),
				toBeAbleToMoveDownwards: async (bool) =>
					bool
						? await expect(MoveDownBtn(locator)).not.toBeDisabled()
						: await expect(MoveDownBtn(locator)).toBeDisabled(),
				toNotHaveMovementButtons: async () => {
					await expect(MoveUpBtn(locator)).not.toBeVisible();
					await expect(MoveDownBtn(locator)).toBeVisible();
				}
			},
			menu: () => {
				const MenuBtn = Menu(refs.page).nth(listItem.position - 1);

				return {
					rename: async (newName = generateValue()) => {
						await MenuBtn.click();
						await MenuItem(refs.page, 'Rename').click();
						const NewNameTextbox = refs.page.getByRole('textbox');
						listItem.name = newName;

						await NewNameTextbox.click();
						await NewNameTextbox.fill(newName);
						await NewNameTextbox.press('Enter');
						await sleep(1);
					},
					delete: async () => {
						await MenuBtn.click();
						await MenuItem(refs.page, 'Delete').click();
						list.splice(itemIndex, 1);
					},
					reorder: (movement) => reorder(locator, listItem, movement),
					expect: {
						toBeAbleToMoveUpwards: async (bool) => {
							await MenuBtn.click();
							const MoveUpOption = MenuItem(refs.page, 'Move up');
							return bool
								? await expect(MoveUpOption).not.toBeDisabled()
								: await expect(MoveUpOption).toBeDisabled();
						},
						toBeAbleToMoveDownwards: async (bool) => {
							await MenuBtn.click();
							const MoveDownOption = MenuItem(refs.page, 'Move down');
							return bool
								? await expect(MoveDownOption).not.toBeDisabled()
								: await expect(MoveDownOption).toBeDisabled();
						}
					}
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

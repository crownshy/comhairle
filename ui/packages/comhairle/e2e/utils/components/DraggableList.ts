import { generateValue, sleep } from '..';
import type { Page } from '../types';
import { expect, type Locator } from '@playwright/test';
import { Refs } from './types';

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
	menu: () => {
		rename: (newName?: string) => Promise<void>;
		delete: () => Promise<void>;
		expect: {
			toBeAbleToMoveUpwards: (bool: boolean) => Promise<void>;
			toBeAbleToMoveDownwards: (bool: boolean) => Promise<void>;
		};
	};
};

const DraggableList = async <const T extends string>(refs: Refs) => {
	const list: DraggableListItem<T>[] = [];

	refs.cleanup(async () => {
		for (let i = 0; i < (await Menu(refs.page).count()); i++) {
			await Menu(refs.page).first().click();
			await MenuItem(refs.page, 'Delete').click();
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
		switch (type) {
			case 'Learn':
				name = 'Topic onboarding Present';
				break;
			case 'Poll':
				name = 'Participant-led poll Show';
				break;
			case 'Survey':
				name = 'Survey Ask participants a';
				break;
			case 'Individual view exploration':
				name = 'Individual view exploration';
				break;
			case 'Prioritisation':
				name = 'Proposal prioritisation';
				break;
			case 'Elicitation':
				name = 'Elicitation Bot Help';
				break;
			case 'Lived experience':
				name = 'Lived Experience Let users';
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
		await sleep(1);

		// list.push({ id, name: generateValue(), position: list.length + 1 });
		//
		// const item = get(list.length - 1);
		//
		// await item.menu().rename(item.name);
	}

	function get(index: number): DraggableListItemComponent<T> {
		const listItem = list[index];
		if (!listItem) {
			throw new Error(`Out of bounds index! List: ${list}, index: ${index}`);
		}

		const locator = refs.page.getByRole('listitem').filter({
			hasText: `${index} ${listItem.name}`
		});
		if (!locator.isVisible()) {
			throw new Error(`Locator not found: "${index} ${listItem.name}"`);
		}

		return {
			...listItem,
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
				const MenuBtn = Menu(refs.page).nth(index);

				return {
					rename: async (newName = generateValue()) => {
						await MenuBtn.click();
						await MenuItem(refs.page, 'Rename').click();
						const NewNameTextbox = refs.page.getByRole('textbox');
						listItem.name = newName;

						await NewNameTextbox.click();
						await NewNameTextbox.fill(newName);
						await NewNameTextbox.press('Enter');
					},
					delete: async () => {
						await MenuBtn.click();
						await MenuItem(refs.page, 'Delete').click();
					},
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

	const expected = {
		toBeEmpty: async () => {
			const count = await refs.page.getByRole('listitem').count();
			expect(count).toBe(0);
		},
		toHaveLength: async (length: number) => {
			const count = await refs.page.getByRole('listitem').count();
			expect(count).toBe(length);
		},
		toInclude: async (id: T) => {
			const count = await refs.page
				.getByRole('listitem')
				.filter({ hasText: list.find((l) => l.id === id)?.name })
				.count();
			expect(count).toBe(2);
		}
	} as const;

	return {
		add,
		expect: expected,
		get,
		count: async () => (await refs.page.getByRole('listitem').count()) / 2
	};
};

export default DraggableList;

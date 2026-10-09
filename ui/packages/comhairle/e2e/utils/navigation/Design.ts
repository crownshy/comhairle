import { sleep } from '..';
import type { Page } from '../types';

type DesignDefaultStepNames =
	| 'New Learn Step'
	| 'New Polis Step'
	| 'New Survey Step'
	| 'Thinking Space'
	| 'Rate the proposals'
	| 'What do you think?'
	| 'New Lived Experience Step';

const Design = {
	// "tab" = Add button in the tab bar
	// "new" = Add button that shows up when there are no steps currently
	// "additional" = Add button that shows up after steps have already been added
	addStep: async (
		page: Page,
		from: 'tab' | 'new' | 'additional',
		type:
			| 'Learn'
			| 'Poll'
			| 'Survey'
			| 'Individual view exploration'
			| 'Prioritisation'
			| 'Elicitation'
			| 'Lived experience'
	): Promise<DesignDefaultStepNames> => {
		let name;
		let stepName: DesignDefaultStepNames;
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
				AddBtn = page
					.getByRole('navigation', { name: 'Workflow steps' })
					.getByRole('button', { name: 'Add step' });
				break;
			case 'new':
			case 'additional':
				AddBtn = page.getByRole('button', { name: 'Add step' }).nth(1);
				break;
			default:
				throw new Error(`Incorrect from: ${from}`);
		}

		await AddBtn.click();
		await page.getByRole('button', { name }).click();
		await page.getByRole('button', { name: '+ Add this step' }).click();
		await sleep(1.5);

		return stepName;
	},
	openStep: async (page: Page, stepName: DesignDefaultStepNames) =>
		page
			.getByRole('list', {
				description:
					'Tab to one the items and press space-bar or enter to start dragging it',
				exact: true
			})
			.getByRole('link', { name: stepName })
			.first()
			.click(),
	subtab: async (page: Page, name: 'Configure') => {
		const links = page.getByRole('link', { name });
		const count = await links.count();

		switch (count) {
			case 0:
				throw new Error(`Link doesn't exist! ${links}`);
			case 1:
				await links.click();
				break;
			default:
				await links.nth(count - 1).click();
		}
	}
};

export default Design;

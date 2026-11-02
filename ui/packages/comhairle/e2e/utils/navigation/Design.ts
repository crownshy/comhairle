import { expect } from '@playwright/test';
import type { Page } from '../types';

type DesignDefaultStepNames =
	| 'New Learn Step'
	| 'New Polis Step'
	| 'New Survey Step'
	| 'Thinking Space'
	| 'Rate the proposals'
	| 'What do you think?'
	| 'New Lived Experience Step';

type StepType =
	| 'Learn'
	| 'Poll'
	| 'Survey'
	| 'Individual view exploration'
	| 'Prioritisation'
	| 'Elicitation'
	| 'Lived experience';

// The palette button's accessible name starts with the tool name and runs on into its blurb.
const STEP_TYPES: Record<StepType, { paletteButton: string; stepName: DesignDefaultStepNames }> = {
	Learn: { paletteButton: 'Topic onboarding Present', stepName: 'New Learn Step' },
	Poll: { paletteButton: 'Participant-led poll Show', stepName: 'New Polis Step' },
	Survey: { paletteButton: 'Survey Ask participants a', stepName: 'New Survey Step' },
	'Individual view exploration': {
		paletteButton: 'Individual view exploration',
		stepName: 'Thinking Space'
	},
	Prioritisation: { paletteButton: 'Proposal prioritisation', stepName: 'Rate the proposals' },
	Elicitation: { paletteButton: 'Elicitation Bot Help', stepName: 'What do you think?' },
	'Lived experience': {
		paletteButton: 'Lived Experience Let users',
		stepName: 'New Lived Experience Step'
	}
};

const stepTabs = (page: Page) =>
	page.getByRole('navigation', { name: 'Workflow steps' }).getByRole('tab');

/** The step cards on the Process design board, in display order. */
const stepCards = (page: Page) =>
	page
		.getByRole('main')
		.getByRole('listitem')
		.filter({ has: page.getByRole('button', { name: 'Step actions' }) });

const Design = {
	stepTabs,
	stepCards,
	// "tab" = Add button in the workflow step strip
	// "new" = Add button on the board, shown both when it is empty and below existing steps
	addStep: async (
		page: Page,
		from: 'tab' | 'new',
		type: StepType
	): Promise<DesignDefaultStepNames> => {
		const { paletteButton, stepName } = STEP_TYPES[type];
		const stepsBefore = await stepTabs(page).count();

		const addButton =
			from === 'tab'
				? page
						.getByRole('navigation', { name: 'Workflow steps' })
						.getByRole('button', { name: 'Add step' })
				: page.getByRole('main').getByRole('button', { name: 'Add step' }).last();
		await addButton.click();

		const dialog = page.getByRole('dialog', { name: 'Add a step' });
		await dialog.getByRole('button', { name: paletteButton }).click();
		await dialog.getByRole('button', { name: '+ Add this step' }).click();
		// The dialog closes only after the conversation layout reloads, which can take seconds,
		// and then navigates back to the board, which reloads the steps once more. Editing a card
		// during that reload loses the edit, so wait until it has finished.
		await expect(dialog).toBeHidden({ timeout: 15_000 });
		await page.waitForLoadState('networkidle');
		await expect(stepTabs(page)).toHaveCount(stepsBefore + 1);

		return stepName;
	},
	openStep: async (page: Page, stepName: DesignDefaultStepNames) => {
		await stepCards(page).getByRole('link', { name: stepName }).first().click();
		// A step opens on its first sub-tab, so wait for that before switching to another.
		await expect(page).toHaveURL(/\/design\/step\/[^/]+\/[a-z]+$/);
	},
	subtab: async (page: Page, name: 'Configure') => {
		const link = page
			.getByRole('navigation', { name: 'Sub sections' })
			.getByRole('link', { name, exact: true });
		// A fresh Learn step saves its first pages on load, and its leave guard cancels in-app
		// navigation until that save lands (Playwright dismisses the confirm), so retry the click.
		await expect(async () => {
			await link.click();
			await expect(page).toHaveURL(new RegExp(`/${name.toLowerCase()}$`), { timeout: 1000 });
		}).toPass();
	}
};

export default Design;

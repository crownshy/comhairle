import { expect } from '@playwright/test';
import { Calendar, Combobox, Textbox, TimeRangePicker } from '../components';
import type { Page } from '../types';
import { eventually } from '../testing';

type Event = {
	title?: string;
	description?: string;
	capacity?: number;
	signupMode?: 'invite' | 'open';
};

type AddParams = {
	page: Page;
	event?: Event;
	from?: 'bar' | 'page';
	withTests?: boolean;
};

const Events = {
	add: async ({ page, event, from, withTests }: AddParams) => {
		await page
			.getByRole('link', { name: 'Add event' })
			.nth(from === 'bar' ? 0 : 1)
			.click();

		const Title = await Textbox('Title', { page });
		const Description = await Textbox('Description', { page });
		const Capacity = await Textbox('Capacity', { page }, { overrideRole: 'spinbutton' });
		const DefaultTimezone = Combobox('Europe/London', { page });
		const Date = Calendar('Event date', { page });
		const Time = TimeRangePicker({ page });
		// const Facilitators = await Textbox('Facilitators', { page });
		const SaveChanges = page.getByRole('button', { name: 'Save changes' });

		if (withTests) {
			await SaveChanges.click();

			await eventually(async () => {
				await expect(page.getByText('Name must have at least 5')).toBeVisible();
				await expect(page.getByText('Description must have at')).toBeVisible();
				await expect(page.getByText('Capacity must be at least')).toBeVisible();
				await expect(page.getByText('Start date must be today or')).toBeVisible();
				await expect(page.getByText('Invalid time').first()).toBeVisible();
			});
		}

		// Title
		await Title.write(event?.title);

		// Description
		await Description.write(event?.description);

		// Capacity
		await Capacity.write(event?.capacity?.toString() ?? '2');

		// Default time zone
		// await DefaultTimezone.select(event?.timezone);

		// Event date
		await Date.pickTomorrow(DefaultTimezone.value());

		// Time
		await Time.fill('03:00 AM', '04:00 AM');

		// Signup mode
		if (event?.signupMode === 'invite') {
			await page.getByRole('radio', { name: 'Invite' }).click();
		} else {
			await page.getByRole('radio', { name: 'Open' }).click();
		}

		// Facilitators
		// await Facilitators.write();

		await SaveChanges.click();

		if (withTests) {
			await page.waitForURL('**/events');
			await expect(page.getByRole('heading', { name: Title.currentValue() })).toBeVisible();
		}
	}
};

export default Events;

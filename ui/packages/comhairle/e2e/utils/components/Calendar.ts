import { find } from '..';
import type { Refs } from './types';

function Calendar(name: string, refs: Refs) {
	const open = async () => (await find(refs.page.getByRole('button', { name }))).click();

	async function pickTomorrow(timeZone: string) {
		await open();

		const date = new Date();
		const month = date.getMonth();

		// Pick tomorrows date
		date.setDate(date.getDate() + 1);
		const currentMonth = date.getMonth();

		// If we've crossed a month boundary go to the next month
		if (month !== currentMonth) {
			await (await find(refs.page.getByRole('button', { name: 'Next' }))).click();
		}

		let name = new Intl.DateTimeFormat('en-US', {
			weekday: 'long',
			month: 'long',
			day: 'numeric',
			timeZone
		}).format(date);
		// Remove the year and add a "," to match the button name
		name = name.replace(/\s\d{4}$/, ',');
		await (await find(refs.page.getByRole('button', { name }))).click();

		await refs.page.locator('body').press('Escape');
	}

	return {
		open,
		pickTomorrow
	};
}

export default Calendar;

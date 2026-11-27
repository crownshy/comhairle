import { expect } from '@playwright/test';
import type { Refs } from './types';

const NOT_SET = 'Not set';

type DateTime = { date: Date; time: `${number}${number}:${number}${number}` };

const calendarDayName = (date: Date) =>
	new Intl.DateTimeFormat('en-US', {
		weekday: 'long',
		month: 'long',
		day: 'numeric',
		year: 'numeric'
	}).format(date);

const displayName = ({ date, time }: DateTime) =>
	`${new Intl.DateTimeFormat('en-GB', { dateStyle: 'long' }).format(date)} at ${time}`;

function DateTimePicker(label: string, refs: Refs) {
	const field = () => refs.page.getByRole('button', { name: new RegExp(`^(Set )?${label}`) });
	const popover = () => refs.page.getByRole('dialog', { name: label });

	async function open() {
		await expect(async () => {
			await field().click();
			await expect(popover()).toBeVisible({ timeout: 1000 });
		}).toPass();
	}

	async function showMonthOf(date: Date) {
		const day = popover().getByRole('button', { name: calendarDayName(date) });
		await expect(async () => {
			if (!(await day.isVisible())) {
				await popover().getByRole('button', { name: 'Next' }).click();
			}
			await expect(day).toBeVisible({ timeout: 500 });
		}).toPass();
		return day;
	}

	/** Opens the picker and enters a future date and a 24-hour time, without saving. */
	async function enter({ date, time }: DateTime) {
		await open();
		await (await showMonthOf(date)).click();
		await popover().getByRole('spinbutton', { name: /hour/i }).click();
		await refs.page.keyboard.type(time.replace(':', ''));
	}

	const confirmButton = () => popover().getByRole('button', { name: 'Set', exact: true });

	async function set(value: DateTime) {
		await enter(value);
		await confirmButton().click();
		await expect(popover()).toBeHidden();
	}

	async function remove() {
		await refs.page.getByRole('button', { name: `Remove ${label}`, exact: true }).click();
	}

	const toShow = (value: DateTime | null) =>
		value === null
			? expect(field()).toHaveText(NOT_SET)
			: expect(field()).toHaveText(displayName(value));

	refs.cleanup?.(async () => {
		const removeButton = refs.page.getByRole('button', {
			name: `Remove ${label}`,
			exact: true
		});
		if (await removeButton.isVisible()) await removeButton.click();
	});

	return { enter, set, remove, confirmButton, popover, toShow } as const;
}

export default DateTimePicker;

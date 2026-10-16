import { find } from '..';
import type { Refs } from './types';

type Time = `${number}${number}:${number}${number} AM` | `${number}${number}:${number}${number} PM`;

const EMPTY = '––:–– AM to––:–– AM';
const FILLED = /:[\d–]{2}[\s]+(AM|PM)[\s]+to[\d–]{2}:[\d–]{2}[\s]+(AM|PM)/;

function TimeRangePicker(refs: Refs) {
	let text: string | undefined;

	const locator = () =>
		text !== undefined
			? find(refs.page.getByText(text))
			: find(refs.page.getByText(EMPTY).or(refs.page.getByText(FILLED)));

	const focus = async () =>
		(await locator()).click({
			// Click to the left-center of the input so it targets the first subinput
			position: {
				x: 10,
				y: 20
			}
		});

	/**
	 * Converts time into key presses so playwright can input it
	 *
	 * @example
	 * toKeyPresses("03:00 AM") => "0300A"
	 * toKeyPresses("11:45 PM") => "1145P"
	 */
	const toKeyPresses = (time: Time) => time.replace(/[:\s]/g, '').slice(0, -1);

	async function fill(start: Time, end: Time) {
		await focus();
		const body = refs.page.locator('body');

		for (const key of toKeyPresses(start)) {
			await body.press(key);
		}
		await body.press('Tab');

		for (const key of toKeyPresses(end)) {
			await body.press(key);
		}
		await body.press('Tab');

		// For some reason the label seems to truncate the first two numbers, so slice(2)
		text = `${start} to${end}`.slice(2);
	}

	return {
		fill
	};
}

export default TimeRangePicker;

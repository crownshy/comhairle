import type { Refs } from './types';

type Time = `${number}${number}:${number}${number} AM` | `${number}${number}:${number}${number} PM`;

function TimeRangePicker(refs: Refs) {
	let text = '––:–– AM to––:–– AM';

	const locator = () => refs.page.getByText(text);

	const focus = () =>
		locator().click({
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

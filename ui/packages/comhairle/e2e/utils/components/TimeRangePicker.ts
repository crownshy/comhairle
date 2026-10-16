import { find } from '..';
import type { Refs } from './types';

type Time = `${number}${number}:${number}${number} AM` | `${number}${number}:${number}${number} PM`;
type Range = {
	start: Time;
	end: Time;
};

const EMPTY = '––:–– AM to––:–– AM';
const FILLED = /:[\d–]{2}[\s]+(AM|PM)[\s]+to[\d–]{2}:[\d–]{2}[\s]+(AM|PM)/;

// For some reason the label seems to truncate the first two numbers, so slice(2)
const timeToLabel = (start: Time, end: Time) => `${start} to${end}`.slice(2);

function TimeRangePicker(range: Range | undefined, refs: Refs) {
	let label: string | undefined =
		range !== undefined ? timeToLabel(range.start, range.end) : undefined;

	const locator = () =>
		label !== undefined
			? find(refs.page.getByText(label))
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

		label = timeToLabel(start, end);
	}

	async function expected() {
		if (!label) {
			return;
		}
		await find(refs.page.getByText(label));
	}

	refs.cleanup?.(async () => {
		if (range) {
			await fill(range.start, range.end);
		}
	});

	return {
		fill,
		expect: expected
	};
}

export default TimeRangePicker;

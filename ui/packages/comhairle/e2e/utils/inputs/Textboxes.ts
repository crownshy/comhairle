import { expect, type Locator } from '@playwright/test';
import { Page } from '../types';
import { generateValue } from '..';

type Textbox = {
	id: string;
	locator: Locator;
	value: string;
};

class Textboxes<const T extends string> {
	#textboxes: Record<string, Textbox> = {};

	constructor(page: Page, inputs: [id: T, name: string][]) {
		for (const [id, name] of inputs) {
			this.#textboxes[id] = {
				id,
				locator: page.getByRole('textbox', { name, exact: true }),
				value: generateValue()
			};
		}
	}

	get(id: T): Textbox {
		return this.#textboxes[id];
	}

	// Function overload
	async write(id: T): Promise<void>;
	async write(
		id: T,
		cleanup: (callback: () => Promise<void>) => void,
		defaultValue: string
	): Promise<void>;

	async write(id: T, cleanup?: (callback: () => Promise<void>) => void, defaultValue?: string) {
		await this.#textboxes[id].locator.click();
		await this.#textboxes[id].locator.fill(this.#textboxes[id].value);
		cleanup?.(async () => {
			await this.#textboxes[id].locator.click();
			await this.#textboxes[id].locator.fill(defaultValue ?? '');
		});
	}

	async expected() {
		for (const textbox of Object.values(this.#textboxes)) {
			await expect(textbox.locator).toHaveValue(textbox.value);
		}
	}
}

export default Textboxes;

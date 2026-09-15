import { expect, type Locator } from '@playwright/test';
import { Page } from '../types';

type Textbox = {
	id: string;
	locator: Locator;
	value: string;
};

class Textboxes<const T extends string> {
	_textboxes: Record<string, Textbox> = {};

	constructor(page: Page, inputs: [id: T, name: string][]) {
		for (const [id, name] of inputs) {
			this._textboxes[id] = {
				id,
				locator: page.getByRole('textbox', { name, exact: true }),
				value: crypto.randomUUID()
			};
		}
	}

	get(id: T): Textbox {
		return this._textboxes[id];
	}

	// Function overload
	async write(id: T): Promise<void>;
	async write(
		id: T,
		cleanup: (callback: () => Promise<void>) => void,
		defaultValue: string
	): Promise<void>;

	async write(id: T, cleanup?: (callback: () => Promise<void>) => void, defaultValue?: string) {
		await this._textboxes[id].locator.click();
		await this._textboxes[id].locator.fill(this._textboxes[id].value);
		cleanup?.(async () => {
			await this._textboxes[id].locator.click();
			await this._textboxes[id].locator.fill(defaultValue ?? '');
		});
	}

	async expected() {
		for (const textbox of Object.values(this._textboxes)) {
			await expect(textbox.locator).toHaveValue(textbox.value);
		}
	}
}

export default Textboxes;

import { expect } from '@playwright/test';
import type { Page } from '../types';
import { exists, generateValue } from '..';

type CollapsibleRichField = {
	id: string;
	name: string;
	fallbackIndex: number;
	value: string;
};

class CollapisbleRichFields<const T extends string> {
	#collapisbleRichFields: Record<string, CollapsibleRichField> = {};
	#page: Page;

	constructor(page: Page, inputs: [id: T, name: string][]) {
		this.#page = page;

		let i = 0;

		for (const [id, name] of inputs) {
			this.#collapisbleRichFields[id] = {
				id,
				name,
				fallbackIndex: i,
				value: generateValue()
			};

			i += 1;
		}
	}

	get(id: T): CollapsibleRichField {
		return this.#collapisbleRichFields[id];
	}

	async #edit(id: T) {
		const btn = this.#page.getByRole('button', { name: this.#collapisbleRichFields[id].name });
		if (await exists(btn)) {
			btn.click();
			return;
		}
		const editWithValue = this.#page.getByRole('button', {
			name: this.#collapisbleRichFields[id].value
		});
		if (await exists(editWithValue)) {
			editWithValue.click();
			return;
		}
		await this.#page
			.getByRole('button', { name: 'Edit' })
			.nth(this.#collapisbleRichFields[id].fallbackIndex)
			.click();
	}

	async #write(index: number, text: string) {
		await this.#page.locator('.tiptap').nth(index).click();
		await this.#page.locator('.tiptap').nth(index).fill(text);
		await this.#page.getByRole('button', { name: 'Done' }).click();
	}

	// Function overload
	async write(id: T): Promise<void>;
	async write(
		id: T,
		cleanup: (callback: () => Promise<void>) => void,
		defaultValue: string
	): Promise<void>;

	async write(id: T, cleanup?: (callback: () => Promise<void>) => void, defaultValue?: string) {
		await this.#edit(id);
		await this.#write(
			this.#collapisbleRichFields[id].fallbackIndex,
			this.#collapisbleRichFields[id].value
		);
		cleanup?.(async () => {
			await this.#edit(id);
			await this.#write(this.#collapisbleRichFields[id].fallbackIndex, defaultValue ?? '');
		});
	}

	async expected() {
		for (const collapisbleRichField of Object.values(this.#collapisbleRichFields)) {
			expect(
				await exists(this.#page.getByRole('button', { name: collapisbleRichField.value }))
			).toBe(true);
		}
	}
}

export default CollapisbleRichFields;

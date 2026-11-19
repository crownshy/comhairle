import { find } from '..';
import type { Page } from '../types';

type Options = {
	index?: number;
};

function Button(name: string, page: Page, options?: Options) {
	return find(page.getByRole('button', { name }).nth(options?.index ?? 0));
}

export default Button;

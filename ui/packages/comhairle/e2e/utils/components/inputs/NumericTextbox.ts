import type { Refs } from '../types';
import Textbox from './Textbox';

type Options = Omit<Parameters<typeof Textbox>[2], 'overrideRole' | 'resetTo'> & {
	resetTo?: number;
};

async function NumericTextbox(name: string, refs: Refs, options?: Options) {
	const Txtbox = await Textbox(name, refs, {
		overrideRole: 'spinbutton',
		resetTo: options?.resetTo?.toString()
	});

	return {
		...Txtbox,
		write: (value: number) => Txtbox.write(value.toString())
	};
}

export default NumericTextbox;

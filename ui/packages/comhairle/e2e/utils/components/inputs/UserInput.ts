import { generateValue } from '../..';

type UserInputReturn = {
	currentValue: string;
	write: (value?: string) => Promise<void>;
	expect: () => Promise<void>;
};

type UserInputParams = {
	originalValue?: string;
	focus: (currentValue: string) => Promise<void>;
	update: (value: string) => Promise<void>;
	expector: (currentValue: string) => Promise<unknown>;
};

const UserInput = async ({
	focus,
	update,
	expector
}: UserInputParams): Promise<UserInputReturn> => {
	let currentValue: string = '';

	return {
		get currentValue() {
			return currentValue;
		},
		async write(value = generateValue()) {
			currentValue = value;
			await focus(currentValue);
			await update(currentValue);
		},
		async expect() {
			await expector(currentValue);
		}
	};
};

export default UserInput;

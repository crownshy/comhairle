import { generateValue } from '../..';

type UserInputReturn = {
	value: () => string;
	write: (value?: string) => Promise<void>;
	expect: () => Promise<void>;
};

type UserInputParams = {
	originalValue?: string;
	focus: () => Promise<void>;
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
		value() {
			return currentValue;
		},
		async write(value = generateValue()) {
			currentValue = value;
			await focus();
			await update(currentValue);
		},
		async expect() {
			await expector(currentValue);
		}
	};
};

export default UserInput;

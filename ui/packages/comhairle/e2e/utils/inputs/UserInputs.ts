import { generateValue } from '..';

type InputType<T> = { id: T; value: string };

type UserInputsReturn<T, U> = {
	get: (id: T) => U;
	write: (
		id: T,
		writer: (input: U) => Promise<void>,
		cleanup?: (input: U) => void
	) => Promise<void>;
	expect: (checker: (input: U) => Promise<void>) => Promise<void>;
};

export type DerivedUserInputsReturn<T, U> = {
	get: (id: T) => U;
	write: (
		id: T,
		cleanupRef?: (callback: () => Promise<void>) => void,
		defaultValue?: string
	) => Promise<void>;
	expect: () => Promise<void>;
};

export type UserInputsInput<T> = [id: T, name: string][];

const UserInputs = <const T extends string, U extends InputType<T>>(
	inputs: UserInputsInput<T>,
	mutator: (name: string, index: number) => Omit<U, 'id' | 'value'>
): UserInputsReturn<T, U> => {
	const _inputs = Object.fromEntries(
		inputs.map((input, i) => [
			input[0],
			{
				id: input[0],
				value: generateValue(),
				...mutator(input[1], i)
			}
		])
	) as Record<T, U>;

	return {
		get(id) {
			return _inputs[id];
		},
		async write(id, writer, cleanup) {
			const input = _inputs[id];
			await writer(input);
			cleanup?.(input);
		},
		async expect(checker) {
			for (const input of Object.values(_inputs) as U[]) {
				await checker(input);
			}
		}
	};
};

export default UserInputs;

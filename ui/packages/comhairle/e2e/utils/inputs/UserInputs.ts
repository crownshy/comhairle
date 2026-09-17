import { generateValue } from '..';

type InputType<T> = { id: T; value: string };

type UserInputsReturn<T, U> = {
	get: (id: T) => U;
	write: (id: T, resetTo: string) => Promise<void>;
	expect: () => Promise<void>;
};

type UserInputsParams<T, U> = {
	inputs: UserInputsInput<T>;
	mutator: (name: string, index: number) => Omit<U, 'id' | 'value'>;
	cleanup: (callback: () => Promise<void>) => void;
	focus: (input: U) => Promise<void>;
	update: (input: U, value: string) => Promise<void>;
	expector: (input: U) => Promise<void>;
};

export type UserInputsInput<T> = [id: T, name: string][];

const UserInputs = <const T extends string, U extends InputType<T>>({
	inputs,
	mutator,
	cleanup,
	focus,
	update,
	expector
}: UserInputsParams<T, U>): UserInputsReturn<T, U> => {
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
		async write(id, resetTo) {
			const input = _inputs[id];
			await focus(input);
			await update(input, input.value);
			cleanup(async () => {
				await focus(input);
				await update(input, resetTo);
			});
		},
		async expect() {
			for (const input of Object.values(_inputs) as U[]) {
				await expector(input);
			}
		}
	};
};

export default UserInputs;

import type { Schema } from '.';

export function validate(form: FormData, schema: Schema): boolean {
	for (const name of form.keys()) {
		if (!(name in schema)) {
			return false;
		}
		const value = form.get(name)?.toString() ?? '';
		const input = schema[name];

		if (input.required && !!value) {
			return false;
		}

		if (input.minlength && value.length < input.minlength) {
			return false;
		}

		if (input.maxlength && value.length > input.maxlength) {
			return false;
		}

		if (input.min && value.length < Number(input.min)) {
			return false;
		}

		if (input.max && value.length > Number(input.max)) {
			return false;
		}

		if (input.pattern && !new RegExp(input.pattern).test(value)) {
			return false;
		}
	}

	return true;
}

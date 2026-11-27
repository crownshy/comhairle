import Media from './Media.svelte';

function FormDataParser(formData: FormData) {
	function get<T extends string>(key: string): T | undefined {
		const value = formData.get(key);
		if (value === null) return undefined;
		if (value instanceof File) throw new Error('Trying to read a string as a file!');
		return value as T;
	}

	function getAll<T extends string>(key: string): T[] | undefined {
		const value = formData.getAll(key);
		if (value === null) return undefined;
		for (const entry of value) {
			if (entry instanceof File) throw new Error('Trying to read a string as a file!');
		}

		return value as T[];
	}

	function getFiles(key: string): File[] | undefined {
		const value = formData.get(key);
		if (value === null) return undefined;
		if (typeof value === 'string') throw new Error('Trying to read a file as a string!');
		return Media.normalise(value);
	}

	return {
		get,
		getAll,
		getFiles
	};
}

export default FormDataParser;

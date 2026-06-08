export function camelToSnakeCase(str: string) {
	return str.replace(/[A-Z]/g, (letter) => `_${letter.toLowerCase()}`);
}

export function camelToSentenceCase(str: string) {
	return str.replace(/[A-Z]/g, (letter) => ` ${letter.toLowerCase()}`);
}

export function snakeToSentenceCase(str: string) {
	return str
		.replace(/^[-_]*(.)/, (_, c) => c.toUpperCase()) // Initial char (after -/_)
		.replace(/[-_]+(.)/g, (_, c) => ' ' + c.toUpperCase());
}

export function snakeCaseKeys(obj: { [key: string]: any }) {
	const temp: { [key: string]: any } = {};
	for (const [key, value] of Object.entries(obj)) {
		if (value && typeof value === 'object' && !Array.isArray(value)) {
			// If value is an object recursively call function on value
			temp[camelToSnakeCase(key)] = snakeCaseKeys(value);
		} else if (typeof value === 'object' && Array.isArray(value)) {
			// If value is an array of object recursively call function on entries
			temp[camelToSnakeCase(key)] = value.map((item) =>
				item && typeof item === 'object' && !Array.isArray(item)
					? snakeCaseKeys(item)
					: item
			);
		} else {
			temp[camelToSnakeCase(key)] = value;
		}
	}
	return temp;
}

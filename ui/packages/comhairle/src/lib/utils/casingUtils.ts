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

export function snakeToCamel(str: string) {
	return str.toLowerCase().replace(/[-_][a-z]/g, (group) => group.slice(-1).toUpperCase());
}

export function recursiveCaseChangeKeys(
	/* eslint-disable-next-line  @typescript-eslint/no-explicit-any */
	obj: { [key: string]: any },
	casingCb: (str: string) => string
) {
	/* eslint-disable-next-line  @typescript-eslint/no-explicit-any */
	const temp: { [key: string]: any } = {};
	for (const [key, value] of Object.entries(obj)) {
		if (value && typeof value === 'object' && !Array.isArray(value)) {
			// If value is an object recursively call function on value
			temp[casingCb(key)] = recursiveCaseChangeKeys(value, casingCb);
		} else if (typeof value === 'object' && Array.isArray(value)) {
			// If value is an array of object recursively call function on entries
			temp[casingCb(key)] = value.map((item) =>
				item && typeof item === 'object' && !Array.isArray(item)
					? recursiveCaseChangeKeys(item, casingCb)
					: item
			);
		} else {
			temp[casingCb(key)] = value;
		}
	}
	return temp;
}

/*
 * Function to capitlise the first letter of each word
 * e.g. capitalise("hello world") outputs "Hello World"
 */
export function capitalise(str: string): string {
	return str
		.split(' ')
		.map((word) => word[0].toUpperCase() + word.slice(1))
		.join(' ');
}

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

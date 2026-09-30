/** The noun as it reads after `count`: "1 statement", "3 statements". Regular plurals only. */
export function pluralise(count: number, noun: string): string {
	return count === 1 ? noun : `${noun}s`;
}

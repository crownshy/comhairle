/**
 * Function to allow you to get the maximum number from an array of objects
 * Using something like Math.max(...arr.map()); Means you have to loop twice, with this you only loop once
 *
 * @param arr - The array you want to find the maximum in
 * @param extractor - Function that will return the number you want to compare against
 *
 * @example
 * const arr = [{ id: 1, total: 10 }, { id: 2, total: 18 }, { id: 3, total: 5 }];
 * const maximum = max(arr, (value) => value.total); // maximum = 18
 *
 */
export function max<T>(arr: T[], extractor: (value: T) => number): number {
	let max = 0;

	for (let i = 0; i < arr.length; i++) {
		const value = extractor(arr[i]);
		if (value > max) {
			max = value;
		}
	}

	return max;
}

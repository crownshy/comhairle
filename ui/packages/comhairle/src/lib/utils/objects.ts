/**
 * Function to copy an object while omitting keys, named to match the TS type of the same name:
 * https://www.typescriptlang.org/docs/handbook/utility-types.html#omittype-keys
 *
 * obj1 = {
 *		a: 1,
 *		b: 2,
 *		c: 3,
 * };
 *
 * obj2 = omit(obj1, "a", "b");
 * obj2 // { c: 3 };
 */
export const omit = <T extends object>(object: T, ...keys: (keyof T)[]) =>
	Object.fromEntries(Object.entries(object).filter(([key]) => !keys.includes(key as keyof T)));

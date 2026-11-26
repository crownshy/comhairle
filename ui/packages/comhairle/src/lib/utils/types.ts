/**
 * Function to type an object or array strictly without requiring any coercion, narrowing or widening
 * Example:
 * const obj = {
 *		name: typed<InputAttr>({}), // Type safety with input now (with `as` you wouldn't get that)
 *		file: typed<FileAttr>({})
 * }
 */
export function typed<T>(obj: T): T {
	return obj;
}

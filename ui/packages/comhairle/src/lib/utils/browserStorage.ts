// localStorage throws in private browsing or when site data is blocked, so callers get a
// null read or a skipped write instead of an exception.

export function readStorage(key: string): string | null {
	if (typeof window === 'undefined') return null;
	try {
		return localStorage.getItem(key);
	} catch {
		return null;
	}
}

export function writeStorage(key: string, value: string): void {
	if (typeof window === 'undefined') return;
	try {
		localStorage.setItem(key, value);
	} catch {
		// Nothing to do: the value only lasts for this visit.
	}
}

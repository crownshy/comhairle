import { goto } from '$app/navigation';

type Options = {
	onDelete?: () => void;
	onAdd?: () => void;
};

export function updateQueryParam(param: string, value: string, options?: Options) {
	// eslint-disable-next-line svelte/prefer-svelte-reactivity
	const url = new URL(location.href);

	if (url.searchParams.get(param) === value || !value) {
		url.searchParams.delete(param);
		options?.onDelete?.();
	} else {
		url.searchParams.set(param, value);
		options?.onAdd?.();
	}

	// eslint-disable-next-line svelte/no-navigation-without-resolve
	goto(url, { replaceState: true, keepFocus: true });
}

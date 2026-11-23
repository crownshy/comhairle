import { goto } from '$app/navigation';
import { page } from '$app/state';
import type { ConversationCardLayout } from '$lib/components/ConversationCard.svelte';
import type { LocalizedConversationDto } from '@crownshy/api-client/api';

type DefaultParams = Record<string, string | number>;

export type ConversationsParams = {
	page: number;
	search: string;
	filter: keyof Pick<
		LocalizedConversationDto,
		'isComplete' | 'isPublic' | 'isInviteOnly' | 'isLive'
	>;
	view: ConversationCardLayout;
};

function refresh(href: string = new URL(page.url).href) {
	// eslint-disable-next-line svelte/no-navigation-without-resolve
	goto(href, { replaceState: true, keepFocus: true, noScroll: true });
}

function get<const T extends DefaultParams>(key: keyof T, url?: URL): string | undefined {
	const Url = url ?? new URL(page.url);
	return Url.searchParams.get(key as string) ?? undefined;
}

function set<const T extends DefaultParams>(key: keyof T, value: T[keyof T], url?: URL) {
	const Url = url ?? new URL(page.url);
	if (!value) {
		Url.searchParams.delete(key as string);
	} else {
		if (typeof value === 'string') {
			Url.searchParams.set(key as string, value);
		} else if (Number.isFinite(value)) {
			Url.searchParams.set(key as string, Math.floor(value).toString());
		}
	}

	return {
		set: <const T extends DefaultParams>(key: keyof T, value: T[keyof T]) =>
			set(key as string, value, Url),
		update: () => refresh(Url.href)
	};
}

function remove<T extends string>(key: T, url?: URL): void {
	const Url = url ?? new URL(page.url);
	Url.searchParams.delete(key);
	refresh(Url.href);
}

const QueryParams = {
	get,
	set,
	remove
};

export default QueryParams;

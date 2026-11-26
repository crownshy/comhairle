import type { ConversationCardLayout } from '$lib/components/ConversationCard.svelte';

export type ConversationsParams = {
	page: number;
	search: string;
	filter: 'open' | 'ended';
	view: ConversationCardLayout;
};

export type MediaLibraryParams = {
	page: number;
	search: string;
	filter: 'audio' | 'image' | 'video';
};

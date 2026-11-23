import type { LocalizedEventDto } from '@crownshy/api-client/api';
import { validateUrl } from '$lib/utils/urlValidation';

type LobbyEvent = Pick<LocalizedEventDto, 'id' | 'format' | 'customEventLink'>;

/** Saved links can predate validation, so anything invalid falls back to the built-in lobby. */
export function lobbyHref(conversationId: string, event: LobbyEvent): string {
	const customLink = event.format === 'online' ? validateUrl(event.customEventLink) : null;
	return customLink ?? `/conversations/${conversationId}/events/${event.id}/live`;
}

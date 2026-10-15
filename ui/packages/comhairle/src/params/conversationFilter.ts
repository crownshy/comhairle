import type { ParamMatcher } from '@sveltejs/kit';
import { isConversationFilterSegment } from '$lib/utils/conversationFilters';

export const match = isConversationFilterSegment satisfies ParamMatcher;

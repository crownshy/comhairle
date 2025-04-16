import { error } from '@sveltejs/kit';
import { defaults, superValidate } from 'sveltekit-superforms';
import { zod } from 'sveltekit-superforms/adapters';
import { z } from 'zod';
import NewConversationSchema  from "./NewConversationSchema"

export const ssr=false;
export const load = async () => {

	const conversationData = defaults(zod(NewConversationSchema));

	const form = await superValidate(conversationData, zod(NewConversationSchema));

	return { form };
};

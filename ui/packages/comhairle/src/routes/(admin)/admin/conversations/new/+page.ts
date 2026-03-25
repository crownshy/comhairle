import { superValidate } from 'sveltekit-superforms';
import { zod } from 'sveltekit-superforms/adapters';
import NewConversationSchema from './NewConversationSchema';

export const ssr = false;
export const load = async () => {
	const form = await superValidate(zod(NewConversationSchema), { errors: false });

	return { form };
};

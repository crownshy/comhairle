import { superValidate } from 'sveltekit-superforms';
import { zod } from 'sveltekit-superforms/adapters';
import NewEventSchema from './NewEventSchema';
import type { PageLoad } from '../$types';

export const load: PageLoad = async ({ parent }) => {
	const { conversation } = await parent();

	const form = await superValidate(zod(NewEventSchema));

	return { form, conversation };
};

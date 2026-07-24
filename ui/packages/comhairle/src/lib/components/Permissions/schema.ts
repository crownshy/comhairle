import { z } from 'zod';
import * as m from '$lib/paraglide/messages';

export const userEmailPermissionsForm = z.object({
	email: z.string().email(m.please_enter_a_valid_email())
});

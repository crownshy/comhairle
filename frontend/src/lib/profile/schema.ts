import { z } from 'zod';
import * as m from '$lib/paraglide/messages';

export const loginFormSchema = z.object({
	email: z.string().email(m.please_enter_a_valid_email()),
	password: z.string().min(1, { message: m.please_enter_a_password() })
});
export type LoginForm = z.infer<typeof loginFormSchema>;

export const signupFormSchema = z
	.object({
		email: z.string().email(m.please_enter_a_valid_email()),
		username: z.string().min(3),
		password: z.string().min(1, { message: m.please_enter_a_password() }).min(8),
		password_confirm: z.string().min(1, { message: 'please confirm password' })
	})
	.refine((body) => body.password === body.password_confirm, {
		message: 'passwords must match',
		path: ['password_confirm']
	});
export type SignupForm = z.infer<typeof signupFormSchema>;

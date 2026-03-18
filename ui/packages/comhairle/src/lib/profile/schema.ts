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
		password: z.string().min(1, { message: m.please_enter_a_password() }).min(16, {
			message:
				'Password must be at least 16 characters long and include characters from at least 3 of 4 categories: uppercase letters, lowercase letters, numbers, and special characters'
		}),
		password_confirm: z.string().min(1, { message: 'please confirm password' })
	})
	.refine((body) => body.password === body.password_confirm, {
		message: 'passwords must match',
		path: ['password_confirm']
	});
export type SignupForm = z.infer<typeof signupFormSchema>;

export const annonLoginFormSchema = z.object({
	username: z.string().min(1)
});
export type AnnonLoginFrom = z.infer<typeof annonLoginFormSchema>;

export const passwordResetCreateFormSchema = z.object({
	email: z.string().email(m.please_enter_a_valid_email())
});
export type PasswordResetCreateForm = z.infer<typeof passwordResetCreateFormSchema>;

export const passwordResetUpdateFormSchema = z
	.object({
		password: z.string().min(16, {
			message:
				'Password must be at least 16 characters long and include characters from at least 3 of 4 categories: uppercase letters, lowercase letters, numbers, and special characters'
		}),
		confirmPassword: z.string().min(16, {
			message: 'Password must be at least 16 characters long'
		})
	})
	.superRefine(({ confirmPassword, password }, ctx) => {
		if (confirmPassword !== password) {
			ctx.addIssue({
				code: 'custom',
				message: 'The passwords did not match',
				path: ['confirmPassword']
			});
		}
	});
export type PasswordResetUpdateForm = z.infer<typeof passwordResetUpdateFormSchema>;

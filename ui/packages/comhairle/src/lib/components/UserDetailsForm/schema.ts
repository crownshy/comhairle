import { z } from 'zod';

export const userDetailsSchema = z
	.object({
		username: z.string().min(1, 'Username is required').optional(),
		password: z
			.string()
			.min(
				16,
				'Password must be at least 16 characters long and include characters from at least 3 of 4 categories: uppercase letters, lowercase letters, numbers, and special characters'
			)
			.optional(),
		confirmPassword: z.string().optional()
	})
	.refine(
		(data) => {
			if (data.password && data.password !== data.confirmPassword) {
				return false;
			}
			return true;
		},
		{
			message: "Passwords don't match",
			path: ['confirmPassword']
		}
	);

export type UserDetailsFormData = z.infer<typeof userDetailsSchema>;

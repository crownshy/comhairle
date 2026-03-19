import { z } from 'zod';

export const upgradeAccountSchema = z
	.object({
		username: z.string().min(1, 'Username is required'),
		email: z.string().email('Please enter a valid email address'),
		password: z
			.string()
			.min(
				16,
				'Password must be at least 16 characters long and include characters from at least 3 of 4 categories: uppercase letters, lowercase letters, numbers, and special characters'
			),
		confirmPassword: z.string().min(1, 'Please confirm your password')
	})
	.refine((data) => data.password === data.confirmPassword, {
		message: "Passwords don't match",
		path: ['confirmPassword']
	});

export type UpgradeAccountFormData = z.infer<typeof upgradeAccountSchema>;

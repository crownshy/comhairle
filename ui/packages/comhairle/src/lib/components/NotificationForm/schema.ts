import { z } from 'zod';

export const notificationFormSchema = z
	.object({
		title: z
			.string()
			.min(1, 'Title is required')
			.max(200, 'Title must be less than 200 characters'),
		content: z
			.string()
			.min(1, 'Content is required')
			.max(50000, 'Content must be less than 50,000 characters'),
		delivery_method: z.enum(['in_app', 'email'], {
			errorMap: () => ({
				message: 'Delivery method must be either "in_app" or "email".'
			})
		})
	})
	.superRefine(() => {});

export type NotificationFormData = z.infer<typeof notificationFormSchema>;

import { z } from 'zod';

export const conversationConfigSchema = z.object({
	title: z
		.string()
		.min(1, 'Title is required')
		.max(200, 'Title must be less than 200 characters'),
	shortDescription: z
		.string()
		.min(1, 'Short description is required')
		.max(500, 'Short description must be less than 500 characters'),
	description: z.string().min(1, 'Description is required'),
	imageUrl: z.string().url('Must be a valid URL').or(z.literal('')),
	privacyPolicy: z.string(),
	faqs: z.string(),
	isPublic: z.boolean(),
	isInviteOnly: z.boolean(),
	autoLogin: z.boolean(),
	enableQaChatBot: z.boolean()
});

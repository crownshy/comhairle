import { z } from 'zod';

export const accessSchema = z.object({
	isPublic: z.boolean(),
	isInviteOnly: z.boolean(),
	autoLogin: z.boolean(),
	enableSignupPrompts: z.boolean(),
	showThankYouPageAnnonInstructions: z.boolean(),
	showThankyouPageFeedbackButton: z.boolean(),
	allowRevisitAfterFinishing: z.boolean()
});

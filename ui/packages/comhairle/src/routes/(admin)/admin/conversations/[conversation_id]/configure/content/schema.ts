import { z } from 'zod';

export const contentSchema = z.object({
	privacyPolicy: z.string().nullish(),
	shortPrivacyPolicy: z.string().nullish(),
	faqs: z.string().nullish(),
	thankYouMessage: z.string().nullish(),
	callToAction: z.string().nullish()
});

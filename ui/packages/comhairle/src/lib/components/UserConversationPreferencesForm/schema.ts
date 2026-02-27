import { z } from "zod";

export const userConversationPreferencesSchema = z.object({
	receiveUpdatesByNotification: z.boolean(),
	receiveUpdatesByEmail: z.boolean(),
	receiveSimilarConversationUpdatesByEmail: z.boolean(),
	receiveSimilarConversationUpdatesByNotification: z.boolean(),
});

export type UserConversationPreferencesFormData = z.infer<typeof userConversationPreferencesSchema>;

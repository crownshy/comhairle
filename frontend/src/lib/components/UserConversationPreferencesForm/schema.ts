import { z } from "zod";

export const userConversationPreferencesSchema = z.object({
	receive_updates_by_notification: z.boolean(),
	receive_updates_by_email: z.boolean(),
	receive_similar_conversation_updates_by_email: z.boolean(),
	receive_similar_conversation_updates_by_notification: z.boolean(),
});

export type UserConversationPreferencesFormData = z.infer<typeof userConversationPreferencesSchema>;
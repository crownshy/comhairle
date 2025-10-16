import { z } from "zod";

export const notificationFormSchema = z.object({
	title: z.string().min(1, "Title is required").max(200, "Title must be less than 200 characters"),
	content: z.string().min(1, "Content is required").max(2000, "Content must be less than 2000 characters"),
});

export type NotificationFormData = z.infer<typeof notificationFormSchema>;

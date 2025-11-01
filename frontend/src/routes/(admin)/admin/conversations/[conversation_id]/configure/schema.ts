import { z } from "zod";

export const conversationConfigSchema = z.object({
	title: z.string().min(1, "Title is required").max(200, "Title must be less than 200 characters"),
	short_description: z.string().min(1, "Short description is required").max(500, "Short description must be less than 500 characters"),
	description: z.string().min(1, "Description is required"),
	image_url: z.string().url("Must be a valid URL").or(z.literal("")),
	is_public: z.boolean(),
	is_invite_only: z.boolean(),
	auto_login: z.boolean()
});

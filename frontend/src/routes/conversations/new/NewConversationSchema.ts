import { z } from "zod";


const NewConversationSchema = z.object({
	title: z.string().min(10),
	short_description: z.string().min(20),
	description: z.string().min(50),
	tags: z.array(z.string()),
	image_url: z.string().url(),
	video_url: z.string().url().optional(),
	is_public: z.boolean(),
	is_invite_only: z.boolean()
});

export default NewConversationSchema;

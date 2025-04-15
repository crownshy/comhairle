import { z } from "zod";
import PlaceholderConvo from "$lib/assets/placeholder_convo.png" 

const NewConversationSchema = z.object({
	title: z.string().min(10),
	short_description: z.string().min(20),
});

export default NewConversationSchema;

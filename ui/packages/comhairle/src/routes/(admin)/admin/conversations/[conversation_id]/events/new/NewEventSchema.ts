import { z } from 'zod';

const NewEventSchema = z.object({
	name: z.string().min(5, { message: 'Name must have at least 5 characters.' }),
	description: z.string().min(20, { message: 'Description must have at least 20 characters.' }),
	capacity: z.number(),
	start_date: z
		.string()
		.date()
		.refine((val) => new Date(val) > new Date(), {
			message: 'Start date must be in the future.'
		}),
	start_time: z
		.string()
		.time()
		.refine((val) => new Date(val) > new Date(), {
			message: 'Start time must be in the future.'
		}),
	end_time: z
		.string()
		.time()
		.refine((val) => new Date(val) > new Date(), {
			message: 'End time must be in the future.'
		}),
	signup_mode: z.enum(['invite', 'open'], {
		errorMap: () => ({
			message: 'Signup mode must be either "invite" or "open".'
		})
	})
});

export default NewEventSchema;

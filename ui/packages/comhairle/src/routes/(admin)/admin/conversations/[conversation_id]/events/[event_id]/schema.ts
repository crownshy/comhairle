import { z } from 'zod';

const EventSchema = z
	.object({
		name: z.string().min(5, { message: 'Name must have at least 5 characters.' }),
		description: z
			.string()
			.min(20, { message: 'Description must have at least 20 characters.' }),
		capacity: z.number(),
		start_date: z
			.string()
			.date()
			.refine((val) => new Date(val) > new Date(), {
				message: 'Start date must be in the future.'
			}),
		start_time: z.string().time(),
		end_time: z.string().time(),
		signup_mode: z.enum(['invite', 'open'], {
			errorMap: () => ({
				message: 'Signup mode must be either "invite" or "open".'
			})
		})
	})
	.superRefine((data, ctx) => {
		if (data.start_time && data.end_time && data.end_time <= data.start_time) {
			ctx.addIssue({
				code: z.ZodIssueCode.custom,
				path: ['end_time'],
				message: 'End time must be after start time.'
			});
		}
	});

export default EventSchema;

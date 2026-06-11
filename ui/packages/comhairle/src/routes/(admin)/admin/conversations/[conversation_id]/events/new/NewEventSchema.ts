import { z } from 'zod';

const NewEventSchema = z
	.object({
		name: z.string().min(5, { message: 'Name must have at least 5 characters.' }),
		description: z
			.string()
			.min(20, { message: 'Description must have at least 20 characters.' }),
		capacity: z
			.number({ invalid_type_error: 'Capacity is required.' })
			.int({ message: 'Capacity must be a whole number.' })
			.min(2, { message: 'Capacity must be at least 2.' }),
		default_time_zone: z.string().default('UTC'),
		start_date: z
			.string()
			.date()
			.refine((val) => new Date(val) >= new Date(new Date().toISOString().split('T')[0]), {
				message: 'Start date must be today or in the future.'
			}),
		start_time: z.string().time(),
		end_time: z.string().time(),
		signup_mode: z.enum(['invite', 'open'], {
			errorMap: () => ({
				message: 'Signup mode must be either "invite" or "open".'
			})
		}),
		facilitators: z
			.array(z.string().email({ message: 'Each facilitator must be a valid email address.' }))
			.default([])
			.refine((arr) => arr.length >= 1, { message: 'Add at least one facilitator.' })
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

export default NewEventSchema;

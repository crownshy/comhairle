import { z } from 'zod';

const EventSchema = z
	.object({
		name: z.string().min(5, { message: 'Name must have at least 5 characters.' }),
		description: z
			.string()
			.min(20, { message: 'Description must have at least 20 characters.' }),
		capacity: z.number(),
		default_time_zone: z.string(),
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
		format: z.string(),
		custom_event_link: z.string().optional()
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

const EventLocationSchema = z.object({
	location_venue_name: z.string(),
	location_address_line_1: z.string(),
	location_address_line_2: z.string().optional(),
	location_address_line_3: z.string().optional(),
	location_city: z.string(),
	location_state_province: z.string(),
	location_postal_code: z.string(),
	location_country_code: z.string()
});

export { EventLocationSchema };

export default EventSchema;

import { z } from 'zod';

const EventLocationSchema = z.object({
	venue_name: z.string(),
	address_line_1: z.string(),
	address_line_2: z.string().optional(),
	address_line_3: z.string().optional(),
	city: z.string(),
	state_province: z.string(),
	postal_code: z.string(),
	country_code: z.string()
});

export default EventLocationSchema;

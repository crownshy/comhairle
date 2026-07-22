import { z } from 'zod';

const MediaSchema = z.object({
	media: z.instanceof(File, { message: 'Please upload a file.' }),
	name: z.string().min(1, { message: 'Required field' }),
	alt: z.string().min(1, { message: 'Required field' })
});

export default MediaSchema;

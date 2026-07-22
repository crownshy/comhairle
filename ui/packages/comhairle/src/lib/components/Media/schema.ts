import { z } from 'zod';

const MediaSchema = z.object({
	media: z.instanceof(File, { message: 'Please upload a file.' }),
	name: z.string().nonempty(),
	alt: z.string().nonempty()
});

export default MediaSchema;

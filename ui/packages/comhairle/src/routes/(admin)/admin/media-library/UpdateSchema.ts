import type { InputAttr } from '$lib/components/EasyForm';
import { typed } from '$lib/utils/types';

const UpdateSchema = {
	name: typed<InputAttr>({
		name: 'name',
		required: true
	}),
	alt: typed<InputAttr>({
		name: 'alt',
		required: true
	})
} as const;

export default UpdateSchema;

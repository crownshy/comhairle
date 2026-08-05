import type { InputAttr } from '$lib/components/EasyForm';
import { typedObj } from '$lib/utils/types';

const UpdateSchema = {
	name: typedObj<InputAttr>({
		name: 'name',
		required: true
	}),
	alt: typedObj<InputAttr>({
		name: 'alt',
		required: true
	})
} as const;

export default UpdateSchema;

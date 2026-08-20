import { typedObj } from '$lib/utils/types';
import { MB } from '$lib/utils/units';
import { type FileAttr, type InputAttr } from '$lib/components/EasyForm';

export const MediaSchema = {
	media: typedObj<FileAttr>({
		name: 'media',
		required: true,
		accept: '.jpg,.jpeg,.png,.gif,.webp,.mp4,.mpeg,.mpg,.webm,.mp3',
		maxSize: 500 * MB
	}),
	name: typedObj<InputAttr>({
		name: 'name',
		required: true
	}),
	alt: typedObj<InputAttr>({
		name: 'alt',
		required: true
	})
} as const;

export default MediaSchema;

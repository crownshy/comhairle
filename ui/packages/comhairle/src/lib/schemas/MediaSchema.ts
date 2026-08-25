import { typedObj } from '$lib/utils/types';
import { MB } from '$lib/utils/units';
import { type FileAttr, type InputAttr } from '$lib/components/EasyForm';
import Media from '$lib/interfaces/Media';

export const MediaSchema = {
	media: typedObj<FileAttr>({
		name: 'media',
		required: true,
		accept: Media.acceptedExtensions(),
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

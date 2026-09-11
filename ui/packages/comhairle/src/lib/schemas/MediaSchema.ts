import { typed } from '$lib/utils/types';
import { MB } from '$lib/utils/units';
import { type FileAttr, type InputAttr } from '$lib/components/EasyForm';
import Media from '$lib/interfaces/Media';

export const MediaSchema = {
	media: typed<FileAttr>({
		name: 'media',
		required: true,
		accept: Media.acceptedExtensions(),
		maxSize: 500 * MB
	}),
	name: typed<InputAttr>({
		name: 'name',
		required: true
	}),
	alt: typed<InputAttr>({
		name: 'alt',
		required: true
	})
} as const;

export default MediaSchema;

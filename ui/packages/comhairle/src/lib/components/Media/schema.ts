import { typedObj } from '$lib/utils/types';
import { type FileAttr, type InputAttr } from '../EasyForm/types';

export const MediaSchema = {
	media: typedObj<FileAttr>({
		name: 'media',
		required: true,
		accept: '.png,.jpg'
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

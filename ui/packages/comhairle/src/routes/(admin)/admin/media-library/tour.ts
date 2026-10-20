import type { Tour } from '$lib/tours/types';

export const mediaLibraryTour: Tour = {
	id: 'admin-media-library',
	steps: [
		{
			element: '[data-tour="page-header"]',
			title: 'Media library',
			description:
				'Images, audio and video you upload here can be reused across every conversation. Edit lets you select and delete several items at once.',
			side: 'bottom'
		},
		{
			element: '[data-tour="media-filters"]',
			title: 'Filter by type',
			description: 'Narrow the library down to audio, image or video.',
			side: 'bottom'
		}
	]
};

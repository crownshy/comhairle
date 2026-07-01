import { tryFetch } from '$lib/utils/errorHandling';
import { fail } from '@sveltejs/kit';
import type { RequestEvent } from './$types';
import Media from '$lib/interfaces/Media';
import type { MediaDto } from '@crownshy/api-client/api';
import climateUk from '$lib/assets/climageuk.jpg';
import beepBox from '$lib/assets/BeepBox-Song.mp3';
import troublshootingVideo from '$lib/assets/(before)cannot_unlink.mp4';
import vtaiwan from '$lib/assets/vtaiwan.jpg';
import seattleUSA from '$lib/assets/seattle_usa.jpg';
import wavesLogoLg from '$lib/assets/waves-logo-lg.png';
import comhairleLogo from '$lib/assets/comhairle_logo.png';
import placeholderConvo from '$lib/assets/placeholder_convo.png';
import comhairleFullLogo from '$lib/assets/comhairle_full_logo.svg';

type ContentType = MediaDto['contentType'] | 'audio/mp3';
type MockData = Pick<MediaDto, 'id'> & Record<'src', string> & Record<'contentType', ContentType>;
export function load() {
	const media: MockData[] = [
		{ id: '1', src: climateUk, contentType: 'image/jpeg' },
		{ id: '2', src: beepBox, contentType: 'audio/mp3' },
		{ id: '3', src: troublshootingVideo, contentType: 'video/mp4' },
		{ id: '4', src: vtaiwan, contentType: 'image/jpeg' },
		{ id: '5', src: seattleUSA, contentType: 'image/jpeg' },
		{ id: '6', src: wavesLogoLg, contentType: 'image/png' },
		{ id: '7', src: comhairleLogo, contentType: 'image/png' },
		{ id: '8', src: placeholderConvo, contentType: 'image/png' },
		{ id: '9', src: comhairleFullLogo, contentType: 'image/png' }
	];

	return {
		media
	};
}
export const actions = {
	upload: async ({ request, fetch }: RequestEvent) => {
		const data = await request.formData();
		const files = data.getAll('media');
		if (files === null) {
			return fail(422, { failures: ["Couldn't find files"] });
		}

		const media = new Media();
		const responses = await media.upload('/api/media', files as File[], { fetchRef: fetch });

		const errors = responses.filter((r) => r.err !== null);
		if (responses.some((r) => r.err !== null)) {
			return fail(422, { failures: errors.map((e) => e.err.message) });
		}

		return new Response('ok', { status: 201 });
	},
	delete: async ({ request, fetch }: RequestEvent) => {
		const data = await request.formData();
		const media = data.getAll('media') as string[];

		const results: { id: string; request: ReturnType<typeof tryFetch> }[] = [];
		for (const id of media) {
			results.push({
				id,
				request: tryFetch(`/api/media/${id}`, { method: 'DELETE' }, fetch)
			});
		}

		const failures: string[] = [];
		for (const result of results) {
			const response = await result.request;
			if (response.err !== null) {
				failures.push(`${result.id} failed to delete, ${response.err.message}`);
			}
		}

		if (failures.length > 0) {
			return fail(500, { failures });
		}

		return new Response('ok', { status: 200 });
	}
};

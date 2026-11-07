import { HttpStatus } from '$lib/utils/constants';
import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import { resolve } from '$app/paths';

export const load: PageServerLoad = async ({ params }) => {
	redirect(
		HttpStatus.MovedPermanently,
		resolve('/(admin)/admin/demographic-forms/[organisation_id]/forms', {
			organisation_id: params.organisation_id
		})
	);
};

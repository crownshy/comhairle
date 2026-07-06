import { redirect } from '@sveltejs/kit';
import type { PageLoad } from './$types';

// The Your Rights section has no landing screen of its own — always show a
// section. Terms of Service is the default (first in the section nav).
export const load: PageLoad = () => {
	redirect(307, '/rights/tos');
};

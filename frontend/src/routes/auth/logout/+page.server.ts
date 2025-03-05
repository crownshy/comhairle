import { redirect } from '@sveltejs/kit';

export const actions = {
	default: async (evt) => {
		evt.cookies.delete('auth-token', { path: '/' });

		// TODO: post to /api/auth/logout

		return redirect(302, '/');
	}
};

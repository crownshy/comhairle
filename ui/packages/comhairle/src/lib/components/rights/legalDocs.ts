import * as m from '$lib/paraglide/messages';

/** The Your Rights documents a participant can reach without leaving their step. */
export type LegalDocId = 'privacy' | 'tos' | 'cookies';

export type LegalDoc = { id: LegalDocId; label: string; href: string };

export function legalDocs(): LegalDoc[] {
	return [
		{ id: 'privacy', label: m.privacy_policy(), href: '/rights/privacy' },
		{ id: 'tos', label: m.terms_of_service(), href: '/rights/tos' },
		{ id: 'cookies', label: m.cookies_settings(), href: '/rights/cookies' }
	];
}

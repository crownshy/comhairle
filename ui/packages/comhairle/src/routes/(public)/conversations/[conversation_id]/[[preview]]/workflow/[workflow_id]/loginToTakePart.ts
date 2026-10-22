import { loginRedirectServer } from '$lib/urls';

/** Sends a signed-out visitor to log in, then back to the step they opened. */
export function redirectToLoginToTakePart(url: URL) {
	loginRedirectServer(
		url.pathname + url.search,
		'Login or signup to take part in the conversation'
	);
}

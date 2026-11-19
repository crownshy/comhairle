import { loginRedirectServer } from '$lib/urls';
import { pathWithQuery } from '$lib/utils/backTo';

/** Sends a signed-out visitor to log in, then back to the step they opened. */
export function redirectToLoginToTakePart(url: URL) {
	loginRedirectServer(pathWithQuery(url), 'Login or signup to take part in the conversation');
}

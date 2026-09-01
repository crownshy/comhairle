import { env } from '$env/dynamic/public';

/**
 * Base URL for API calls made while rendering on the server.
 *
 * `url.origin` is the site's public address, so leaving it in place sends every call a load
 * function makes out of the cluster and back in through the ingress. Point
 * `PUBLIC_INTERNAL_API_BASE_URL` at the API service's internal address to keep that traffic
 * inside. Include whatever path prefix the API is mounted on, e.g.
 * `http://comhairle-api:3000/api`. Unset falls back to the public origin, which is what
 * local dev wants.
 */
export function serverApiBaseUrl(url: URL): string {
	return env.PUBLIC_INTERNAL_API_BASE_URL || `${url.origin}/api`;
}

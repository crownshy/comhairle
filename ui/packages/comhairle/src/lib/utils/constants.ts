/**
 * Reserved knowledge-base document name for the auto-synced learn-step content.
 * Mirrors `LEARN_CONTENT_DOCUMENT_NAME` in the API (`api/src/routes/documents.rs`).
 */
export const LEARN_CONTENT_DOCUMENT_NAME = 'comhairle_learning_step_material.pdf';

export enum HttpStatus {
	TemporaryRedirect = 307,
	PermanentRedirect = 308,
	BadRequest = 400,
	Unauthoized = 401,
	Forbidden = 403,
	NotFound = 404,
	Conflict = 409,
	UnprocessableContent = 422,
	InternalServerError = 500
}

import { redirect } from '@sveltejs/kit';
import { notifications } from './notifications.svelte';
import { goto } from '$app/navigation';

export function conversation_url(conversation_id: string, isPreview: boolean = false) {
	return `/conversations/${conversation_id}${isPreview ? '/preview' : ''}`;
}

export function manage_conversation_url(conversation_id: string) {
	return `/admin/conversations/${conversation_id}/configure`;
}

export function workflow_url(
	conversation_id: string,
	workflow_id: string,
	isPreview: boolean = false
) {
	return conversation_url(conversation_id, isPreview) + `/workflow/${workflow_id}`;
}

export function next_workflow_step_url(
	conversation_id: string,
	workflow_id: string,
	isPreview: boolean = false
) {
	return workflow_url(conversation_id, workflow_id, isPreview) + `/next`;
}

export function thank_you_page(
	conversation_id: string,
	workflow_id: string,
	isPreview: boolean = false
) {
	return workflow_url(conversation_id, workflow_id, isPreview) + '/thank_you';
}

export function workflow_step_url(
	conversation_id: string,
	workflow_id: string,
	workflow_step_id: string,
	isPreview: boolean = false
) {
	return workflow_url(conversation_id, workflow_id, isPreview) + `/s/${workflow_step_id}`;
}

export function report_url(
	conversation_id: string,
	workflow_id: string,
	isPreview: boolean = false
) {
	return conversation_url(conversation_id, isPreview) + '/report';
}

// Redirect to the login page with a link back to the current
// context.
export function loginRedirect(backTo: string, message?: string) {
	if (message) {
		notifications.addFlash({ message, priority: 'INFO' });
	}
	goto(`/auth/login?backTo=${encodeURIComponent(backTo)}`);
}

// Redirect to the signup page with a link back to the current
// context.
export function signupRedirect(backTo: string, message?: string) {
	if (message) {
		notifications.addFlash({ message, priority: 'INFO' });
	}
	goto(`/auth/signup?backTo=${encodeURIComponent(backTo)}`);
}

// Redirect to the signup annon page with a link back to the current
// context.
export function signupAnnonRedirect(backTo: string, message?: string) {
	if (message) {
		notifications.addFlash({ message, priority: 'INFO' });
	}
	goto(`/auth/anonymous-signup?backTo=${encodeURIComponent(backTo)}`);
}

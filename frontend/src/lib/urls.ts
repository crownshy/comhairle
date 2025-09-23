import { redirect } from "@sveltejs/kit"
import { notifications } from "./notifications.svelte"
import { goto } from "$app/navigation"

export function conversation_url(conversation_id: string) {
	return `/conversations/${conversation_id}`
}

export function manage_conversation_url(conversation_id: string) {
	return `/admin/conversations/${conversation_id}`
}

export function workflow_url(conversation_id: string, workflow_id: string) {
	return conversation_url(conversation_id) + `/workflow/${workflow_id}`
}

export function workflow_step_url(conversation_id: string, workflow_id: string, step: number) {
	return workflow_url(conversation_id, workflow_id) + `/s/${step}`
}

export function report_url(conversation_id: string, workflow_id: string) {
	return conversation_url(conversation_id) + "/report"
}

// Redirect to the login page with a link back to the current 
// context.
export function loginRedirect(backTo: string, message?: string) {
	if (message) {
		notifications.addFlash({ message, priority: "INFO" });
	}
	goto(`/auth/login?backTo=${backTo}`)
}

// Redirect to the signup page with a link back to the current 
// context.
export function signupRedirect(backTo: string, message?: string) {
	if (message) {
		notifications.addFlash({ message, priority: "INFO" });
	}
	goto(`/auth/signup?backTo=${backTo}`)
}

// Redirect to the signup annon page with a link back to the current 
// context.
export function signupAnnonRedirect(backTo: string, message?: string) {
	if (message) {
		notifications.addFlash({ message, priority: "INFO" });
	}
	goto(`/auth/anomyous-signup?backTo=${backTo}`)
}


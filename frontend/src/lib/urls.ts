import { redirect } from "@sveltejs/kit"
import { notifications } from "./notifications.svelte"

export function conversation_url(conversation_id:string){
  return `/conversations/${conversation_id}`
}

export function manage_conversation_url(conversation_id:string){
  return `/admin/conversations/${conversation_id}`
}

export function workflow_url(conversation_id:string, workflow_id:string){
  return conversation_url(conversation_id) + `/workflow/${workflow_id}`
}

export function workflow_step_url(conversation_id:string, workflow_id:string, step:number){
  return workflow_url(conversation_id,workflow_id) + `/s/${step}`
}

export function report_url(conversation_id:string, workflow_id:string){
  return conversation_url(conversation_id) +"/report"
}

export function loginRedirect(backTo: string, message?: string){
  if(message){
    notifications.addFlash({message, priority:"WARNING"});
  }
  redirect(302,`/login?back_to=${JSON.stringify(backTo)}`)
}

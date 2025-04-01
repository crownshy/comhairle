import {env} from "$env/dynamic/public"
import type { Conversation } from "./types"

export async function getConversation( conversation_id:string): Promise<Conversation>{
  let response = await fetch(`${env.PUBLIC_API_SERVER}/conversation/${conversation_id}`);
  let body = await response.json()
  if (response.ok){
    return await body
  } 
  throw Error(body.err)
}

export async function getWorkflow( conversation_id:string, workflow_id:string): Promise<Workflow>{
  let response = await fetch(`${env.PUBLIC_API_SERVER}/conversation/${conversation_id}/workflow/${workflow_id}`) 
  let body = await response.json()
  if (response.ok){
    return await body
  } 
  throw Error(body.err)
}

export async function getWorkflowSteps(conversation_id:string, workflow_id:string): Promise<Array<WorkflowStep>>{
  let response = await fetch(`${env.PUBLIC_API_SERVER}/conversation/${conversation_id}/workflow/${workflow_id}/workflow_step`) 
  let body = await response.json()
  if (response.ok){
    return await body
  } 
  throw Error(body.err)
}


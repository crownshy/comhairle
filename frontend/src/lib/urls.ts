export function conversation_url(conversation_id:string){
  return `/conversations/${conversation_id}`
}

export function workflow_url(conversation_id:string, workflow_id:string){
  return conversation_url(conversation_id) + `/workflow/${workflow_id}`
}

export function workflow_step_url(conversation_id:string, workflow_id:string, step:number){
  return workflow_url(conversation_id,workflow_id) + `/s/${step}`
}

export function report_url(conversation_id:string, workflow_id:string){
  return workflow_url(conversation_id,workflow_id) +"/report"
}

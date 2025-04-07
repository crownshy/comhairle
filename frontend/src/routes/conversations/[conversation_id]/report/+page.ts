import { apiClient } from "$lib/api/client"
import { notifications } from "$lib/notifications.svelte"
import { redirect } from "@sveltejs/kit"
import type {PageLoad} from "./$types"

export const load:PageLoad = async ({params})=>{
  let conversation_id = params.conversation_id

  try{
    let conversation = await apiClient.GetConversation({params:{conversation_id}})
    let workflows = await apiClient.ListWorkflows({params: {conversation_id}})
    let workflow_steps = await apiClient.ListWorkflowSteps({params:{conversation_id, workflow_id: workflows[0].id }})
    let workflow_stats = await apiClient.GetWorkflowStats({params: {conversation_id, workflow_id: workflows[0].id }})
    return {conversation, workflows, workflow_steps, workflow_stats}

  }
  catch(e){
    console.log(e)
    notifications.addFlash({message:"Could not find conversation", priority:"ERROR"})    
    throw redirect(302, "/")
  }

}

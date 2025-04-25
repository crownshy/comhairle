import { notifications } from "$lib/notifications.svelte"
import { redirect } from "@sveltejs/kit"
import type {PageLoad} from "./$types"

export const load:PageLoad = async ({params,parent})=>{
  let {api,conversation, workflows} = await parent();
  let conversation_id = conversation.id

  try{
    let conversation = await api.GetConversation({params:{conversation_id}})
    let workflows = await api.ListWorkflows({params: {conversation_id}})
    let workflow_steps = await api.ListWorkflowSteps({params:{conversation_id, workflow_id: workflows[0].id }})
    let workflow_stats = await api.GetWorkflowStats({params: {conversation_id, workflow_id: workflows[0].id }})
    return {conversation, workflows, workflow_steps, workflow_stats}

  }
  catch(e){
    console.log(e)
    notifications.addFlash({message:"Could not find conversation", priority:"ERROR"})    
    throw redirect(302, "/")
  }

}

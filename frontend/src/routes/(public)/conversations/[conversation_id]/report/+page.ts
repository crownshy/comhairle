import { notifications } from "$lib/notifications.svelte"
import { redirect } from "@sveltejs/kit"
import type {PageLoad} from "./$types"

export const load:PageLoad = async ({params,parent})=>{
  let {api,conversation, workflows} = await parent();
  let conversation_id = conversation.id

  try{
    let report = await api.GetReportForConversation({params: {conversation_id: conversation.id}})
    try{
      
      let workflow_steps = await api.ListWorkflowSteps({params:{conversation_id, workflow_id: workflows[0].id }})
      let workflow_stats = await api.GetWorkflowStats({params: {conversation_id, workflow_id: workflows[0].id }})

      return {conversation, workflows, workflow_steps, workflow_stats,report}
    }
    catch(e){
      notifications.addFlash({message:"Something went wrong", priority:"ERROR"})
    }
  }
  catch(e){
    notifications.addFlash({message:"No such report", priority:"WARNING"})
    redirect(302, "/")
  }
}

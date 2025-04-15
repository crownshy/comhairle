import type {PageLoad} from "./$types"
import {notifications} from "$lib/notifications.svelte"
import { redirect } from "@sveltejs/kit";


export const load :PageLoad = async ({params,parent})=>{
  let conversation_id  = params.conversation_id;
  let {api} = await parent();
  try{
    let conversation = await api.GetConversation({params:{conversation_id}})
    let workflows = await api.ListWorkflows({params:{conversation_id}});
    let stats = undefined;
    let workflow_steps = undefined;

    if(workflows.length >0){
      stats = await api.GetWorkflowStats({params: {conversation_id, workflow_id: workflows[0].id}})
      workflow_steps = await api.ListWorkflowSteps({params:{conversation_id, workflow_id: workflows[0].id}})
    }
    console.log("Retuning ", {conversation,workflows,stats})
    return {conversation, workflows,stats, workflow_steps}  
  }
  catch(e){
    console.log(e)
    notifications.addFlash({message:"No such conversation", priority:"WARNING"})
    redirect(301,"/admin")
  }

}

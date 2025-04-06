import type {PageLoad} from "./$types"
import { notifications } from "$lib/notifications.svelte";
import { redirect } from "@sveltejs/kit";
import {apiClient} from "$lib/api/client"

export const load :PageLoad = async (event)=>{
    let conversation_id = event.params.conversation_id
    try{
      let conversation = await apiClient.GetConversation({params:{conversation_id}})
      let workflows = await apiClient.ListWorkflows({params:{conversation_id}});

      let participation = await apiClient.GetUserParticipation({params:{conversation_id, workflow_id:workflows[0].id }})

      return {conversation,workflows,participation}
    }
    catch(e){
      redirect(307,"/") 
    }
}

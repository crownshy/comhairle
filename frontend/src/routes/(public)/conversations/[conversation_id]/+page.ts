import type {PageLoad} from "./$types"
import { notifications } from "$lib/notifications.svelte";
import { redirect } from "@sveltejs/kit";
import { loginRedirect } from "$lib/urls";

export const load :PageLoad = async (event)=>{
    let {api}= await event.parent() 
    let conversation_id = event.params.conversation_id
    try{
      let conversation = await api.GetConversation({params:{conversation_id}})
      let workflows = await api.ListWorkflows({params:{conversation_id}});

      let participation = await api.GetUserParticipation({params:{conversation_id, workflow_id:workflows[0].id }})

      return {conversation,workflows,participation, api}
    }
    catch(e){
      loginRedirect(`/conversations/${event.params.conversation_id}`,"Login to take part")
      redirect(307,"/") 
    }
}

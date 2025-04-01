import type {PageLoad} from "./$types"
import { notifications } from "$lib/notifications.svelte";
import { redirect } from "@sveltejs/kit";

export const load :PageLoad = async (event)=>{
    let conversation_id = event.params.conversation_id
    let response = await event.fetch(`/api/conversation/${conversation_id}`);    
    if(response.ok){
      let conversation = await response.json()
      let workflows = await event.fetch(`/api/conversation/${conversation_id}/workflow`).then((r)=>r.json())
      return {conversation,workflows}
    }
    redirect(307,"/")
}

import {conversations} from "$lib/mock_data"
import { notifications } from "$lib/notifications.svelte";
import { redirect } from "@sveltejs/kit";

export async function load(event){

  console.log("converation id ", event.params.conversation_id)
  let conversation_id = event.params.conversation_id
    let response = await event.fetch(`/api/conversation/${conversation_id}`);    
    if(response.ok){
      let conversation = await response.json()
      return {conversation}
    }

    redirect(307,"/")
}

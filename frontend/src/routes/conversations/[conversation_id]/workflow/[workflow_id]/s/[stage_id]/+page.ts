import { redirect } from "@sveltejs/kit";
import type { PageLoad } from './$types';

export const load: PageLoad = async (event)=>{
    let conversation_id = event.params.conversation_id
    let workflow_id= event.params.workflow_id
    let step_id = parseInt(event.params.stage_id)
    let response = await event.fetch(`/api/conversation/${conversation_id}`);    

    if(response.ok){
      let conversation = await response.json()
      let workflow_steps = await event.fetch(`/api/conversation/${conversation_id}/workflow/${workflow_id}/workflow_step`).then((r)=>r.json())

      return {conversation,workflow_steps, step:workflow_steps[step_id]}
    }

    redirect(307,"/")
}

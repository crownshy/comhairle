import { loginRedirect } from "$lib/urls";
import type { LayoutLoad} from './$types';

export const load :LayoutLoad =async ({parent, params})=>{
    
    let {api,user}= await parent() 
    let conversation_id = params.conversation_id

    try{
      let conversation = await api.GetConversation({params:{conversation_id}})
      let workflows = await api.ListWorkflows({params:{conversation_id:conversation.id}});

      let participation

      if(user){
        participation = await api.GetUserParticipation({params:{conversation_id: conversation.id, workflow_id:workflows[0].id }})
      }
      else{
        participation = null
      }

      return {conversation,workflows,participation, api}

    }
    catch(e){
      console.log("SOMETHING WENT WRONG")
      console.log(e)
      loginRedirect(`/conversations/${params.conversation_id}`,"Login to take part")
    }
}

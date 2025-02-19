import {conversations} from "$lib/mock_data"

export function load({params}:{params:{conversation_id:string}}){
  let conversation_id = params.conversation_id

  let conversation = conversations.find((convo)=>convo.id === conversation_id)
  return {conversation}
}

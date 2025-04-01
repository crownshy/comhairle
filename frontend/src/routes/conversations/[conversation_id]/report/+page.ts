import type {PageLoad} from "./$types"

export const load:PageLoad = ({params})=>{
  let conversation_id = params.conversation_id

  let conversation = conversations.find((convo)=>convo.id === conversation_id)
  return {conversation}
}

import type { LayoutLoad} from './$types';

export const load: LayoutLoad = async({parent})=>{
  let {api}= await parent();
  console.log("REFETCHING LAyOUT")

  try{
    let conversations = await api.GetOwnedConversations()
    return {conversations}
  }

  catch(e){
    console.log("User unauthorized ", e)
  }

}

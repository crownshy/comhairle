import { notifications } from '$lib/notifications.svelte';
import { redirect } from '@sveltejs/kit';
import type { LayoutLoad} from './$types';

export const load: LayoutLoad = async({parent})=>{
  let {api}= await parent();

  try{
    let conversations = await api.GetOwnedConversations()
    return {conversations}
  }

  catch(e){
    if(e.status===401){
      notifications.addFlash({message:"You are not authorised", priority:"WARNING"})
      redirect(302,"/")      
    }
  }

}

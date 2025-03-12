import {conversations} from "$lib/mock_data"

export async function load(event){
  try{
    let conversations = await event.fetch("/api/conversation").then((r)=>r.json())
    console.log("conversations ",conversations)
    return {conversations}
  }
  catch(e){
    console.log("ERROR")
    console.log(e)
  }
}

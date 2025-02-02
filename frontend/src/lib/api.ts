import {env} from "$env/dynamic/public"
export async function getData(){
  console.log("Getting data from ",env.PUBLIC_API_SERVER + "/data")
  return fetch(env.PUBLIC_API_SERVER + "/data").then((a)=>a.json()) 
}

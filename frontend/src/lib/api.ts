import {PUBLIC_API_SERVER} from "$env/static/public"
export async function getData(){
  console.log("Getting data from ",PUBLIC_API_SERVER + "/data")
  return fetch(PUBLIC_API_SERVER + "/data").then((a)=>a.json()) 
}

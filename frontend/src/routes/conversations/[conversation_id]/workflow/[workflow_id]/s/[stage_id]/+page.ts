import { redirect } from "@sveltejs/kit";
import type { PageLoad } from './$types';
import {apiClient} from "$lib/api/client"
import { notifications} from '$lib/notifications.svelte';
import { report_url, workflow_step_url } from "$lib/urls";

export const load: PageLoad = async (event)=>{
    let {conversation_id, workflow_id, stage_id} = event.params
    try{
       let stepNo = parseInt(stage_id)

       let conversation = await apiClient.GetConversation({params:{conversation_id}}) 
       let workflow_steps = await apiClient.ListWorkflowSteps({params:{conversation_id,workflow_id}})
       let step = workflow_steps.find(ws=>ws.step_order = stepNo);
       let progress = await apiClient.GetUserProgress({params:{conversation_id,workflow_id }})

       let next_undone_step_id = progress.find(p=>p.status !=="done")?.workflow_step_id;
       console.log({progress,workflow_steps})

       //If the current step exists       
       if(step){
         let step_status = progress.find((s)=> s.workflow_step_id===step.id)?.status;
         // but is done 
         if(step_status==="done"){
           console.log("current step is done")
           // and there is a later step to complete
           if(next_undone_step_id){
             // goto that step
             let next_step = workflow_steps.find(ws=>ws.id === next_undone_step_id)!;
             console.log("next undone step is ", next_step)
             console.log("redirecting to ",workflow_step_url(conversation_id,workflow_id, next_step.step_order))
             redirect(300,workflow_step_url(conversation_id,workflow_id, next_step.step_order))
           }
           else{
             console.log("All steps compleated. Going to report")
             // Otherwise go to the report
             redirect(300,report_url(conversation_id,workflow_id))
           }
         }
       }
       // If there current step doesnt exist
       // but there is one to be done. Get the first uncompleated step
       else{
         console.log("failed to find step. Trying to find first not done")
         if(next_undone_step_id){
           let next_step = workflow_steps.find(ws=>ws.id = next_undone_step_id)!;
           redirect(300,workflow_step_url(conversation_id,workflow_id, next_step.step_order))
         }
       }

       return {conversation, workflow_steps, step }
    }
    //TODO figure out how to type this from the generated api
    catch(e:any){
      /// Throw if error is a redirect
      if(e.status === 300){
        throw e 
      }
      //TODO we probably want some error codes to match on here
      // rather than the plain text
      if(e.response.data.err==="User Required for this route"){
        notifications.addFlash({message:"Login or signup to take part in the conversation", priority:"INFO"})
        redirect(307,"/auth/login")
      }
      redirect(307,"/")
    }
}

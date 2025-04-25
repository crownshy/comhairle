import { isRedirect, redirect } from "@sveltejs/kit";
import type { PageLoad } from './$types';
import { notifications} from '$lib/notifications.svelte';
import { report_url, workflow_step_url } from "$lib/urls";

export const load: PageLoad = async (event)=>{
    let {api,conversation, workflows} = await event.parent();
    
    let conversation_id = conversation.id;
    let {workflow_id, stage_id} = event.params
    try{
       let stepNo = parseInt(stage_id)

       let workflow_steps = await api.ListWorkflowSteps({params:{conversation_id,workflow_id}})
       let step = workflow_steps.find(ws=>ws.step_order === stepNo);
       let progress = await api.GetUserProgress({params:{conversation_id,workflow_id }})

       let next_undone_step_id = progress.find(p=>p.status !=="done")?.workflow_step_id;
       let next_step = workflow_steps.find(ws=>ws.id === next_undone_step_id)!;


       //If the current step exists       
       if(step){
         let step_status = progress.find((s)=> s.workflow_step_id===step.id)?.status;
         // but is done 
         if(step_status==="done"){
           // and there is a later step to complete
           if(next_undone_step_id){
             redirect(300,workflow_step_url(conversation_id,workflow_id, next_step.step_order))
           }
           else{
             // Otherwise go to the report
             redirect(300,report_url(conversation_id,workflow_id))
           }
         }
       }
       // If there current step doesnt exist
       // but there is one to be done. Get the first uncompleated step
       else{
         if(next_undone_step_id){
           redirect(300,workflow_step_url(conversation_id,workflow_id, next_step.step_order))
         }
       }

       return {conversation, workflow_steps, step, api}
    }
    //TODO figure out how to type this from the generated api
    catch(e:any){
      /// Throw if error is a redirect
      if(isRedirect(e)){
        console.log(e)
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

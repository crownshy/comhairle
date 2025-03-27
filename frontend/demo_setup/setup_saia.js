(async ()=>{
  let conversatioon = {
      "title" : "How can AI help with Climate Change?",
      "short_description" : "We want your help understanding how AI might be able to help fight climate change",
      "description" : "",
      "image_url" : "https://www.neilsahota.com/wp-content/uploads/2023/08/Climate-Change-AI-1.jpg",
      "tags" : ["climate", "ai"],
      "is_public" : true,
      "is_invite_only" : false,
      "slug" : "climate_and_ai"
  }

  let workflow = {
      "name": "public_workflow",
      "description": "A workflow to gather the publics ideas about how AI could fight climate change",
      "is_active":true,
      "is_public":true
  }

  let workflow_steps = [
    {
      name: "Lean about AI",
      step_order: 1,
      activation_rule:"manual",
      description: "Before you can tell us your ideas lets learn a bit about the core ideas around AI and what climate change is",
      is_offline: false,
      tool_config:{
        "learn":{    
        }
      }
    },
    {
      name: "Tell us about yourself",
      step_order: 2,
      activation_rule:"manual",
      description: "Tell us a bit about yourself! We want to understand where you are comming from and what your past experince with AI is",
      is_offline: false,
      tool_config:{
        "survey":{
          "survey_id": "",
          "survey_url": "https://forms.crown-shy.com"
        }
      }
    },
    {
      name: "Comunity exploration",
      step_order: 2,
      activation_rule:"manual",
      description: "Tell us a bit about yourself! We want to understand where you are comming from and what your past experince with AI is",
      is_offline: false,
      tool_config:{
        "polis":{
          "polis_id":"" ,
          "polis_url":"https://poliscommunity.crown-shy.com" 
        }
      }
    }];

})()


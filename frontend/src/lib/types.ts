export type Conversation={
  id: string,
  name:string,
  short_description:string,
  description:string,
  steps:Array<ConversationStep>,
  image_url:string
  created_at: Date,
  opened_at: Date,
  closed_at:Date
  active_participants: number
  is_complete:boolean
  video_url?:string 
  audio_url?:string 
}

export enum Tool{
  Polis
}

export type ConversationStep={
  id:string,
  title: string,
  instructions: string,
  tool: Tool,
  tool_id: string,
}

export type WorkflowStep={
  id:string
  name:string
}

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

export const conversations: Array<Conversation>=[
  {
    id:"1a70a203-e357-4809-82e0-50a03b00990d",
    name:"Robotic Surgery",
  introduction_text: `Robotic surgery is a way for doctors to perform operations using special machines with tiny tools. One big advantage is that these robots can move very precisely, which helps doctors make smaller cuts. This means patients often feel less pain, heal faster, and have smaller scars. Also, robotic surgery allows doctors to see inside the body better with high-quality cameras, making it easier to do delicate work. For some surgeries, robots can even help doctors reach areas that are hard to get to with regular tools.

However, robotic surgery also has some downsides. The machines are very expensive, so not all hospitals can afford them. Also, doctors need special training to use them properly, which takes time and effort. Sometimes, a robot might have technical problems, which could delay a surgery. And while robotic surgery can be very helpful, not all operations can be done this way, so doctors still need to use traditional methods for many procedures.`,
  steps:[{
    id: "cd5cd836-9e34-42fd-9db7-6b3075e9124b",
    title: "Exploring community opinions",
    instructions: "We are going to use a tool called polis to explore what the community thinks about the topic",
    tool: Tool.Polis,
    tool_id: "8jsn9ua38m",    
  }],
  short_text:"Should the terminator be operating on you",
  banner_image:"https://www.freemalaysiatoday.com/_next/image/?url=https%3A%2F%2Fmedia.freemalaysiatoday.com%2Fwp-content%2Fuploads%2F2022%2F09%2FRobotic-Surgical-Assistant-bernama.jpg&w=1200&q=75",
  created_at: new Date(),
  opened_at: new Date(),
  closed_at: new Date(),
  active_participants:100,
  video_url: "https://www.youtube.com/embed/Bj_NjtsjUsI?si=mJoR-s2ii8EfKsTw",
  audio_url: "https://drive.google.com/file/d/11D8V9y0jKWW9S06hJFR-aCdUdErpWELG/view?usp=sharing",
  is_active:true
  },
  
  {
    id: "9e5742c2-84aa-4613-b09e-f9c8adbb5014",
    name:"Should AI be involved in diagnosing people?",
    introduction_text: `AI (Artificial Intelligence) is becoming an important tool for doctors when diagnosing cancer. One big advantage is that AI can look at medical scans, like X-rays or MRIs, very quickly and spot signs of cancer that might be hard for doctors to see. This can help find cancer earlier, which makes treatment more effective. AI can also compare a patient’s test results with thousands of other cases to give doctors better information on what kind of cancer it might be and what treatments could work best.

However, AI isn’t perfect and has some downsides. Sometimes, AI can make mistakes, like missing a cancer or thinking something is cancer when it’s not. Because of this, doctors still need to check AI’s work carefully. Also, AI needs a lot of data to learn from, and if the data isn’t diverse enough, it might not work well for every patient. Another challenge is that AI technology can be expensive, and not all hospitals have access to it yet. While AI can be a great helper, doctors still play the most important role in making sure patients get the right diagnosis and treatment.`,
  short_text:"What are your concerns when thinking about ways in which AI is involved in diagnosis",
  steps:[],
  banner_image:"https://www.goincognito.co/wp-content/uploads/2023/07/pred.jpg",
  created_at: new Date(),
  opened_at: new Date(),
  closed_at: new Date(),
  active_participants:200,
  is_active:true
  },
  {
    id: "fcfd1a72-b6d3-4dc6-a989-3652b86d0c25",
    short_text:"What are the dangers in police using AI?",
    name:"To what extent should the police use AI to predict issues",
    introduction_text: `Predictive policing is when police use computers and AI to help guess where crimes might happen in the future. The system looks at past crime data and patterns to help officers know where to patrol more often. This can help police stop crimes before they happen and make neighborhoods safer. It can also help police use their time wisely by focusing on areas that need the most attention.

However, predictive policing also has problems. If the computer uses unfair or incorrect data, it might target certain areas or groups of people more than others, even if they haven't done anything wrong. This can make some communities feel like they are being treated unfairly. Also, computers can only predict based on past crimes, which means they might not always be accurate about what will happen next. Because of this, police need to be careful and use predictive policing as just one tool, along with fair and thoughtful decision-making.`,
  steps:[],
  banner_image:"https://i1.pickpik.com/photos/622/555/820/bodyworn-body-camera-police-body-camera-law-enforcement-preview.jpg",
  created_at: new Date(),
  opened_at: new Date(),
  closed_at: new Date(),
  active_participants:200,
  is_active:false
  },
]

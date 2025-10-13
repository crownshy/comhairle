import { MessagesSquare, Headset, Video, ListChecks, BookOpen, Bot, Building, type Icon } from "lucide-svelte"

export type ToolDescription = {
	name: string,
	description: string,
	icon: typeof Icon,
	infoLink: string,
	available: boolean
}
export const AvailableTools: Array<ToolDescription> = [
	{
		name: "Polis",
		description: "Ask for peoples views. Allow them to vote on others views. Understand the landscape of opinions",
		icon: MessagesSquare,
		infoLink: "polis",
		available: true
	},
	{
		name: "Learn",
		description: "Present participants with information to help them learn about the topic at hand",
		icon: BookOpen,
		infoLink: "learn",
		available: true
	},
	{
		name: "Survey",
		description: "Ask participants a series of questions",
		icon: ListChecks,
		infoLink: "heyform",
		available: true
	},
	{
		name: "Elicitation Bot",
		description: "Help participants refine and capture their views through and AI bot mediated interaction",
		icon: Bot,
		infoLink: "elicitation_bot",
		available: true
	},
	{
		name: "Lived Experience",
		description: "Let users record short videos of their lived experience.",
		icon: Video,
		infoLink: "lived_experience",
		available: true
	},
	{
		name: "Online Group Conversation",
		description: "Host a series of online conversations with participants. Record and analyse their conversations.",
		icon: Headset,
		infoLink: "online_group_conversation",
		available: false
	},

	{
		name: "Offline Event",
		description: "Hold a series of offline events and capture what was said there",
		icon: Building,
		infoLink: "offline_event",
		available: false
	}
] 

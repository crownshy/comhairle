export const TOOL_NAME = "elicitationbot"

import ElicitationBotUI from './ElicitationBotEmbed.svelte'
import ElicitationBotReport from './ElicitationBotReport.svelte'
import ElicitationBotManage from './ElicitationBotManage.svelte'
import ElicitationBotChat from './ElicitationBotChat.svelte'
import ExtractedClaims from './ExtractedClaims.svelte'

export {
	ElicitationBotUI as UserUI,
	ElicitationBotManage as ManageUI,
	ElicitationBotReport as ReportUI,
	ElicitationBotChat,
	ExtractedClaims
}

export * from './types'

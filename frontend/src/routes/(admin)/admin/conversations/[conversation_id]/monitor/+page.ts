import type { PageLoad } from "./$types"

export const load: PageLoad = async ({ parent }) => {
	let { conversation, workflows, workflow_steps, api } = await parent()
	let workflowStats = await api.GetWorkflowStats({ params: { conversation_id: conversation.id, workflow_id: workflows[0].id } })
	try {
		let conversationStats = await api.GetWorkflowStats({ params: { conversation_id: conversation.id, workflow_id: workflows[0].id } })

		return { conversation, workflows, workflow_steps, workflowStats, conversationStats }
	} catch (e) {
		console.log(e)
	}
}

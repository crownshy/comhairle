import { ws, type WebSocketMessage } from '$lib/api/websockets.svelte';

// Workflow-specific message types
export type WorkflowWebSocketMessage =
	| { type: 'user_started_workflow_step'; payload: { workflow_step_id: string } }
	| { type: 'user_finished_workflow_step'; payload: { workflow_step_id: string } }
	| { type: 'user_idle'; payload: { workflow_step_id: string } };

export interface WorkflowStepActivity {
	workflow_step_id: string;
	user_id: string;
	username?: string;
	status: 'started' | 'finished' | 'idle';
	timestamp: number;
}

export class WorkflowService {
	// Track active users per workflow step
	private activeUsers = $state<Map<string, Set<string>>>(new Map());

	// Track the current user's active workflow step
	currentWorkflowStep = $state<string | null>(null);

	// History of workflow activities
	activityLog = $state<WorkflowStepActivity[]>([]);

	constructor() {
		this.setupListeners();
	}

	private setupListeners() {
		// Listen for all messages and filter for workflow-specific ones
		ws.onMessage((message: WebSocketMessage) => {
			switch (message.type) {
				case 'user_started_workflow_step':
					this.handleUserStarted(
						(
							message as WorkflowWebSocketMessage & {
								type: 'user_started_workflow_step';
							}
						).payload.workflow_step_id
					);
					break;

				case 'user_finished_workflow_step':
					this.handleUserFinished(
						(
							message as WorkflowWebSocketMessage & {
								type: 'user_finished_workflow_step';
							}
						).payload.workflow_step_id
					);
					break;

				case 'user_idle':
					this.handleUserIdle(
						(message as WorkflowWebSocketMessage & { type: 'user_idle' }).payload
							.workflow_step_id
					);
					break;

				case 'user_joined':
					console.log(
						`User ${message.payload.username || message.payload.user_id} joined`
					);
					break;

				case 'user_left':
					console.log(`User ${message.payload.username || message.payload.user_id} left`);
					break;

				case 'broadcast':
					console.log('Workflow broadcast:', message.payload.message);
					break;
			}
		});
	}

	// Start a workflow step
	startWorkflowStep(workflowStepId: string) {
		// End previous step if exists
		if (this.currentWorkflowStep) {
			this.finishWorkflowStep(this.currentWorkflowStep);
		}

		this.currentWorkflowStep = workflowStepId;

		// Send workflow-specific message through base WebSocket
		ws.send({
			type: 'user_started_workflow_step',
			payload: { workflow_step_id: workflowStepId }
		} as WorkflowWebSocketMessage);

		console.log(`Started workflow step: ${workflowStepId}`);
	}

	// Finish a workflow step
	finishWorkflowStep(workflowStepId: string) {
		if (this.currentWorkflowStep === workflowStepId) {
			this.currentWorkflowStep = null;
		}

		ws.send({
			type: 'user_finished_workflow_step',
			payload: { workflow_step_id: workflowStepId }
		} as WorkflowWebSocketMessage);

		console.log(`Finished workflow step: ${workflowStepId}`);
	}

	// Mark user as idle on a workflow step
	markIdle(workflowStepId: string) {
		ws.send({
			type: 'user_idle',
			payload: { workflow_step_id: workflowStepId }
		} as WorkflowWebSocketMessage);

		console.log(`Marked idle on workflow step: ${workflowStepId}`);
	}

	// Get active users for a specific workflow step
	getActiveUsers(workflowStepId: string): number {
		return this.activeUsers.get(workflowStepId)?.size || 0;
	}

	// Check if currently on a workflow step
	isOnWorkflowStep(workflowStepId: string): boolean {
		return this.currentWorkflowStep === workflowStepId;
	}

	// Private handlers for tracking state
	private handleUserStarted(workflowStepId: string) {
		if (!this.activeUsers.has(workflowStepId)) {
			this.activeUsers.set(workflowStepId, new Set());
		}
		// You could track specific user IDs here if the backend sends them

		this.activityLog.push({
			workflow_step_id: workflowStepId,
			user_id: 'unknown', // Backend would need to send this
			status: 'started',
			timestamp: Date.now()
		});

		// Keep only last 100 activities
		if (this.activityLog.length > 100) {
			this.activityLog = this.activityLog.slice(-100);
		}
	}

	private handleUserFinished(workflowStepId: string) {
		const users = this.activeUsers.get(workflowStepId);
		if (users) {
			// Would remove specific user if we tracked them
			if (users.size === 0) {
				this.activeUsers.delete(workflowStepId);
			}
		}

		this.activityLog.push({
			workflow_step_id: workflowStepId,
			user_id: 'unknown',
			status: 'finished',
			timestamp: Date.now()
		});

		if (this.activityLog.length > 100) {
			this.activityLog = this.activityLog.slice(-100);
		}
	}

	private handleUserIdle(workflowStepId: string) {
		this.activityLog.push({
			workflow_step_id: workflowStepId,
			user_id: 'unknown',
			status: 'idle',
			timestamp: Date.now()
		});

		if (this.activityLog.length > 100) {
			this.activityLog = this.activityLog.slice(-100);
		}
	}

	// Cleanup method to call when service is no longer needed
	destroy() {
		if (this.currentWorkflowStep) {
			this.finishWorkflowStep(this.currentWorkflowStep);
		}
	}
}

// Singleton instance
export const workflowService = new WorkflowService();

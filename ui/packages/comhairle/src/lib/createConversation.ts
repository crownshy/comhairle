import { apiClient } from '@crownshy/api-client/client';
import type { ConversationDto, CreateWorkflowStep } from '@crownshy/api-client/api';
import PlaceholderConvo from '$lib/assets/placeholder_convo.png';
import { getTemplate, type CreationStep } from './conversation_templates';

/**
 * Auto-generated title for eagerly-created conversations, e.g.
 * "Untitled 2027-07-02 2:24:30PM". Uses local browser time; the word
 * "Untitled" is intentionally not localised (throwaway default).
 *
 * Seconds are included so two conversations created in the same minute produce
 * distinct titles — the backend slugifies the title under a unique constraint,
 * so identical titles otherwise collide (DuplicateSlug).
 */
export function generateUntitledTitle(now: Date = new Date()): string {
	const yyyy = now.getFullYear();
	const mm = String(now.getMonth() + 1).padStart(2, '0');
	const dd = String(now.getDate()).padStart(2, '0');

	let hours = now.getHours();
	const meridiem = hours >= 12 ? 'PM' : 'AM';
	hours = hours % 12 || 12;
	const minutes = String(now.getMinutes()).padStart(2, '0');
	const seconds = String(now.getSeconds()).padStart(2, '0');

	return `Untitled ${yyyy}-${mm}-${dd} ${hours}:${minutes}:${seconds}${meridiem}`;
}

/**
 * Eagerly create a conversation (create-then-edit, see ADR-0002).
 *
 * Both "Start from blank" (templateKey omitted / unknown) and template
 * "Get started" flows call this. Descriptions start empty and are completed
 * later on the configure tab; the API applies no length validation.
 *
 * @returns the created conversation (caller handles navigation/notifications)
 */
export async function createConversation(
	opts: { templateKey?: string } = {}
): Promise<ConversationDto> {
	const template = opts.templateKey ? getTemplate(opts.templateKey) : undefined;
	const creationSteps: CreationStep[] = template?.creationSteps ?? [];
	const creationEvents = template?.creationEvents ?? [];

	const conversation = await apiClient.CreateConversation({
		title: generateUntitledTitle(),
		short_description: '',
		description: '',
		tags: [],
		image_url: PlaceholderConvo,
		primary_locale: 'en',
		supported_languages: ['en'],
		is_public: false,
		is_live: false,
		is_invite_only: false
	});

	const workflow = await apiClient.CreateConversationWorkflow(
		{
			name: 'Default Workflow',
			description: 'The default workflow',
			is_active: true,
			is_public: true,
			auto_login: false
		},
		{ params: { conversation_id: conversation.id } }
	);

	for (const step of creationSteps) {
		await apiClient.CreateConversationWorkflowStep(step as unknown as CreateWorkflowStep, {
			params: {
				conversation_id: conversation.id,
				workflow_id: workflow.id
			}
		});
	}

	for (const event of creationEvents) {
		const start = new Date();
		const end = new Date(start.getTime() + event.durationMinutes * 60_000);
		await apiClient.CreateEvent(
			{
				name: event.name,
				description: event.description,
				signup_mode: event.signup_mode,
				start_time: start.toISOString(),
				end_time: end.toISOString()
			},
			{ params: { conversation_id: conversation.id } }
		);
	}

	return conversation;
}

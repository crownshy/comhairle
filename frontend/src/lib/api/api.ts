import { makeApi, Zodios, type ZodiosOptions } from "@zodios/core";
import { z } from "zod";


export const AnnonLoginRequest = z.object({ username: z.string() }).passthrough();
export type AnnonLoginRequest = z.infer<typeof AnnonLoginRequest>;
export const UserAuthType = z.enum(["annon", "email_password", "scot_account"]);
export type UserAuthType = z.infer<typeof UserAuthType>;
export const User = z.object({ auth_type: UserAuthType, avatar_url: z.union([z.string(), z.null()]).optional(), email: z.union([z.string(), z.null()]).optional(), email_verified: z.boolean(), id: z.string().uuid(), password: z.union([z.string(), z.null()]).optional(), username: z.union([z.string(), z.null()]).optional() }).passthrough();
export type User = z.infer<typeof User>;
export const LoginRequest = z.object({ email: z.string(), password: z.string() }).passthrough();
export type LoginRequest = z.infer<typeof LoginRequest>;
export const SignupRequest = z.object({ avatar_url: z.union([z.string(), z.null()]).optional(), email: z.string(), password: z.string(), username: z.string() }).passthrough();
export type SignupRequest = z.infer<typeof SignupRequest>;
export const VerifyEmailTokenRequest = z.object({ token: z.string() }).passthrough();
export type VerifyEmailTokenRequest = z.infer<typeof VerifyEmailTokenRequest>;
export const ResendVerificationEmailRequest = z.object({ id: z.string() }).passthrough();
export type ResendVerificationEmailRequest = z.infer<typeof ResendVerificationEmailRequest>;
export const CreatePasswordResetRequest = z.object({ email: z.string() }).passthrough();
export type CreatePasswordResetRequest = z.infer<typeof CreatePasswordResetRequest>;
export const PasswordResetUpdateRequest = z.object({ confirm_password: z.string(), password: z.string(), token: z.string() }).passthrough();
export type PasswordResetUpdateRequest = z.infer<typeof PasswordResetUpdateRequest>;
export const ResourceType = z.union([z.literal("Site"), z.object({ Conversation: z.string().uuid() })]);
export type ResourceType = z.infer<typeof ResourceType>;
export const ResourceRole = z.enum(["Admin", "SuperAdmin"]);
export type ResourceRole = z.infer<typeof ResourceRole>;
export const UserRoles = z.object({ resource: ResourceType, roles: z.array(ResourceRole) }).passthrough();
export type UserRoles = z.infer<typeof UserRoles>;
export const Conversation = z.object({ chat_bot_id: z.union([z.string(), z.null()]).optional(), created_at: z.string().datetime({ offset: true }), default_workflow_id: z.union([z.string(), z.null()]).optional(), description: z.string().uuid(), enable_qa_chat_bot: z.boolean(), id: z.string().uuid(), image_url: z.string(), is_complete: z.boolean(), is_invite_only: z.boolean(), is_public: z.boolean(), knowledge_base_id: z.union([z.string(), z.null()]).optional(), owner_id: z.string().uuid(), primary_locale: z.string(), short_description: z.string().uuid(), slug: z.union([z.string(), z.null()]).optional(), supported_languages: z.array(z.string()), tags: z.array(z.string()), title: z.string().uuid(), updated_at: z.string().datetime({ offset: true }), video_url: z.union([z.string(), z.null()]).optional() }).passthrough();
export type Conversation = z.infer<typeof Conversation>;
export const created_after = z.union([z.string(), z.null()]).optional();
export type created_after = z.infer<typeof created_after>;
export const is_complete = z.union([z.boolean(), z.null()]).optional();
export type is_complete = z.infer<typeof is_complete>;
export const limit = z.union([z.number(), z.null()]).optional();
export type limit = z.infer<typeof limit>;
export const LocalisedConversation = z.object({ chat_bot_id: z.union([z.string(), z.null()]).optional(), created_at: z.string().datetime({ offset: true }), default_workflow_id: z.union([z.string(), z.null()]).optional(), description: z.string(), enable_qa_chat_bot: z.boolean(), id: z.string().uuid(), image_url: z.string(), is_complete: z.boolean(), is_invite_only: z.boolean(), is_public: z.boolean(), knowledge_base_id: z.union([z.string(), z.null()]).optional(), owner_id: z.string().uuid(), primary_locale: z.string(), short_description: z.string(), slug: z.union([z.string(), z.null()]).optional(), supported_languages: z.array(z.string()), tags: z.array(z.string()), title: z.string(), updated_at: z.string().datetime({ offset: true }), video_url: z.union([z.string(), z.null()]).optional() }).passthrough();
export type LocalisedConversation = z.infer<typeof LocalisedConversation>;
export const PaginatedResults_for_LocalisedConversation = z.object({ records: z.array(LocalisedConversation), total: z.number().int() }).passthrough();
export type PaginatedResults_for_LocalisedConversation = z.infer<typeof PaginatedResults_for_LocalisedConversation>;
export const UpdateUserRequest = z.object({ email_verified: z.union([z.boolean(), z.null()]), password: z.union([z.string(), z.null()]), username: z.union([z.string(), z.null()]) }).partial().passthrough();
export type UpdateUserRequest = z.infer<typeof UpdateUserRequest>;
export const UpgradeAccountRequest = z.object({ email: z.string(), password: z.string(), username: z.string() }).passthrough();
export type UpgradeAccountRequest = z.infer<typeof UpgradeAccountRequest>;
export const UserConversationPreferences = z.object({ conversation_id: z.string().uuid(), created_at: z.string().datetime({ offset: true }), id: z.string().uuid(), receive_similar_conversation_updates_by_email: z.boolean(), receive_similar_conversation_updates_by_notification: z.boolean(), receive_updates_by_email: z.boolean(), receive_updates_by_notification: z.boolean(), updated_at: z.string().datetime({ offset: true }), user_id: z.string().uuid() }).passthrough();
export type UserConversationPreferences = z.infer<typeof UserConversationPreferences>;
export const UpdateUserConversationPreferences = z.object({ receive_similar_conversation_updates_by_email: z.union([z.boolean(), z.null()]), receive_similar_conversation_updates_by_notification: z.union([z.boolean(), z.null()]), receive_updates_by_email: z.union([z.boolean(), z.null()]), receive_updates_by_notification: z.union([z.boolean(), z.null()]) }).partial().passthrough();
export type UpdateUserConversationPreferences = z.infer<typeof UpdateUserConversationPreferences>;
export const DeliveryMethod = z.enum(["in_app", "email"]);
export type DeliveryMethod = z.infer<typeof DeliveryMethod>;
export const NotificationContextType = z.enum(["site", "conversation"]);
export type NotificationContextType = z.infer<typeof NotificationContextType>;
export const NotificationType = z.enum(["info", "warning", "error", "success"]);
export type NotificationType = z.infer<typeof NotificationType>;
export const Notification = z.object({ content: z.string(), context_id: z.union([z.string(), z.null()]).optional(), context_type: NotificationContextType, created_at: z.string().datetime({ offset: true }), id: z.string().uuid(), notification_type: NotificationType, title: z.string(), updated_at: z.string().datetime({ offset: true }) }).passthrough();
export type Notification = z.infer<typeof Notification>;
export const NotificationWithDelivery = z.object({ created_at: z.string().datetime({ offset: true }), delivered_at: z.string().datetime({ offset: true }), delivery_method: DeliveryMethod, id: z.string().uuid(), notification: Notification, notification_id: z.string().uuid(), read_at: z.union([z.string(), z.null()]).optional(), updated_at: z.string().datetime({ offset: true }), user_id: z.string().uuid() }).passthrough();
export type NotificationWithDelivery = z.infer<typeof NotificationWithDelivery>;
export const PaginatedResults_for_NotificationWithDelivery = z.object({ records: z.array(NotificationWithDelivery), total: z.number().int() }).passthrough();
export type PaginatedResults_for_NotificationWithDelivery = z.infer<typeof PaginatedResults_for_NotificationWithDelivery>;
export const UnreadCount = z.object({ count: z.number().int() }).passthrough();
export type UnreadCount = z.infer<typeof UnreadCount>;
export const NotificationDelivery = z.object({ created_at: z.string().datetime({ offset: true }), delivered_at: z.string().datetime({ offset: true }), delivery_method: DeliveryMethod, id: z.string().uuid(), notification_id: z.string().uuid(), read_at: z.union([z.string(), z.null()]).optional(), updated_at: z.string().datetime({ offset: true }), user_id: z.string().uuid() }).passthrough();
export type NotificationDelivery = z.infer<typeof NotificationDelivery>;
export const TextFormat = z.union([z.literal("plain"), z.literal("markdown"), z.literal("rich")]);
export type TextFormat = z.infer<typeof TextFormat>;
export const CreateTextContentRequest = z.object({ content: z.string(), format: TextFormat, primary_locale: z.string() }).passthrough();
export type CreateTextContentRequest = z.infer<typeof CreateTextContentRequest>;
export const TextContent = z.object({ created_at: z.string().datetime({ offset: true }), format: TextFormat, id: z.string().uuid(), primary_locale: z.string(), updated_at: z.string().datetime({ offset: true }) }).passthrough();
export type TextContent = z.infer<typeof TextContent>;
export const TextTranslation = z.object({ ai_generated: z.boolean(), content: z.string(), content_id: z.string().uuid(), created_at: z.string().datetime({ offset: true }), id: z.string().uuid(), locale: z.string(), requires_validation: z.boolean(), updated_at: z.string().datetime({ offset: true }) }).passthrough();
export type TextTranslation = z.infer<typeof TextTranslation>;
export const TextContentWithTranslations = z.object({ created_at: z.string().datetime({ offset: true }), format: TextFormat, id: z.string().uuid(), primary_locale: z.string(), translations: z.array(TextTranslation), updated_at: z.string().datetime({ offset: true }) }).passthrough();
export type TextContentWithTranslations = z.infer<typeof TextContentWithTranslations>;
export const UpdateTextContent = z.object({ format: z.union([TextFormat, z.null()]), primary_locale: z.union([z.string(), z.null()]) }).partial().passthrough();
export type UpdateTextContent = z.infer<typeof UpdateTextContent>;
export const UpdateTextTranslation = z.object({ ai_generated: z.union([z.boolean(), z.null()]), content: z.union([z.string(), z.null()]), locale: z.union([z.string(), z.null()]), requires_validation: z.union([z.boolean(), z.null()]) }).partial().passthrough();
export type UpdateTextTranslation = z.infer<typeof UpdateTextTranslation>;
export const CreateOrUpdateTextTranslationRequest = z.object({ ai_generated: z.union([z.boolean(), z.null()]).optional(), content: z.string(), requires_validation: z.union([z.boolean(), z.null()]).optional() }).passthrough();
export type CreateOrUpdateTextTranslationRequest = z.infer<typeof CreateOrUpdateTextTranslationRequest>;
export const CreateConversation = z.object({ default_workflow_id: z.union([z.string(), z.null()]).optional(), description: z.string(), enable_qa_chat_bot: z.union([z.boolean(), z.null()]).optional(), image_url: z.string(), is_invite_only: z.boolean(), is_public: z.boolean(), primary_locale: z.string(), short_description: z.string(), slug: z.union([z.string(), z.null()]).optional(), supported_languages: z.array(z.string()), tags: z.union([z.array(z.string()), z.null()]).optional(), title: z.string(), video_url: z.union([z.string(), z.null()]).optional() }).passthrough();
export type CreateConversation = z.infer<typeof CreateConversation>;
export const Translation = z.object({ text_content: TextContent, text_translations: z.array(TextTranslation) }).passthrough();
export type Translation = z.infer<typeof Translation>;
export const ConversationWithTranslations = z.object({ chat_bot_id: z.union([z.string(), z.null()]).optional(), created_at: z.string().datetime({ offset: true }), default_workflow_id: z.union([z.string(), z.null()]).optional(), description: z.string(), enable_qa_chat_bot: z.boolean(), id: z.string().uuid(), image_url: z.string(), is_complete: z.boolean(), is_invite_only: z.boolean(), is_public: z.boolean(), knowledge_base_id: z.union([z.string(), z.null()]).optional(), owner_id: z.string().uuid(), primary_locale: z.string(), short_description: z.string(), slug: z.union([z.string(), z.null()]).optional(), supported_languages: z.array(z.string()), tags: z.array(z.string()), title: z.string(), translations: z.record(Translation), updated_at: z.string().datetime({ offset: true }), video_url: z.union([z.string(), z.null()]).optional() }).passthrough();
export type ConversationWithTranslations = z.infer<typeof ConversationWithTranslations>;
export const ConversationResponse = z.union([LocalisedConversation, ConversationWithTranslations]);
export type ConversationResponse = z.infer<typeof ConversationResponse>;
export const PartialConversation = z.object({ chat_bot_id: z.union([z.string(), z.null()]), default_workflow_id: z.union([z.string(), z.null()]), description: z.union([z.string(), z.null()]), enable_qa_chat_bot: z.union([z.boolean(), z.null()]), image_url: z.union([z.string(), z.null()]), is_complete: z.union([z.boolean(), z.null()]), is_invite_only: z.union([z.boolean(), z.null()]), is_public: z.union([z.boolean(), z.null()]), knowledge_base_id: z.union([z.string(), z.null()]), primary_locale: z.union([z.string(), z.null()]), short_description: z.union([z.string(), z.null()]), slug: z.union([z.string(), z.null()]), supported_languages: z.union([z.array(z.string()), z.null()]), tags: z.union([z.array(z.string()), z.null()]), title: z.union([z.string(), z.null()]), video_url: z.union([z.string(), z.null()]) }).partial().passthrough();
export type PartialConversation = z.infer<typeof PartialConversation>;
export const SendNotificationRequest = z.object({ content: z.string(), delivery_method: z.union([DeliveryMethod, z.null()]).optional(), notification_type: z.union([NotificationType, z.null()]).optional(), title: z.string() }).passthrough();
export type SendNotificationRequest = z.infer<typeof SendNotificationRequest>;
export const RegisterEmailRequest = z.object({ email: z.string(), receive_similar_conversation_updates_by_email: z.boolean(), receive_updates_by_email: z.boolean() }).passthrough();
export type RegisterEmailRequest = z.infer<typeof RegisterEmailRequest>;
export const RegisterEmailResponse = z.object({ conversation_id: z.string().uuid(), email: z.string(), id: z.string().uuid(), message: z.string() }).passthrough();
export type RegisterEmailResponse = z.infer<typeof RegisterEmailResponse>;
export const Workflow = z.object({ auto_login: z.boolean(), conversation_id: z.string().uuid(), created_at: z.string().datetime({ offset: true }), description: z.string(), id: z.string().uuid(), is_active: z.boolean(), is_public: z.boolean(), name: z.string(), owner_id: z.string().uuid(), updated_at: z.string().datetime({ offset: true }) }).passthrough();
export type Workflow = z.infer<typeof Workflow>;
export const CreateWorkflow = z.object({ auto_login: z.boolean(), description: z.string(), is_active: z.boolean(), is_public: z.boolean(), name: z.string() }).passthrough();
export type CreateWorkflow = z.infer<typeof CreateWorkflow>;
export const ActivationRule = z.literal("manual");
export type ActivationRule = z.infer<typeof ActivationRule>;
export const LocalisedPage = z.object({ content: z.string(), type: z.literal("markdown") }).passthrough();
export type LocalisedPage = z.infer<typeof LocalisedPage>;
export const Page = z.array(LocalisedPage);
export type Page = z.infer<typeof Page>;
export const ToolConfig = z.union([z.object({ admin_password: z.string(), admin_user: z.string(), poll_id: z.string(), server_url: z.string(), type: z.literal("polis") }).passthrough(), z.object({ pages: z.array(Page), type: z.literal("learn") }).passthrough(), z.object({ admin_password: z.string(), admin_user: z.string(), project_id: z.string(), survey_id: z.string(), survey_url: z.string(), type: z.literal("heyform"), workspace_id: z.string() }).passthrough(), z.object({ max_time: z.number().int(), to_see: z.number().int(), type: z.literal("stories") }).passthrough(), z.object({ type: z.literal("elicitationbot") }).passthrough()]);
export type ToolConfig = z.infer<typeof ToolConfig>;
export const WorkflowStep = z.object({ activation_rule: ActivationRule, created_at: z.string().datetime({ offset: true }), description: z.string().uuid(), id: z.string().uuid(), is_offline: z.boolean(), name: z.string().uuid(), required: z.boolean(), step_order: z.number().int(), tool_config: ToolConfig, updated_at: z.string().datetime({ offset: true }), workflow_id: z.string().uuid() }).passthrough();
export type WorkflowStep = z.infer<typeof WorkflowStep>;
export const DailySignupStats = z.object({ day: z.string().datetime({ offset: true }), users: z.number().int() }).passthrough();
export type DailySignupStats = z.infer<typeof DailySignupStats>;
export const WorkflowStepStats = z.object({ completed: z.number().int(), id: z.string().uuid(), started: z.number().int() }).passthrough();
export type WorkflowStepStats = z.infer<typeof WorkflowStepStats>;
export const WorkflowStats = z.object({ signup_stats: z.array(DailySignupStats), step_stats: z.array(WorkflowStepStats), total_users: z.number().int() }).passthrough();
export type WorkflowStats = z.infer<typeof WorkflowStats>;
export const PartialWorkflow = z.object({ auto_login: z.union([z.boolean(), z.null()]), description: z.union([z.string(), z.null()]), is_active: z.union([z.boolean(), z.null()]), is_public: z.union([z.boolean(), z.null()]), name: z.union([z.string(), z.null()]) }).partial().passthrough();
export type PartialWorkflow = z.infer<typeof PartialWorkflow>;
export const UserParticipation = z.object({ created_at: z.string().datetime({ offset: true }), id: z.string().uuid(), updated_at: z.string().datetime({ offset: true }), user_id: z.string().uuid(), workflow_id: z.string().uuid() }).passthrough();
export type UserParticipation = z.infer<typeof UserParticipation>;
export const LocalisedWorkflowStep = z.object({ activation_rule: ActivationRule, created_at: z.string().datetime({ offset: true }), description: z.string(), id: z.string().uuid(), is_offline: z.boolean(), name: z.string(), required: z.boolean(), step_order: z.number().int(), tool_config: ToolConfig, updated_at: z.string().datetime({ offset: true }), workflow_id: z.string().uuid() }).passthrough();
export type LocalisedWorkflowStep = z.infer<typeof LocalisedWorkflowStep>;
export const ToolSetup = z.union([z.object({ topic: z.string(), type: z.literal("polis") }).passthrough(), z.object({ pages: z.array(Page), type: z.literal("learn") }).passthrough(), z.object({ type: z.literal("heyform") }).passthrough(), z.object({ max_time: z.number().int(), to_see: z.number().int(), type: z.literal("stories") }).passthrough(), z.object({ type: z.literal("elicitationbot") }).passthrough()]);
export type ToolSetup = z.infer<typeof ToolSetup>;
export const CreateWorkflowStep = z.object({ activation_rule: ActivationRule, description: z.string(), is_offline: z.boolean(), name: z.string(), required: z.boolean(), step_order: z.number().int(), tool_setup: ToolSetup }).passthrough();
export type CreateWorkflowStep = z.infer<typeof CreateWorkflowStep>;
export const PartialWorkflowStep = z.object({ activation_rule: z.union([ActivationRule, z.null()]), description: z.union([z.string(), z.null()]), is_offline: z.union([z.boolean(), z.null()]), name: z.union([z.string(), z.null()]), required: z.union([z.boolean(), z.null()]), step_order: z.union([z.number(), z.null()]), tool_config: z.union([ToolConfig, z.null()]) }).partial().passthrough();
export type PartialWorkflowStep = z.infer<typeof PartialWorkflowStep>;
export const ProgressStatus = z.enum(["not_started", "in_progress", "done"]);
export type ProgressStatus = z.infer<typeof ProgressStatus>;
export const UserProgress = z.object({ created_at: z.string().datetime({ offset: true }), id: z.string().uuid(), status: ProgressStatus, updated_at: z.string().datetime({ offset: true }), user_id: z.string().uuid(), workflow_step_id: z.string().uuid() }).passthrough();
export type UserProgress = z.infer<typeof UserProgress>;
export const InviteType = z.union([z.object({ email: z.string() }), z.object({ user: z.string().uuid() }), z.literal("singleuse"), z.literal("open")]);
export type InviteType = z.infer<typeof InviteType>;
export const LoginBehaviour = z.union([z.literal("manual"), z.literal("auto_create_annon")]);
export type LoginBehaviour = z.infer<typeof LoginBehaviour>;
export const InviteStatus = z.union([z.literal("pending"), z.literal("open"), z.literal("accepted"), z.literal("rejected"), z.literal("expired")]);
export type InviteStatus = z.infer<typeof InviteStatus>;
export const Invite = z.object({ accept_count: z.number().int(), conversation_id: z.string().uuid(), created_at: z.string().datetime({ offset: true }), created_by: z.string().uuid(), expires_at: z.union([z.string(), z.null()]).optional(), id: z.string().uuid(), invite_type: InviteType, login_behaviour: LoginBehaviour, status: InviteStatus, tags: z.array(z.string()), updated_at: z.string().datetime({ offset: true }), workflow_id: z.union([z.string(), z.null()]).optional(), workflow_step_id: z.union([z.string(), z.null()]).optional() }).passthrough();
export type Invite = z.infer<typeof Invite>;
export const CreateInviteDTO = z.object({ expires_at: z.union([z.string(), z.null()]).optional(), invite_type: InviteType, login_behaviour: LoginBehaviour.optional() }).passthrough();
export type CreateInviteDTO = z.infer<typeof CreateInviteDTO>;
export const DailyResponseStats = z.object({ accept: z.number().int(), day: z.string().datetime({ offset: true }), reject: z.number().int() }).passthrough();
export type DailyResponseStats = z.infer<typeof DailyResponseStats>;
export const Feedback = z.object({ content: z.string(), conversation_id: z.string().uuid(), created_at: z.string().datetime({ offset: true }), created_by: z.string().uuid(), id: z.string().uuid(), updated_at: z.string().datetime({ offset: true }) }).passthrough();
export type Feedback = z.infer<typeof Feedback>;
export const ReportImpact = z.object({ created_at: z.string().datetime({ offset: true }), created_by: z.string().uuid(), details: z.string(), id: z.string().uuid(), kind: z.string(), report_id: z.string().uuid(), title: z.string(), updated_at: z.string().datetime({ offset: true }) }).passthrough();
export type ReportImpact = z.infer<typeof ReportImpact>;
export const PolisReport = z.null();
export type PolisReport = z.infer<typeof PolisReport>;
export const HeyFormReport = z.null();
export type HeyFormReport = z.infer<typeof HeyFormReport>;
export const LearnReport = z.null();
export type LearnReport = z.infer<typeof LearnReport>;
export const StoriesReport = z.null();
export type StoriesReport = z.infer<typeof StoriesReport>;
export const ElicitationBotReport = z.null();
export type ElicitationBotReport = z.infer<typeof ElicitationBotReport>;
export const ReportConfig = z.union([z.object({ Polis: PolisReport }), z.object({ HeyForm: HeyFormReport }), z.object({ Learn: LearnReport }), z.object({ Stories: StoriesReport }), z.object({ ElicitationBot: ElicitationBotReport })]);
export type ReportConfig = z.infer<typeof ReportConfig>;
export const ReportSectionConfig = z.object({ ai_generated: z.boolean(), config: ReportConfig, verified: z.boolean(), workflow_step_id: z.string().uuid() }).passthrough();
export type ReportSectionConfig = z.infer<typeof ReportSectionConfig>;
export const ReportSectionConfigs = z.array(ReportSectionConfig);
export type ReportSectionConfigs = z.infer<typeof ReportSectionConfigs>;
export const FullReportDTO = z.object({ conversation_id: z.string().uuid(), created_at: z.string().datetime({ offset: true }), facilitator_feedback: z.array(Feedback), id: z.string().uuid(), impacts: z.array(ReportImpact), is_public: z.boolean(), participant_feedback: z.array(Feedback), section_configs: ReportSectionConfigs, summary: z.string(), updated_at: z.string().datetime({ offset: true }) }).passthrough();
export type FullReportDTO = z.infer<typeof FullReportDTO>;
export const PartialReport = z.object({ conversation_id: z.union([z.string(), z.null()]), is_public: z.union([z.boolean(), z.null()]), section_configs: z.union([ReportSectionConfigs, z.null()]), summary: z.union([z.string(), z.null()]) }).partial().passthrough();
export type PartialReport = z.infer<typeof PartialReport>;
export const Report = z.object({ conversation_id: z.string().uuid(), created_at: z.string().datetime({ offset: true }), id: z.string().uuid(), is_public: z.boolean(), section_configs: ReportSectionConfigs, summary: z.string(), updated_at: z.string().datetime({ offset: true }) }).passthrough();
export type Report = z.infer<typeof Report>;
export const PartialReportImpact = z.object({ created_at: z.union([z.string(), z.null()]), created_by: z.union([z.string(), z.null()]), details: z.union([z.string(), z.null()]), id: z.union([z.string(), z.null()]), kind: z.union([z.string(), z.null()]), report_id: z.union([z.string(), z.null()]), title: z.union([z.string(), z.null()]), updated_at: z.union([z.string(), z.null()]) }).partial().passthrough();
export type PartialReportImpact = z.infer<typeof PartialReportImpact>;
export const CreateImpactDTO = z.object({ details: z.string(), kind: z.string(), title: z.string() }).passthrough();
export type CreateImpactDTO = z.infer<typeof CreateImpactDTO>;
export const CreateFeedbackDTO = z.object({ content: z.string() }).passthrough();
export type CreateFeedbackDTO = z.infer<typeof CreateFeedbackDTO>;
export const PartialFeedback = z.object({ content: z.union([z.string(), z.null()]) }).partial().passthrough();
export type PartialFeedback = z.infer<typeof PartialFeedback>;
export const WebSocketStats = z.object({ connected_users: z.array(z.string().uuid()), total_connections: z.number().int().gte(0) }).passthrough();
export type WebSocketStats = z.infer<typeof WebSocketStats>;
export const BroadcastMessage = z.object({ authenticated_only: z.union([z.boolean(), z.null()]).optional(), message: z.string() }).passthrough();
export type BroadcastMessage = z.infer<typeof BroadcastMessage>;
export const BroadcastResponse = z.object({ message: z.string(), sent_to: z.number().int().gte(0) }).passthrough();
export type BroadcastResponse = z.infer<typeof BroadcastResponse>;
export const SendToUserMessage = z.object({ message: z.string(), user_id: z.string().uuid() }).passthrough();
export type SendToUserMessage = z.infer<typeof SendToUserMessage>;


export const schemas = {
	AnnonLoginRequest,
	UserAuthType,
	User,
	LoginRequest,
	SignupRequest,
	VerifyEmailTokenRequest,
	ResendVerificationEmailRequest,
	CreatePasswordResetRequest,
	PasswordResetUpdateRequest,
	ResourceType,
	ResourceRole,
	UserRoles,
	Conversation,
	created_after,
	is_complete,
	limit,
	LocalisedConversation,
	PaginatedResults_for_LocalisedConversation,
	UpdateUserRequest,
	UpgradeAccountRequest,
	UserConversationPreferences,
	UpdateUserConversationPreferences,
	DeliveryMethod,
	NotificationContextType,
	NotificationType,
	Notification,
	NotificationWithDelivery,
	PaginatedResults_for_NotificationWithDelivery,
	UnreadCount,
	NotificationDelivery,
	TextFormat,
	CreateTextContentRequest,
	TextContent,
	TextTranslation,
	TextContentWithTranslations,
	UpdateTextContent,
	UpdateTextTranslation,
	CreateOrUpdateTextTranslationRequest,
	CreateConversation,
	Translation,
	ConversationWithTranslations,
	ConversationResponse,
	PartialConversation,
	SendNotificationRequest,
	RegisterEmailRequest,
	RegisterEmailResponse,
	Workflow,
	CreateWorkflow,
	ActivationRule,
	LocalisedPage,
	Page,
	ToolConfig,
	WorkflowStep,
	DailySignupStats,
	WorkflowStepStats,
	WorkflowStats,
	PartialWorkflow,
	UserParticipation,
	LocalisedWorkflowStep,
	ToolSetup,
	CreateWorkflowStep,
	PartialWorkflowStep,
	ProgressStatus,
	UserProgress,
	InviteType,
	LoginBehaviour,
	InviteStatus,
	Invite,
	CreateInviteDTO,
	DailyResponseStats,
	Feedback,
	ReportImpact,
	PolisReport,
	HeyFormReport,
	LearnReport,
	StoriesReport,
	ElicitationBotReport,
	ReportConfig,
	ReportSectionConfig,
	ReportSectionConfigs,
	FullReportDTO,
	PartialReport,
	Report,
	PartialReportImpact,
	CreateImpactDTO,
	CreateFeedbackDTO,
	PartialFeedback,
	WebSocketStats,
	BroadcastMessage,
	BroadcastResponse,
	SendToUserMessage,
};

const endpoints = makeApi([
	{
		method: "get",
		path: "/auth/current_user",
		alias: "CurrentUser",
		requestFormat: "json",
		response: User,
	},
	{
		method: "post",
		path: "/auth/login",
		alias: "LoginUser",
		requestFormat: "json",
		parameters: [
			{
				name: "body",
				description: `Expected payload for a login request`,
				type: "Body",
				schema: LoginRequest
			},
		],
		response: User,
	},
	{
		method: "post",
		path: "/auth/login_annon",
		alias: "LoginAnnonUser",
		requestFormat: "json",
		parameters: [
			{
				name: "body",
				description: `Expected payload for an annon login request`,
				type: "Body",
				schema: z.object({ username: z.string() }).passthrough()
			},
		],
		response: User,
	},
	{
		method: "post",
		path: "/auth/logout",
		alias: "LogoutUser",
		requestFormat: "json",
		response: z.record(z.string()),
	},
	{
		method: "post",
		path: "/auth/password_reset_create",
		alias: "PasswordResetCreate",
		requestFormat: "json",
		parameters: [
			{
				name: "body",
				type: "Body",
				schema: z.object({ email: z.string() }).passthrough()
			},
		],
		response: z.void(),
	},
	{
		method: "post",
		path: "/auth/password_reset_update",
		alias: "PasswordResetUpdate",
		requestFormat: "json",
		parameters: [
			{
				name: "body",
				type: "Body",
				schema: PasswordResetUpdateRequest
			},
		],
		response: z.void(),
	},
	{
		method: "post",
		path: "/auth/resend_verification_email",
		alias: "ResendVerificationEmail",
		requestFormat: "json",
		parameters: [
			{
				name: "body",
				type: "Body",
				schema: z.object({ id: z.string() }).passthrough()
			},
		],
		response: z.void(),
	},
	{
		method: "post",
		path: "/auth/signup",
		alias: "SignUp",
		requestFormat: "json",
		parameters: [
			{
				name: "body",
				description: `Expected payload for a signin request`,
				type: "Body",
				schema: SignupRequest
			},
		],
		response: User,
	},
	{
		method: "post",
		path: "/auth/signup_annon",
		alias: "SignupAnnonUser",
		requestFormat: "json",
		response: User,
	},
	{
		method: "get",
		path: "/auth/test_requires_roles/:conversation_id",
		alias: "TestRequiresRoles",
		requestFormat: "json",
		response: User,
	},
	{
		method: "post",
		path: "/auth/verify_email_token",
		alias: "VerifyEmailToken",
		requestFormat: "json",
		parameters: [
			{
				name: "body",
				type: "Body",
				schema: z.object({ token: z.string() }).passthrough()
			},
		],
		response: User,
	},
	{
		method: "get",
		path: "/conversation",
		alias: "ListConverastions",
		description: `List conversations`,
		requestFormat: "json",
		parameters: [
			{
				name: "created_after",
				type: "Query",
				schema: created_after
			},
			{
				name: "created_before",
				type: "Query",
				schema: created_after
			},
			{
				name: "is_complete",
				type: "Query",
				schema: is_complete
			},
			{
				name: "is_invite_only",
				type: "Query",
				schema: is_complete
			},
			{
				name: "is_public",
				type: "Query",
				schema: is_complete
			},
			{
				name: "owner_id",
				type: "Query",
				schema: created_after
			},
			{
				name: "title",
				type: "Query",
				schema: created_after
			},
			{
				name: "limit",
				type: "Query",
				schema: limit
			},
			{
				name: "offset",
				type: "Query",
				schema: limit
			},
		],
		response: PaginatedResults_for_LocalisedConversation,
	},
	{
		method: "post",
		path: "/conversation",
		alias: "CreateConversation",
		description: `Creates a new conversation`,
		requestFormat: "json",
		parameters: [
			{
				name: "body",
				type: "Body",
				schema: CreateConversation
			},
		],
		response: LocalisedConversation,
	},
	{
		method: "get",
		path: "/conversation/:conversation_id",
		alias: "GetConversation",
		description: `Get a conversation by id or slug. If user is admin and withTranslations&#x3D;true, returns detailed translation data.`,
		requestFormat: "json",
		parameters: [
			{
				name: "withTranslations",
				type: "Query",
				schema: z.boolean().optional().default(false)
			},
		],
		response: ConversationResponse,
	},
	{
		method: "put",
		path: "/conversation/:conversation_id",
		alias: "UpdateConversation",
		description: `Update a conversation`,
		requestFormat: "json",
		parameters: [
			{
				name: "body",
				type: "Body",
				schema: PartialConversation
			},
		],
		response: LocalisedConversation,
	},
	{
		method: "delete",
		path: "/conversation/:conversation_id",
		alias: "DeleteConversation",
		description: `Delete the conversation and all related content`,
		requestFormat: "json",
		response: LocalisedConversation,
	},
	{
		method: "post",
		path: "/conversation/:conversation_id/email-updates",
		alias: "RegisterEmailForUpdates",
		description: `Allows non-logged-in users to register their email address to receive updates about a public conversation. If the email is already registered, returns existing registration.`,
		requestFormat: "json",
		parameters: [
			{
				name: "body",
				type: "Body",
				schema: RegisterEmailRequest
			},
		],
		response: RegisterEmailResponse,
	},
	{
		method: "get",
		path: "/conversation/:conversation_id/feedback",
		alias: "ListFeedbackForConversation",
		requestFormat: "json",
		response: Feedback,
	},
	{
		method: "post",
		path: "/conversation/:conversation_id/feedback",
		alias: "CreateFeedback",
		requestFormat: "json",
		parameters: [
			{
				name: "body",
				type: "Body",
				schema: z.object({ content: z.string() }).passthrough()
			},
		],
		response: Feedback,
	},
	{
		method: "put",
		path: "/conversation/:conversation_id/feedback/:feedback_id",
		alias: "UpdateFeedback",
		requestFormat: "json",
		parameters: [
			{
				name: "body",
				type: "Body",
				schema: PartialFeedback
			},
		],
		response: Feedback,
	},
	{
		method: "get",
		path: "/conversation/:conversation_id/invite",
		alias: "ListInvitesForConversation",
		requestFormat: "json",
		response: z.array(Invite),
	},
	{
		method: "post",
		path: "/conversation/:conversation_id/invite",
		alias: "CreateInvite",
		requestFormat: "json",
		parameters: [
			{
				name: "body",
				type: "Body",
				schema: CreateInviteDTO
			},
		],
		response: Invite,
	},
	{
		method: "get",
		path: "/conversation/:conversation_id/invite/:invite_id",
		alias: "GetInvite",
		requestFormat: "json",
		response: Invite,
	},
	{
		method: "delete",
		path: "/conversation/:conversation_id/invite/:invite_id",
		alias: "DeleteInvite",
		requestFormat: "json",
		response: Invite,
	},
	{
		method: "post",
		path: "/conversation/:conversation_id/invite/:invite_id/accept",
		alias: "AcceptInvite",
		requestFormat: "json",
		response: Invite,
	},
	{
		method: "post",
		path: "/conversation/:conversation_id/invite/:invite_id/reject",
		alias: "RejectInvite",
		requestFormat: "json",
		response: Invite,
	},
	{
		method: "get",
		path: "/conversation/:conversation_id/invite/:invite_id/stats",
		alias: "GetInviteStats",
		requestFormat: "json",
		response: z.array(DailyResponseStats),
	},
	{
		method: "post",
		path: "/conversation/:conversation_id/notifications",
		alias: "SendNotificationToParticipants",
		description: `Creates a notification and sends it to all users participating in workflows within the conversation. Only conversation owners can send notifications.`,
		requestFormat: "json",
		parameters: [
			{
				name: "body",
				type: "Body",
				schema: SendNotificationRequest
			},
		],
		response: z.unknown(),
	},
	{
		method: "get",
		path: "/conversation/:conversation_id/report",
		alias: "GetReportForConversation",
		requestFormat: "json",
		response: FullReportDTO,
	},
	{
		method: "put",
		path: "/conversation/:conversation_id/report",
		alias: "UpdateReport",
		requestFormat: "json",
		parameters: [
			{
				name: "body",
				type: "Body",
				schema: PartialReport
			},
		],
		response: Report,
	},
	{
		method: "post",
		path: "/conversation/:conversation_id/report",
		alias: "GenerateReportForConversation",
		requestFormat: "json",
		response: FullReportDTO,
	},
	{
		method: "get",
		path: "/conversation/:conversation_id/report/:report_id/impacts",
		alias: "ListImpactsForReport",
		requestFormat: "json",
		response: ReportImpact,
	},
	{
		method: "put",
		path: "/conversation/:conversation_id/report/:report_id/impacts",
		alias: "UpdateImpact",
		requestFormat: "json",
		parameters: [
			{
				name: "body",
				type: "Body",
				schema: PartialReportImpact
			},
		],
		response: ReportImpact,
	},
	{
		method: "post",
		path: "/conversation/:conversation_id/report/:report_id/impacts",
		alias: "CreateImpact",
		requestFormat: "json",
		parameters: [
			{
				name: "body",
				type: "Body",
				schema: CreateImpactDTO
			},
		],
		response: ReportImpact,
	},
	{
		method: "get",
		path: "/conversation/:conversation_id/workflow",
		alias: "ListWorkflows",
		requestFormat: "json",
		response: z.array(Workflow),
	},
	{
		method: "post",
		path: "/conversation/:conversation_id/workflow",
		alias: "CreateWorkflow",
		requestFormat: "json",
		parameters: [
			{
				name: "body",
				type: "Body",
				schema: CreateWorkflow
			},
		],
		response: Workflow,
	},
	{
		method: "get",
		path: "/conversation/:conversation_id/workflow/:workflow_id",
		alias: "GetWorkflow",
		requestFormat: "json",
		response: Workflow,
	},
	{
		method: "put",
		path: "/conversation/:conversation_id/workflow/:workflow_id",
		alias: "UpdateWorkflow",
		requestFormat: "json",
		parameters: [
			{
				name: "body",
				type: "Body",
				schema: PartialWorkflow
			},
		],
		response: Workflow,
	},
	{
		method: "delete",
		path: "/conversation/:conversation_id/workflow/:workflow_id",
		alias: "DeleteWorkflow",
		requestFormat: "json",
		response: Workflow,
	},
	{
		method: "delete",
		path: "/conversation/:conversation_id/workflow/:workflow_id/leave",
		alias: "UnregisterUserForWorkflow",
		requestFormat: "json",
		response: UserParticipation,
	},
	{
		method: "get",
		path: "/conversation/:conversation_id/workflow/:workflow_id/next",
		alias: "NextWorkflowStepForUser",
		requestFormat: "json",
		response: z.union([WorkflowStep, z.null()]),
	},
	{
		method: "get",
		path: "/conversation/:conversation_id/workflow/:workflow_id/participation",
		alias: "GetUserParticipation",
		requestFormat: "json",
		response: z.union([UserParticipation, z.null()]),
	},
	{
		method: "get",
		path: "/conversation/:conversation_id/workflow/:workflow_id/progress",
		alias: "GetUserProgress",
		requestFormat: "json",
		response: z.array(UserProgress),
	},
	{
		method: "put",
		path: "/conversation/:conversation_id/workflow/:workflow_id/progress/:workflow_step_id",
		alias: "SetUserProgress",
		requestFormat: "json",
		parameters: [
			{
				name: "body",
				description: `Defines the type of authentication has been used to create The user`,
				type: "Body",
				schema: z.enum(["not_started", "in_progress", "done"])
			},
		],
		response: UserProgress,
	},
	{
		method: "post",
		path: "/conversation/:conversation_id/workflow/:workflow_id/register",
		alias: "RegisterUserForWorkflow",
		requestFormat: "json",
		response: UserParticipation,
	},
	{
		method: "get",
		path: "/conversation/:conversation_id/workflow/:workflow_id/stats",
		alias: "GetWorkflowStats",
		requestFormat: "json",
		response: WorkflowStats,
	},
	{
		method: "get",
		path: "/conversation/:conversation_id/workflow/:workflow_id/workflow_step",
		alias: "ListWorkflowSteps",
		requestFormat: "json",
		response: z.array(LocalisedWorkflowStep),
	},
	{
		method: "post",
		path: "/conversation/:conversation_id/workflow/:workflow_id/workflow_step",
		alias: "CreateWorkflowStep",
		requestFormat: "json",
		parameters: [
			{
				name: "body",
				type: "Body",
				schema: CreateWorkflowStep
			},
		],
		response: LocalisedWorkflowStep,
	},
	{
		method: "get",
		path: "/conversation/:conversation_id/workflow/:workflow_id/workflow_step/:workflow_step_id",
		alias: "GetWorkflowStep",
		requestFormat: "json",
		response: LocalisedWorkflowStep,
	},
	{
		method: "put",
		path: "/conversation/:conversation_id/workflow/:workflow_id/workflow_step/:workflow_step_id",
		alias: "UpdateWorkflowStep",
		requestFormat: "json",
		parameters: [
			{
				name: "body",
				type: "Body",
				schema: PartialWorkflowStep
			},
		],
		response: WorkflowStep,
	},
	{
		method: "delete",
		path: "/conversation/:conversation_id/workflow/:workflow_id/workflow_step/:workflow_step_id",
		alias: "DeleteWorkflowStep",
		requestFormat: "json",
		response: WorkflowStep,
	},
	{
		method: "get",
		path: "/docs",
		alias: "getDocs",
		description: `This documentation page.`,
		requestFormat: "json",
		response: z.void(),
	},
	{
		method: "get",
		path: "/docs/redoc",
		alias: "getDocsredoc",
		description: `This documentation page.`,
		requestFormat: "json",
		response: z.void(),
	},
	{
		method: "get",
		path: "/docs/swagger",
		alias: "getDocsswagger",
		description: `This documentation page.`,
		requestFormat: "json",
		response: z.void(),
	},
	{
		method: "get",
		path: "/notifications",
		alias: "getNotifications",
		description: `Returns a paginated list of all notification deliveries for the authenticated user`,
		requestFormat: "json",
		parameters: [
			{
				name: "limit",
				type: "Query",
				schema: limit
			},
			{
				name: "offset",
				type: "Query",
				schema: limit
			},
		],
		response: PaginatedResults_for_NotificationWithDelivery,
	},
	{
		method: "put",
		path: "/notifications/delivery/:delivery_id/read",
		alias: "MarkNotificationAsRead",
		description: `Marks a specific notification delivery as read for the current user`,
		requestFormat: "json",
		response: NotificationDelivery,
	},
	{
		method: "put",
		path: "/notifications/read-all",
		alias: "MarkAllNotificationsAsRead",
		description: `Marks all unread notification deliveries as read for the current user`,
		requestFormat: "json",
		response: z.unknown(),
	},
	{
		method: "get",
		path: "/notifications/unread",
		alias: "getNotificationsunread",
		description: `Returns a paginated list of unread notification deliveries for the authenticated user`,
		requestFormat: "json",
		parameters: [
			{
				name: "limit",
				type: "Query",
				schema: limit
			},
			{
				name: "offset",
				type: "Query",
				schema: limit
			},
		],
		response: PaginatedResults_for_NotificationWithDelivery,
	},
	{
		method: "get",
		path: "/notifications/unread/count",
		alias: "getNotificationsunreadcount",
		description: `Returns the count of unread notifications for the authenticated user`,
		requestFormat: "json",
		response: z.object({ count: z.number().int() }).passthrough(),
	},
	{
		method: "get",
		path: "/tools/polis/admin_login",
		alias: "PolisAdminLogin",
		description: `Used to login the current user to the specified workflow id polis`,
		requestFormat: "json",
		parameters: [
			{
				name: "workflow_step_id",
				type: "Query",
				schema: z.string().uuid()
			},
		],
		response: z.string(),
	},
	{
		method: "post",
		path: "/translations",
		alias: "CreateTextContent",
		description: `Create a new TextContent entry that can hold translations`,
		requestFormat: "json",
		parameters: [
			{
				name: "body",
				type: "Body",
				schema: CreateTextContentRequest
			},
		],
		response: TextContent,
	},
	{
		method: "get",
		path: "/translations/:text_content_id",
		alias: "GetTextContentWithTranslations",
		description: `Get a TextContent entry with all its translations`,
		requestFormat: "json",
		response: TextContentWithTranslations,
	},
	{
		method: "put",
		path: "/translations/:text_content_id",
		alias: "UpdateTextContent",
		description: `Update a TextContent entry`,
		requestFormat: "json",
		parameters: [
			{
				name: "body",
				description: `Data transfer object for updating existing text content.

This struct contains optional fields that can be updated on a TextContent record. Only the provided (Some) fields will be updated in the database.`,
				type: "Body",
				schema: UpdateTextContent
			},
		],
		response: TextContent,
	},
	{
		method: "delete",
		path: "/translations/:text_content_id",
		alias: "DeleteTextContent",
		description: `Delete a TextContent entry and all its translations`,
		requestFormat: "json",
		response: TextContent,
	},
	{
		method: "get",
		path: "/translations/:text_content_id/:locale",
		alias: "GetTextTranslation",
		description: `Get a translation for a specific TextContent and locale`,
		requestFormat: "json",
		response: TextTranslation,
	},
	{
		method: "put",
		path: "/translations/:text_content_id/:locale",
		alias: "UpdateTextTranslation",
		description: `Update an existing translation for a specific locale`,
		requestFormat: "json",
		parameters: [
			{
				name: "body",
				description: `Data transfer object for updating existing text translations.

This struct contains optional fields that can be updated on a TextTranslation record. Only the provided (Some) fields will be updated in the database.`,
				type: "Body",
				schema: UpdateTextTranslation
			},
		],
		response: TextTranslation,
	},
	{
		method: "post",
		path: "/translations/:text_content_id/:locale",
		alias: "CreateOrUpdateTextTranslation",
		description: `Create a new translation or update existing one for a specific locale`,
		requestFormat: "json",
		parameters: [
			{
				name: "body",
				type: "Body",
				schema: CreateOrUpdateTextTranslationRequest
			},
		],
		response: TextTranslation,
	},
	{
		method: "delete",
		path: "/translations/:text_content_id/:locale",
		alias: "DeleteTextTranslation",
		description: `Delete a translation for a specific locale`,
		requestFormat: "json",
		response: TextTranslation,
	},
	{
		method: "post",
		path: "/translations/:text_content_id/:locale/translate",
		alias: "AutomaticallyGenerateTranslation",
		description: `Use the primary_locale language and translate this language from it using the tarnslation service`,
		requestFormat: "json",
		response: TextTranslation,
	},
	{
		method: "post",
		path: "/translations/:text_content_id/translate",
		alias: "GenerateAllTranslations",
		description: `Use the default locale content as the reference text and generate automatic translations for each language form it`,
		requestFormat: "json",
		response: TextContentWithTranslations,
	},
	{
		method: "get",
		path: "/user/conversations",
		alias: "GetConversationsUserIsParticipatingIn",
		description: `Returns a list of all the conversations the user has taken part in`,
		requestFormat: "json",
		response: z.array(Conversation),
	},
	{
		method: "put",
		path: "/user/details",
		alias: "UpdateUserDetails",
		description: `Update user details (username and/or password)`,
		requestFormat: "json",
		parameters: [
			{
				name: "body",
				type: "Body",
				schema: UpdateUserRequest
			},
		],
		response: User,
	},
	{
		method: "get",
		path: "/user/owned_conversations",
		alias: "GetOwnedConversations",
		description: `Gets a list of the conversations a user owns`,
		requestFormat: "json",
		parameters: [
			{
				name: "created_after",
				type: "Query",
				schema: created_after
			},
			{
				name: "created_before",
				type: "Query",
				schema: created_after
			},
			{
				name: "is_complete",
				type: "Query",
				schema: is_complete
			},
			{
				name: "is_invite_only",
				type: "Query",
				schema: is_complete
			},
			{
				name: "is_public",
				type: "Query",
				schema: is_complete
			},
			{
				name: "owner_id",
				type: "Query",
				schema: created_after
			},
			{
				name: "title",
				type: "Query",
				schema: created_after
			},
			{
				name: "limit",
				type: "Query",
				schema: limit
			},
			{
				name: "offset",
				type: "Query",
				schema: limit
			},
		],
		response: PaginatedResults_for_LocalisedConversation,
	},
	{
		method: "get",
		path: "/user/preferences",
		alias: "GetAllUserConversationPreferences",
		description: `Returns all conversation notification preferences for the authenticated user`,
		requestFormat: "json",
		response: z.array(UserConversationPreferences),
	},
	{
		method: "get",
		path: "/user/preferences/conversation/:conversation_id",
		alias: "GetUserPreferenceForConversation",
		description: `Returns the notification preferences for a specific conversation`,
		requestFormat: "json",
		response: UserConversationPreferences,
	},
	{
		method: "put",
		path: "/user/preferences/conversation/:conversation_id",
		alias: "UpdateUserPreferenceForConversation",
		description: `Updates notification preferences for a specific conversation`,
		requestFormat: "json",
		parameters: [
			{
				name: "body",
				type: "Body",
				schema: UpdateUserConversationPreferences
			},
		],
		response: UserConversationPreferences,
	},
	{
		method: "get",
		path: "/user/roles",
		alias: "GetUserRoles",
		description: `Gets a list of roles the current user has`,
		requestFormat: "json",
		response: z.array(UserRoles),
	},
	{
		method: "put",
		path: "/user/upgrade",
		alias: "UpgradeAccount",
		description: `Upgrade anonymous account to email/password account`,
		requestFormat: "json",
		parameters: [
			{
				name: "body",
				type: "Body",
				schema: UpgradeAccountRequest
			},
		],
		response: User,
	},
	{
		method: "post",
		path: "/ws/broadcast",
		alias: "BroadcastMessage",
		requestFormat: "json",
		parameters: [
			{
				name: "body",
				type: "Body",
				schema: BroadcastMessage
			},
		],
		response: BroadcastResponse,
	},
	{
		method: "post",
		path: "/ws/broadcast/:workflow_id",
		alias: "BroadcastMessageToWorkflowParticipants",
		requestFormat: "json",
		parameters: [
			{
				name: "body",
				type: "Body",
				schema: BroadcastMessage
			},
		],
		response: BroadcastResponse,
	},
	{
		method: "post",
		path: "/ws/send",
		alias: "SendToUser",
		requestFormat: "json",
		parameters: [
			{
				name: "body",
				type: "Body",
				schema: SendToUserMessage
			},
		],
		response: BroadcastResponse,
	},
	{
		method: "get",
		path: "/ws/stats",
		alias: "GetWebSocketStats",
		requestFormat: "json",
		response: WebSocketStats,
	},
]);

export const api = new Zodios(endpoints);

export function createApiClient(baseUrl: string, options?: ZodiosOptions) {
    return new Zodios(baseUrl, endpoints, options);
}

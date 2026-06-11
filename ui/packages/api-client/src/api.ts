import {
  makeApi,
  Zodios,
  type ZodiosOptions,
  type ZodiosInstance,
  type ZodiosEndpointDefinitions,
} from "@zodios/core";
import { z } from "zod";

export const AnnonLoginRequest = z
  .object({ username: z.string() })
  .passthrough();
export type AnnonLoginRequest = z.infer<typeof AnnonLoginRequest>;
export const UserAuthType = z.enum([
  "annon",
  "email_password",
  "otp",
  "scot_account",
]);
export type UserAuthType = z.infer<typeof UserAuthType>;
export const UserDto = z
  .object({
    authType: UserAuthType,
    avatarUrl: z.union([z.string(), z.null()]).optional(),
    email: z.union([z.string(), z.null()]).optional(),
    emailVerified: z.boolean(),
    id: z.string().uuid(),
    organizationId: z.union([z.string(), z.null()]).optional(),
    username: z.union([z.string(), z.null()]).optional(),
  })
  .passthrough();
export type UserDto = z.infer<typeof UserDto>;
export const LoginRequest = z
  .object({ email: z.string(), password: z.string() })
  .passthrough();
export type LoginRequest = z.infer<typeof LoginRequest>;
export const OtpLoginRequest = z
  .object({ code: z.string(), email: z.string() })
  .passthrough();
export type OtpLoginRequest = z.infer<typeof OtpLoginRequest>;
export const SignupRequest = z
  .object({
    avatar_url: z.union([z.string(), z.null()]).optional(),
    email: z.string(),
    password: z.string(),
    username: z.string(),
  })
  .passthrough();
export type SignupRequest = z.infer<typeof SignupRequest>;
export const OtpSignupRequest = z
  .object({
    email: z.string(),
    username: z.union([z.string(), z.null()]).optional(),
  })
  .passthrough();
export type OtpSignupRequest = z.infer<typeof OtpSignupRequest>;
export const CreateOtpRequest = z
  .object({
    email: z.string(),
    redirect_url: z.union([z.string(), z.null()]).optional(),
  })
  .passthrough();
export type CreateOtpRequest = z.infer<typeof CreateOtpRequest>;
export const VerifyOtpTokenRequest = z
  .object({ token: z.string() })
  .passthrough();
export type VerifyOtpTokenRequest = z.infer<typeof VerifyOtpTokenRequest>;
export const VerifyEmailTokenRequest = z
  .object({ token: z.string() })
  .passthrough();
export type VerifyEmailTokenRequest = z.infer<typeof VerifyEmailTokenRequest>;
export const ResendVerificationEmailRequest = z
  .object({ id: z.string() })
  .passthrough();
export type ResendVerificationEmailRequest = z.infer<
  typeof ResendVerificationEmailRequest
>;
export const CreatePasswordResetRequest = z
  .object({ email: z.string() })
  .passthrough();
export type CreatePasswordResetRequest = z.infer<
  typeof CreatePasswordResetRequest
>;
export const PasswordResetUpdateRequest = z
  .object({
    confirm_password: z.string(),
    password: z.string(),
    token: z.string(),
  })
  .passthrough();
export type PasswordResetUpdateRequest = z.infer<
  typeof PasswordResetUpdateRequest
>;
export const ResourceType = z.union([
  z.literal("Site"),
  z.object({ Conversation: z.string().uuid() }),
]);
export type ResourceType = z.infer<typeof ResourceType>;
export const ResourceRole = z.enum(["Admin", "SuperAdmin"]);
export type ResourceRole = z.infer<typeof ResourceRole>;
export const UserRoles = z
  .object({ resource: ResourceType, roles: z.array(ResourceRole) })
  .passthrough();
export type UserRoles = z.infer<typeof UserRoles>;
export const LocalizedConversationDto = z
  .object({
    callToAction: z.union([z.string(), z.null()]).optional(),
    chatBotId: z.union([z.string(), z.null()]).optional(),
    description: z.string(),
    enableQaChatBot: z.boolean(),
    enableSignupPrompts: z.boolean(),
    faqs: z.union([z.string(), z.null()]).optional(),
    id: z.string().uuid(),
    imageUrl: z.string(),
    isComplete: z.boolean(),
    isInviteOnly: z.boolean(),
    isLive: z.boolean(),
    isPublic: z.boolean(),
    knowledgeBaseId: z.union([z.string(), z.null()]).optional(),
    organizationId: z.union([z.string(), z.null()]).optional(),
    primaryLocale: z.string(),
    privacyPolicy: z.union([z.string(), z.null()]).optional(),
    shortDescription: z.string(),
    shortPrivacyPolicy: z.union([z.string(), z.null()]).optional(),
    showThankYouPageAnnonInstructions: z.boolean(),
    slug: z.union([z.string(), z.null()]).optional(),
    supportedLanguages: z.array(z.string()),
    tags: z.array(z.string()),
    thankYouMessage: z.union([z.string(), z.null()]).optional(),
    title: z.string(),
    videoUrl: z.union([z.string(), z.null()]).optional(),
  })
  .passthrough();
export type LocalizedConversationDto = z.infer<typeof LocalizedConversationDto>;
export const created_after = z.union([z.string(), z.null()]).optional();
export type created_after = z.infer<typeof created_after>;
export const is_complete = z.union([z.boolean(), z.null()]).optional();
export type is_complete = z.infer<typeof is_complete>;
export const limit = z.union([z.number(), z.null()]).optional();
export type limit = z.infer<typeof limit>;
export const PaginatedResults_for_LocalizedConversationDto = z
  .object({
    records: z.array(LocalizedConversationDto),
    total: z.number().int(),
  })
  .passthrough();
export type PaginatedResults_for_LocalizedConversationDto = z.infer<
  typeof PaginatedResults_for_LocalizedConversationDto
>;
export const UpdateUserRequest = z
  .object({
    email_verified: z.union([z.boolean(), z.null()]),
    organization_id: z.union([z.string(), z.null()]),
    password: z.union([z.string(), z.null()]),
    username: z.union([z.string(), z.null()]),
  })
  .partial()
  .passthrough();
export type UpdateUserRequest = z.infer<typeof UpdateUserRequest>;
export const UpgradeAccountRequest = z
  .object({ email: z.string(), password: z.string(), username: z.string() })
  .passthrough();
export type UpgradeAccountRequest = z.infer<typeof UpgradeAccountRequest>;
export const UserConversationPreferencesDto = z
  .object({
    conversationId: z.string().uuid(),
    id: z.string().uuid(),
    receiveSimilarConversationUpdatesByEmail: z.boolean(),
    receiveSimilarConversationUpdatesByNotification: z.boolean(),
    receiveUpdatesByEmail: z.boolean(),
    receiveUpdatesByNotification: z.boolean(),
    userId: z.string().uuid(),
  })
  .passthrough();
export type UserConversationPreferencesDto = z.infer<
  typeof UserConversationPreferencesDto
>;
export const UpdateUserConversationPreferences = z
  .object({
    receiveSimilarConversationUpdatesByEmail: z.union([z.boolean(), z.null()]),
    receiveSimilarConversationUpdatesByNotification: z.union([
      z.boolean(),
      z.null(),
    ]),
    receiveUpdatesByEmail: z.union([z.boolean(), z.null()]),
    receiveUpdatesByNotification: z.union([z.boolean(), z.null()]),
  })
  .partial()
  .passthrough();
export type UpdateUserConversationPreferences = z.infer<
  typeof UpdateUserConversationPreferences
>;
export const UserProfileDto = z
  .object({
    age: z.union([z.number(), z.null()]).optional(),
    consented: z.boolean(),
    createdAt: z.string().datetime({ offset: true }),
    ethnicity: z.union([z.string(), z.null()]).optional(),
    gender: z.union([z.string(), z.null()]).optional(),
    id: z.string().uuid(),
    politicalParty: z.union([z.string(), z.null()]).optional(),
    updatedAt: z.string().datetime({ offset: true }),
    userId: z.string().uuid(),
    zipcode: z.union([z.string(), z.null()]).optional(),
  })
  .passthrough();
export type UserProfileDto = z.infer<typeof UserProfileDto>;
export const UpsertUserProfileRequest = z
  .object({
    age: z.union([z.number(), z.null()]),
    consented: z.union([z.boolean(), z.null()]),
    ethnicity: z.union([z.string(), z.null()]),
    gender: z.union([z.string(), z.null()]),
    politicalParty: z.union([z.string(), z.null()]),
    zipcode: z.union([z.string(), z.null()]),
  })
  .partial()
  .passthrough();
export type UpsertUserProfileRequest = z.infer<typeof UpsertUserProfileRequest>;
export const DeliveryMethod = z.enum(["in_app", "email"]);
export type DeliveryMethod = z.infer<typeof DeliveryMethod>;
export const NotificationContextType = z.enum(["site", "conversation"]);
export type NotificationContextType = z.infer<typeof NotificationContextType>;
export const NotificationType = z.enum(["info", "warning", "error", "success"]);
export type NotificationType = z.infer<typeof NotificationType>;
export const NotificationDto = z
  .object({
    content: z.string(),
    contextId: z.union([z.string(), z.null()]).optional(),
    contextType: NotificationContextType,
    createdAt: z.string().datetime({ offset: true }),
    id: z.string().uuid(),
    notificationType: NotificationType,
    title: z.string(),
  })
  .passthrough();
export type NotificationDto = z.infer<typeof NotificationDto>;
export const NotificationWithDelivery = z
  .object({
    createdAt: z.string().datetime({ offset: true }),
    deliveredAt: z.string().datetime({ offset: true }),
    deliveryMethod: DeliveryMethod,
    id: z.string().uuid(),
    notification: NotificationDto,
    notificationId: z.string().uuid(),
    readAt: z.union([z.string(), z.null()]).optional(),
    userId: z.string().uuid(),
  })
  .passthrough();
export type NotificationWithDelivery = z.infer<typeof NotificationWithDelivery>;
export const PaginatedResults_for_NotificationWithDelivery = z
  .object({
    records: z.array(NotificationWithDelivery),
    total: z.number().int(),
  })
  .passthrough();
export type PaginatedResults_for_NotificationWithDelivery = z.infer<
  typeof PaginatedResults_for_NotificationWithDelivery
>;
export const UnreadCount = z.object({ count: z.number().int() }).passthrough();
export type UnreadCount = z.infer<typeof UnreadCount>;
export const NotificationDelivery = z
  .object({
    created_at: z.string().datetime({ offset: true }),
    delivered_at: z.string().datetime({ offset: true }),
    delivery_method: DeliveryMethod,
    id: z.string().uuid(),
    notification_id: z.string().uuid(),
    read_at: z.union([z.string(), z.null()]).optional(),
    updated_at: z.string().datetime({ offset: true }),
    user_id: z.string().uuid(),
  })
  .passthrough();
export type NotificationDelivery = z.infer<typeof NotificationDelivery>;
export const TextFormat = z.union([
  z.literal("plain"),
  z.literal("markdown"),
  z.literal("rich"),
]);
export type TextFormat = z.infer<typeof TextFormat>;
export const CreateTextContentRequest = z
  .object({
    content: z.string(),
    format: TextFormat,
    primary_locale: z.string(),
  })
  .passthrough();
export type CreateTextContentRequest = z.infer<typeof CreateTextContentRequest>;
export const TextContentDto = z
  .object({
    format: TextFormat,
    id: z.string().uuid(),
    primaryLocale: z.string(),
  })
  .passthrough();
export type TextContentDto = z.infer<typeof TextContentDto>;
export const TextTranslationDto = z
  .object({
    aiGenerated: z.boolean(),
    content: z.string(),
    contentId: z.string().uuid(),
    id: z.string().uuid(),
    locale: z.string(),
    requiresValidation: z.boolean(),
  })
  .passthrough();
export type TextTranslationDto = z.infer<typeof TextTranslationDto>;
export const TextContentWithTranslations = z
  .object({
    format: TextFormat,
    id: z.string().uuid(),
    primaryLocale: z.string(),
    translations: z.array(TextTranslationDto),
  })
  .passthrough();
export type TextContentWithTranslations = z.infer<
  typeof TextContentWithTranslations
>;
export const UpdateTextContent = z
  .object({
    format: z.union([TextFormat, z.null()]),
    primary_locale: z.union([z.string(), z.null()]),
  })
  .partial()
  .passthrough();
export type UpdateTextContent = z.infer<typeof UpdateTextContent>;
export const UpdateTextTranslation = z
  .object({
    ai_generated: z.union([z.boolean(), z.null()]),
    content: z.union([z.string(), z.null()]),
    locale: z.union([z.string(), z.null()]),
    requires_validation: z.union([z.boolean(), z.null()]),
  })
  .partial()
  .passthrough();
export type UpdateTextTranslation = z.infer<typeof UpdateTextTranslation>;
export const CreateOrUpdateTextTranslationRequest = z
  .object({
    ai_generated: z.union([z.boolean(), z.null()]).optional(),
    content: z.string(),
    requires_validation: z.union([z.boolean(), z.null()]).optional(),
  })
  .passthrough();
export type CreateOrUpdateTextTranslationRequest = z.infer<
  typeof CreateOrUpdateTextTranslationRequest
>;
export const ModerationStatus = z.enum(["accepted", "rejected", "pending"]);
export type ModerationStatus = z.infer<typeof ModerationStatus>;
export const PolisStatementAux = z
  .object({
    created_at: z.string().datetime({ offset: true }),
    id: z.string().uuid(),
    is_seed: z.boolean(),
    moderation_reason: z.union([z.string(), z.null()]).optional(),
    moderation_status: ModerationStatus,
    polis_conversation_id: z.string(),
    polis_statement_id: z.number().int(),
    statement_text: z.string(),
    themes: z.array(z.string()),
    updated_at: z.string().datetime({ offset: true }),
    user_id: z.union([z.string(), z.null()]).optional(),
    visible_statement_when_submitted: z
      .union([z.string(), z.null()])
      .optional(),
    workflow_step_id: z.string().uuid(),
    zid: z.number().int(),
  })
  .passthrough();
export type PolisStatementAux = z.infer<typeof PolisStatementAux>;
export const CreatePolisStatementAux = z
  .object({
    is_seed: z.boolean(),
    moderation_reason: z.union([z.string(), z.null()]).optional(),
    moderation_status: ModerationStatus.optional(),
    polis_conversation_id: z.string(),
    polis_statement_id: z.number().int(),
    statement_text: z.string(),
    themes: z.array(z.string()),
    visible_statement_when_submitted: z
      .union([z.string(), z.null()])
      .optional(),
    workflow_step_id: z.string().uuid(),
    zid: z.number().int(),
  })
  .passthrough();
export type CreatePolisStatementAux = z.infer<typeof CreatePolisStatementAux>;
export const UpdatePolisStatementAux = z
  .object({
    moderation_reason: z.union([z.string(), z.null()]),
    moderation_status: z.union([ModerationStatus, z.null()]),
    statement_text: z.union([z.string(), z.null()]),
    themes: z.union([z.array(z.string()), z.null()]),
    visible_statement_when_submitted: z.union([z.string(), z.null()]),
  })
  .partial()
  .passthrough();
export type UpdatePolisStatementAux = z.infer<typeof UpdatePolisStatementAux>;
export const SyncStatementAuxRequest = z
  .object({ workflow_step_id: z.string().uuid() })
  .passthrough();
export type SyncStatementAuxRequest = z.infer<typeof SyncStatementAuxRequest>;
export const SyncStatementAuxResponse = z
  .object({
    skipped_invalid_xid: z.number().int().gte(0),
    statements: z.array(PolisStatementAux),
    synced: z.number().int().gte(0),
  })
  .passthrough();
export type SyncStatementAuxResponse = z.infer<typeof SyncStatementAuxResponse>;
export const ThemeStatistic = z
  .object({ count: z.number().int(), theme: z.string() })
  .passthrough();
export type ThemeStatistic = z.infer<typeof ThemeStatistic>;
export const Story = z
  .object({
    id: z.string().uuid(),
    transcript_id: z.union([z.string(), z.null()]).optional(),
    user_id: z.string().uuid(),
    video_id: z.string().uuid(),
    workflow_step_id: z.string().uuid(),
  })
  .passthrough();
export type Story = z.infer<typeof Story>;
export const ComhairleMessageReference = z
  .object({
    content: z.string(),
    dataset_id: z.string(),
    document_id: z.string(),
    document_name: z.string(),
    id: z.string(),
  })
  .passthrough();
export type ComhairleMessageReference = z.infer<
  typeof ComhairleMessageReference
>;
export const ComhairleSessionMessage = z
  .object({
    content: z.string(),
    id: z.string(),
    reference: z
      .union([z.array(ComhairleMessageReference), z.null()])
      .optional(),
    role: z.string(),
  })
  .passthrough();
export type ComhairleSessionMessage = z.infer<typeof ComhairleSessionMessage>;
export const ComhairleAgentSession = z
  .object({
    agent_id: z.string(),
    configuration: z.unknown(),
    id: z.string(),
    messages: z.array(ComhairleSessionMessage),
  })
  .passthrough();
export type ComhairleAgentSession = z.infer<typeof ComhairleAgentSession>;
export const ConversationRequest = z
  .object({ question: z.string() })
  .passthrough();
export type ConversationRequest = z.infer<typeof ConversationRequest>;
export const Translation = z
  .object({
    textContent: TextContentDto,
    textTranslations: z.array(TextTranslationDto),
  })
  .passthrough();
export type Translation = z.infer<typeof Translation>;
export const ProposalTranslations = z
  .object({ body: Translation, title: Translation })
  .passthrough();
export type ProposalTranslations = z.infer<typeof ProposalTranslations>;
export const ProposalWithTranslations = z
  .object({
    body: z.string(),
    createdAt: z.string().datetime({ offset: true }),
    id: z.string().uuid(),
    title: z.string(),
    translations: ProposalTranslations,
    updatedAt: z.string().datetime({ offset: true }),
    workflowStepId: z.string().uuid(),
  })
  .passthrough();
export type ProposalWithTranslations = z.infer<typeof ProposalWithTranslations>;
export const LocalizedProposalDto = z
  .object({
    body: z.string(),
    id: z.string().uuid(),
    title: z.string(),
    workflowStepId: z.string().uuid(),
  })
  .passthrough();
export type LocalizedProposalDto = z.infer<typeof LocalizedProposalDto>;
export const ProposalsListResponse = z.union([
  z.array(ProposalWithTranslations),
  z.array(LocalizedProposalDto),
]);
export type ProposalsListResponse = z.infer<typeof ProposalsListResponse>;
export const CreateProposalRequest = z
  .object({
    body: z.string(),
    title: z.string(),
    workflow_step_id: z.string().uuid(),
  })
  .passthrough();
export type CreateProposalRequest = z.infer<typeof CreateProposalRequest>;
export const ProposalDto = z
  .object({
    body: z.string().uuid(),
    id: z.string().uuid(),
    title: z.string().uuid(),
    workflowStepId: z.string().uuid(),
  })
  .passthrough();
export type ProposalDto = z.infer<typeof ProposalDto>;
export const ResponseValue = z.union([z.number(), z.string()]);
export type ResponseValue = z.infer<typeof ResponseValue>;
export const Response = z
  .object({ question_id: z.string().uuid(), value: ResponseValue })
  .passthrough();
export type Response = z.infer<typeof Response>;
export const QuestionResponses = z.array(Response);
export type QuestionResponses = z.infer<typeof QuestionResponses>;
export const ProposalResponseDto = z
  .object({
    id: z.string().uuid(),
    proposalId: z.string().uuid(),
    response: QuestionResponses,
    userId: z.string().uuid(),
  })
  .passthrough();
export type ProposalResponseDto = z.infer<typeof ProposalResponseDto>;
export const CreateResponse = z
  .object({ question_responses: z.array(Response) })
  .passthrough();
export type CreateResponse = z.infer<typeof CreateResponse>;
export const ConversationRequest2 = z
  .object({
    history: z.string(),
    question_intent: z.string(),
    starting_question: z.string(),
    workflow_step_id: z.string().uuid(),
  })
  .passthrough();
export type ConversationRequest2 = z.infer<typeof ConversationRequest2>;
export const AnswerStatus = z.enum(["pending", "approved", "declined"]);
export type AnswerStatus = z.infer<typeof AnswerStatus>;
export const status = z.union([AnswerStatus, z.null()]).optional();
export type status = z.infer<typeof status>;
export const ThinkingSpaceAnswerDto = z
  .object({
    answer: z.string(),
    id: z.string().uuid(),
    isFollowUp: z.boolean(),
    otherQuestions: z.array(z.string()),
    question: z.string(),
    rootQuestionId: z.union([z.string(), z.null()]).optional(),
    status: AnswerStatus,
    workflowStepId: z.string().uuid(),
  })
  .passthrough();
export type ThinkingSpaceAnswerDto = z.infer<typeof ThinkingSpaceAnswerDto>;
export const CreateAnswerRequest = z
  .object({
    answer: z.string(),
    is_follow_up: z.union([z.boolean(), z.null()]).optional(),
    other_questions: z.union([z.array(z.string()), z.null()]).optional(),
    question: z.string(),
    root_question_id: z.union([z.string(), z.null()]).optional(),
    workflow_step_id: z.string().uuid(),
  })
  .passthrough();
export type CreateAnswerRequest = z.infer<typeof CreateAnswerRequest>;
export const UpdateAnswer = z
  .object({
    answer: z.union([z.string(), z.null()]),
    status: z.union([AnswerStatus, z.null()]),
  })
  .partial()
  .passthrough();
export type UpdateAnswer = z.infer<typeof UpdateAnswer>;
export const GenerateThinkingSpaceSummary = z
  .object({ workflow_step_id: z.string().uuid() })
  .passthrough();
export type GenerateThinkingSpaceSummary = z.infer<
  typeof GenerateThinkingSpaceSummary
>;
export const ThinkingSpaceSummaryDto = z
  .object({
    id: z.string().uuid(),
    isAiGenerated: z.boolean(),
    summary: z.string(),
    userId: z.string().uuid(),
    workflowStepId: z.string().uuid(),
  })
  .passthrough();
export type ThinkingSpaceSummaryDto = z.infer<typeof ThinkingSpaceSummaryDto>;
export const UpdateCreateThinkingSpace = z
  .object({
    summary: z.string(),
    summary_id: z.union([z.string(), z.null()]).optional(),
    workflow_step_id: z.string().uuid(),
  })
  .passthrough();
export type UpdateCreateThinkingSpace = z.infer<
  typeof UpdateCreateThinkingSpace
>;
export const ThinkingSpaceFollowUpQuestionDto = z
  .object({
    followUpQuestions: z.array(z.string()),
    id: z.string().uuid(),
    rootQuestionId: z.string().uuid(),
    userId: z.string().uuid(),
    workflowStepId: z.string().uuid(),
  })
  .passthrough();
export type ThinkingSpaceFollowUpQuestionDto = z.infer<
  typeof ThinkingSpaceFollowUpQuestionDto
>;
export const CreateFollowUpQuestions = z
  .object({
    follow_up_questions: z.array(z.string()),
    root_question_id: z.string().uuid(),
    workflow_step_id: z.string().uuid(),
  })
  .passthrough();
export type CreateFollowUpQuestions = z.infer<typeof CreateFollowUpQuestions>;
export const UpdateFollowUpQuestions = z
  .object({ follow_up_questions: z.array(z.string()) })
  .passthrough();
export type UpdateFollowUpQuestions = z.infer<typeof UpdateFollowUpQuestions>;
export const CreateConversation = z
  .object({
    default_workflow_id: z.union([z.string(), z.null()]).optional(),
    description: z.string(),
    enable_qa_chat_bot: z.union([z.boolean(), z.null()]).optional(),
    image_url: z.string(),
    is_invite_only: z.boolean(),
    is_live: z.boolean(),
    is_public: z.boolean(),
    primary_locale: z.string(),
    short_description: z.string(),
    slug: z.union([z.string(), z.null()]).optional(),
    supported_languages: z.array(z.string()),
    tags: z.union([z.array(z.string()), z.null()]).optional(),
    title: z.string(),
    video_url: z.union([z.string(), z.null()]).optional(),
  })
  .passthrough();
export type CreateConversation = z.infer<typeof CreateConversation>;
export const ConversationDto = z
  .object({
    callToAction: z.union([z.string(), z.null()]).optional(),
    chatBotId: z.union([z.string(), z.null()]).optional(),
    description: z.string().uuid(),
    enableQaChatBot: z.boolean(),
    enableSignupPrompts: z.boolean(),
    faqs: z.union([z.string(), z.null()]).optional(),
    id: z.string().uuid(),
    imageUrl: z.string(),
    isComplete: z.boolean(),
    isInviteOnly: z.boolean(),
    isLive: z.boolean(),
    isPublic: z.boolean(),
    knowledgeBaseId: z.union([z.string(), z.null()]).optional(),
    organizationId: z.union([z.string(), z.null()]).optional(),
    primaryLocale: z.string(),
    privacyPolicy: z.union([z.string(), z.null()]).optional(),
    shortDescription: z.string().uuid(),
    shortPrivacyPolicy: z.union([z.string(), z.null()]).optional(),
    showThankYouPageAnnonInstructions: z.boolean(),
    slug: z.union([z.string(), z.null()]).optional(),
    supportedLanguages: z.array(z.string()),
    tags: z.array(z.string()),
    thankYouMessage: z.union([z.string(), z.null()]).optional(),
    title: z.string().uuid(),
    videoUrl: z.union([z.string(), z.null()]).optional(),
  })
  .passthrough();
export type ConversationDto = z.infer<typeof ConversationDto>;
export const Translation2 = z
  .object({
    textContent: TextContentDto,
    textTranslations: z.array(TextTranslationDto),
  })
  .passthrough();
export type Translation2 = z.infer<typeof Translation2>;
export const ConversationTranslations = z
  .object({
    callToAction: z.union([Translation2, z.null()]).optional(),
    description: Translation2,
    faqs: z.union([Translation2, z.null()]).optional(),
    privacyPolicy: z.union([Translation2, z.null()]).optional(),
    shortDescription: Translation2,
    shortPrivacyPolicy: z.union([Translation2, z.null()]).optional(),
    thankYouMessage: z.union([Translation2, z.null()]).optional(),
    title: Translation2,
  })
  .passthrough();
export type ConversationTranslations = z.infer<typeof ConversationTranslations>;
export const ConversationWithTranslations = z
  .object({
    callToAction: z.union([z.string(), z.null()]).optional(),
    chatBotId: z.union([z.string(), z.null()]).optional(),
    createdAt: z.string().datetime({ offset: true }),
    defaultWorkflowId: z.union([z.string(), z.null()]).optional(),
    description: z.string(),
    enableQaChatBot: z.boolean(),
    enableSignupPrompts: z.boolean(),
    faqs: z.union([z.string(), z.null()]).optional(),
    id: z.string().uuid(),
    imageUrl: z.string(),
    isComplete: z.boolean(),
    isInviteOnly: z.boolean(),
    isLive: z.boolean(),
    isPublic: z.boolean(),
    knowledgeBaseId: z.union([z.string(), z.null()]).optional(),
    organizationId: z.union([z.string(), z.null()]).optional(),
    ownerId: z.string().uuid(),
    primaryLocale: z.string(),
    privacyPolicy: z.union([z.string(), z.null()]).optional(),
    shortDescription: z.string(),
    shortPrivacyPolicy: z.union([z.string(), z.null()]).optional(),
    showThankYouPageAnnonInstructions: z.boolean(),
    slug: z.union([z.string(), z.null()]).optional(),
    supportedLanguages: z.array(z.string()),
    tags: z.array(z.string()),
    thankYouMessage: z.union([z.string(), z.null()]).optional(),
    title: z.string(),
    translations: ConversationTranslations,
    updatedAt: z.string().datetime({ offset: true }),
    videoUrl: z.union([z.string(), z.null()]).optional(),
  })
  .passthrough();
export type ConversationWithTranslations = z.infer<
  typeof ConversationWithTranslations
>;
export const ConversationResponse = z.union([
  ConversationWithTranslations,
  LocalizedConversationDto,
]);
export type ConversationResponse = z.infer<typeof ConversationResponse>;
export const PartialConversation = z
  .object({
    call_to_action: z.union([z.string(), z.null()]),
    chat_bot_id: z.union([z.string(), z.null()]),
    default_workflow_id: z.union([z.string(), z.null()]),
    description: z.union([z.string(), z.null()]),
    enable_qa_chat_bot: z.union([z.boolean(), z.null()]),
    enable_signup_prompts: z.union([z.boolean(), z.null()]),
    faqs: z.union([z.string(), z.null()]),
    image_url: z.union([z.string(), z.null()]),
    is_complete: z.union([z.boolean(), z.null()]),
    is_invite_only: z.union([z.boolean(), z.null()]),
    is_live: z.union([z.boolean(), z.null()]),
    is_public: z.union([z.boolean(), z.null()]),
    knowledge_base_id: z.union([z.string(), z.null()]),
    primary_locale: z.union([z.string(), z.null()]),
    privacy_policy: z.union([z.string(), z.null()]),
    short_description: z.union([z.string(), z.null()]),
    short_privacy_policy: z.union([z.string(), z.null()]),
    show_thank_you_page_annon_instructions: z.union([z.boolean(), z.null()]),
    slug: z.union([z.string(), z.null()]),
    supported_languages: z.union([z.array(z.string()), z.null()]),
    tags: z.union([z.array(z.string()), z.null()]),
    thank_you_message: z.union([z.string(), z.null()]),
    title: z.union([z.string(), z.null()]),
    video_url: z.union([z.string(), z.null()]),
  })
  .partial()
  .passthrough();
export type PartialConversation = z.infer<typeof PartialConversation>;
export const SendNotificationRequest = z
  .object({
    content: z.string(),
    delivery_method: z.union([DeliveryMethod, z.null()]).optional(),
    html_content: z.union([z.string(), z.null()]).optional(),
    notification_type: z.union([NotificationType, z.null()]).optional(),
    test_email_recipient: z.union([z.string(), z.null()]).optional(),
    title: z.string(),
  })
  .passthrough();
export type SendNotificationRequest = z.infer<typeof SendNotificationRequest>;
export const SendEmailNotificationResponse = z
  .object({
    failedRecipients: z.array(z.string()).optional().default([]),
    message: z.string(),
    notificationId: z.string().uuid(),
    participantsNotified: z.number().int(),
  })
  .passthrough();
export type SendEmailNotificationResponse = z.infer<
  typeof SendEmailNotificationResponse
>;
export const NotificationRecipientsResponse = z
  .object({
    emailRecipientCount: z.number().int(),
    emailRecipients: z.array(z.string()),
    participantCount: z.number().int(),
  })
  .passthrough();
export type NotificationRecipientsResponse = z.infer<
  typeof NotificationRecipientsResponse
>;
export const RegisterEmailRequest = z
  .object({
    email: z.string(),
    receive_similar_conversation_updates_by_email: z.boolean(),
    receive_updates_by_email: z.boolean(),
  })
  .passthrough();
export type RegisterEmailRequest = z.infer<typeof RegisterEmailRequest>;
export const RegisterEmailResponse = z
  .object({
    conversationId: z.string().uuid(),
    email: z.string(),
    id: z.string().uuid(),
    message: z.string(),
  })
  .passthrough();
export type RegisterEmailResponse = z.infer<typeof RegisterEmailResponse>;
export const WorkflowDto = z
  .object({
    autoLogin: z.boolean(),
    conversationId: z.union([z.string(), z.null()]).optional(),
    createdAt: z.string().datetime({ offset: true }),
    description: z.string(),
    eventId: z.union([z.string(), z.null()]).optional(),
    id: z.string().uuid(),
    isActive: z.boolean(),
    isPublic: z.boolean(),
    name: z.string(),
    regionId: z.union([z.string(), z.null()]).optional(),
  })
  .passthrough();
export type WorkflowDto = z.infer<typeof WorkflowDto>;
export const CreateWorkflow = z
  .object({
    auto_login: z.boolean(),
    description: z.string(),
    is_active: z.boolean(),
    is_public: z.boolean(),
    name: z.string(),
    region_id: z.union([z.string(), z.null()]).optional(),
  })
  .passthrough();
export type CreateWorkflow = z.infer<typeof CreateWorkflow>;
export const PartialWorkflow = z
  .object({
    auto_login: z.union([z.boolean(), z.null()]),
    description: z.union([z.string(), z.null()]),
    event_id: z.union([z.string(), z.null()]),
    is_active: z.union([z.boolean(), z.null()]),
    is_public: z.union([z.boolean(), z.null()]),
    name: z.union([z.string(), z.null()]),
    region_id: z.union([z.string(), z.null()]),
  })
  .partial()
  .passthrough();
export type PartialWorkflow = z.infer<typeof PartialWorkflow>;
export const ActivationRule = z.literal("manual");
export type ActivationRule = z.infer<typeof ActivationRule>;
export const LearnPage = z
  .object({ text_content_id: z.string().uuid() })
  .passthrough();
export type LearnPage = z.infer<typeof LearnPage>;
export const LocalizedPage = z
  .object({ content: z.string(), type: z.literal("markdown") })
  .passthrough();
export type LocalizedPage = z.infer<typeof LocalizedPage>;
export const LearnPageEntry = z.union([LearnPage, z.array(LocalizedPage)]);
export type LearnPageEntry = z.infer<typeof LearnPageEntry>;
export const Category = z
  .object({ label: z.string(), value: z.number() })
  .passthrough();
export type Category = z.infer<typeof Category>;
export const QuestionType = z.union([
  z.literal("text"),
  z.object({
    likert_scale: z.object({ categories: z.array(Category) }).passthrough(),
  }),
  z.object({
    continuous: z
      .object({
        max_label: z.string().default(""),
        max_value: z.number().default(10),
        min_label: z.string().default(""),
        min_value: z.number().default(0),
        sub_steps: z.number().int().default(10),
      })
      .partial()
      .passthrough(),
  }),
]);
export type QuestionType = z.infer<typeof QuestionType>;
export const Question = z
  .object({ id: z.string().uuid(), text: z.string(), type: QuestionType })
  .passthrough();
export type Question = z.infer<typeof Question>;
export const ThinkingSpaceQuestion = z
  .object({ id: z.string().uuid(), intent: z.string(), text: z.string() })
  .passthrough();
export type ThinkingSpaceQuestion = z.infer<typeof ThinkingSpaceQuestion>;
export const ToolConfig = z.union([
  z
    .object({
      admin_password: z.string(),
      admin_user: z.string(),
      poll_id: z.string(),
      required_votes: z.union([z.number(), z.null()]).optional(),
      server_url: z.string(),
      show_remaining_statements: z.boolean().optional().default(true),
      type: z.literal("polis"),
    })
    .passthrough(),
  z
    .object({ pages: z.array(LearnPageEntry), type: z.literal("learn") })
    .passthrough(),
  z
    .object({
      admin_password: z.string(),
      admin_user: z.string(),
      project_id: z.string(),
      server_url: z.string().optional().default("forms.comhairle.scot"),
      survey_id: z.string(),
      survey_url: z.string(),
      type: z.literal("heyform"),
      workspace_id: z.string(),
    })
    .passthrough(),
  z
    .object({
      max_time: z.number().int(),
      to_see: z.number().int(),
      type: z.literal("stories"),
    })
    .passthrough(),
  z
    .object({ topic: z.string(), type: z.literal("elicitationbot") })
    .passthrough(),
  z
    .object({
      questions: z.array(Question),
      randomize_order: z.boolean(),
      type: z.literal("prioritization"),
    })
    .passthrough(),
  z
    .object({
      follow_up_rounds_count: z.number().int().gte(0),
      root_questions: z.array(ThinkingSpaceQuestion),
      topic: z.string(),
      type: z.literal("thinkingspace"),
    })
    .passthrough(),
]);
export type ToolConfig = z.infer<typeof ToolConfig>;
export const WorkflowStep = z
  .object({
    activation_rule: ActivationRule,
    can_revisit: z.boolean(),
    created_at: z.string().datetime({ offset: true }),
    description: z.string().uuid(),
    id: z.string().uuid(),
    is_offline: z.boolean(),
    name: z.string().uuid(),
    preview_tool_config: ToolConfig,
    request_user_share_permission: z.boolean(),
    required: z.boolean(),
    step_order: z.number().int(),
    tool_config: z.union([ToolConfig, z.null()]).optional(),
    updated_at: z.string().datetime({ offset: true }),
    workflow_id: z.string().uuid(),
  })
  .passthrough();
export type WorkflowStep = z.infer<typeof WorkflowStep>;
export const DailySignupStats = z
  .object({
    day: z.string().datetime({ offset: true }),
    users: z.number().int(),
  })
  .passthrough();
export type DailySignupStats = z.infer<typeof DailySignupStats>;
export const WorkflowStepStats = z
  .object({
    completed: z.number().int(),
    id: z.string().uuid(),
    started: z.number().int(),
  })
  .passthrough();
export type WorkflowStepStats = z.infer<typeof WorkflowStepStats>;
export const WorkflowStats = z
  .object({
    signupStats: z.array(DailySignupStats),
    stepStats: z.array(WorkflowStepStats),
    totalUsers: z.number().int(),
  })
  .passthrough();
export type WorkflowStats = z.infer<typeof WorkflowStats>;
export const DemographicCategory = z
  .object({
    category: z.string(),
    count: z.number().int(),
    value: z.union([z.string(), z.null()]).optional(),
  })
  .passthrough();
export type DemographicCategory = z.infer<typeof DemographicCategory>;
export const DemographicReport = z
  .object({
    ageRanges: z.array(DemographicCategory),
    ethnicity: z.array(DemographicCategory),
    gender: z.array(DemographicCategory),
    politicalParty: z.array(DemographicCategory),
    totalParticipants: z.number().int(),
    zipcodeCounts: z.record(z.number().int()),
  })
  .passthrough();
export type DemographicReport = z.infer<typeof DemographicReport>;
export const UserParticipation = z
  .object({
    created_at: z.string().datetime({ offset: true }),
    id: z.string().uuid(),
    updated_at: z.string().datetime({ offset: true }),
    user_id: z.string().uuid(),
    workflow_id: z.string().uuid(),
  })
  .passthrough();
export type UserParticipation = z.infer<typeof UserParticipation>;
export const Translation3 = z
  .object({
    textContent: TextContentDto,
    textTranslations: z.array(TextTranslationDto),
  })
  .passthrough();
export type Translation3 = z.infer<typeof Translation3>;
export const WorkflowStepTranslations = z
  .object({ description: Translation3, name: Translation3 })
  .passthrough();
export type WorkflowStepTranslations = z.infer<typeof WorkflowStepTranslations>;
export const WorkflowStepWithTranslations = z
  .object({
    activationRule: ActivationRule,
    canRevisit: z.boolean(),
    createdAt: z.string().datetime({ offset: true }),
    description: z.string(),
    id: z.string().uuid(),
    isOffline: z.boolean(),
    name: z.string(),
    previewToolConfig: ToolConfig,
    requestUserSharePermission: z.boolean(),
    required: z.boolean(),
    stepOrder: z.number().int(),
    toolConfig: z.union([ToolConfig, z.null()]).optional(),
    translations: WorkflowStepTranslations,
    updatedAt: z.string().datetime({ offset: true }),
    workflowId: z.string().uuid(),
  })
  .passthrough();
export type WorkflowStepWithTranslations = z.infer<
  typeof WorkflowStepWithTranslations
>;
export const ProgressStatus = z.enum(["not_started", "in_progress", "done"]);
export type ProgressStatus = z.infer<typeof ProgressStatus>;
export const LocalizedWorkflowStepWithProgressDto = z
  .object({
    activationRule: ActivationRule,
    canRevisit: z.boolean(),
    description: z.string(),
    id: z.string().uuid(),
    isOffline: z.boolean(),
    name: z.string(),
    previewToolConfig: ToolConfig,
    progressStatus: ProgressStatus,
    requestUserSharePermission: z.boolean(),
    required: z.boolean(),
    stepOrder: z.number().int(),
    toolConfig: z.union([ToolConfig, z.null()]).optional(),
    workflowId: z.string().uuid(),
  })
  .passthrough();
export type LocalizedWorkflowStepWithProgressDto = z.infer<
  typeof LocalizedWorkflowStepWithProgressDto
>;
export const LocalizedWorkflowStepDto = z
  .object({
    activationRule: ActivationRule,
    canRevisit: z.boolean(),
    description: z.string(),
    id: z.string().uuid(),
    isOffline: z.boolean(),
    name: z.string(),
    previewToolConfig: ToolConfig,
    requestUserSharePermission: z.boolean(),
    required: z.boolean(),
    stepOrder: z.number().int(),
    toolConfig: z.union([ToolConfig, z.null()]).optional(),
    workflowId: z.string().uuid(),
  })
  .passthrough();
export type LocalizedWorkflowStepDto = z.infer<typeof LocalizedWorkflowStepDto>;
export const WorkflowStepsListResponse = z.union([
  z.array(WorkflowStepWithTranslations),
  z.array(LocalizedWorkflowStepWithProgressDto),
  z.array(LocalizedWorkflowStepDto),
]);
export type WorkflowStepsListResponse = z.infer<
  typeof WorkflowStepsListResponse
>;
export const SetupQuestion = z
  .object({ text: z.string(), type: QuestionType })
  .passthrough();
export type SetupQuestion = z.infer<typeof SetupQuestion>;
export const ThinkingSpaceSetupQuestion = z
  .object({ intent: z.string(), text: z.string() })
  .passthrough();
export type ThinkingSpaceSetupQuestion = z.infer<
  typeof ThinkingSpaceSetupQuestion
>;
export const ToolSetup = z.union([
  z
    .object({
      required_votes: z.union([z.number(), z.null()]).optional(),
      show_remaining_statements: z.boolean().optional().default(true),
      topic: z.string(),
      type: z.literal("polis"),
    })
    .passthrough(),
  z
    .object({ pages: z.array(LearnPageEntry), type: z.literal("learn") })
    .passthrough(),
  z
    .object({
      server_url: z.string().optional().default("forms.comhairle.scot"),
      type: z.literal("heyform"),
    })
    .passthrough(),
  z
    .object({
      max_time: z.number().int(),
      to_see: z.number().int(),
      type: z.literal("stories"),
    })
    .passthrough(),
  z
    .object({ topic: z.string(), type: z.literal("elicitationbot") })
    .passthrough(),
  z
    .object({
      questions: z.array(SetupQuestion),
      randomize_order: z.boolean(),
      type: z.literal("prioritization"),
    })
    .passthrough(),
  z
    .object({
      follow_up_rounds_count: z.number().int().gte(0),
      root_questions: z.array(ThinkingSpaceSetupQuestion),
      topic: z.string(),
      type: z.literal("thinkingspace"),
    })
    .passthrough(),
]);
export type ToolSetup = z.infer<typeof ToolSetup>;
export const CreateWorkflowStep = z
  .object({
    activation_rule: ActivationRule,
    description: z.string(),
    is_offline: z.boolean(),
    name: z.string(),
    required: z.boolean(),
    step_order: z.number().int(),
    tool_setup: ToolSetup,
  })
  .passthrough();
export type CreateWorkflowStep = z.infer<typeof CreateWorkflowStep>;
export const WorkflowStepDto = z
  .object({
    activationRule: ActivationRule,
    canRevisit: z.boolean(),
    description: z.string().uuid(),
    id: z.string().uuid(),
    isOffline: z.boolean(),
    name: z.string().uuid(),
    previewToolConfig: ToolConfig,
    requestUserSharePermission: z.boolean(),
    required: z.boolean(),
    stepOrder: z.number().int(),
    toolConfig: z.union([ToolConfig, z.null()]).optional(),
    workflowId: z.string().uuid(),
  })
  .passthrough();
export type WorkflowStepDto = z.infer<typeof WorkflowStepDto>;
export const PartialWorkflowStep = z
  .object({
    activation_rule: z.union([ActivationRule, z.null()]),
    can_revisit: z.union([z.boolean(), z.null()]),
    description: z.union([z.string(), z.null()]),
    is_offline: z.union([z.boolean(), z.null()]),
    name: z.union([z.string(), z.null()]),
    preview_tool_config: z.union([ToolConfig, z.null()]),
    request_user_share_permission: z.union([z.boolean(), z.null()]),
    required: z.union([z.boolean(), z.null()]),
    step_order: z.union([z.number(), z.null()]),
    tool_config: z.union([ToolConfig, z.null()]),
  })
  .partial()
  .passthrough();
export type PartialWorkflowStep = z.infer<typeof PartialWorkflowStep>;
export const UserProgressDto = z
  .object({
    id: z.string().uuid(),
    permissionToShareWithOrganizers: z.boolean(),
    status: ProgressStatus,
    userId: z.string().uuid(),
    workflowStepId: z.string().uuid(),
  })
  .passthrough();
export type UserProgressDto = z.infer<typeof UserProgressDto>;
export const UpdateUserProgress = z
  .object({
    permission_to_share_with_organizers: z.union([z.boolean(), z.null()]),
    permission_to_share_with_other_participants: z.union([
      z.boolean(),
      z.null(),
    ]),
    status: z.union([ProgressStatus, z.null()]),
  })
  .partial()
  .passthrough();
export type UpdateUserProgress = z.infer<typeof UpdateUserProgress>;
export const InviteType = z.union([
  z.object({ email: z.string() }),
  z.object({ user: z.string().uuid() }),
  z.literal("singleuse"),
  z.literal("open"),
]);
export type InviteType = z.infer<typeof InviteType>;
export const LoginBehaviour = z.union([
  z.literal("manual"),
  z.literal("auto_create_annon"),
]);
export type LoginBehaviour = z.infer<typeof LoginBehaviour>;
export const InviteStatus = z.union([
  z.literal("pending"),
  z.literal("open"),
  z.literal("accepted"),
  z.literal("rejected"),
  z.literal("expired"),
]);
export type InviteStatus = z.infer<typeof InviteStatus>;
export const InviteDto = z
  .object({
    acceptCount: z.number().int(),
    conversationId: z.string().uuid(),
    createdAt: z.string().datetime({ offset: true }),
    createdBy: z.union([z.string(), z.null()]).optional(),
    eventId: z.union([z.string(), z.null()]).optional(),
    expiresAt: z.union([z.string(), z.null()]).optional(),
    id: z.string().uuid(),
    inviteType: InviteType,
    label: z.union([z.string(), z.null()]).optional(),
    loginBehaviour: LoginBehaviour,
    status: InviteStatus,
    tags: z.array(z.string()),
    workflowId: z.union([z.string(), z.null()]).optional(),
    workflowStepId: z.union([z.string(), z.null()]).optional(),
  })
  .passthrough();
export type InviteDto = z.infer<typeof InviteDto>;
export const CreateInviteDTO = z
  .object({
    event_id: z.union([z.string(), z.null()]).optional(),
    expires_at: z.union([z.string(), z.null()]).optional(),
    invite_type: InviteType,
    label: z.union([z.string(), z.null()]).optional(),
    login_behaviour: LoginBehaviour.optional(),
  })
  .passthrough();
export type CreateInviteDTO = z.infer<typeof CreateInviteDTO>;
export const PartialInvite = z
  .object({
    accept_count: z.union([z.number(), z.null()]),
    conversation_id: z.union([z.string(), z.null()]),
    event_id: z.union([z.string(), z.null()]),
    expires_at: z.union([z.string(), z.null()]),
    invite_type: z.union([InviteType, z.null()]),
    label: z.union([z.string(), z.null()]),
    login_behaviour: z.union([LoginBehaviour, z.null()]),
    status: z.union([InviteStatus, z.null()]),
    tags: z.union([z.array(z.string()), z.null()]),
    workflow_id: z.union([z.string(), z.null()]),
    workflow_step_id: z.union([z.string(), z.null()]),
  })
  .partial()
  .passthrough();
export type PartialInvite = z.infer<typeof PartialInvite>;
export const DailyResponseStats = z
  .object({
    accept: z.number().int(),
    day: z.string().datetime({ offset: true }),
    reject: z.number().int(),
  })
  .passthrough();
export type DailyResponseStats = z.infer<typeof DailyResponseStats>;
export const FeedbackDto = z
  .object({
    content: z.string(),
    conversationId: z.string().uuid(),
    id: z.string().uuid(),
  })
  .passthrough();
export type FeedbackDto = z.infer<typeof FeedbackDto>;
export const ReportImpactDto = z
  .object({
    createdAt: z.string().datetime({ offset: true }),
    createdBy: z.string().uuid(),
    details: z.string(),
    id: z.string().uuid(),
    kind: z.string(),
    reportId: z.string().uuid(),
    title: z.string(),
  })
  .passthrough();
export type ReportImpactDto = z.infer<typeof ReportImpactDto>;
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
export const PrioritizationReport = z.null();
export type PrioritizationReport = z.infer<typeof PrioritizationReport>;
export const ThinkingSpaceReport = z.null();
export type ThinkingSpaceReport = z.infer<typeof ThinkingSpaceReport>;
export const ReportConfig = z.union([
  z.object({ Polis: PolisReport }),
  z.object({ HeyForm: HeyFormReport }),
  z.object({ Learn: LearnReport }),
  z.object({ Stories: StoriesReport }),
  z.object({ ElicitationBot: ElicitationBotReport }),
  z.object({ Prioritization: PrioritizationReport }),
  z.object({ ThinkingSpace: ThinkingSpaceReport }),
]);
export type ReportConfig = z.infer<typeof ReportConfig>;
export const ReportSectionConfig = z
  .object({
    ai_generated: z.boolean(),
    config: ReportConfig,
    verified: z.boolean(),
    workflow_step_id: z.string().uuid(),
  })
  .passthrough();
export type ReportSectionConfig = z.infer<typeof ReportSectionConfig>;
export const ReportSectionConfigs = z.array(ReportSectionConfig);
export type ReportSectionConfigs = z.infer<typeof ReportSectionConfigs>;
export const FullReportDto = z
  .object({
    conversationId: z.string().uuid(),
    createdAt: z.string().datetime({ offset: true }),
    facilitatorFeedback: z.array(FeedbackDto),
    id: z.string().uuid(),
    impacts: z.array(ReportImpactDto),
    isPublic: z.boolean(),
    participantFeedback: z.array(FeedbackDto),
    sectionConfigs: ReportSectionConfigs,
    summary: z.string(),
  })
  .passthrough();
export type FullReportDto = z.infer<typeof FullReportDto>;
export const PartialReport = z
  .object({
    conversation_id: z.union([z.string(), z.null()]),
    is_public: z.union([z.boolean(), z.null()]),
    section_configs: z.union([ReportSectionConfigs, z.null()]),
    summary: z.union([z.string(), z.null()]),
  })
  .partial()
  .passthrough();
export type PartialReport = z.infer<typeof PartialReport>;
export const ReportDto = z
  .object({
    conversationId: z.string().uuid(),
    createdAt: z.string().datetime({ offset: true }),
    id: z.string().uuid(),
    isPublic: z.boolean(),
    sectionConfigs: ReportSectionConfigs,
    summary: z.string(),
  })
  .passthrough();
export type ReportDto = z.infer<typeof ReportDto>;
export const PartialReportImpact = z
  .object({
    created_at: z.union([z.string(), z.null()]),
    created_by: z.union([z.string(), z.null()]),
    details: z.union([z.string(), z.null()]),
    id: z.union([z.string(), z.null()]),
    kind: z.union([z.string(), z.null()]),
    report_id: z.union([z.string(), z.null()]),
    title: z.union([z.string(), z.null()]),
    updated_at: z.union([z.string(), z.null()]),
  })
  .partial()
  .passthrough();
export type PartialReportImpact = z.infer<typeof PartialReportImpact>;
export const CreateImpactDTO = z
  .object({ details: z.string(), kind: z.string(), title: z.string() })
  .passthrough();
export type CreateImpactDTO = z.infer<typeof CreateImpactDTO>;
export const CreateFeedbackDTO = z
  .object({ content: z.string() })
  .passthrough();
export type CreateFeedbackDTO = z.infer<typeof CreateFeedbackDTO>;
export const PartialFeedback = z
  .object({ content: z.union([z.string(), z.null()]) })
  .partial()
  .passthrough();
export type PartialFeedback = z.infer<typeof PartialFeedback>;
export const ComhairleChatSession = z
  .object({
    chat_id: z.string(),
    id: z.string(),
    messages: z.array(ComhairleSessionMessage),
    name: z.union([z.string(), z.null()]).optional(),
  })
  .passthrough();
export type ComhairleChatSession = z.infer<typeof ComhairleChatSession>;
export const ChatConversationRequest = z
  .object({ question: z.string() })
  .passthrough();
export type ChatConversationRequest = z.infer<typeof ChatConversationRequest>;
export const page_size = z
  .union([z.number(), z.null()])
  .optional()
  .default(400);
export type page_size = z.infer<typeof page_size>;
export const ComhairleDocument = z
  .object({
    id: z.string(),
    name: z.string(),
    parse_progress: z.number(),
    parse_status: z.string(),
    size: z.number().int(),
  })
  .passthrough();
export type ComhairleDocument = z.infer<typeof ComhairleDocument>;
export const UploadFileResponse = z
  .object({
    document: ComhairleDocument,
    job_id: z.string().uuid(),
    message: z.string(),
  })
  .passthrough();
export type UploadFileResponse = z.infer<typeof UploadFileResponse>;
export const Order = z.enum(["asc", "desc"]);
export type Order = z.infer<typeof Order>;
export const created_at = z.union([Order, z.null()]).optional();
export type created_at = z.infer<typeof created_at>;
export const CapacityStatus = z.enum(["full", "available"]);
export type CapacityStatus = z.infer<typeof CapacityStatus>;
export const capacity_status = z.union([CapacityStatus, z.null()]).optional();
export type capacity_status = z.infer<typeof capacity_status>;
export const TimeStatus = z.enum(["past", "future"]);
export type TimeStatus = z.infer<typeof TimeStatus>;
export const time_status = z.union([TimeStatus, z.null()]).optional();
export type time_status = z.infer<typeof time_status>;
export const BasicEventAgendaItem = z
  .object({
    description: z.string(),
    estimated_time: z.number().int().gte(0),
    title: z.string(),
  })
  .passthrough();
export type BasicEventAgendaItem = z.infer<typeof BasicEventAgendaItem>;
export const BreakoutRoomAgendaItem = z
  .object({
    estimated_time: z.number().int().gte(0),
    instructions: z.string(),
    max_per_room: z.union([z.number(), z.null()]).optional(),
    prompt: z.string(),
    time_limit: z.union([z.number(), z.null()]).optional(),
  })
  .passthrough();
export type BreakoutRoomAgendaItem = z.infer<typeof BreakoutRoomAgendaItem>;
export const EventAgendaItem = z.union([
  z.object({ Basic: BasicEventAgendaItem }),
  z.object({ BreakoutRoom: BreakoutRoomAgendaItem }),
]);
export type EventAgendaItem = z.infer<typeof EventAgendaItem>;
export const EventFormat = z.enum(["online", "in_person"]);
export type EventFormat = z.infer<typeof EventFormat>;
export const EventLocation = z
  .object({
    address_line_1: z.string(),
    address_line_2: z.union([z.string(), z.null()]).optional(),
    address_line_3: z.union([z.string(), z.null()]).optional(),
    city: z.string(),
    country_code: z.string(),
    postal_code: z.string(),
    state_province: z.string(),
    venue_name: z.string(),
  })
  .passthrough();
export type EventLocation = z.infer<typeof EventLocation>;
export const LocalizedEventDto = z
  .object({
    agenda: z.array(EventAgendaItem),
    capacity: z.union([z.number(), z.null()]).optional(),
    conversationId: z.string().uuid(),
    createdAt: z.string().datetime({ offset: true }),
    currentAttendance: z.union([z.number(), z.null()]).optional(),
    description: z.string(),
    endTime: z.string().datetime({ offset: true }),
    format: EventFormat,
    id: z.string().uuid(),
    location: z.union([EventLocation, z.null()]).optional(),
    name: z.string(),
    signupMode: z.string(),
    startTime: z.string().datetime({ offset: true }),
    videoMeetingId: z.union([z.string(), z.null()]).optional(),
  })
  .passthrough();
export type LocalizedEventDto = z.infer<typeof LocalizedEventDto>;
export const PaginatedResults_for_LocalizedEventDto = z
  .object({ records: z.array(LocalizedEventDto), total: z.number().int() })
  .passthrough();
export type PaginatedResults_for_LocalizedEventDto = z.infer<
  typeof PaginatedResults_for_LocalizedEventDto
>;
export const CreateEvent = z
  .object({
    agenda: z.union([z.array(EventAgendaItem), z.null()]).optional(),
    capacity: z.union([z.number(), z.null()]).optional(),
    default_time_zone: z.union([z.string(), z.null()]).optional(),
    description: z.string(),
    end_time: z.string().datetime({ offset: true }),
    location: z.union([EventLocation, z.null()]).optional(),
    name: z.string(),
    signup_mode: z.string(),
    start_time: z.string().datetime({ offset: true }),
  })
  .passthrough();
export type CreateEvent = z.infer<typeof CreateEvent>;
export const EventDto = z
  .object({
    agenda: z.array(EventAgendaItem),
    capacity: z.union([z.number(), z.null()]).optional(),
    conversationId: z.string().uuid(),
    createdAt: z.string().datetime({ offset: true }),
    description: z.string().uuid(),
    endTime: z.string().datetime({ offset: true }),
    format: EventFormat,
    id: z.string().uuid(),
    location: z.union([EventLocation, z.null()]).optional(),
    name: z.string().uuid(),
    signupMode: z.string(),
    startTime: z.string().datetime({ offset: true }),
    videoMeetingId: z.union([z.string(), z.null()]).optional(),
  })
  .passthrough();
export type EventDto = z.infer<typeof EventDto>;
export const Translation4 = z
  .object({
    textContent: TextContentDto,
    textTranslations: z.array(TextTranslationDto),
  })
  .passthrough();
export type Translation4 = z.infer<typeof Translation4>;
export const EventTranslations = z
  .object({ description: Translation4, name: Translation4 })
  .passthrough();
export type EventTranslations = z.infer<typeof EventTranslations>;
export const EventWithTranslations = z
  .object({
    agenda: z.array(EventAgendaItem),
    capacity: z.union([z.number(), z.null()]).optional(),
    conversationId: z.string().uuid(),
    createdAt: z.string().datetime({ offset: true }),
    defaultTimeZone: z.string(),
    description: z.string(),
    endTime: z.string().datetime({ offset: true }),
    format: EventFormat,
    id: z.string().uuid(),
    location: z.union([EventLocation, z.null()]).optional(),
    name: z.string(),
    signupMode: z.string(),
    startTime: z.string().datetime({ offset: true }),
    translations: EventTranslations,
    updatedAt: z.string().datetime({ offset: true }),
    videoMeetingId: z.union([z.string(), z.null()]).optional(),
  })
  .passthrough();
export type EventWithTranslations = z.infer<typeof EventWithTranslations>;
export const EventResponse = z.union([
  LocalizedEventDto,
  EventWithTranslations,
]);
export type EventResponse = z.infer<typeof EventResponse>;
export const PartialEvent = z
  .object({
    agenda: z.union([z.array(EventAgendaItem), z.null()]).default(null),
    capacity: z.union([z.number(), z.null()]),
    default_time_zone: z.union([z.string(), z.null()]),
    description: z.union([z.string(), z.null()]),
    end_time: z.union([z.string(), z.null()]),
    format: z.union([EventFormat, z.null()]),
    location: z.union([EventLocation, z.null()]),
    name: z.union([z.string(), z.null()]),
    signup_mode: z.union([z.string(), z.null()]),
    start_time: z.union([z.string(), z.null()]),
  })
  .partial()
  .passthrough();
export type PartialEvent = z.infer<typeof PartialEvent>;
export const JwtResponse = z
  .object({ isModerator: z.boolean(), jwt: z.string() })
  .passthrough();
export type JwtResponse = z.infer<typeof JwtResponse>;
export const ProcessTranscriptionResponse = z
  .object({ job_ids: z.array(z.string().uuid()), message: z.string() })
  .passthrough();
export type ProcessTranscriptionResponse = z.infer<
  typeof ProcessTranscriptionResponse
>;
export const SubmitReportResponse = z
  .object({ success: z.boolean(), url: z.string() })
  .passthrough();
export type SubmitReportResponse = z.infer<typeof SubmitReportResponse>;
export const EventAttendanceEtx = z
  .object({
    createdAt: z.string().datetime({ offset: true }),
    email: z.union([z.string(), z.null()]).optional(),
    eventId: z.string().uuid(),
    id: z.string().uuid(),
    role: z.string(),
    updatedAt: z.string().datetime({ offset: true }),
    userId: z.string().uuid(),
  })
  .passthrough();
export type EventAttendanceEtx = z.infer<typeof EventAttendanceEtx>;
export const PaginatedResults_for_EventAttendanceEtx = z
  .object({ records: z.array(EventAttendanceEtx), total: z.number().int() })
  .passthrough();
export type PaginatedResults_for_EventAttendanceEtx = z.infer<
  typeof PaginatedResults_for_EventAttendanceEtx
>;
export const CreateEventAttendanceRequest = z
  .object({
    role: z.string(),
    user_email: z.union([z.string(), z.null()]).optional(),
  })
  .passthrough();
export type CreateEventAttendanceRequest = z.infer<
  typeof CreateEventAttendanceRequest
>;
export const EventAttendanceDto = z
  .object({
    createdAt: z.string().datetime({ offset: true }),
    eventId: z.string().uuid(),
    id: z.string().uuid(),
    role: z.string(),
    userId: z.string().uuid(),
  })
  .passthrough();
export type EventAttendanceDto = z.infer<typeof EventAttendanceDto>;
export const UpdateEventAttendanceRequest = z
  .object({ role: z.union([z.string(), z.null()]) })
  .partial()
  .passthrough();
export type UpdateEventAttendanceRequest = z.infer<
  typeof UpdateEventAttendanceRequest
>;
export const CreateFacilitatorRequest = z
  .object({ email: z.string() })
  .passthrough();
export type CreateFacilitatorRequest = z.infer<typeof CreateFacilitatorRequest>;
export const WebSocketStats = z
  .object({
    connected_users: z.array(z.string().uuid()),
    total_connections: z.number().int().gte(0),
  })
  .passthrough();
export type WebSocketStats = z.infer<typeof WebSocketStats>;
export const BroadcastMessage = z
  .object({
    authenticated_only: z.union([z.boolean(), z.null()]).optional(),
    message: z.string(),
  })
  .passthrough();
export type BroadcastMessage = z.infer<typeof BroadcastMessage>;
export const BroadcastResponse = z
  .object({ message: z.string(), sent_to: z.number().int().gte(0) })
  .passthrough();
export type BroadcastResponse = z.infer<typeof BroadcastResponse>;
export const SendToUserMessage = z
  .object({ message: z.string(), user_id: z.string().uuid() })
  .passthrough();
export type SendToUserMessage = z.infer<typeof SendToUserMessage>;
export const OrganizationType = z.enum(["non_profit", "governmental", "other"]);
export type OrganizationType = z.infer<typeof OrganizationType>;
export const LocalizedOrganizationDto = z
  .object({
    createdAt: z.string().datetime({ offset: true }),
    description: z.string(),
    externalUrl: z.union([z.string(), z.null()]).optional(),
    id: z.string().uuid(),
    mission: z.string(),
    name: z.string(),
    orgType: OrganizationType,
    regions: z.array(z.string().uuid()),
  })
  .passthrough();
export type LocalizedOrganizationDto = z.infer<typeof LocalizedOrganizationDto>;
export const PaginatedResults_for_LocalizedOrganizationDto = z
  .object({
    records: z.array(LocalizedOrganizationDto),
    total: z.number().int(),
  })
  .passthrough();
export type PaginatedResults_for_LocalizedOrganizationDto = z.infer<
  typeof PaginatedResults_for_LocalizedOrganizationDto
>;
export const CreateOrganization = z
  .object({
    description: z.string(),
    external_url: z.union([z.string(), z.null()]).optional(),
    mission: z.string(),
    name: z.string(),
    org_type: OrganizationType,
    regions: z.union([z.array(z.string().uuid()), z.null()]).optional(),
  })
  .passthrough();
export type CreateOrganization = z.infer<typeof CreateOrganization>;
export const OrganizationDto = z
  .object({
    createdAt: z.string().datetime({ offset: true }),
    description: z.string().uuid(),
    externalUrl: z.union([z.string(), z.null()]).optional(),
    id: z.string().uuid(),
    mission: z.string().uuid(),
    name: z.string(),
    orgType: OrganizationType,
    regions: z.array(z.string().uuid()),
  })
  .passthrough();
export type OrganizationDto = z.infer<typeof OrganizationDto>;
export const PartialOrganization = z
  .object({
    external_url: z.union([z.string(), z.null()]),
    name: z.union([z.string(), z.null()]),
    org_type: z.union([OrganizationType, z.null()]),
    regions: z.union([z.array(z.string().uuid()), z.null()]),
  })
  .partial()
  .passthrough();
export type PartialOrganization = z.infer<typeof PartialOrganization>;
export const RegionType = z.enum(["custom", "official"]);
export type RegionType = z.infer<typeof RegionType>;
export const LocalizedRegionDto = z
  .object({
    created_at: z.string().datetime({ offset: true }),
    description: z.string(),
    id: z.string().uuid(),
    name: z.string(),
    official_id: z.union([z.string(), z.null()]).optional(),
    region_type: RegionType,
  })
  .passthrough();
export type LocalizedRegionDto = z.infer<typeof LocalizedRegionDto>;
export const PaginatedResults_for_LocalizedRegionDto = z
  .object({ records: z.array(LocalizedRegionDto), total: z.number().int() })
  .passthrough();
export type PaginatedResults_for_LocalizedRegionDto = z.infer<
  typeof PaginatedResults_for_LocalizedRegionDto
>;
export const CreateRegion = z
  .object({
    description: z.string(),
    name: z.string(),
    official_id: z.union([z.string(), z.null()]).optional(),
    region_type: RegionType,
  })
  .passthrough();
export type CreateRegion = z.infer<typeof CreateRegion>;
export const RegionDto = z
  .object({
    created_at: z.string().datetime({ offset: true }),
    description: z.string().uuid(),
    id: z.string().uuid(),
    name: z.string().uuid(),
    official_id: z.union([z.string(), z.null()]).optional(),
    region_type: RegionType,
  })
  .passthrough();
export type RegionDto = z.infer<typeof RegionDto>;
export const PartialRegion = z
  .object({
    official_id: z.union([z.string(), z.null()]),
    region_type: z.union([RegionType, z.null()]),
  })
  .partial()
  .passthrough();
export type PartialRegion = z.infer<typeof PartialRegion>;
export const MediaContentType = z.enum([
  "image/jpeg",
  "image/png",
  "image/gif",
  "image/webp",
  "video/mp4",
  "video/mpeg",
  "video/webm",
]);
export type MediaContentType = z.infer<typeof MediaContentType>;
export const content_type = z.union([MediaContentType, z.null()]).optional();
export type content_type = z.infer<typeof content_type>;
export const MediaDto = z
  .object({
    contentType: MediaContentType,
    createdAt: z.string().datetime({ offset: true }),
    filename: z.string(),
    id: z.string().uuid(),
    ownerId: z.string().uuid(),
    storageKey: z.string(),
    storeName: z.string(),
  })
  .passthrough();
export type MediaDto = z.infer<typeof MediaDto>;
export const PaginatedResults_for_MediaDto = z
  .object({ records: z.array(MediaDto), total: z.number().int() })
  .passthrough();
export type PaginatedResults_for_MediaDto = z.infer<
  typeof PaginatedResults_for_MediaDto
>;
export const Job = z
  .object({
    completion_message: z.union([z.string(), z.null()]).optional(),
    created_at: z.string().datetime({ offset: true }),
    error: z.union([z.string(), z.null()]).optional(),
    finished_at: z.union([z.string(), z.null()]).optional(),
    id: z.string().uuid(),
    progress: z.union([z.number(), z.null()]).optional(),
    status: z.union([z.string(), z.null()]).optional(),
    step: z.union([z.string(), z.null()]).optional(),
  })
  .passthrough();
export type Job = z.infer<typeof Job>;
export const PaginatedResults_for_Job = z
  .object({ records: z.array(Job), total: z.number().int() })
  .passthrough();
export type PaginatedResults_for_Job = z.infer<typeof PaginatedResults_for_Job>;
export const CreateJob = z
  .object({
    progress: z.union([z.number(), z.null()]),
    step: z.union([z.string(), z.null()]),
  })
  .partial()
  .passthrough();
export type CreateJob = z.infer<typeof CreateJob>;
export const ComhairleServices = z
  .object({ botService: z.boolean(), translationService: z.boolean() })
  .passthrough();
export type ComhairleServices = z.infer<typeof ComhairleServices>;
export const CreateApiKeyRequest = z
  .object({ name: z.string(), prefix: z.string() })
  .passthrough();
export type CreateApiKeyRequest = z.infer<typeof CreateApiKeyRequest>;
export const CreateResponse2 = z.object({ key: z.string() }).passthrough();
export type CreateResponse2 = z.infer<typeof CreateResponse2>;
export const EmailTemplateSlots = z.union([
  z
    .object({
      body: z.string(),
      footer: z.string(),
      heading: z.string(),
      intro: z.string(),
      type: z.literal("conversation_invite"),
    })
    .passthrough(),
  z
    .object({
      body: z.string(),
      footer: z.string(),
      heading: z.string(),
      intro: z.string(),
      type: z.literal("event_registration_confirmation"),
    })
    .passthrough(),
]);
export type EmailTemplateSlots = z.infer<typeof EmailTemplateSlots>;
export const EmailTemplateConfigDto = z
  .object({
    createdAt: z.string().datetime({ offset: true }),
    emailType: z.string(),
    id: z.string().uuid(),
    organizationId: z.union([z.string(), z.null()]).optional(),
    ownerId: z.string().uuid(),
    slots: EmailTemplateSlots,
  })
  .passthrough();
export type EmailTemplateConfigDto = z.infer<typeof EmailTemplateConfigDto>;
export const CreateEmailTemplateConfig = z
  .object({ slots: EmailTemplateSlots })
  .passthrough();
export type CreateEmailTemplateConfig = z.infer<
  typeof CreateEmailTemplateConfig
>;
export const UpdateEmailTemplateConfig = z
  .object({ slots: z.union([EmailTemplateSlots, z.null()]) })
  .partial()
  .passthrough();
export type UpdateEmailTemplateConfig = z.infer<
  typeof UpdateEmailTemplateConfig
>;
export const SlotSchemaDefinition = z
  .object({
    hint: z.string(),
    key: z.string(),
    label: z.string(),
    max_chars: z.union([z.number(), z.null()]).optional(),
    required: z.boolean(),
  })
  .passthrough();
export type SlotSchemaDefinition = z.infer<typeof SlotSchemaDefinition>;
export const EmailTypeSchema = z
  .object({ email_type: z.string(), slots: z.array(SlotSchemaDefinition) })
  .passthrough();
export type EmailTypeSchema = z.infer<typeof EmailTypeSchema>;

export const schemas: Record<string, z.ZodType<any>> = {
  AnnonLoginRequest,
  UserAuthType,
  UserDto,
  LoginRequest,
  OtpLoginRequest,
  SignupRequest,
  OtpSignupRequest,
  CreateOtpRequest,
  VerifyOtpTokenRequest,
  VerifyEmailTokenRequest,
  ResendVerificationEmailRequest,
  CreatePasswordResetRequest,
  PasswordResetUpdateRequest,
  ResourceType,
  ResourceRole,
  UserRoles,
  LocalizedConversationDto,
  created_after,
  is_complete,
  limit,
  PaginatedResults_for_LocalizedConversationDto,
  UpdateUserRequest,
  UpgradeAccountRequest,
  UserConversationPreferencesDto,
  UpdateUserConversationPreferences,
  UserProfileDto,
  UpsertUserProfileRequest,
  DeliveryMethod,
  NotificationContextType,
  NotificationType,
  NotificationDto,
  NotificationWithDelivery,
  PaginatedResults_for_NotificationWithDelivery,
  UnreadCount,
  NotificationDelivery,
  TextFormat,
  CreateTextContentRequest,
  TextContentDto,
  TextTranslationDto,
  TextContentWithTranslations,
  UpdateTextContent,
  UpdateTextTranslation,
  CreateOrUpdateTextTranslationRequest,
  ModerationStatus,
  PolisStatementAux,
  CreatePolisStatementAux,
  UpdatePolisStatementAux,
  SyncStatementAuxRequest,
  SyncStatementAuxResponse,
  ThemeStatistic,
  Story,
  ComhairleMessageReference,
  ComhairleSessionMessage,
  ComhairleAgentSession,
  ConversationRequest,
  Translation,
  ProposalTranslations,
  ProposalWithTranslations,
  LocalizedProposalDto,
  ProposalsListResponse,
  CreateProposalRequest,
  ProposalDto,
  ResponseValue,
  Response,
  QuestionResponses,
  ProposalResponseDto,
  CreateResponse,
  ConversationRequest2,
  AnswerStatus,
  status,
  ThinkingSpaceAnswerDto,
  CreateAnswerRequest,
  UpdateAnswer,
  GenerateThinkingSpaceSummary,
  ThinkingSpaceSummaryDto,
  UpdateCreateThinkingSpace,
  ThinkingSpaceFollowUpQuestionDto,
  CreateFollowUpQuestions,
  UpdateFollowUpQuestions,
  CreateConversation,
  ConversationDto,
  Translation2,
  ConversationTranslations,
  ConversationWithTranslations,
  ConversationResponse,
  PartialConversation,
  SendNotificationRequest,
  SendEmailNotificationResponse,
  NotificationRecipientsResponse,
  RegisterEmailRequest,
  RegisterEmailResponse,
  WorkflowDto,
  CreateWorkflow,
  PartialWorkflow,
  ActivationRule,
  LearnPage,
  LocalizedPage,
  LearnPageEntry,
  Category,
  QuestionType,
  Question,
  ThinkingSpaceQuestion,
  ToolConfig,
  WorkflowStep,
  DailySignupStats,
  WorkflowStepStats,
  WorkflowStats,
  DemographicCategory,
  DemographicReport,
  UserParticipation,
  Translation3,
  WorkflowStepTranslations,
  WorkflowStepWithTranslations,
  ProgressStatus,
  LocalizedWorkflowStepWithProgressDto,
  LocalizedWorkflowStepDto,
  WorkflowStepsListResponse,
  SetupQuestion,
  ThinkingSpaceSetupQuestion,
  ToolSetup,
  CreateWorkflowStep,
  WorkflowStepDto,
  PartialWorkflowStep,
  UserProgressDto,
  UpdateUserProgress,
  InviteType,
  LoginBehaviour,
  InviteStatus,
  InviteDto,
  CreateInviteDTO,
  PartialInvite,
  DailyResponseStats,
  FeedbackDto,
  ReportImpactDto,
  PolisReport,
  HeyFormReport,
  LearnReport,
  StoriesReport,
  ElicitationBotReport,
  PrioritizationReport,
  ThinkingSpaceReport,
  ReportConfig,
  ReportSectionConfig,
  ReportSectionConfigs,
  FullReportDto,
  PartialReport,
  ReportDto,
  PartialReportImpact,
  CreateImpactDTO,
  CreateFeedbackDTO,
  PartialFeedback,
  ComhairleChatSession,
  ChatConversationRequest,
  page_size,
  ComhairleDocument,
  UploadFileResponse,
  Order,
  created_at,
  CapacityStatus,
  capacity_status,
  TimeStatus,
  time_status,
  BasicEventAgendaItem,
  BreakoutRoomAgendaItem,
  EventAgendaItem,
  EventFormat,
  EventLocation,
  LocalizedEventDto,
  PaginatedResults_for_LocalizedEventDto,
  CreateEvent,
  EventDto,
  Translation4,
  EventTranslations,
  EventWithTranslations,
  EventResponse,
  PartialEvent,
  JwtResponse,
  ProcessTranscriptionResponse,
  SubmitReportResponse,
  EventAttendanceEtx,
  PaginatedResults_for_EventAttendanceEtx,
  CreateEventAttendanceRequest,
  EventAttendanceDto,
  UpdateEventAttendanceRequest,
  CreateFacilitatorRequest,
  WebSocketStats,
  BroadcastMessage,
  BroadcastResponse,
  SendToUserMessage,
  OrganizationType,
  LocalizedOrganizationDto,
  PaginatedResults_for_LocalizedOrganizationDto,
  CreateOrganization,
  OrganizationDto,
  PartialOrganization,
  RegionType,
  LocalizedRegionDto,
  PaginatedResults_for_LocalizedRegionDto,
  CreateRegion,
  RegionDto,
  PartialRegion,
  MediaContentType,
  content_type,
  MediaDto,
  PaginatedResults_for_MediaDto,
  Job,
  PaginatedResults_for_Job,
  CreateJob,
  ComhairleServices,
  CreateApiKeyRequest,
  CreateResponse2,
  EmailTemplateSlots,
  EmailTemplateConfigDto,
  CreateEmailTemplateConfig,
  UpdateEmailTemplateConfig,
  SlotSchemaDefinition,
  EmailTypeSchema,
};

const endpoints = makeApi([
  {
    method: "post",
    path: "/api_keys",
    alias: "postApi_keys",
    requestFormat: "json",
    parameters: [
      {
        name: "body",
        type: "Body",
        schema: CreateApiKeyRequest,
      },
    ],
    response: z.object({ key: z.string() }).passthrough(),
  },
  {
    method: "post",
    path: "/auth/create_otp",
    alias: "CreateOtp",
    requestFormat: "json",
    parameters: [
      {
        name: "body",
        type: "Body",
        schema: CreateOtpRequest,
      },
    ],
    response: z.void(),
  },
  {
    method: "get",
    path: "/auth/current_user",
    alias: "CurrentUser",
    requestFormat: "json",
    response: UserDto,
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
        schema: LoginRequest,
      },
    ],
    response: UserDto,
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
        schema: z.object({ username: z.string() }).passthrough(),
      },
    ],
    response: UserDto,
  },
  {
    method: "post",
    path: "/auth/login_otp",
    alias: "LoginOtpUser",
    description: `Login a user with a one time passcode`,
    requestFormat: "json",
    parameters: [
      {
        name: "body",
        type: "Body",
        schema: OtpLoginRequest,
      },
    ],
    response: UserDto,
  },
  {
    method: "post",
    path: "/auth/login_otp_token",
    alias: "LoginOtpToken",
    requestFormat: "json",
    parameters: [
      {
        name: "body",
        type: "Body",
        schema: z.object({ token: z.string() }).passthrough(),
      },
    ],
    response: UserDto,
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
        schema: z.object({ email: z.string() }).passthrough(),
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
        schema: PasswordResetUpdateRequest,
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
        schema: z.object({ id: z.string() }).passthrough(),
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
        schema: SignupRequest,
      },
    ],
    response: UserDto,
  },
  {
    method: "post",
    path: "/auth/signup_annon",
    alias: "SignupAnnonUser",
    requestFormat: "json",
    response: UserDto,
  },
  {
    method: "post",
    path: "/auth/signup_otp",
    alias: "SignupOtp",
    requestFormat: "json",
    parameters: [
      {
        name: "body",
        type: "Body",
        schema: OtpSignupRequest,
      },
    ],
    response: UserDto,
  },
  {
    method: "get",
    path: "/auth/test_api_key_extraction",
    alias: "getAuthtest_api_key_extraction",
    requestFormat: "json",
    response: UserDto,
  },
  {
    method: "get",
    path: "/auth/test_requires_roles/:conversation_id",
    alias: "TestRequiresRoles",
    requestFormat: "json",
    response: UserDto,
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
        schema: z.object({ token: z.string() }).passthrough(),
      },
    ],
    response: UserDto,
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
        schema: created_after,
      },
      {
        name: "created_before",
        type: "Query",
        schema: created_after,
      },
      {
        name: "is_complete",
        type: "Query",
        schema: is_complete,
      },
      {
        name: "is_invite_only",
        type: "Query",
        schema: is_complete,
      },
      {
        name: "is_live",
        type: "Query",
        schema: is_complete,
      },
      {
        name: "is_public",
        type: "Query",
        schema: is_complete,
      },
      {
        name: "keyword",
        type: "Query",
        schema: created_after,
      },
      {
        name: "organization_id",
        type: "Query",
        schema: created_after,
      },
      {
        name: "owner_id",
        type: "Query",
        schema: created_after,
      },
      {
        name: "limit",
        type: "Query",
        schema: limit,
      },
      {
        name: "offset",
        type: "Query",
        schema: limit,
      },
    ],
    response: PaginatedResults_for_LocalizedConversationDto,
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
        schema: CreateConversation,
      },
    ],
    response: ConversationDto,
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
        schema: z.boolean().optional().default(false),
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
        schema: PartialConversation,
      },
    ],
    response: ConversationDto,
  },
  {
    method: "delete",
    path: "/conversation/:conversation_id",
    alias: "DeleteConversation",
    description: `Delete the conversation and all related content`,
    requestFormat: "json",
    response: ConversationDto,
  },
  {
    method: "get",
    path: "/conversation/:conversation_id/chat_sessions",
    alias: "GetChatSessionHistory",
    requestFormat: "json",
    response: ComhairleChatSession,
  },
  {
    method: "post",
    path: "/conversation/:conversation_id/chat_sessions",
    alias: "postConversationConversation_idchat_sessions",
    description: `Streamed LLM response.

⚠️ This endpoint returns a streaming response on success.
Generated API clients are NOT suitable for consuming this endpoint.
Use a raw HTTP request and process the response body incrementally.`,
    requestFormat: "json",
    parameters: [
      {
        name: "body",
        type: "Body",
        schema: z.object({ question: z.string() }).passthrough(),
      },
    ],
    response: z.void(),
  },
  {
    method: "get",
    path: "/conversation/:conversation_id/contacts/export",
    alias: "ExportConversationContacts",
    description: `Exports a CSV file containing all users who have opted in to receive email updates for this conversation`,
    requestFormat: "json",
    response: z.void(),
  },
  {
    method: "get",
    path: "/conversation/:conversation_id/demographics/export",
    alias: "ExportConversationDemographics",
    description: `Exports a CSV file containing demographic data for users participating in the conversation&#x27;s workflow. Only includes consented users. Requires conversation ownership.`,
    requestFormat: "json",
    response: z.void(),
  },
  {
    method: "get",
    path: "/conversation/:conversation_id/documents",
    alias: "ListDocuments",
    requestFormat: "json",
    parameters: [
      {
        name: "name",
        type: "Query",
        schema: created_after,
      },
      {
        name: "order_by",
        type: "Query",
        schema: created_after,
      },
      {
        name: "page",
        type: "Query",
        schema: limit,
      },
      {
        name: "page_size",
        type: "Query",
        schema: page_size,
      },
      {
        name: "title",
        type: "Query",
        schema: created_after,
      },
    ],
    response: z.array(ComhairleDocument),
  },
  {
    method: "post",
    path: "/conversation/:conversation_id/documents",
    alias: "postConversationConversation_iddocuments",
    description: `⚠️ This endpoint requires multipart/form-data.

Generated API clients may not support file uploads.

Use FormData and a raw HTTP request.

**Example (curl):**
&#x60;&#x60;&#x60;bash
curl -X POST \
-H &#x27;Cookie: auth-token&#x3D;...;&#x27; \
&#x27;localhost:3000/conversation/__CONVERSATION_ID__/upload_documents&#x27; \
--form &#x27;file&#x3D;@/path-to-document.pdf&#x27;
&#x60;&#x60;&#x60;
                            `,
    requestFormat: "form-data",
    parameters: [
      {
        name: "body",
        description: `multipart form data`,
        type: "Body",
        schema: z.array(z.any()),
      },
    ],
    response: UploadFileResponse,
  },
  {
    method: "get",
    path: "/conversation/:conversation_id/documents/:document_id",
    alias: "GetDocument",
    requestFormat: "json",
    response: ComhairleDocument,
  },
  {
    method: "delete",
    path: "/conversation/:conversation_id/documents/:document_id",
    alias: "DeleteDocument",
    requestFormat: "json",
    response: z.void(),
  },
  {
    method: "get",
    path: "/conversation/:conversation_id/documents/:document_id/download",
    alias: "DownloadDocument",
    requestFormat: "json",
    response: z.void(),
  },
  {
    method: "post",
    path: "/conversation/:conversation_id/documents/:document_id/parse",
    alias: "ParseDocument",
    requestFormat: "json",
    response: z.void(),
  },
  {
    method: "post",
    path: "/conversation/:conversation_id/documents/:document_id/stop_parse",
    alias: "StopParsingDocument",
    requestFormat: "json",
    response: z.void(),
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
        schema: RegisterEmailRequest,
      },
    ],
    response: RegisterEmailResponse,
  },
  {
    method: "get",
    path: "/conversation/:conversation_id/events",
    alias: "ListEvents",
    description: `Paginated list of events for a conversation with optional filtering and ordering`,
    requestFormat: "json",
    parameters: [
      {
        name: "created_at",
        type: "Query",
        schema: created_at,
      },
      {
        name: "name",
        type: "Query",
        schema: created_at,
      },
      {
        name: "start_time",
        type: "Query",
        schema: created_at,
      },
      {
        name: "capacity_status",
        type: "Query",
        schema: capacity_status,
      },
      {
        name: "conversation_id",
        type: "Query",
        schema: created_after,
      },
      {
        name: "time_status",
        type: "Query",
        schema: time_status,
      },
      {
        name: "limit",
        type: "Query",
        schema: limit,
      },
      {
        name: "offset",
        type: "Query",
        schema: limit,
      },
    ],
    response: PaginatedResults_for_LocalizedEventDto,
  },
  {
    method: "post",
    path: "/conversation/:conversation_id/events",
    alias: "CreateEvent",
    description: `Create a new event`,
    requestFormat: "json",
    parameters: [
      {
        name: "body",
        type: "Body",
        schema: CreateEvent,
      },
    ],
    response: EventDto,
  },
  {
    method: "get",
    path: "/conversation/:conversation_id/events/:event_id",
    alias: "GetEvent",
    description: `Event an event by id`,
    requestFormat: "json",
    parameters: [
      {
        name: "withTranslations",
        type: "Query",
        schema: z.boolean().optional().default(false),
      },
    ],
    response: EventResponse,
  },
  {
    method: "put",
    path: "/conversation/:conversation_id/events/:event_id",
    alias: "UpdateEvent",
    description: `Update an event`,
    requestFormat: "json",
    parameters: [
      {
        name: "body",
        type: "Body",
        schema: PartialEvent,
      },
    ],
    response: EventDto,
  },
  {
    method: "delete",
    path: "/conversation/:conversation_id/events/:event_id",
    alias: "DeleteEvent",
    description: `Delete an event`,
    requestFormat: "json",
    response: EventDto,
  },
  {
    method: "get",
    path: "/conversation/:conversation_id/events/:event_id/attendances",
    alias: "ListEventAttendances",
    description: `List attendances for a conversation event with optional filtering
                        and ordering`,
    requestFormat: "json",
    parameters: [
      {
        name: "created_at",
        type: "Query",
        schema: created_at,
      },
      {
        name: "role",
        type: "Query",
        schema: created_after,
      },
      {
        name: "limit",
        type: "Query",
        schema: limit,
      },
      {
        name: "offset",
        type: "Query",
        schema: limit,
      },
    ],
    response: PaginatedResults_for_EventAttendanceEtx,
  },
  {
    method: "post",
    path: "/conversation/:conversation_id/events/:event_id/attendances",
    alias: "CreateEventAttendance",
    description: `Create a new attendance for a conversation event`,
    requestFormat: "json",
    parameters: [
      {
        name: "body",
        type: "Body",
        schema: CreateEventAttendanceRequest,
      },
    ],
    response: EventAttendanceDto,
  },
  {
    method: "get",
    path: "/conversation/:conversation_id/events/:event_id/attendances/:attendance_id",
    alias: "GetEventAttendance",
    description: `Get and event attendance by id`,
    requestFormat: "json",
    response: EventAttendanceDto,
  },
  {
    method: "put",
    path: "/conversation/:conversation_id/events/:event_id/attendances/:attendance_id",
    alias: "UpdateEventAttendance",
    description: `Update an event attendance by id`,
    requestFormat: "json",
    parameters: [
      {
        name: "body",
        type: "Body",
        schema: UpdateEventAttendanceRequest,
      },
    ],
    response: EventAttendanceDto,
  },
  {
    method: "delete",
    path: "/conversation/:conversation_id/events/:event_id/attendances/:attendance_id",
    alias: "DeleteEventAttendance",
    description: `Delete an event attendance by id`,
    requestFormat: "json",
    response: EventAttendanceDto,
  },
  {
    method: "post",
    path: "/conversation/:conversation_id/events/:event_id/attendances/facilitator",
    alias: "CreateFacilitatorEventAttendance",
    description: `Create a new attendance for a conversation event with facilitator role`,
    requestFormat: "json",
    parameters: [
      {
        name: "body",
        type: "Body",
        schema: z.object({ email: z.string() }).passthrough(),
      },
    ],
    response: EventAttendanceDto,
  },
  {
    method: "get",
    path: "/conversation/:conversation_id/events/:event_id/auth",
    alias: "GetEventJWT",
    description: `Get a auth JWT for an event`,
    requestFormat: "json",
    response: JwtResponse,
  },
  {
    method: "post",
    path: "/conversation/:conversation_id/events/:event_id/report",
    alias: "SubmitEventReport",
    description: `Submit categorization report to bulk storage`,
    requestFormat: "json",
    parameters: [
      {
        name: "room_id",
        type: "Query",
        schema: created_after,
      },
    ],
    response: SubmitReportResponse,
  },
  {
    method: "post",
    path: "/conversation/:conversation_id/events/:event_id/transcriptions",
    alias: "ProcessVideoCallTranscriptions",
    description: `Triggers transcription processing in a background worker`,
    requestFormat: "json",
    response: ProcessTranscriptionResponse,
  },
  {
    method: "get",
    path: "/conversation/:conversation_id/events/:event_id/workflows",
    alias: "ListEventWorkflows",
    requestFormat: "json",
    response: z.array(WorkflowDto),
  },
  {
    method: "post",
    path: "/conversation/:conversation_id/events/:event_id/workflows",
    alias: "CreateEventWorkflow",
    requestFormat: "json",
    parameters: [
      {
        name: "body",
        type: "Body",
        schema: CreateWorkflow,
      },
    ],
    response: WorkflowDto,
  },
  {
    method: "get",
    path: "/conversation/:conversation_id/events/:event_id/workflows/:workflow_id",
    alias: "GetEventWorkflow",
    requestFormat: "json",
    response: WorkflowDto,
  },
  {
    method: "put",
    path: "/conversation/:conversation_id/events/:event_id/workflows/:workflow_id",
    alias: "UpdateEventWorkflow",
    requestFormat: "json",
    parameters: [
      {
        name: "body",
        type: "Body",
        schema: PartialWorkflow,
      },
    ],
    response: WorkflowDto,
  },
  {
    method: "delete",
    path: "/conversation/:conversation_id/events/:event_id/workflows/:workflow_id",
    alias: "DeleteEventWorkflow",
    requestFormat: "json",
    response: WorkflowDto,
  },
  {
    method: "get",
    path: "/conversation/:conversation_id/events/:event_id/workflows/:workflow_id/workflow_steps",
    alias: "ListEventWorkflowSteps",
    description: `
List the workflow steps associated with this workflow.

Use query param withTranslations&#x3D;true to get the translation data for each step.

Use query param withUserProgress&#x3D;true to get the active user&#x27;s progress status for each step.`,
    requestFormat: "json",
    parameters: [
      {
        name: "withTranslations",
        type: "Query",
        schema: z.boolean().optional().default(false),
      },
      {
        name: "withUserProgress",
        type: "Query",
        schema: z.boolean().optional().default(false),
      },
    ],
    response: WorkflowStepsListResponse,
  },
  {
    method: "post",
    path: "/conversation/:conversation_id/events/:event_id/workflows/:workflow_id/workflow_steps",
    alias: "CreateEventWorkflowStep",
    requestFormat: "json",
    parameters: [
      {
        name: "body",
        type: "Body",
        schema: CreateWorkflowStep,
      },
    ],
    response: WorkflowStepDto,
  },
  {
    method: "get",
    path: "/conversation/:conversation_id/events/:event_id/workflows/:workflow_id/workflow_steps/:workflow_step_id",
    alias: "GetEventWorkflowStep",
    requestFormat: "json",
    response: LocalizedWorkflowStepDto,
  },
  {
    method: "put",
    path: "/conversation/:conversation_id/events/:event_id/workflows/:workflow_id/workflow_steps/:workflow_step_id",
    alias: "UpdateEventWorkflowStep",
    requestFormat: "json",
    parameters: [
      {
        name: "body",
        type: "Body",
        schema: PartialWorkflowStep,
      },
    ],
    response: WorkflowStepDto,
  },
  {
    method: "delete",
    path: "/conversation/:conversation_id/events/:event_id/workflows/:workflow_id/workflow_steps/:workflow_step_id",
    alias: "DeleteEventWorkflowStep",
    requestFormat: "json",
    response: WorkflowStepDto,
  },
  {
    method: "get",
    path: "/conversation/:conversation_id/feedback",
    alias: "ListFeedbackForConversation",
    requestFormat: "json",
    response: FeedbackDto,
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
        schema: z.object({ content: z.string() }).passthrough(),
      },
    ],
    response: FeedbackDto,
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
        schema: PartialFeedback,
      },
    ],
    response: FeedbackDto,
  },
  {
    method: "get",
    path: "/conversation/:conversation_id/invite",
    alias: "ListInvitesForConversation",
    requestFormat: "json",
    response: z.array(InviteDto),
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
        schema: CreateInviteDTO,
      },
    ],
    response: InviteDto,
  },
  {
    method: "get",
    path: "/conversation/:conversation_id/invite/:invite_id",
    alias: "GetInvite",
    requestFormat: "json",
    response: InviteDto,
  },
  {
    method: "delete",
    path: "/conversation/:conversation_id/invite/:invite_id",
    alias: "DeleteInvite",
    requestFormat: "json",
    response: InviteDto,
  },
  {
    method: "patch",
    path: "/conversation/:conversation_id/invite/:invite_id",
    alias: "UpdateInvite",
    requestFormat: "json",
    parameters: [
      {
        name: "body",
        type: "Body",
        schema: PartialInvite,
      },
    ],
    response: InviteDto,
  },
  {
    method: "post",
    path: "/conversation/:conversation_id/invite/:invite_id/accept",
    alias: "AcceptInvite",
    requestFormat: "json",
    response: InviteDto,
  },
  {
    method: "post",
    path: "/conversation/:conversation_id/invite/:invite_id/events",
    alias: "AutoRegisterEventAttendance",
    requestFormat: "json",
    response: InviteDto,
  },
  {
    method: "post",
    path: "/conversation/:conversation_id/invite/:invite_id/reject",
    alias: "RejectInvite",
    requestFormat: "json",
    response: InviteDto,
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
    path: "/conversation/:conversation_id/invite/events",
    alias: "CreateEventInvite",
    description: `Create an invite for a given event`,
    requestFormat: "json",
    parameters: [
      {
        name: "body",
        type: "Body",
        schema: CreateInviteDTO,
      },
    ],
    response: InviteDto,
  },
  {
    method: "get",
    path: "/conversation/:conversation_id/invite/events/:event_id",
    alias: "ListInvitesForEvent",
    requestFormat: "json",
    response: z.array(InviteDto),
  },
  {
    method: "put",
    path: "/conversation/:conversation_id/launch",
    alias: "LaunchConversation",
    description: `Makes the conversation live for participants`,
    requestFormat: "json",
    response: ConversationDto,
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
        schema: SendNotificationRequest,
      },
    ],
    response: SendEmailNotificationResponse,
  },
  {
    method: "get",
    path: "/conversation/:conversation_id/notifications/recipients",
    alias: "GetNotificationRecipients",
    description: `Returns participant count for in-app delivery and the list of email addresses opted in to broadcast emails. Owner-only.`,
    requestFormat: "json",
    response: NotificationRecipientsResponse,
  },
  {
    method: "get",
    path: "/conversation/:conversation_id/report",
    alias: "GetReportForConversation",
    requestFormat: "json",
    response: FullReportDto,
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
        schema: PartialReport,
      },
    ],
    response: ReportDto,
  },
  {
    method: "post",
    path: "/conversation/:conversation_id/report",
    alias: "GenerateReportForConversation",
    requestFormat: "json",
    response: FullReportDto,
  },
  {
    method: "get",
    path: "/conversation/:conversation_id/report/:report_id/impacts",
    alias: "ListImpactsForReport",
    requestFormat: "json",
    response: z.array(ReportImpactDto),
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
        schema: PartialReportImpact,
      },
    ],
    response: ReportImpactDto,
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
        schema: CreateImpactDTO,
      },
    ],
    response: ReportImpactDto,
  },
  {
    method: "get",
    path: "/conversation/:conversation_id/workflow",
    alias: "ListConversationWorkflows",
    requestFormat: "json",
    response: z.array(WorkflowDto),
  },
  {
    method: "post",
    path: "/conversation/:conversation_id/workflow",
    alias: "CreateConversationWorkflow",
    requestFormat: "json",
    parameters: [
      {
        name: "body",
        type: "Body",
        schema: CreateWorkflow,
      },
    ],
    response: WorkflowDto,
  },
  {
    method: "get",
    path: "/conversation/:conversation_id/workflow/:workflow_id",
    alias: "GetConversationWorkflow",
    requestFormat: "json",
    response: WorkflowDto,
  },
  {
    method: "put",
    path: "/conversation/:conversation_id/workflow/:workflow_id",
    alias: "UpdateConversationWorkflow",
    requestFormat: "json",
    parameters: [
      {
        name: "body",
        type: "Body",
        schema: PartialWorkflow,
      },
    ],
    response: WorkflowDto,
  },
  {
    method: "delete",
    path: "/conversation/:conversation_id/workflow/:workflow_id",
    alias: "DeleteConversationWorkflow",
    requestFormat: "json",
    response: WorkflowDto,
  },
  {
    method: "delete",
    path: "/conversation/:conversation_id/workflow/:workflow_id/leave",
    alias: "UnregisterUserForConversationWorkflow",
    requestFormat: "json",
    response: UserParticipation,
  },
  {
    method: "get",
    path: "/conversation/:conversation_id/workflow/:workflow_id/next",
    alias: "NextConversationWorkflowStepForUser",
    requestFormat: "json",
    response: z.union([WorkflowStep, z.null()]),
  },
  {
    method: "get",
    path: "/conversation/:conversation_id/workflow/:workflow_id/participation",
    alias: "GetUserConversationParticipation",
    requestFormat: "json",
    response: z.union([UserParticipation, z.null()]),
  },
  {
    method: "get",
    path: "/conversation/:conversation_id/workflow/:workflow_id/participation_report",
    alias: "GetConversationWorkflowParticipationReport",
    requestFormat: "json",
    response: DemographicReport,
  },
  {
    method: "get",
    path: "/conversation/:conversation_id/workflow/:workflow_id/progress",
    alias: "GetUserProgress",
    requestFormat: "json",
    response: z.array(UserProgressDto),
  },
  {
    method: "put",
    path: "/conversation/:conversation_id/workflow/:workflow_id/progress/:workflow_step_id",
    alias: "SetUserProgress",
    requestFormat: "json",
    parameters: [
      {
        name: "body",
        type: "Body",
        schema: UpdateUserProgress,
      },
    ],
    response: UserProgressDto,
  },
  {
    method: "post",
    path: "/conversation/:conversation_id/workflow/:workflow_id/register",
    alias: "RegisterUserForConversationWorkflow",
    requestFormat: "json",
    response: UserParticipation,
  },
  {
    method: "get",
    path: "/conversation/:conversation_id/workflow/:workflow_id/stats",
    alias: "GetConversationWorkflowStats",
    requestFormat: "json",
    response: WorkflowStats,
  },
  {
    method: "get",
    path: "/conversation/:conversation_id/workflow/:workflow_id/workflow_step",
    alias: "ListConversationWorkflowSteps",
    description: `
List the workflow steps associated with this workflow.

Use query param withTranslations&#x3D;true to get the translation data for each step.

Use query param withUserProgress&#x3D;true to get the active user&#x27;s progress status for each step.`,
    requestFormat: "json",
    parameters: [
      {
        name: "withTranslations",
        type: "Query",
        schema: z.boolean().optional().default(false),
      },
      {
        name: "withUserProgress",
        type: "Query",
        schema: z.boolean().optional().default(false),
      },
    ],
    response: WorkflowStepsListResponse,
  },
  {
    method: "post",
    path: "/conversation/:conversation_id/workflow/:workflow_id/workflow_step",
    alias: "CreateConversationWorkflowStep",
    requestFormat: "json",
    parameters: [
      {
        name: "body",
        type: "Body",
        schema: CreateWorkflowStep,
      },
    ],
    response: WorkflowStepDto,
  },
  {
    method: "get",
    path: "/conversation/:conversation_id/workflow/:workflow_id/workflow_step/:workflow_step_id",
    alias: "GetConversationWorkflowStep",
    requestFormat: "json",
    response: LocalizedWorkflowStepDto,
  },
  {
    method: "put",
    path: "/conversation/:conversation_id/workflow/:workflow_id/workflow_step/:workflow_step_id",
    alias: "UpdateConversationWorkflowStep",
    requestFormat: "json",
    parameters: [
      {
        name: "body",
        type: "Body",
        schema: PartialWorkflowStep,
      },
    ],
    response: WorkflowStepDto,
  },
  {
    method: "delete",
    path: "/conversation/:conversation_id/workflow/:workflow_id/workflow_step/:workflow_step_id",
    alias: "DeleteConversationWorkflowStep",
    requestFormat: "json",
    response: WorkflowStepDto,
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
    path: "/email_template_configs",
    alias: "ListEmailTemplateConfigs",
    description: `List custom email template configurations`,
    requestFormat: "json",
    parameters: [
      {
        name: "organization_id",
        type: "Query",
        schema: created_after,
      },
      {
        name: "owner_id",
        type: "Query",
        schema: created_after,
      },
    ],
    response: z.array(EmailTemplateConfigDto),
  },
  {
    method: "post",
    path: "/email_template_configs",
    alias: "CreateEmailTemplateConfig",
    description: `Create custom content for specific email template`,
    requestFormat: "json",
    parameters: [
      {
        name: "body",
        type: "Body",
        schema: CreateEmailTemplateConfig,
      },
    ],
    response: EmailTemplateConfigDto,
  },
  {
    method: "get",
    path: "/email_template_configs/:email_config_id",
    alias: "GetEmailTemplateConfig",
    description: `Get custom email template configuration`,
    requestFormat: "json",
    response: EmailTemplateConfigDto,
  },
  {
    method: "put",
    path: "/email_template_configs/:email_config_id",
    alias: "UpdateEmailTemplateConfig",
    description: `Update custom email template configuration`,
    requestFormat: "json",
    parameters: [
      {
        name: "body",
        type: "Body",
        schema: UpdateEmailTemplateConfig,
      },
    ],
    response: EmailTemplateConfigDto,
  },
  {
    method: "delete",
    path: "/email_template_configs/:email_config_id",
    alias: "DeleteEmailTemplateConfig",
    description: `Delete custom email template configuration`,
    requestFormat: "json",
    response: EmailTemplateConfigDto,
  },
  {
    method: "get",
    path: "/email_template_configs/schemas",
    alias: "ListEmailSlotSchemas",
    description: `List all slot schemas for each email template type`,
    requestFormat: "json",
    response: z.array(EmailTypeSchema),
  },
  {
    method: "get",
    path: "/jobs",
    alias: "ListJobs",
    requestFormat: "json",
    parameters: [
      {
        name: "completion_message",
        type: "Query",
        schema: created_after,
      },
      {
        name: "progress",
        type: "Query",
        schema: created_after,
      },
      {
        name: "status",
        type: "Query",
        schema: created_after,
      },
      {
        name: "step",
        type: "Query",
        schema: created_after,
      },
      {
        name: "limit",
        type: "Query",
        schema: limit,
      },
      {
        name: "offset",
        type: "Query",
        schema: limit,
      },
    ],
    response: PaginatedResults_for_Job,
  },
  {
    method: "post",
    path: "/jobs",
    alias: "CreateJob",
    requestFormat: "json",
    parameters: [
      {
        name: "body",
        type: "Body",
        schema: CreateJob,
      },
    ],
    response: Job,
  },
  {
    method: "get",
    path: "/jobs/:job_id",
    alias: "GetJob",
    requestFormat: "json",
    response: Job,
  },
  {
    method: "delete",
    path: "/jobs/:job_id",
    alias: "DeleteJob",
    requestFormat: "json",
    response: z.void(),
  },
  {
    method: "get",
    path: "/media",
    alias: "ListMedia",
    description: `List media records`,
    requestFormat: "json",
    parameters: [
      {
        name: "created_at",
        type: "Query",
        schema: created_at,
      },
      {
        name: "filename",
        type: "Query",
        schema: created_at,
      },
      {
        name: "content_type",
        type: "Query",
        schema: content_type,
      },
      {
        name: "owner_id",
        type: "Query",
        schema: created_after,
      },
      {
        name: "limit",
        type: "Query",
        schema: limit,
      },
      {
        name: "offset",
        type: "Query",
        schema: limit,
      },
    ],
    response: PaginatedResults_for_MediaDto,
  },
  {
    method: "post",
    path: "/media",
    alias: "postMedia",
    description: `
Upload a media resource to the bulk_storage_service 
and create a new record in the database.


This endpoint requires multipart/form-data.

Generated API clients may not support file uploads.

Use FormData and a raw HTTP request.

**Example (curl):**
&#x60;&#x60;&#x60;bash
curl -X POST \
-H &#x27;Cookie: auth-token&#x3D;...;&#x27; \
&#x27;localhost:3000/media&#x27; \
--form &#x27;file&#x3D;@/path-to-document.pdf&#x27;
&#x60;&#x60;&#x60;
                            `,
    requestFormat: "form-data",
    parameters: [
      {
        name: "body",
        description: `multipart form data`,
        type: "Body",
        schema: z.array(z.any()),
      },
    ],
    response: MediaDto,
  },
  {
    method: "get",
    path: "/media/:media_id",
    alias: "GetMedia",
    description: `Get media record by id`,
    requestFormat: "json",
    response: MediaDto,
  },
  {
    method: "delete",
    path: "/media/:media_id",
    alias: "DeleteMedia",
    description: `Delete media record by id`,
    requestFormat: "json",
    response: MediaDto,
  },
  {
    method: "get",
    path: "/notifications",
    alias: "GetAllNotifications",
    description: `Returns a paginated list of all notification deliveries for the authenticated user`,
    requestFormat: "json",
    parameters: [
      {
        name: "limit",
        type: "Query",
        schema: limit,
      },
      {
        name: "offset",
        type: "Query",
        schema: limit,
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
    alias: "GetUnreadNotifications",
    description: `Returns a paginated list of unread notification deliveries for the authenticated user`,
    requestFormat: "json",
    parameters: [
      {
        name: "limit",
        type: "Query",
        schema: limit,
      },
      {
        name: "offset",
        type: "Query",
        schema: limit,
      },
    ],
    response: PaginatedResults_for_NotificationWithDelivery,
  },
  {
    method: "get",
    path: "/notifications/unread/count",
    alias: "GetUnreadNotificationsCount",
    description: `Returns the count of unread notifications for the authenticated user`,
    requestFormat: "json",
    response: z.object({ count: z.number().int() }).passthrough(),
  },
  {
    method: "get",
    path: "/organizations",
    alias: "ListOrganizations",
    description: `Paginated list of organizations with optional ordering`,
    requestFormat: "json",
    parameters: [
      {
        name: "created_at",
        type: "Query",
        schema: created_at,
      },
      {
        name: "name",
        type: "Query",
        schema: created_at,
      },
      {
        name: "region_id",
        type: "Query",
        schema: created_after,
      },
      {
        name: "limit",
        type: "Query",
        schema: limit,
      },
      {
        name: "offset",
        type: "Query",
        schema: limit,
      },
    ],
    response: PaginatedResults_for_LocalizedOrganizationDto,
  },
  {
    method: "post",
    path: "/organizations",
    alias: "CreateOrganization",
    description: `Create a new organization`,
    requestFormat: "json",
    parameters: [
      {
        name: "body",
        type: "Body",
        schema: CreateOrganization,
      },
    ],
    response: OrganizationDto,
  },
  {
    method: "get",
    path: "/organizations/:organization_id",
    alias: "GetOrganization",
    description: `Get an organization by id`,
    requestFormat: "json",
    response: LocalizedOrganizationDto,
  },
  {
    method: "put",
    path: "/organizations/:organization_id",
    alias: "UpdateOrganization",
    description: `Update an organization`,
    requestFormat: "json",
    parameters: [
      {
        name: "body",
        type: "Body",
        schema: PartialOrganization,
      },
    ],
    response: OrganizationDto,
  },
  {
    method: "delete",
    path: "/organizations/:organization_id",
    alias: "DeleteOrganization",
    description: `Delete an organization`,
    requestFormat: "json",
    response: OrganizationDto,
  },
  {
    method: "get",
    path: "/regions",
    alias: "ListRegions",
    description: `Paginated list of regions with optional ordering`,
    requestFormat: "json",
    parameters: [
      {
        name: "created_at",
        type: "Query",
        schema: created_at,
      },
      {
        name: "name",
        type: "Query",
        schema: created_at,
      },
      {
        name: "organization_id",
        type: "Query",
        schema: created_after,
      },
      {
        name: "limit",
        type: "Query",
        schema: limit,
      },
      {
        name: "offset",
        type: "Query",
        schema: limit,
      },
    ],
    response: PaginatedResults_for_LocalizedRegionDto,
  },
  {
    method: "post",
    path: "/regions",
    alias: "CreateRegion",
    description: `Create a new region`,
    requestFormat: "json",
    parameters: [
      {
        name: "body",
        type: "Body",
        schema: CreateRegion,
      },
    ],
    response: RegionDto,
  },
  {
    method: "get",
    path: "/regions/:region_id",
    alias: "GetRegion",
    description: `Get a region by id`,
    requestFormat: "json",
    response: LocalizedRegionDto,
  },
  {
    method: "put",
    path: "/regions/:region_id",
    alias: "UpdateRegion",
    description: `Update a region`,
    requestFormat: "json",
    parameters: [
      {
        name: "body",
        type: "Body",
        schema: PartialRegion,
      },
    ],
    response: RegionDto,
  },
  {
    method: "delete",
    path: "/regions/:region_id",
    alias: "DeleteRegion",
    description: `Delete a region`,
    requestFormat: "json",
    response: RegionDto,
  },
  {
    method: "get",
    path: "/services",
    alias: "ListSupportedServices",
    description: `List of services supported (configured) by current Comhairle server`,
    requestFormat: "json",
    response: ComhairleServices,
  },
  {
    method: "get",
    path: "/tools/elicitation_bot/workflow_step/:workflow_step_id",
    alias: "GetElicitationBotSessionHistory",
    description: `Returns a user session for an elicitation bot including message history`,
    requestFormat: "json",
    response: ComhairleAgentSession,
  },
  {
    method: "post",
    path: "/tools/elicitation_bot/workflow_step/:workflow_step_id",
    alias: "postToolselicitation_botworkflow_stepWorkflow_step_id",
    description: `
Streamed LLM response.
⚠️ This endpoint returns a streaming response on success.
Generated API clients are NOT suitable for consuming this endpoint.
Use a raw HTTP request and process the response body incrementally.
`,
    requestFormat: "json",
    parameters: [
      {
        name: "body",
        type: "Body",
        schema: z.object({ question: z.string() }).passthrough(),
      },
    ],
    response: z.void(),
  },
  {
    method: "post",
    path: "/tools/polis/admin_login",
    alias: "PolisAdminLogin",
    description: `Logs into Polis as admin and returns session cookie`,
    requestFormat: "json",
    parameters: [
      {
        name: "workflow_step_id",
        type: "Query",
        schema: z.string().uuid(),
      },
    ],
    response: z.void(),
  },
  {
    method: "get",
    path: "/tools/polis/report_data",
    alias: "PolisGetReportData",
    description: `Fetches the polis data export for a given workflow step`,
    requestFormat: "json",
    parameters: [
      {
        name: "workflow_step_id",
        type: "Query",
        schema: z.string().uuid(),
      },
    ],
    response: z.void(),
  },
  {
    method: "get",
    path: "/tools/polis/statement_aux",
    alias: "PolisListStatementAux",
    description: `Returns auxiliary statement data filtered by workflow_step_id and/or polis_conversation_id (at least one is required)`,
    requestFormat: "json",
    parameters: [
      {
        name: "polis_conversation_id",
        type: "Query",
        schema: created_after,
      },
      {
        name: "workflow_step_id",
        type: "Query",
        schema: created_after,
      },
    ],
    response: z.array(PolisStatementAux),
  },
  {
    method: "post",
    path: "/tools/polis/statement_aux",
    alias: "PolisCreateStatementAux",
    description: `Creates a polis_statement_aux row capturing statement text, moderation status, themes and the visible statement at submission time`,
    requestFormat: "json",
    parameters: [
      {
        name: "body",
        type: "Body",
        schema: CreatePolisStatementAux,
      },
    ],
    response: PolisStatementAux,
  },
  {
    method: "put",
    path: "/tools/polis/statement_aux/:id",
    alias: "PolisUpdateStatementAux",
    description: `Updates statement_text, moderation_status, themes, visible_statement_when_submitted, or moderation_reason`,
    requestFormat: "json",
    parameters: [
      {
        name: "body",
        type: "Body",
        schema: UpdatePolisStatementAux,
      },
    ],
    response: PolisStatementAux,
  },
  {
    method: "post",
    path: "/tools/polis/statement_aux/sync",
    alias: "PolisSyncStatementAux",
    description: `Fetches comments and xid mappings from Polis and upserts a row per statement. Existing rows have their statement_text and is_seed refreshed; moderation_status, moderation_reason, themes, visible_statement_when_submitted and user_id are preserved.`,
    requestFormat: "json",
    parameters: [
      {
        name: "body",
        type: "Body",
        schema: z.object({ workflow_step_id: z.string().uuid() }).passthrough(),
      },
    ],
    response: SyncStatementAuxResponse,
  },
  {
    method: "get",
    path: "/tools/polis/statement_aux/theme_stats",
    alias: "PolisStatementAuxThemeStats",
    description: `Returns the count of polis_statement_aux rows tagged with each theme, filtered by workflow_step_id and/or polis_conversation_id (at least one is required)`,
    requestFormat: "json",
    parameters: [
      {
        name: "polis_conversation_id",
        type: "Query",
        schema: created_after,
      },
      {
        name: "workflow_step_id",
        type: "Query",
        schema: created_after,
      },
    ],
    response: z.array(ThemeStatistic),
  },
  {
    method: "get",
    path: "/tools/prioritization/proposals",
    alias: "ListProposals",
    description: `List proposals for a given prioritization tool workflow_step. Admin callers may pass &#x60;withTranslations&#x3D;true&#x60; to receive raw TextContentId references plus full translation data so the admin UI can drive the standard TranslatableField component.`,
    requestFormat: "json",
    parameters: [
      {
        name: "withTranslations",
        type: "Query",
        schema: z.boolean().optional().default(false),
      },
      {
        name: "workflowStepId",
        type: "Query",
        schema: z.string().uuid(),
      },
    ],
    response: ProposalsListResponse,
  },
  {
    method: "post",
    path: "/tools/prioritization/proposals",
    alias: "CreateProposal",
    description: `
Create a new prioritization tool proposal for a given prioritization tool workflow_step
`,
    requestFormat: "json",
    parameters: [
      {
        name: "body",
        type: "Body",
        schema: CreateProposalRequest,
      },
    ],
    response: ProposalDto,
  },
  {
    method: "delete",
    path: "/tools/prioritization/proposals/:proposal_id",
    alias: "DeleteProposal",
    description: `Delete a prioritization tool proposal`,
    requestFormat: "json",
    response: ProposalDto,
  },
  {
    method: "get",
    path: "/tools/prioritization/proposals/:proposal_id/responses",
    alias: "ListProposalResponses",
    description: `List responses for a prioritization tool proposal`,
    requestFormat: "json",
    response: z.array(ProposalResponseDto),
  },
  {
    method: "post",
    path: "/tools/prioritization/proposals/:proposal_id/responses",
    alias: "CreateProposalResponse",
    description: `
Create a response for prioritization tool proposal
`,
    requestFormat: "json",
    parameters: [
      {
        name: "body",
        type: "Body",
        schema: CreateResponse,
      },
    ],
    response: ProposalResponseDto,
  },
  {
    method: "get",
    path: "/tools/stories/:story_id",
    alias: "GetStory",
    description: `Returns a story by id`,
    requestFormat: "json",
    response: Story,
  },
  {
    method: "get",
    path: "/tools/stories/workflow_step/:workflow_step_id",
    alias: "GetStories",
    description: `Returns stories for the current workflow step if it is a stories endpoint`,
    requestFormat: "json",
    response: z.array(Story),
  },
  {
    method: "post",
    path: "/tools/stories/workflow_step/:workflow_step_id",
    alias: "SaveStory",
    description: `Record a user story for the current user and workflow step`,
    requestFormat: "json",
    response: z.void(),
  },
  {
    method: "post",
    path: "/tools/thinking_space",
    alias: "postToolsthinking_space",
    description: `
Streamed LLM response.
⚠️ This endpoint returns a streaming response on success.
Generated API clients are NOT suitable for consuming this endpoint.
Use a raw HTTP request and process the response body incrementally.
`,
    requestFormat: "json",
    parameters: [
      {
        name: "body",
        type: "Body",
        schema: ConversationRequest2,
      },
    ],
    response: z.void(),
  },
  {
    method: "get",
    path: "/tools/thinking_space/answers",
    alias: "ListThinkingSpaceAnswers",
    description: `List answer for thinking space workflow step`,
    requestFormat: "json",
    parameters: [
      {
        name: "status",
        type: "Query",
        schema: status,
      },
      {
        name: "user_id",
        type: "Query",
        schema: created_after,
      },
      {
        name: "workflow_step_id",
        type: "Query",
        schema: z.string().uuid(),
      },
    ],
    response: z.array(ThinkingSpaceAnswerDto),
  },
  {
    method: "post",
    path: "/tools/thinking_space/answers",
    alias: "CreateThinkingSpaceAnswer",
    description: `Create an answer for thinking space workflow step question`,
    requestFormat: "json",
    parameters: [
      {
        name: "body",
        type: "Body",
        schema: CreateAnswerRequest,
      },
    ],
    response: ThinkingSpaceAnswerDto,
  },
  {
    method: "put",
    path: "/tools/thinking_space/answers/:answer_id",
    alias: "UpdateThinkingSpaceAnswer",
    description: `Update an answer for thinking space workflow step question`,
    requestFormat: "json",
    parameters: [
      {
        name: "body",
        type: "Body",
        schema: UpdateAnswer,
      },
    ],
    response: ThinkingSpaceAnswerDto,
  },
  {
    method: "get",
    path: "/tools/thinking_space/follow_ups",
    alias: "ListThinkingSpaceFollowUpQuestions",
    description: `List thinking space follow up questions`,
    requestFormat: "json",
    parameters: [
      {
        name: "workflow_step_id",
        type: "Query",
        schema: z.string().uuid(),
      },
      {
        name: "root_question_id",
        type: "Query",
        schema: created_after,
      },
      {
        name: "user_id",
        type: "Query",
        schema: created_after,
      },
    ],
    response: z.array(ThinkingSpaceFollowUpQuestionDto),
  },
  {
    method: "post",
    path: "/tools/thinking_space/follow_ups",
    alias: "CreateThinkingSpaceFollowUpQuestions",
    description: `Create thinking space follow up questions`,
    requestFormat: "json",
    parameters: [
      {
        name: "body",
        type: "Body",
        schema: CreateFollowUpQuestions,
      },
    ],
    response: ThinkingSpaceFollowUpQuestionDto,
  },
  {
    method: "put",
    path: "/tools/thinking_space/follow_ups/:follow_up_id",
    alias: "UpdateThinkingSpaceFollowUpQuestions",
    description: `Update thinking space follow up questions`,
    requestFormat: "json",
    parameters: [
      {
        name: "body",
        type: "Body",
        schema: UpdateFollowUpQuestions,
      },
    ],
    response: ThinkingSpaceFollowUpQuestionDto,
  },
  {
    method: "get",
    path: "/tools/thinking_space/summaries",
    alias: "ListThinkingSpaceSummaries",
    description: `List thinking space summaries`,
    requestFormat: "json",
    parameters: [
      {
        name: "is_ai_generated",
        type: "Query",
        schema: is_complete,
      },
      {
        name: "user_id",
        type: "Query",
        schema: created_after,
      },
      {
        name: "workflow_step_id",
        type: "Query",
        schema: z.string().uuid(),
      },
    ],
    response: z.array(ThinkingSpaceSummaryDto),
  },
  {
    method: "post",
    path: "/tools/thinking_space/summaries",
    alias: "UpdateOrCreateThinkingSpaceSummary",
    description: `Update a summary if already exists or create a new summary`,
    requestFormat: "json",
    parameters: [
      {
        name: "body",
        type: "Body",
        schema: UpdateCreateThinkingSpace,
      },
    ],
    response: ThinkingSpaceSummaryDto,
  },
  {
    method: "get",
    path: "/tools/thinking_space/summaries/:summary_id",
    alias: "GetThinkingSpaceSummary",
    description: `Get a thinking space summary by id`,
    requestFormat: "json",
    parameters: [
      {
        name: "workflow_step_id",
        type: "Query",
        schema: z.string().uuid(),
      },
    ],
    response: ThinkingSpaceSummaryDto,
  },
  {
    method: "post",
    path: "/tools/thinking_space/summaries/generate",
    alias: "GenerateThinkingSpaceSummary",
    description: `Generates a thinking space summary via bot service agent`,
    requestFormat: "json",
    parameters: [
      {
        name: "body",
        type: "Body",
        schema: z.object({ workflow_step_id: z.string().uuid() }).passthrough(),
      },
    ],
    response: ThinkingSpaceSummaryDto,
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
        schema: CreateTextContentRequest,
      },
    ],
    response: TextContentDto,
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
        schema: UpdateTextContent,
      },
    ],
    response: TextContentDto,
  },
  {
    method: "delete",
    path: "/translations/:text_content_id",
    alias: "DeleteTextContent",
    description: `Delete a TextContent entry and all its translations`,
    requestFormat: "json",
    response: TextContentDto,
  },
  {
    method: "get",
    path: "/translations/:text_content_id/:locale",
    alias: "GetTextTranslation",
    description: `Get a translation for a specific TextContent and locale`,
    requestFormat: "json",
    response: TextTranslationDto,
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
        schema: UpdateTextTranslation,
      },
    ],
    response: TextTranslationDto,
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
        schema: CreateOrUpdateTextTranslationRequest,
      },
    ],
    response: TextTranslationDto,
  },
  {
    method: "delete",
    path: "/translations/:text_content_id/:locale",
    alias: "DeleteTextTranslation",
    description: `Delete a translation for a specific locale`,
    requestFormat: "json",
    response: TextTranslationDto,
  },
  {
    method: "post",
    path: "/translations/:text_content_id/:locale/translate",
    alias: "AutomaticallyGenerateTranslation",
    description: `Use the primary_locale language and translate this language from it using the tarnslation service`,
    requestFormat: "json",
    response: TextTranslationDto,
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
    response: z.array(LocalizedConversationDto),
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
        schema: UpdateUserRequest,
      },
    ],
    response: UserDto,
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
        schema: created_after,
      },
      {
        name: "created_before",
        type: "Query",
        schema: created_after,
      },
      {
        name: "is_complete",
        type: "Query",
        schema: is_complete,
      },
      {
        name: "is_invite_only",
        type: "Query",
        schema: is_complete,
      },
      {
        name: "is_live",
        type: "Query",
        schema: is_complete,
      },
      {
        name: "is_public",
        type: "Query",
        schema: is_complete,
      },
      {
        name: "keyword",
        type: "Query",
        schema: created_after,
      },
      {
        name: "organization_id",
        type: "Query",
        schema: created_after,
      },
      {
        name: "owner_id",
        type: "Query",
        schema: created_after,
      },
      {
        name: "limit",
        type: "Query",
        schema: limit,
      },
      {
        name: "offset",
        type: "Query",
        schema: limit,
      },
    ],
    response: PaginatedResults_for_LocalizedConversationDto,
  },
  {
    method: "get",
    path: "/user/preferences",
    alias: "GetAllUserConversationPreferences",
    description: `Returns all conversation notification preferences for the authenticated user`,
    requestFormat: "json",
    response: z.array(UserConversationPreferencesDto),
  },
  {
    method: "get",
    path: "/user/preferences/conversation/:conversation_id",
    alias: "GetUserPreferenceForConversation",
    description: `Returns the notification preferences for a specific conversation`,
    requestFormat: "json",
    response: UserConversationPreferencesDto,
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
        schema: UpdateUserConversationPreferences,
      },
    ],
    response: UserConversationPreferencesDto,
  },
  {
    method: "get",
    path: "/user/profile",
    alias: "GetUserProfile",
    description: `Get the current user&#x27;s profile`,
    requestFormat: "json",
    response: UserProfileDto,
  },
  {
    method: "put",
    path: "/user/profile",
    alias: "UpsertUserProfile",
    description: `Create or update the current user&#x27;s profile`,
    requestFormat: "json",
    parameters: [
      {
        name: "body",
        type: "Body",
        schema: UpsertUserProfileRequest,
      },
    ],
    response: UserProfileDto,
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
        schema: UpgradeAccountRequest,
      },
    ],
    response: UserDto,
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
        schema: BroadcastMessage,
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
        schema: BroadcastMessage,
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
        schema: SendToUserMessage,
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
] as const satisfies ZodiosEndpointDefinitions);

export const api: ZodiosInstance<typeof endpoints> = new Zodios(endpoints);

export function createApiClient(
  baseUrl: string,
  options?: ZodiosOptions
): ZodiosInstance<typeof endpoints> {
  return new Zodios(baseUrl, endpoints, options);
}

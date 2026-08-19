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
    allowRevisitAfterFinishing: z.boolean(),
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
    metadata: z.unknown(),
    organizationId: z.union([z.string(), z.null()]).optional(),
    primaryLocale: z.string(),
    privacyPolicy: z.union([z.string(), z.null()]).optional(),
    shortDescription: z.string(),
    shortPrivacyPolicy: z.union([z.string(), z.null()]).optional(),
    showThankYouPageAnnonInstructions: z.boolean(),
    showThankyouPageFeedbackButton: z.boolean(),
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
export const OrganizationType = z.enum(["non_profit", "governmental", "other"]);
export type OrganizationType = z.infer<typeof OrganizationType>;
export const LocalizedOrganizationDto = z
  .object({
    contactEmail: z.union([z.string(), z.null()]).optional(),
    createdAt: z.string().datetime({ offset: true }),
    description: z.string(),
    externalUrl: z.union([z.string(), z.null()]).optional(),
    id: z.string().uuid(),
    metadata: z.unknown().optional(),
    mission: z.string(),
    name: z.string(),
    orgType: OrganizationType,
    regions: z.array(z.string().uuid()),
  })
  .passthrough();
export type LocalizedOrganizationDto = z.infer<typeof LocalizedOrganizationDto>;
export const UserOrganizationAccess = z
  .object({
    canDelete: z.boolean(),
    canManageTeam: z.boolean(),
    canUpdate: z.boolean(),
    isAssociated: z.boolean(),
    organization: LocalizedOrganizationDto,
  })
  .passthrough();
export type UserOrganizationAccess = z.infer<typeof UserOrganizationAccess>;
export const UserOrganizationsResponse = z
  .object({
    canCreateOrganization: z.boolean(),
    organizations: z.array(UserOrganizationAccess),
  })
  .passthrough();
export type UserOrganizationsResponse = z.infer<
  typeof UserOrganizationsResponse
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
export const GroupVoteCounts = z
  .object({
    agrees: z.number().int().gte(0),
    disagrees: z.number().int().gte(0),
    group_id: z.number().int().gte(0),
    passes: z.number().int().gte(0),
  })
  .passthrough();
export type GroupVoteCounts = z.infer<typeof GroupVoteCounts>;
export const VoteCounts = z
  .object({
    agrees: z.number().int().gte(0),
    disagrees: z.number().int().gte(0),
    passes: z.number().int().gte(0),
  })
  .passthrough();
export type VoteCounts = z.infer<typeof VoteCounts>;
export const CommentReportData = z
  .object({
    divisiveness: z.union([z.number(), z.null()]).optional(),
    group_informed_consensus: z.union([z.number(), z.null()]).optional(),
    group_votes: z.array(GroupVoteCounts),
    is_seed: z.boolean(),
    overall_votes: VoteCounts,
    text: z.string(),
    tid: z.number().int().gte(0),
  })
  .passthrough();
export type CommentReportData = z.infer<typeof CommentReportData>;
export const RepresentativeComment = z
  .object({ text: z.string(), tid: z.number().int().gte(0) })
  .passthrough();
export type RepresentativeComment = z.infer<typeof RepresentativeComment>;
export const GroupReportData = z
  .object({
    group_id: z.number().int().gte(0),
    members: z.array(z.number().int().gte(0)),
    representative_comments: z.array(RepresentativeComment),
    total_members: z.number().int().gte(0),
  })
  .passthrough();
export type GroupReportData = z.infer<typeof GroupReportData>;
export const PcaPosition = z
  .object({ x: z.number(), y: z.number() })
  .passthrough();
export type PcaPosition = z.infer<typeof PcaPosition>;
export const ParticipantReportData = z
  .object({
    group_id: z.union([z.number(), z.null()]).optional(),
    pca_position: z.union([PcaPosition, z.null()]).optional(),
    pid: z.number().int().gte(0),
  })
  .passthrough();
export type ParticipantReportData = z.infer<typeof ParticipantReportData>;
export const WikiPollReport = z
  .object({
    comments: z.array(CommentReportData),
    groups: z.array(GroupReportData),
    participants: z.array(ParticipantReportData),
  })
  .passthrough();
export type WikiPollReport = z.infer<typeof WikiPollReport>;
export const UpdatePolisConfigRequest = z
  .object({
    description: z.union([z.string(), z.null()]).optional(),
    is_active: z.union([z.boolean(), z.null()]).optional(),
    strict_moderation: z.union([z.boolean(), z.null()]).optional(),
    topic: z.union([z.string(), z.null()]).optional(),
    workflow_step_id: z.string().uuid(),
  })
  .passthrough();
export type UpdatePolisConfigRequest = z.infer<typeof UpdatePolisConfigRequest>;
export const WikiPoll = z
  .object({
    is_active: z.union([z.boolean(), z.null()]).optional(),
    poll_id: z.string(),
  })
  .passthrough();
export type WikiPoll = z.infer<typeof WikiPoll>;
export const PostSeedRequest = z
  .object({ statement_text: z.string(), workflow_step_id: z.string().uuid() })
  .passthrough();
export type PostSeedRequest = z.infer<typeof PostSeedRequest>;
export const PostSeedResponse = z
  .object({ polis_statement_id: z.string() })
  .passthrough();
export type PostSeedResponse = z.infer<typeof PostSeedResponse>;
export const ModerationStatus = z.enum(["accepted", "rejected", "pending"]);
export type ModerationStatus = z.infer<typeof ModerationStatus>;
export const PolisStatementAux = z
  .object({
    created_at: z.string().datetime({ offset: true }),
    id: z.string().uuid(),
    is_seed: z.boolean(),
    moderation_reason: z.union([z.string(), z.null()]).optional(),
    moderation_status: ModerationStatus,
    original_statement_id: z.union([z.string(), z.null()]).optional(),
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
export const ThemeRequest = z.object({ theme: z.string() }).passthrough();
export type ThemeRequest = z.infer<typeof ThemeRequest>;
export const ModerationDecisionRequest = z.enum(["accept", "reject"]);
export type ModerationDecisionRequest = z.infer<
  typeof ModerationDecisionRequest
>;
export const ModerateStatementAuxRequest = z
  .object({
    decision: ModerationDecisionRequest,
    moderation_reason: z.union([z.string(), z.null()]).optional(),
  })
  .passthrough();
export type ModerateStatementAuxRequest = z.infer<
  typeof ModerateStatementAuxRequest
>;
export const ModerateStatementAuxBatchRequest = z
  .object({
    decision: ModerationDecisionRequest,
    ids: z.array(z.string().uuid()),
    moderation_reason: z.union([z.string(), z.null()]).optional(),
  })
  .passthrough();
export type ModerateStatementAuxBatchRequest = z.infer<
  typeof ModerateStatementAuxBatchRequest
>;
export const ModerateBatchFailure = z
  .object({ error: z.string(), id: z.string().uuid() })
  .passthrough();
export type ModerateBatchFailure = z.infer<typeof ModerateBatchFailure>;
export const ModerateStatementAuxBatchResponse = z
  .object({
    failed: z.array(ModerateBatchFailure),
    succeeded: z.array(PolisStatementAux),
  })
  .passthrough();
export type ModerateStatementAuxBatchResponse = z.infer<
  typeof ModerateStatementAuxBatchResponse
>;
export const SplitStatementRequest = z
  .object({ replacements: z.array(z.string()) })
  .passthrough();
export type SplitStatementRequest = z.infer<typeof SplitStatementRequest>;
export const SplitStatementResponse = z
  .object({
    original: PolisStatementAux,
    replacements: z.array(PolisStatementAux),
  })
  .passthrough();
export type SplitStatementResponse = z.infer<typeof SplitStatementResponse>;
export const FormField = z
  .object({
    description: z.unknown().optional(),
    frozen: z.union([z.boolean(), z.null()]).optional(),
    hide: z.union([z.boolean(), z.null()]).optional(),
    id: z.string(),
    kind: z.string(),
    layout: z
      .union([z.object({}).partial().passthrough(), z.null()])
      .optional(),
    properties: z
      .union([z.object({}).partial().passthrough(), z.null()])
      .optional(),
    title: z.unknown().optional(),
    validations: z
      .union([z.object({}).partial().passthrough(), z.null()])
      .optional(),
    width: z.union([z.number(), z.null()]).optional(),
  })
  .passthrough();
export type FormField = z.infer<typeof FormField>;
export const FormSettings = z
  .object({
    active: z.union([z.boolean(), z.null()]),
    allowArchive: z.union([z.boolean(), z.null()]),
    enableQuestionList: z.union([z.boolean(), z.null()]),
    locale: z.union([z.string(), z.null()]),
    published: z.union([z.boolean(), z.null()]),
  })
  .partial()
  .passthrough();
export type FormSettings = z.infer<typeof FormSettings>;
export const FormTheme = z
  .object({
    answerTextColor: z.union([z.string(), z.null()]),
    backgroundBrightness: z.union([z.number(), z.null()]),
    backgroundColor: z.union([z.string(), z.null()]),
    backgroundImage: z.union([z.string(), z.null()]),
    buttonBackground: z.union([z.string(), z.null()]),
    buttonTextColor: z.union([z.string(), z.null()]),
    customCSS: z.union([z.string(), z.null()]),
    fontFamily: z.union([z.string(), z.null()]),
    logo: z.union([z.string(), z.null()]),
    questionTextColor: z.union([z.string(), z.null()]),
  })
  .partial()
  .passthrough();
export type FormTheme = z.infer<typeof FormTheme>;
export const ThemeSettings = z
  .object({ theme: z.union([FormTheme, z.null()]) })
  .partial()
  .passthrough();
export type ThemeSettings = z.infer<typeof ThemeSettings>;
export const Form = z
  .object({
    description: z.union([z.string(), z.null()]).optional(),
    draft: z.union([z.boolean(), z.null()]).optional(),
    fields: z.union([z.array(FormField), z.null()]).optional(),
    id: z.string(),
    interactiveMode: z.union([z.number(), z.null()]).optional(),
    kind: z.union([z.number(), z.null()]).optional(),
    name: z.union([z.string(), z.null()]).optional(),
    projectId: z.string(),
    settings: z.union([FormSettings, z.null()]).optional(),
    status: z.union([z.number(), z.null()]).optional(),
    teamId: z.string(),
    themeSettings: z.union([ThemeSettings, z.null()]).optional(),
  })
  .passthrough();
export type Form = z.infer<typeof Form>;
export const FormReportResponse = z
  .object({
    average: z.number(),
    chooses: z.union([z.array(z.unknown()), z.null()]).optional(),
    count: z.number().int().gte(0),
    id: z.string(),
    kind: z.union([z.string(), z.null()]).optional(),
    title: z.union([z.string(), z.null()]).optional(),
    total: z.number().int().gte(0),
  })
  .passthrough();
export type FormReportResponse = z.infer<typeof FormReportResponse>;
export const FormReportAnswer = z
  .object({
    endAt: z.number().int(),
    kind: z.string(),
    submissionId: z.string(),
    value: z.unknown().optional(),
  })
  .passthrough();
export type FormReportAnswer = z.infer<typeof FormReportAnswer>;
export const FormReportSubmission = z
  .object({ _id: z.string(), answers: z.array(FormReportAnswer) })
  .passthrough();
export type FormReportSubmission = z.infer<typeof FormReportSubmission>;
export const FormReport = z
  .object({
    responses: z.array(FormReportResponse),
    submissions: z.array(FormReportSubmission),
  })
  .passthrough();
export type FormReport = z.infer<typeof FormReport>;
export const SubmissionCategory = z.enum([
  "inbox",
  "spam",
  "starred",
  "archive",
]);
export type SubmissionCategory = z.infer<typeof SubmissionCategory>;
export const HiddenFieldAnswer = z
  .object({
    id: z.string(),
    name: z.string(),
    value: z.union([z.string(), z.null()]).optional(),
  })
  .passthrough();
export type HiddenFieldAnswer = z.infer<typeof HiddenFieldAnswer>;
export const Submission = z
  .object({
    answers: z.array(z.object({}).partial().passthrough()),
    category: z.union([SubmissionCategory, z.null()]).optional(),
    endAt: z.number().int(),
    hiddenFields: z.union([z.array(HiddenFieldAnswer), z.null()]).optional(),
    id: z.string(),
    title: z.union([z.string(), z.null()]).optional(),
    variables: z.union([z.array(z.unknown()), z.null()]).optional(),
  })
  .passthrough();
export type Submission = z.infer<typeof Submission>;
export const Submissions = z
  .object({ submissions: z.array(Submission), total: z.number().int().gte(0) })
  .passthrough();
export type Submissions = z.infer<typeof Submissions>;
export const InsightChoice = z
  .object({ count: z.number().int(), id: z.string(), label: z.string() })
  .passthrough();
export type InsightChoice = z.infer<typeof InsightChoice>;
export const InsightSubmission = z
  .object({
    submission_id: z.string(),
    submitted_at: z.union([z.number(), z.null()]).optional(),
    value: z.unknown(),
  })
  .passthrough();
export type InsightSubmission = z.infer<typeof InsightSubmission>;
export const InsightQuestion = z
  .object({
    answered: z.number().int().gte(0),
    choices: z.union([z.array(InsightChoice), z.null()]).optional(),
    id: z.string(),
    kind: z.union([z.string(), z.null()]).optional(),
    properties: z
      .union([z.object({}).partial().passthrough(), z.null()])
      .optional(),
    submissions: z.union([z.array(InsightSubmission), z.null()]).optional(),
    title: z.string(),
    total: z.number().int().gte(0),
  })
  .passthrough();
export type InsightQuestion = z.infer<typeof InsightQuestion>;
export const SurveyInsights = z
  .object({ questions: z.array(InsightQuestion) })
  .passthrough();
export type SurveyInsights = z.infer<typeof SurveyInsights>;
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
    positions: z.union([z.array(z.array(z.number())), z.null()]).optional(),
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
export const Translation2 = z
  .object({
    textContent: TextContentDto,
    textTranslations: z.array(TextTranslationDto),
  })
  .passthrough();
export type Translation2 = z.infer<typeof Translation2>;
export const SectionWithTranslationsDto = z
  .object({
    body: z.string(),
    bodyTranslations: Translation2,
    id: z.string().uuid(),
    position: z.number().int(),
  })
  .passthrough();
export type SectionWithTranslationsDto = z.infer<
  typeof SectionWithTranslationsDto
>;
export const Translation = z
  .object({
    textContent: TextContentDto,
    textTranslations: z.array(TextTranslationDto),
  })
  .passthrough();
export type Translation = z.infer<typeof Translation>;
export const ProposalWithTranslationsDto = z
  .object({
    id: z.string().uuid(),
    sections: z.array(SectionWithTranslationsDto),
    title: z.string(),
    titleTranslations: Translation,
    workflowStepId: z.string().uuid(),
  })
  .passthrough();
export type ProposalWithTranslationsDto = z.infer<
  typeof ProposalWithTranslationsDto
>;
export const LocalizedProposalSectionDto = z
  .object({
    body: z.string(),
    id: z.string().uuid(),
    position: z.number().int(),
  })
  .passthrough();
export type LocalizedProposalSectionDto = z.infer<
  typeof LocalizedProposalSectionDto
>;
export const LocalizedProposalDto = z
  .object({
    id: z.string().uuid(),
    sections: z.array(LocalizedProposalSectionDto),
    title: z.string(),
    workflowStepId: z.string().uuid(),
  })
  .passthrough();
export type LocalizedProposalDto = z.infer<typeof LocalizedProposalDto>;
export const ProposalsListResponse = z.union([
  z.array(ProposalWithTranslationsDto),
  z.array(LocalizedProposalDto),
]);
export type ProposalsListResponse = z.infer<typeof ProposalsListResponse>;
export const CreateProposalRequest = z
  .object({
    sections: z.array(z.string()).optional().default([]),
    title: z.string(),
    workflow_step_id: z.string().uuid(),
  })
  .passthrough();
export type CreateProposalRequest = z.infer<typeof CreateProposalRequest>;
export const ProposalSectionDto = z
  .object({
    body: z.string().uuid(),
    id: z.string().uuid(),
    position: z.number().int(),
  })
  .passthrough();
export type ProposalSectionDto = z.infer<typeof ProposalSectionDto>;
export const ProposalDto = z
  .object({
    id: z.string().uuid(),
    sections: z.array(ProposalSectionDto),
    title: z.string().uuid(),
    workflowStepId: z.string().uuid(),
  })
  .passthrough();
export type ProposalDto = z.infer<typeof ProposalDto>;
export const CreateSectionRequest = z
  .object({
    body: z.string(),
    position: z.union([z.number(), z.null()]).optional().default(null),
  })
  .passthrough();
export type CreateSectionRequest = z.infer<typeof CreateSectionRequest>;
export const ResponseValue = z.union([z.number(), z.string()]);
export type ResponseValue = z.infer<typeof ResponseValue>;
export const Response = z
  .object({
    question_id: z.string().uuid(),
    section_id: z.union([z.string(), z.null()]).optional(),
    value: ResponseValue,
  })
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
export const RankedProposal = z
  .object({
    alignmentRating: z.number(),
    id: z.string().uuid(),
    responses: z.array(ProposalResponseDto),
    sections: z.array(LocalizedProposalSectionDto),
    title: z.string(),
    workflowStepId: z.string().uuid(),
  })
  .passthrough();
export type RankedProposal = z.infer<typeof RankedProposal>;
export const PrioritizationInsightsResponse = z
  .object({ rankedProposals: z.array(RankedProposal) })
  .passthrough();
export type PrioritizationInsightsResponse = z.infer<
  typeof PrioritizationInsightsResponse
>;
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
    aiGeneratedSummary: z.union([z.string(), z.null()]).optional(),
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
export const AnswersByRoot = z
  .object({
    followUps: z.array(ThinkingSpaceAnswerDto),
    root: ThinkingSpaceAnswerDto,
  })
  .passthrough();
export type AnswersByRoot = z.infer<typeof AnswersByRoot>;
export const ThinkingSpaceUserInsights = z
  .object({
    answers: z.array(AnswersByRoot),
    summary: ThinkingSpaceSummaryDto,
    userId: z.string().uuid(),
  })
  .passthrough();
export type ThinkingSpaceUserInsights = z.infer<
  typeof ThinkingSpaceUserInsights
>;
export const ThinkingSpaceInsightsResponse = z
  .object({ users: z.array(ThinkingSpaceUserInsights) })
  .passthrough();
export type ThinkingSpaceInsightsResponse = z.infer<
  typeof ThinkingSpaceInsightsResponse
>;
export const CreateConversation = z
  .object({
    default_workflow_id: z.union([z.string(), z.null()]).optional(),
    description: z.string(),
    enable_qa_chat_bot: z.union([z.boolean(), z.null()]).optional(),
    image: z.union([z.string(), z.null()]).optional(),
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
    allowRevisitAfterFinishing: z.boolean(),
    callToAction: z.union([z.string(), z.null()]).optional(),
    chatBotId: z.union([z.string(), z.null()]).optional(),
    description: z.string().uuid(),
    enableQaChatBot: z.boolean(),
    enableSignupPrompts: z.boolean(),
    faqs: z.union([z.string(), z.null()]).optional(),
    id: z.string().uuid(),
    image: z.union([z.string(), z.null()]).optional(),
    isComplete: z.boolean(),
    isInviteOnly: z.boolean(),
    isLive: z.boolean(),
    isPublic: z.boolean(),
    knowledgeBaseId: z.union([z.string(), z.null()]).optional(),
    metadata: z.unknown(),
    organizationId: z.union([z.string(), z.null()]).optional(),
    primaryLocale: z.string(),
    privacyPolicy: z.union([z.string(), z.null()]).optional(),
    shortDescription: z.string().uuid(),
    shortPrivacyPolicy: z.union([z.string(), z.null()]).optional(),
    showThankYouPageAnnonInstructions: z.boolean(),
    showThankyouPageFeedbackButton: z.boolean(),
    slug: z.union([z.string(), z.null()]).optional(),
    supportedLanguages: z.array(z.string()),
    tags: z.array(z.string()),
    thankYouMessage: z.union([z.string(), z.null()]).optional(),
    title: z.string().uuid(),
    videoUrl: z.union([z.string(), z.null()]).optional(),
  })
  .passthrough();
export type ConversationDto = z.infer<typeof ConversationDto>;
export const Translation3 = z
  .object({
    textContent: TextContentDto,
    textTranslations: z.array(TextTranslationDto),
  })
  .passthrough();
export type Translation3 = z.infer<typeof Translation3>;
export const ConversationTranslations = z
  .object({
    callToAction: z.union([Translation3, z.null()]).optional(),
    description: Translation3,
    faqs: z.union([Translation3, z.null()]).optional(),
    privacyPolicy: z.union([Translation3, z.null()]).optional(),
    shortDescription: Translation3,
    shortPrivacyPolicy: z.union([Translation3, z.null()]).optional(),
    thankYouMessage: z.union([Translation3, z.null()]).optional(),
    title: Translation3,
  })
  .passthrough();
export type ConversationTranslations = z.infer<typeof ConversationTranslations>;
export const ConversationWithTranslations = z
  .object({
    allowRevisitAfterFinishing: z.boolean(),
    callToAction: z.union([z.string(), z.null()]).optional(),
    chatBotId: z.union([z.string(), z.null()]).optional(),
    createdAt: z.string().datetime({ offset: true }),
    defaultWorkflowId: z.union([z.string(), z.null()]).optional(),
    description: z.string(),
    enableQaChatBot: z.boolean(),
    enableSignupPrompts: z.boolean(),
    faqs: z.union([z.string(), z.null()]).optional(),
    id: z.string().uuid(),
    image: z.union([z.string(), z.null()]).optional(),
    isComplete: z.boolean(),
    isInviteOnly: z.boolean(),
    isLive: z.boolean(),
    isPublic: z.boolean(),
    knowledgeBaseId: z.union([z.string(), z.null()]).optional(),
    metadata: z.unknown(),
    organizationId: z.union([z.string(), z.null()]).optional(),
    ownerId: z.string().uuid(),
    primaryLocale: z.string(),
    privacyPolicy: z.union([z.string(), z.null()]).optional(),
    shortDescription: z.string(),
    shortPrivacyPolicy: z.union([z.string(), z.null()]).optional(),
    showThankYouPageAnnonInstructions: z.boolean(),
    showThankyouPageFeedbackButton: z.boolean(),
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
    allow_revisit_after_finishing: z.union([z.boolean(), z.null()]),
    call_to_action: z.union([z.string(), z.null()]),
    chat_bot_id: z.union([z.string(), z.null()]),
    default_workflow_id: z.union([z.string(), z.null()]),
    description: z.union([z.string(), z.null()]),
    enable_qa_chat_bot: z.union([z.boolean(), z.null()]),
    enable_signup_prompts: z.union([z.boolean(), z.null()]),
    faqs: z.union([z.string(), z.null()]),
    image: z.union([z.string(), z.null()]),
    is_complete: z.union([z.boolean(), z.null()]),
    is_invite_only: z.union([z.boolean(), z.null()]),
    is_live: z.union([z.boolean(), z.null()]),
    is_public: z.union([z.boolean(), z.null()]),
    knowledge_base_id: z.union([z.string(), z.null()]),
    metadata: z.unknown(),
    organization_id: z.union([z.string(), z.null()]),
    primary_locale: z.union([z.string(), z.null()]),
    privacy_policy: z.union([z.string(), z.null()]),
    short_description: z.union([z.string(), z.null()]),
    short_privacy_policy: z.union([z.string(), z.null()]),
    show_thank_you_page_annon_instructions: z.union([z.boolean(), z.null()]),
    show_thankyou_page_feedback_button: z.union([z.boolean(), z.null()]),
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
export const OrganizationWithPermissionDto = z
  .object({ id: z.string().uuid(), name: z.string(), roleName: z.string() })
  .passthrough();
export type OrganizationWithPermissionDto = z.infer<
  typeof OrganizationWithPermissionDto
>;
export const CohostInfo = z
  .object({ organization_id: z.string().uuid() })
  .passthrough();
export type CohostInfo = z.infer<typeof CohostInfo>;
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
  .object({ label: z.string().uuid(), value: z.number() })
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
        max_label: z.string().uuid(),
        max_value: z.number().optional().default(10),
        min_label: z.string().uuid(),
        min_value: z.number().optional().default(0),
        sub_steps: z.number().int().optional().default(10),
      })
      .passthrough(),
  }),
]);
export type QuestionType = z.infer<typeof QuestionType>;
export const Question = z
  .object({
    id: z.string().uuid(),
    text: z.string().uuid(),
    type: QuestionType,
  })
  .passthrough();
export type Question = z.infer<typeof Question>;
export const ThinkingSpaceQuestion = z
  .object({
    id: z.string().uuid(),
    intent: z.string().uuid(),
    text: z.string().uuid(),
  })
  .passthrough();
export type ThinkingSpaceQuestion = z.infer<typeof ThinkingSpaceQuestion>;
export const ToolConfig = z.union([
  z
    .object({
      admin_password: z.string(),
      admin_user: z.string(),
      description: z.union([z.string(), z.null()]).optional().default(null),
      is_active: z.union([z.boolean(), z.null()]).optional().default(null),
      label_seeds_as_conversation_starter: z
        .boolean()
        .optional()
        .default(false),
      poll_id: z.string(),
      required_votes: z.union([z.number(), z.null()]).optional(),
      server_url: z.string(),
      show_remaining_statements: z.boolean().optional().default(true),
      strict_moderation: z
        .union([z.boolean(), z.null()])
        .optional()
        .default(null),
      topic: z.union([z.string(), z.null()]).optional().default(null),
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
      alignment_question_id: z.union([z.string(), z.null()]).optional(),
      questions: z.array(Question),
      randomize_order: z.boolean(),
      required_reviews: z
        .union([z.number(), z.null()])
        .optional()
        .default(null),
      section_questions: z.array(Question).optional().default([]),
      type: z.literal("prioritization"),
    })
    .passthrough(),
  z
    .object({
      follow_up_rounds_count: z.number().int().gte(0),
      root_questions: z.array(ThinkingSpaceQuestion),
      topic: z.string().uuid(),
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
export const UserParticipationDto = z
  .object({
    created_at: z.string().datetime({ offset: true }),
    id: z.string().uuid(),
    sealed: z.boolean(),
    updated_at: z.string().datetime({ offset: true }),
    user_id: z.string().uuid(),
    workflow_id: z.string().uuid(),
  })
  .passthrough();
export type UserParticipationDto = z.infer<typeof UserParticipationDto>;
export const TranslationDto = z
  .object({
    textContent: TextContentDto,
    textTranslations: z.array(TextTranslationDto),
  })
  .passthrough();
export type TranslationDto = z.infer<typeof TranslationDto>;
export const JsonFieldWithTranslations = z
  .object({ localized: z.string(), translations: TranslationDto })
  .passthrough();
export type JsonFieldWithTranslations = z.infer<
  typeof JsonFieldWithTranslations
>;
export const CategoryWithTranslations = z
  .object({ label: JsonFieldWithTranslations, value: z.number() })
  .passthrough();
export type CategoryWithTranslations = z.infer<typeof CategoryWithTranslations>;
export const QuestionTypeWithTranslations = z.union([
  z.literal("text"),
  z.object({
    likert_scale: z
      .object({ categories: z.array(CategoryWithTranslations) })
      .passthrough(),
  }),
  z.object({
    continuous: z
      .object({
        max_label: JsonFieldWithTranslations,
        max_value: z.number(),
        min_label: JsonFieldWithTranslations,
        min_value: z.number(),
        sub_steps: z.number().int(),
      })
      .passthrough(),
  }),
]);
export type QuestionTypeWithTranslations = z.infer<
  typeof QuestionTypeWithTranslations
>;
export const QuestionWithTranslations = z
  .object({
    id: z.string().uuid(),
    text: JsonFieldWithTranslations,
    type: QuestionTypeWithTranslations,
  })
  .passthrough();
export type QuestionWithTranslations = z.infer<typeof QuestionWithTranslations>;
export const ThinkingSpaceQuestionWithTranslations = z
  .object({
    id: z.string().uuid(),
    intent: JsonFieldWithTranslations,
    text: JsonFieldWithTranslations,
  })
  .passthrough();
export type ThinkingSpaceQuestionWithTranslations = z.infer<
  typeof ThinkingSpaceQuestionWithTranslations
>;
export const ToolConfigWithTranslations = z.union([
  z
    .object({
      admin_password: z.string(),
      admin_user: z.string(),
      description: z.union([z.string(), z.null()]).optional().default(null),
      is_active: z.union([z.boolean(), z.null()]).optional().default(null),
      label_seeds_as_conversation_starter: z
        .boolean()
        .optional()
        .default(false),
      poll_id: z.string(),
      required_votes: z.union([z.number(), z.null()]).optional(),
      server_url: z.string(),
      show_remaining_statements: z.boolean().optional().default(true),
      strict_moderation: z
        .union([z.boolean(), z.null()])
        .optional()
        .default(null),
      topic: z.union([z.string(), z.null()]).optional().default(null),
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
      alignment_question_id: z.union([z.string(), z.null()]).optional(),
      questions: z.array(QuestionWithTranslations),
      randomize_order: z.boolean(),
      required_reviews: z.union([z.number(), z.null()]).optional(),
      section_questions: z.array(QuestionWithTranslations),
      type: z.literal("prioritization"),
    })
    .passthrough(),
  z
    .object({
      follow_up_rounds_count: z.number().int().gte(0),
      root_questions: z.array(ThinkingSpaceQuestionWithTranslations),
      topic: JsonFieldWithTranslations,
      type: z.literal("thinkingspace"),
    })
    .passthrough(),
]);
export type ToolConfigWithTranslations = z.infer<
  typeof ToolConfigWithTranslations
>;
export const Translation4 = z
  .object({
    textContent: TextContentDto,
    textTranslations: z.array(TextTranslationDto),
  })
  .passthrough();
export type Translation4 = z.infer<typeof Translation4>;
export const WorkflowStepTranslations = z
  .object({ description: Translation4, name: Translation4 })
  .passthrough();
export type WorkflowStepTranslations = z.infer<typeof WorkflowStepTranslations>;
export const WorkflowStepWithTranslationsDto = z
  .object({
    activationRule: ActivationRule,
    canRevisit: z.boolean(),
    description: z.string(),
    id: z.string().uuid(),
    isOffline: z.boolean(),
    name: z.string(),
    previewToolConfig: ToolConfigWithTranslations,
    requestUserSharePermission: z.boolean(),
    required: z.boolean(),
    stepOrder: z.number().int(),
    toolConfig: z.union([ToolConfigWithTranslations, z.null()]).optional(),
    translations: WorkflowStepTranslations,
    workflowId: z.string().uuid(),
  })
  .passthrough();
export type WorkflowStepWithTranslationsDto = z.infer<
  typeof WorkflowStepWithTranslationsDto
>;
export const LocalizedCategory = z
  .object({ label: z.string(), value: z.number() })
  .passthrough();
export type LocalizedCategory = z.infer<typeof LocalizedCategory>;
export const LocalizedQuestionType = z.union([
  z.literal("text"),
  z.object({
    likert_scale: z
      .object({ categories: z.array(LocalizedCategory) })
      .passthrough(),
  }),
  z.object({
    continuous: z
      .object({
        max_label: z.string(),
        max_value: z.number(),
        min_label: z.string(),
        min_value: z.number(),
        sub_steps: z.number().int(),
      })
      .passthrough(),
  }),
]);
export type LocalizedQuestionType = z.infer<typeof LocalizedQuestionType>;
export const LocalizedQuestion = z
  .object({
    id: z.string().uuid(),
    text: z.string(),
    type: LocalizedQuestionType,
  })
  .passthrough();
export type LocalizedQuestion = z.infer<typeof LocalizedQuestion>;
export const LocalizedThinkingSpaceQuestion = z
  .object({ id: z.string().uuid(), intent: z.string(), text: z.string() })
  .passthrough();
export type LocalizedThinkingSpaceQuestion = z.infer<
  typeof LocalizedThinkingSpaceQuestion
>;
export const LocalizedToolConfig = z.union([
  z
    .object({
      admin_password: z.string(),
      admin_user: z.string(),
      description: z.union([z.string(), z.null()]).optional().default(null),
      is_active: z.union([z.boolean(), z.null()]).optional().default(null),
      label_seeds_as_conversation_starter: z
        .boolean()
        .optional()
        .default(false),
      poll_id: z.string(),
      required_votes: z.union([z.number(), z.null()]).optional(),
      server_url: z.string(),
      show_remaining_statements: z.boolean().optional().default(true),
      strict_moderation: z
        .union([z.boolean(), z.null()])
        .optional()
        .default(null),
      topic: z.union([z.string(), z.null()]).optional().default(null),
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
      alignment_question_id: z.union([z.string(), z.null()]).optional(),
      questions: z.array(LocalizedQuestion),
      randomize_order: z.boolean(),
      required_reviews: z.union([z.number(), z.null()]).optional(),
      section_questions: z.array(LocalizedQuestion),
      type: z.literal("prioritization"),
    })
    .passthrough(),
  z
    .object({
      follow_up_rounds_count: z.number().int().gte(0),
      root_questions: z.array(LocalizedThinkingSpaceQuestion),
      topic: z.string(),
      type: z.literal("thinkingspace"),
    })
    .passthrough(),
]);
export type LocalizedToolConfig = z.infer<typeof LocalizedToolConfig>;
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
    previewToolConfig: LocalizedToolConfig,
    progressStatus: ProgressStatus,
    requestUserSharePermission: z.boolean(),
    required: z.boolean(),
    stepOrder: z.number().int(),
    toolConfig: z.union([LocalizedToolConfig, z.null()]).optional(),
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
    previewToolConfig: LocalizedToolConfig,
    requestUserSharePermission: z.boolean(),
    required: z.boolean(),
    stepOrder: z.number().int(),
    toolConfig: z.union([LocalizedToolConfig, z.null()]).optional(),
    workflowId: z.string().uuid(),
  })
  .passthrough();
export type LocalizedWorkflowStepDto = z.infer<typeof LocalizedWorkflowStepDto>;
export const WorkflowStepsListResponse = z.union([
  z.array(WorkflowStepWithTranslationsDto),
  z.array(LocalizedWorkflowStepWithProgressDto),
  z.array(LocalizedWorkflowStepDto),
]);
export type WorkflowStepsListResponse = z.infer<
  typeof WorkflowStepsListResponse
>;
export const SetupCategory = z
  .object({ label: z.string(), value: z.number() })
  .passthrough();
export type SetupCategory = z.infer<typeof SetupCategory>;
export const SetupQuestionType = z.union([
  z.literal("text"),
  z.object({
    likert_scale: z
      .object({ categories: z.array(SetupCategory) })
      .passthrough(),
  }),
  z.object({
    continuous: z
      .object({
        max_label: z.string(),
        max_value: z.number(),
        min_label: z.string(),
        min_value: z.number(),
        sub_steps: z.number().int(),
      })
      .passthrough(),
  }),
]);
export type SetupQuestionType = z.infer<typeof SetupQuestionType>;
export const SetupQuestion = z
  .object({ text: z.string(), type: SetupQuestionType })
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
export const RecruitmentTargetDto = z
  .object({
    bucket: z.string(),
    createdAt: z.string().datetime({ offset: true }),
    id: z.string().uuid(),
    metric: z.string(),
    targetCount: z.number().int(),
    updatedAt: z.string().datetime({ offset: true }),
    workflowId: z.string().uuid(),
  })
  .passthrough();
export type RecruitmentTargetDto = z.infer<typeof RecruitmentTargetDto>;
export const CreateRecruitmentTarget = z
  .object({
    bucket: z.string(),
    metric: z.string(),
    target_count: z.number().int(),
  })
  .passthrough();
export type CreateRecruitmentTarget = z.infer<typeof CreateRecruitmentTarget>;
export const PartialRecruitmentTarget = z
  .object({
    bucket: z.union([z.string(), z.null()]),
    metric: z.union([z.string(), z.null()]),
    target_count: z.union([z.number(), z.null()]),
  })
  .partial()
  .passthrough();
export type PartialRecruitmentTarget = z.infer<typeof PartialRecruitmentTarget>;
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
export const Translation5 = z
  .object({
    textContent: TextContentDto,
    textTranslations: z.array(TextTranslationDto),
  })
  .passthrough();
export type Translation5 = z.infer<typeof Translation5>;
export const ReportTranslations = z
  .object({ summary: Translation5 })
  .passthrough();
export type ReportTranslations = z.infer<typeof ReportTranslations>;
export const ReportWithTranslations = z
  .object({
    conversationId: z.string().uuid(),
    createdAt: z.string().datetime({ offset: true }),
    id: z.string().uuid(),
    isPublic: z.boolean(),
    sectionConfigs: ReportSectionConfigs,
    summary: z.string(),
    translations: ReportTranslations,
    updatedAt: z.string().datetime({ offset: true }),
  })
  .passthrough();
export type ReportWithTranslations = z.infer<typeof ReportWithTranslations>;
export const LocalizedReportDto = z
  .object({
    conversationId: z.string().uuid(),
    createdAt: z.string().datetime({ offset: true }),
    id: z.string().uuid(),
    isPublic: z.boolean(),
    sectionConfigs: ReportSectionConfigs,
    summary: z.string(),
  })
  .passthrough();
export type LocalizedReportDto = z.infer<typeof LocalizedReportDto>;
export const FullReportDto = z.union([
  ReportWithTranslations,
  LocalizedReportDto,
]);
export type FullReportDto = z.infer<typeof FullReportDto>;
export const PartialReport = z
  .object({
    conversation_id: z.union([z.string(), z.null()]),
    is_public: z.union([z.boolean(), z.null()]),
    section_configs: z.union([ReportSectionConfigs, z.null()]),
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
    summary: z.string().uuid(),
  })
  .passthrough();
export type ReportDto = z.infer<typeof ReportDto>;
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
export const FeedbackDto = z
  .object({
    content: z.string(),
    conversationId: z.string().uuid(),
    id: z.string().uuid(),
  })
  .passthrough();
export type FeedbackDto = z.infer<typeof FeedbackDto>;
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
export const SyncLearningContentResponse = z
  .object({
    document: z.union([ComhairleDocument, z.null()]).optional(),
    job_id: z.union([z.string(), z.null()]).optional(),
    message: z.string(),
  })
  .passthrough();
export type SyncLearningContentResponse = z.infer<
  typeof SyncLearningContentResponse
>;
export const LearnContentPage = z
  .object({ content: z.string(), is_rich: z.boolean() })
  .passthrough();
export type LearnContentPage = z.infer<typeof LearnContentPage>;
export const LearnContentSection = z
  .object({ heading: z.string(), pages: z.array(LearnContentPage) })
  .passthrough();
export type LearnContentSection = z.infer<typeof LearnContentSection>;
export const LearnContentResponse = z
  .object({ sections: z.array(LearnContentSection) })
  .passthrough();
export type LearnContentResponse = z.infer<typeof LearnContentResponse>;
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
    metadata: z.unknown().optional(),
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
    metadata: z.unknown().optional(),
    name: z.string().uuid(),
    signupMode: z.string(),
    startTime: z.string().datetime({ offset: true }),
    videoMeetingId: z.union([z.string(), z.null()]).optional(),
  })
  .passthrough();
export type EventDto = z.infer<typeof EventDto>;
export const BreakoutSeat = z
  .object({
    invite_id: z.union([z.string(), z.null()]),
    is_moderator: z.boolean().default(false),
    user_id: z.union([z.string(), z.null()]),
  })
  .partial()
  .passthrough();
export type BreakoutSeat = z.infer<typeof BreakoutSeat>;
export const BreakoutPlanRoom = z
  .object({ seats: z.array(BreakoutSeat).default([]) })
  .partial()
  .passthrough();
export type BreakoutPlanRoom = z.infer<typeof BreakoutPlanRoom>;
export const Translation6 = z
  .object({
    textContent: TextContentDto,
    textTranslations: z.array(TextTranslationDto),
  })
  .passthrough();
export type Translation6 = z.infer<typeof Translation6>;
export const EventTranslations = z
  .object({ description: Translation6, name: Translation6 })
  .passthrough();
export type EventTranslations = z.infer<typeof EventTranslations>;
export const EventWithTranslations = z
  .object({
    agenda: z.array(EventAgendaItem),
    breakoutPlan: z.array(BreakoutPlanRoom),
    capacity: z.union([z.number(), z.null()]).optional(),
    conversationId: z.string().uuid(),
    createdAt: z.string().datetime({ offset: true }),
    defaultTimeZone: z.string(),
    description: z.string(),
    endTime: z.string().datetime({ offset: true }),
    format: EventFormat,
    id: z.string().uuid(),
    location: z.union([EventLocation, z.null()]).optional(),
    metadata: z.unknown().optional(),
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
    metadata: z.unknown(),
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
export const BreakoutSeatDto = z
  .object({
    inviteId: z.union([z.string(), z.null()]).optional(),
    isModerator: z.boolean(),
    label: z.string(),
    pending: z.boolean(),
    userId: z.union([z.string(), z.null()]).optional(),
  })
  .passthrough();
export type BreakoutSeatDto = z.infer<typeof BreakoutSeatDto>;
export const BreakoutRoomDto = z
  .object({ seats: z.array(BreakoutSeatDto) })
  .passthrough();
export type BreakoutRoomDto = z.infer<typeof BreakoutRoomDto>;
export const BreakoutPlanDto = z
  .object({ rooms: z.array(BreakoutRoomDto) })
  .passthrough();
export type BreakoutPlanDto = z.infer<typeof BreakoutPlanDto>;
export const SaveBreakoutPlanRequest = z
  .object({ rooms: z.array(BreakoutPlanRoom) })
  .passthrough();
export type SaveBreakoutPlanRequest = z.infer<typeof SaveBreakoutPlanRequest>;
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
export const AudioFormat = z.enum([
  "wav",
  "mp3",
  "m4a",
  "mp4",
  "ogg",
  "flac",
  "webm",
]);
export type AudioFormat = z.infer<typeof AudioFormat>;
export const AudioRecordingStatus = z.union([
  z.literal("awaiting_upload"),
  z.literal("transcribing"),
  z.literal("categorizing"),
  z.literal("complete"),
  z.literal("transcription_failed"),
  z.literal("categorization_failed"),
]);
export type AudioRecordingStatus = z.infer<typeof AudioRecordingStatus>;
export const AudioRecordingDto = z
  .object({
    createdAt: z.string().datetime({ offset: true }),
    eventId: z.string().uuid(),
    fileExtension: AudioFormat,
    id: z.string().uuid(),
    name: z.string(),
    s3KeyPrefix: z.string(),
    status: AudioRecordingStatus,
    updatedAt: z.string().datetime({ offset: true }),
  })
  .passthrough();
export type AudioRecordingDto = z.infer<typeof AudioRecordingDto>;
export const CreateRecordingRequest = z
  .object({ fileExtension: AudioFormat, name: z.string() })
  .passthrough();
export type CreateRecordingRequest = z.infer<typeof CreateRecordingRequest>;
export const CreateRecordingResponse = z
  .object({ recording: AudioRecordingDto, uploadUrl: z.string() })
  .passthrough();
export type CreateRecordingResponse = z.infer<typeof CreateRecordingResponse>;
export const RecordingDownloadUrls = z
  .object({
    recordingUrl: z.string(),
    reportUrl: z.string(),
    transcriptUrl: z.string(),
  })
  .passthrough();
export type RecordingDownloadUrls = z.infer<typeof RecordingDownloadUrls>;
export const RecordingDetailResponse = z
  .object({ downloads: RecordingDownloadUrls, recording: AudioRecordingDto })
  .passthrough();
export type RecordingDetailResponse = z.infer<typeof RecordingDetailResponse>;
export const DeleteRecordingResponse = z
  .object({ recording: AudioRecordingDto })
  .passthrough();
export type DeleteRecordingResponse = z.infer<typeof DeleteRecordingResponse>;
export const ProcessRecordingResponse = z
  .object({ jobId: z.string().uuid(), message: z.string() })
  .passthrough();
export type ProcessRecordingResponse = z.infer<typeof ProcessRecordingResponse>;
export const SubmitReportResponse = z
  .object({ success: z.boolean(), url: z.string() })
  .passthrough();
export type SubmitReportResponse = z.infer<typeof SubmitReportResponse>;
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
    contact_email: z.union([z.string(), z.null()]).optional(),
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
    contactEmail: z.union([z.string(), z.null()]).optional(),
    createdAt: z.string().datetime({ offset: true }),
    description: z.string().uuid(),
    externalUrl: z.union([z.string(), z.null()]).optional(),
    id: z.string().uuid(),
    metadata: z.unknown().optional(),
    mission: z.string().uuid(),
    name: z.string(),
    orgType: OrganizationType,
    regions: z.array(z.string().uuid()),
  })
  .passthrough();
export type OrganizationDto = z.infer<typeof OrganizationDto>;
export const UpdateOrganizationBody = z
  .object({
    contact_email: z.union([z.string(), z.null()]),
    description: z.union([z.string(), z.null()]),
    external_url: z.union([z.string(), z.null()]),
    metadata: z.unknown(),
    mission: z.union([z.string(), z.null()]),
    name: z.union([z.string(), z.null()]),
    org_type: z.union([OrganizationType, z.null()]),
    regions: z.union([z.array(z.string().uuid()), z.null()]),
  })
  .partial()
  .passthrough();
export type UpdateOrganizationBody = z.infer<typeof UpdateOrganizationBody>;
export const OrganizationTeamRole = z.enum(["member", "admin"]);
export type OrganizationTeamRole = z.infer<typeof OrganizationTeamRole>;
export const OrganizationTeamUserDto = z
  .object({
    email: z.union([z.string(), z.null()]).optional(),
    id: z.string().uuid(),
    role: OrganizationTeamRole,
    username: z.union([z.string(), z.null()]).optional(),
  })
  .passthrough();
export type OrganizationTeamUserDto = z.infer<typeof OrganizationTeamUserDto>;
export const OrganizationTeamResponseDto = z
  .object({ members: z.array(OrganizationTeamUserDto) })
  .passthrough();
export type OrganizationTeamResponseDto = z.infer<
  typeof OrganizationTeamResponseDto
>;
export const UpsertOrganizationUserBody = z
  .object({
    allow_create_user: z.union([z.boolean(), z.null()]).optional(),
    email: z.string(),
    role: z.union([OrganizationTeamRole, z.null()]).optional(),
  })
  .passthrough();
export type UpsertOrganizationUserBody = z.infer<
  typeof UpsertOrganizationUserBody
>;
export const UpsertOrganizationUserResponseDto = z
  .object({
    createdAccount: z.boolean(),
    emailed: z.boolean(),
    user: OrganizationTeamUserDto,
  })
  .passthrough();
export type UpsertOrganizationUserResponseDto = z.infer<
  typeof UpsertOrganizationUserResponseDto
>;
export const UpdateOrganizationMemberRoleBody = z
  .object({ role: OrganizationTeamRole })
  .passthrough();
export type UpdateOrganizationMemberRoleBody = z.infer<
  typeof UpdateOrganizationMemberRoleBody
>;
export const RegionType = z.enum(["custom", "official"]);
export type RegionType = z.infer<typeof RegionType>;
export const LocalizedRegionDto = z
  .object({
    created_at: z.string().datetime({ offset: true }),
    description: z.string(),
    id: z.string().uuid(),
    metadata: z.unknown().optional(),
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
    metadata: z.unknown().optional(),
    name: z.string().uuid(),
    official_id: z.union([z.string(), z.null()]).optional(),
    region_type: RegionType,
  })
  .passthrough();
export type RegionDto = z.infer<typeof RegionDto>;
export const PartialRegion = z
  .object({
    metadata: z.unknown(),
    official_id: z.union([z.string(), z.null()]),
    region_type: z.union([RegionType, z.null()]),
  })
  .partial()
  .passthrough();
export type PartialRegion = z.infer<typeof PartialRegion>;
export const RegionAreaLinksDto = z
  .object({
    area_ids: z.array(z.string().uuid()),
    region_id: z.string().uuid(),
  })
  .passthrough();
export type RegionAreaLinksDto = z.infer<typeof RegionAreaLinksDto>;
export const RegionAreaLinksRequestDto = z
  .object({ area_ids: z.array(z.string().uuid()) })
  .passthrough();
export type RegionAreaLinksRequestDto = z.infer<
  typeof RegionAreaLinksRequestDto
>;
export const RegionAreaDto = z
  .object({
    createdAt: z.string().datetime({ offset: true }),
    id: z.string().uuid(),
    zipPrefix: z.string(),
  })
  .passthrough();
export type RegionAreaDto = z.infer<typeof RegionAreaDto>;
export const CreateRegionArea = z
  .object({ zip_prefix: z.string() })
  .passthrough();
export type CreateRegionArea = z.infer<typeof CreateRegionArea>;
export const PartialRegionArea = z
  .object({ zip_prefix: z.union([z.string(), z.null()]) })
  .partial()
  .passthrough();
export type PartialRegionArea = z.infer<typeof PartialRegionArea>;
export const MediaContentType = z.enum([
  "image/jpeg",
  "image/png",
  "image/gif",
  "image/webp",
  "video/mp4",
  "video/mpeg",
  "video/webm",
  "audio/mpeg",
]);
export type MediaContentType = z.infer<typeof MediaContentType>;
export const content_type = z.union([MediaContentType, z.null()]).optional();
export type content_type = z.infer<typeof content_type>;
export const MediaDto = z
  .object({
    alt: z.string(),
    contentType: MediaContentType,
    createdAt: z.string().datetime({ offset: true }),
    filename: z.string(),
    id: z.string().uuid(),
    name: z.string(),
    ownerId: z.string().uuid(),
    storageKey: z.string(),
    storeName: z.string(),
    url: z.string(),
  })
  .passthrough();
export type MediaDto = z.infer<typeof MediaDto>;
export const PaginatedResults_for_MediaDto = z
  .object({ records: z.array(MediaDto), total: z.number().int() })
  .passthrough();
export type PaginatedResults_for_MediaDto = z.infer<
  typeof PaginatedResults_for_MediaDto
>;
export const MediaEditableFields = z
  .object({
    alt: z.union([z.string(), z.null()]),
    name: z.union([z.string(), z.null()]),
  })
  .partial()
  .passthrough();
export type MediaEditableFields = z.infer<typeof MediaEditableFields>;
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
export const EmailType = z.enum([
  "conversation_invite",
  "event_registration_invite",
  "event_registration_confirmation",
  "event_reminder",
]);
export type EmailType = z.infer<typeof EmailType>;
export const email_type = z.union([EmailType, z.null()]).optional();
export type email_type = z.infer<typeof email_type>;
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
      type: z.literal("event_registration_invite"),
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
  z
    .object({
      body: z.string(),
      footer: z.string(),
      heading: z.string(),
      intro: z.string(),
      type: z.literal("event_reminder"),
    })
    .passthrough(),
]);
export type EmailTemplateSlots = z.infer<typeof EmailTemplateSlots>;
export const EmailTemplateConfigDto = z
  .object({
    createdAt: z.string().datetime({ offset: true }),
    emailType: EmailType,
    id: z.string().uuid(),
    organizationId: z.union([z.string(), z.null()]).optional(),
    ownerId: z.string().uuid(),
    slots: EmailTemplateSlots,
    subject: z.union([z.string(), z.null()]).optional(),
  })
  .passthrough();
export type EmailTemplateConfigDto = z.infer<typeof EmailTemplateConfigDto>;
export const CreateEmailTemplateConfig = z
  .object({
    slots: EmailTemplateSlots,
    subject: z.union([z.string(), z.null()]).optional(),
  })
  .passthrough();
export type CreateEmailTemplateConfig = z.infer<
  typeof CreateEmailTemplateConfig
>;
export const UpdateEmailTemplateConfig = z
  .object({
    slots: z.union([EmailTemplateSlots, z.null()]),
    subject: z.union([z.string(), z.null()]),
  })
  .partial()
  .passthrough();
export type UpdateEmailTemplateConfig = z.infer<
  typeof UpdateEmailTemplateConfig
>;
export const ContentType = z.enum(["plain_text", "rich_text"]);
export type ContentType = z.infer<typeof ContentType>;
export const SlotSchemaDefinition = z
  .object({
    content_type: ContentType,
    default_content: z.string(),
    hint: z.string(),
    key: z.string(),
    label: z.string(),
  })
  .passthrough();
export type SlotSchemaDefinition = z.infer<typeof SlotSchemaDefinition>;
export const EmailTypeSchema = z
  .object({
    default_subject: z.string(),
    email_type: EmailType,
    slots: z.array(SlotSchemaDefinition),
    template: z.string(),
    variables: z.array(z.string()),
  })
  .passthrough();
export type EmailTypeSchema = z.infer<typeof EmailTypeSchema>;
export const PreviewEmailTemplateConfigRequest = z
  .object({ slots: EmailTemplateSlots })
  .passthrough();
export type PreviewEmailTemplateConfigRequest = z.infer<
  typeof PreviewEmailTemplateConfigRequest
>;
export const PreviewEmailTemplateConfigResponse = z
  .object({ html: z.string() })
  .passthrough();
export type PreviewEmailTemplateConfigResponse = z.infer<
  typeof PreviewEmailTemplateConfigResponse
>;
export const ResourcePermission = z
  .object({
    grant_reason: z.string(),
    granted_at: z.string().datetime({ offset: true }),
    granted_by: z.union([z.string(), z.null()]).optional(),
    id: z.string().uuid(),
    organization_id: z.union([z.string(), z.null()]).optional(),
    resource_id: z.string().uuid(),
    resource_type: z.string(),
    role_name: z.string(),
    user_id: z.union([z.string(), z.null()]).optional(),
  })
  .passthrough();
export type ResourcePermission = z.infer<typeof ResourcePermission>;
export const PaginatedResults_for_ResourcePermission = z
  .object({ records: z.array(ResourcePermission), total: z.number().int() })
  .passthrough();
export type PaginatedResults_for_ResourcePermission = z.infer<
  typeof PaginatedResults_for_ResourcePermission
>;
export const GrantPermissionBody = z
  .object({
    grant_reason: z.string(),
    organization_id: z.union([z.string(), z.null()]).optional(),
    role_name: z.string(),
    user_email: z.union([z.string(), z.null()]).optional(),
    user_id: z.union([z.string(), z.null()]).optional(),
  })
  .passthrough();
export type GrantPermissionBody = z.infer<typeof GrantPermissionBody>;
export const UserWithPermissionDto = z
  .object({
    email: z.union([z.string(), z.null()]).optional(),
    id: z.string().uuid(),
    roleName: z.string(),
    username: z.union([z.string(), z.null()]).optional(),
  })
  .passthrough();
export type UserWithPermissionDto = z.infer<typeof UserWithPermissionDto>;

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
  OrganizationType,
  LocalizedOrganizationDto,
  UserOrganizationAccess,
  UserOrganizationsResponse,
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
  GroupVoteCounts,
  VoteCounts,
  CommentReportData,
  RepresentativeComment,
  GroupReportData,
  PcaPosition,
  ParticipantReportData,
  WikiPollReport,
  UpdatePolisConfigRequest,
  WikiPoll,
  PostSeedRequest,
  PostSeedResponse,
  ModerationStatus,
  PolisStatementAux,
  CreatePolisStatementAux,
  UpdatePolisStatementAux,
  SyncStatementAuxRequest,
  SyncStatementAuxResponse,
  ThemeStatistic,
  ThemeRequest,
  ModerationDecisionRequest,
  ModerateStatementAuxRequest,
  ModerateStatementAuxBatchRequest,
  ModerateBatchFailure,
  ModerateStatementAuxBatchResponse,
  SplitStatementRequest,
  SplitStatementResponse,
  FormField,
  FormSettings,
  FormTheme,
  ThemeSettings,
  Form,
  FormReportResponse,
  FormReportAnswer,
  FormReportSubmission,
  FormReport,
  SubmissionCategory,
  HiddenFieldAnswer,
  Submission,
  Submissions,
  InsightChoice,
  InsightSubmission,
  InsightQuestion,
  SurveyInsights,
  Story,
  ComhairleMessageReference,
  ComhairleSessionMessage,
  ComhairleAgentSession,
  ConversationRequest,
  Translation2,
  SectionWithTranslationsDto,
  Translation,
  ProposalWithTranslationsDto,
  LocalizedProposalSectionDto,
  LocalizedProposalDto,
  ProposalsListResponse,
  CreateProposalRequest,
  ProposalSectionDto,
  ProposalDto,
  CreateSectionRequest,
  ResponseValue,
  Response,
  QuestionResponses,
  ProposalResponseDto,
  CreateResponse,
  RankedProposal,
  PrioritizationInsightsResponse,
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
  AnswersByRoot,
  ThinkingSpaceUserInsights,
  ThinkingSpaceInsightsResponse,
  CreateConversation,
  ConversationDto,
  Translation3,
  ConversationTranslations,
  ConversationWithTranslations,
  ConversationResponse,
  PartialConversation,
  OrganizationWithPermissionDto,
  CohostInfo,
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
  UserParticipationDto,
  TranslationDto,
  JsonFieldWithTranslations,
  CategoryWithTranslations,
  QuestionTypeWithTranslations,
  QuestionWithTranslations,
  ThinkingSpaceQuestionWithTranslations,
  ToolConfigWithTranslations,
  Translation4,
  WorkflowStepTranslations,
  WorkflowStepWithTranslationsDto,
  LocalizedCategory,
  LocalizedQuestionType,
  LocalizedQuestion,
  LocalizedThinkingSpaceQuestion,
  LocalizedToolConfig,
  ProgressStatus,
  LocalizedWorkflowStepWithProgressDto,
  LocalizedWorkflowStepDto,
  WorkflowStepsListResponse,
  SetupCategory,
  SetupQuestionType,
  SetupQuestion,
  ThinkingSpaceSetupQuestion,
  ToolSetup,
  CreateWorkflowStep,
  WorkflowStepDto,
  PartialWorkflowStep,
  UserProgressDto,
  UpdateUserProgress,
  RecruitmentTargetDto,
  CreateRecruitmentTarget,
  PartialRecruitmentTarget,
  InviteType,
  LoginBehaviour,
  InviteStatus,
  InviteDto,
  CreateInviteDTO,
  PartialInvite,
  DailyResponseStats,
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
  Translation5,
  ReportTranslations,
  ReportWithTranslations,
  LocalizedReportDto,
  FullReportDto,
  PartialReport,
  ReportDto,
  ReportImpactDto,
  PartialReportImpact,
  CreateImpactDTO,
  FeedbackDto,
  CreateFeedbackDTO,
  PartialFeedback,
  ComhairleChatSession,
  ChatConversationRequest,
  page_size,
  ComhairleDocument,
  UploadFileResponse,
  SyncLearningContentResponse,
  LearnContentPage,
  LearnContentSection,
  LearnContentResponse,
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
  BreakoutSeat,
  BreakoutPlanRoom,
  Translation6,
  EventTranslations,
  EventWithTranslations,
  EventResponse,
  PartialEvent,
  JwtResponse,
  BreakoutSeatDto,
  BreakoutRoomDto,
  BreakoutPlanDto,
  SaveBreakoutPlanRequest,
  EventAttendanceEtx,
  PaginatedResults_for_EventAttendanceEtx,
  CreateEventAttendanceRequest,
  EventAttendanceDto,
  UpdateEventAttendanceRequest,
  CreateFacilitatorRequest,
  AudioFormat,
  AudioRecordingStatus,
  AudioRecordingDto,
  CreateRecordingRequest,
  CreateRecordingResponse,
  RecordingDownloadUrls,
  RecordingDetailResponse,
  DeleteRecordingResponse,
  ProcessRecordingResponse,
  SubmitReportResponse,
  WebSocketStats,
  BroadcastMessage,
  BroadcastResponse,
  SendToUserMessage,
  PaginatedResults_for_LocalizedOrganizationDto,
  CreateOrganization,
  OrganizationDto,
  UpdateOrganizationBody,
  OrganizationTeamRole,
  OrganizationTeamUserDto,
  OrganizationTeamResponseDto,
  UpsertOrganizationUserBody,
  UpsertOrganizationUserResponseDto,
  UpdateOrganizationMemberRoleBody,
  RegionType,
  LocalizedRegionDto,
  PaginatedResults_for_LocalizedRegionDto,
  CreateRegion,
  RegionDto,
  PartialRegion,
  RegionAreaLinksDto,
  RegionAreaLinksRequestDto,
  RegionAreaDto,
  CreateRegionArea,
  PartialRegionArea,
  MediaContentType,
  content_type,
  MediaDto,
  PaginatedResults_for_MediaDto,
  MediaEditableFields,
  Job,
  PaginatedResults_for_Job,
  CreateJob,
  ComhairleServices,
  CreateApiKeyRequest,
  CreateResponse2,
  EmailType,
  email_type,
  EmailTemplateSlots,
  EmailTemplateConfigDto,
  CreateEmailTemplateConfig,
  UpdateEmailTemplateConfig,
  ContentType,
  SlotSchemaDefinition,
  EmailTypeSchema,
  PreviewEmailTemplateConfigRequest,
  PreviewEmailTemplateConfigResponse,
  ResourcePermission,
  PaginatedResults_for_ResourcePermission,
  GrantPermissionBody,
  UserWithPermissionDto,
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
    path: "/conversation/:conversation_id/cohosts",
    alias: "ListConversationCoHostOrganizations",
    description: `Returns organizations that hold the conversation co-host role for this conversation.`,
    requestFormat: "json",
    response: z.array(OrganizationWithPermissionDto),
  },
  {
    method: "post",
    path: "/conversation/:conversation_id/cohosts",
    alias: "AddConversationCoHostOrganization",
    description: `Grants the conversation co-host role to the specified organization.`,
    requestFormat: "json",
    parameters: [
      {
        name: "body",
        type: "Body",
        schema: z.object({ organization_id: z.string().uuid() }).passthrough(),
      },
    ],
    response: OrganizationWithPermissionDto,
  },
  {
    method: "delete",
    path: "/conversation/:conversation_id/cohosts/:cohost_id",
    alias: "RemoveConversationCoHostOrganization",
    description: `Revokes the conversation co-host role from the specified organization.`,
    requestFormat: "json",
    response: OrganizationWithPermissionDto,
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
    alias: "PostDocuments",
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
    method: "get",
    path: "/conversation/:conversation_id/documents/learn_content",
    alias: "GetLearnContent",
    requestFormat: "json",
    response: LearnContentResponse,
  },
  {
    method: "post",
    path: "/conversation/:conversation_id/documents/sync_learning_content",
    alias: "SyncLearningContent",
    requestFormat: "form-data",
    parameters: [
      {
        name: "body",
        description: `multipart form data`,
        type: "Body",
        schema: z.array(z.any()),
      },
    ],
    response: SyncLearningContentResponse,
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
    path: "/conversation/:conversation_id/events/:event_id/audio_recordings",
    alias: "ListAudioRecordings",
    description: `List all audio recordings for an event with their processing status.`,
    requestFormat: "json",
    response: z.array(AudioRecordingDto),
  },
  {
    method: "post",
    path: "/conversation/:conversation_id/events/:event_id/audio_recordings",
    alias: "CreateAudioRecording",
    description: `Create a named audio recording for an event and return a presigned S3 URL for uploading its audio.`,
    requestFormat: "json",
    parameters: [
      {
        name: "body",
        description: `Request body for creating an audio recording and requesting its upload URL.`,
        type: "Body",
        schema: CreateRecordingRequest,
      },
    ],
    response: CreateRecordingResponse,
  },
  {
    method: "get",
    path: "/conversation/:conversation_id/events/:event_id/audio_recordings/:recording_id",
    alias: "GetAudioRecording",
    description: `Get an audio recording&#x27;s details and presigned S3 URLs for its audio, transcript, and report.`,
    requestFormat: "json",
    response: RecordingDetailResponse,
  },
  {
    method: "delete",
    path: "/conversation/:conversation_id/events/:event_id/audio_recordings/:recording_id",
    alias: "DeleteAudioRecording",
    description: `Delete an audio recording and best-effort-clean its files from bulk storage. Useful for clearing stuck rows left behind by a failed upload.`,
    requestFormat: "json",
    response: DeleteRecordingResponse,
  },
  {
    method: "post",
    path: "/conversation/:conversation_id/events/:event_id/audio_recordings/:recording_id/process",
    alias: "ProcessAudioRecording",
    description: `Enqueue a background job to transcribe and categorize a single audio recording.`,
    requestFormat: "json",
    response: ProcessRecordingResponse,
  },
  {
    method: "post",
    path: "/conversation/:conversation_id/events/:event_id/audio_recordings/:recording_id/report",
    alias: "SubmitAudioRecordingReport",
    description: `Webhook for the categorization service to submit a recording&#x27;s report. Authenticated by HMAC signature headers.`,
    requestFormat: "json",
    response: SubmitReportResponse,
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
    method: "get",
    path: "/conversation/:conversation_id/events/:event_id/breakout",
    alias: "GetEventBreakoutPlan",
    requestFormat: "json",
    response: BreakoutPlanDto,
  },
  {
    method: "put",
    path: "/conversation/:conversation_id/events/:event_id/breakout",
    alias: "SaveEventBreakoutPlan",
    requestFormat: "json",
    parameters: [
      {
        name: "body",
        description: `Body for saving an edited plan.`,
        type: "Body",
        schema: SaveBreakoutPlanRequest,
      },
    ],
    response: BreakoutPlanDto,
  },
  {
    method: "post",
    path: "/conversation/:conversation_id/events/:event_id/breakout/seed",
    alias: "SeedEventBreakoutPlan",
    requestFormat: "json",
    response: BreakoutPlanDto,
  },
  {
    method: "get",
    path: "/conversation/:conversation_id/events/:event_id/metadata",
    alias: "GetEventMetadata",
    description: `Get event metadata`,
    requestFormat: "json",
    response: z.unknown(),
  },
  {
    method: "patch",
    path: "/conversation/:conversation_id/events/:event_id/metadata",
    alias: "PatchEventMetadata",
    description: `Merge a JSON object into event.metadata at the top level using jsonb concatenation`,
    requestFormat: "json",
    parameters: [
      {
        name: "body",
        type: "Body",
        schema: z.unknown(),
      },
    ],
    response: EventDto,
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
    method: "patch",
    path: "/conversation/:conversation_id/metadata",
    alias: "PatchConversationMetadata",
    description: `Accepts a JSON object and merges it into the conversation&#x27;s &#x60;metadata&#x60; jsonb column at the top level. Keys in the body overwrite existing keys; keys not present are left untouched. Nested objects are replaced, not deep-merged.`,
    requestFormat: "json",
    parameters: [
      {
        name: "body",
        type: "Body",
        schema: z.unknown(),
      },
    ],
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
    parameters: [
      {
        name: "withTranslations",
        type: "Query",
        schema: z.boolean().optional().default(false),
      },
    ],
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
    response: z.union([UserParticipationDto, z.null()]),
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
    method: "get",
    path: "/conversation/:conversation_id/workflow/:workflow_id/recruitment_targets",
    alias: "ListRecruitmentTargets",
    requestFormat: "json",
    response: z.array(RecruitmentTargetDto),
  },
  {
    method: "post",
    path: "/conversation/:conversation_id/workflow/:workflow_id/recruitment_targets",
    alias: "CreateRecruitmentTarget",
    description: `Records the target number of participants for a given demographic metric/bucket combination on this workflow. Upserts on (workflow_id, metric, bucket).`,
    requestFormat: "json",
    parameters: [
      {
        name: "body",
        type: "Body",
        schema: CreateRecruitmentTarget,
      },
    ],
    response: RecruitmentTargetDto,
  },
  {
    method: "get",
    path: "/conversation/:conversation_id/workflow/:workflow_id/recruitment_targets/:recruitment_target_id",
    alias: "GetRecruitmentTarget",
    requestFormat: "json",
    response: RecruitmentTargetDto,
  },
  {
    method: "put",
    path: "/conversation/:conversation_id/workflow/:workflow_id/recruitment_targets/:recruitment_target_id",
    alias: "UpdateRecruitmentTarget",
    requestFormat: "json",
    parameters: [
      {
        name: "body",
        type: "Body",
        schema: PartialRecruitmentTarget,
      },
    ],
    response: RecruitmentTargetDto,
  },
  {
    method: "delete",
    path: "/conversation/:conversation_id/workflow/:workflow_id/recruitment_targets/:recruitment_target_id",
    alias: "DeleteRecruitmentTarget",
    requestFormat: "json",
    response: RecruitmentTargetDto,
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
        name: "email_type",
        type: "Query",
        schema: email_type,
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
    path: "/email_template_configs/:email_config_id/schemas",
    alias: "GetEmailTemplateSchema",
    description: `Get template schemas for an email config`,
    requestFormat: "json",
    response: EmailTypeSchema,
  },
  {
    method: "post",
    path: "/email_template_configs/preview",
    alias: "PreviewEmailTemplateConfig",
    description: `Preview appearance of custom email before sending`,
    requestFormat: "json",
    parameters: [
      {
        name: "body",
        type: "Body",
        schema: PreviewEmailTemplateConfigRequest,
      },
    ],
    response: z.object({ html: z.string() }).passthrough(),
  },
  {
    method: "get",
    path: "/email_template_configs/schemas",
    alias: "ListEmailTemplateSchemas",
    description: `List all template schemas for each email template type`,
    requestFormat: "json",
    response: z.array(EmailTypeSchema).min(4).max(4),
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
    response: z.array(MediaDto),
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
    method: "patch",
    path: "/media/:media_id",
    alias: "UpdateMedia",
    description: `Update a media record by id`,
    requestFormat: "json",
    parameters: [
      {
        name: "body",
        type: "Body",
        schema: MediaEditableFields,
      },
    ],
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
        schema: UpdateOrganizationBody,
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
    method: "post",
    path: "/organizations/:organization_id/members",
    alias: "AddOrganizationMember",
    description: `Adds a member by email and bootstraps an account when needed`,
    requestFormat: "json",
    parameters: [
      {
        name: "body",
        type: "Body",
        schema: UpsertOrganizationUserBody,
      },
    ],
    response: UpsertOrganizationUserResponseDto,
  },
  {
    method: "delete",
    path: "/organizations/:organization_id/members/:user_id",
    alias: "RemoveOrganizationMember",
    description: `Removes a user&#x27;s organization membership`,
    requestFormat: "json",
    parameters: [
      {
        name: "organization_id",
        type: "Path",
        schema: z.string().uuid(),
      },
      {
        name: "user_id",
        type: "Path",
        schema: z.string().uuid(),
      },
    ],
    response: z.void(),
  },
  {
    method: "put",
    path: "/organizations/:organization_id/members/:user_id/role",
    alias: "UpdateOrganizationMemberRole",
    description: `Updates organization member role between member and admin`,
    requestFormat: "json",
    parameters: [
      {
        name: "body",
        type: "Body",
        schema: UpdateOrganizationMemberRoleBody,
      },
      {
        name: "organization_id",
        type: "Path",
        schema: z.string().uuid(),
      },
      {
        name: "user_id",
        type: "Path",
        schema: z.string().uuid(),
      },
    ],
    response: z.void(),
  },
  {
    method: "get",
    path: "/organizations/:organization_id/metadata",
    alias: "GetOrganizationMetadata",
    description: `Get organization metadata`,
    requestFormat: "json",
    response: z.unknown(),
  },
  {
    method: "patch",
    path: "/organizations/:organization_id/metadata",
    alias: "PatchOrganizationMetadata",
    description: `Merge a JSON object into organization.metadata at the top level using jsonb concatenation`,
    requestFormat: "json",
    parameters: [
      {
        name: "body",
        type: "Body",
        schema: z.unknown(),
      },
    ],
    response: OrganizationDto,
  },
  {
    method: "get",
    path: "/organizations/:organization_id/team",
    alias: "GetOrganizationTeam",
    description: `Returns members and administrators for an organization`,
    requestFormat: "json",
    response: OrganizationTeamResponseDto,
  },
  {
    method: "get",
    path: "/permissions",
    alias: "ListPermissions",
    description: `Returns role assignments using offset-based pagination. Optionally filter by user_id, organization_id, or role_name. Use the &#x60;offset&#x60; and &#x60;limit&#x60; query params to page through results.`,
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
      {
        name: "organization_id",
        type: "Query",
        schema: created_after,
      },
      {
        name: "role_name",
        type: "Query",
        schema: created_after,
      },
      {
        name: "user_id",
        type: "Query",
        schema: created_after,
      },
    ],
    response: PaginatedResults_for_ResourcePermission,
  },
  {
    method: "get",
    path: "/permissions/:resource_type/:resource_id",
    alias: "ListResourcePermissions",
    description: `Returns role assignments for a specific resource using offset-based pagination. Optionally filter by user_id, organization_id, or role_name. The caller must hold the Owner role on the resource.`,
    requestFormat: "json",
    parameters: [
      {
        name: "resource_id",
        type: "Path",
        schema: z.string().uuid(),
      },
      {
        name: "resource_type",
        type: "Path",
        schema: z.string(),
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
      {
        name: "organization_id",
        type: "Query",
        schema: created_after,
      },
      {
        name: "role_name",
        type: "Query",
        schema: created_after,
      },
      {
        name: "user_id",
        type: "Query",
        schema: created_after,
      },
    ],
    response: PaginatedResults_for_ResourcePermission,
  },
  {
    method: "post",
    path: "/permissions/:resource_type/:resource_id",
    alias: "GrantPermission",
    description: `Grants a role to a user or organisation on a resource. The caller must hold the Owner role on the resource.`,
    requestFormat: "json",
    parameters: [
      {
        name: "body",
        description: `Represents a request body for granting a permission to a user or organization.`,
        type: "Body",
        schema: GrantPermissionBody,
      },
      {
        name: "resource_id",
        type: "Path",
        schema: z.string().uuid(),
      },
      {
        name: "resource_type",
        type: "Path",
        schema: z.string(),
      },
    ],
    response: ResourcePermission,
  },
  {
    method: "delete",
    path: "/permissions/:resource_type/:resource_id",
    alias: "RevokePermission",
    description: `Revokes a role from a user or organisation on a resource. The actor (user_id or organization_id) and role_name are provided as query parameters. The caller must hold the Owner role on the resource.`,
    requestFormat: "json",
    parameters: [
      {
        name: "resource_id",
        type: "Path",
        schema: z.string().uuid(),
      },
      {
        name: "resource_type",
        type: "Path",
        schema: z.string(),
      },
      {
        name: "organization_id",
        type: "Query",
        schema: created_after,
      },
      {
        name: "role_name",
        type: "Query",
        schema: z.string(),
      },
      {
        name: "user_id",
        type: "Query",
        schema: created_after,
      },
    ],
    response: z.void(),
  },
  {
    method: "get",
    path: "/permissions/:resource_type/:resource_id/users",
    alias: "ListUsersWithPermission",
    description: `List users with a give permission (role + resource_type) for a given resource`,
    requestFormat: "json",
    parameters: [
      {
        name: "resource_id",
        type: "Path",
        schema: z.string().uuid(),
      },
      {
        name: "resource_type",
        type: "Path",
        schema: z.string(),
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
      {
        name: "organization_id",
        type: "Query",
        schema: created_after,
      },
      {
        name: "role_name",
        type: "Query",
        schema: created_after,
      },
      {
        name: "user_id",
        type: "Query",
        schema: created_after,
      },
    ],
    response: z.array(UserWithPermissionDto),
  },
  {
    method: "get",
    path: "/permissions/by-action/:action",
    alias: "ListPermissionsByAction",
    description: `Returns resources of the specified type that the caller can perform the specified action on. Optionally filter by user_id. Use the &#x60;offset&#x60; and &#x60;limit&#x60; query params to page through results.`,
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
      {
        name: "user_id",
        type: "Query",
        schema: created_after,
      },
    ],
    response: z.array(ResourcePermission),
  },
  {
    method: "get",
    path: "/region_areas",
    alias: "ListRegionAreas",
    requestFormat: "json",
    response: z.array(RegionAreaDto),
  },
  {
    method: "post",
    path: "/region_areas",
    alias: "CreateRegionArea",
    requestFormat: "json",
    parameters: [
      {
        name: "body",
        type: "Body",
        schema: z.object({ zip_prefix: z.string() }).passthrough(),
      },
    ],
    response: RegionAreaDto,
  },
  {
    method: "get",
    path: "/region_areas/:region_area_id",
    alias: "GetRegionArea",
    requestFormat: "json",
    response: RegionAreaDto,
  },
  {
    method: "put",
    path: "/region_areas/:region_area_id",
    alias: "UpdateRegionArea",
    requestFormat: "json",
    parameters: [
      {
        name: "body",
        type: "Body",
        schema: PartialRegionArea,
      },
    ],
    response: RegionAreaDto,
  },
  {
    method: "delete",
    path: "/region_areas/:region_area_id",
    alias: "DeleteRegionArea",
    requestFormat: "json",
    response: RegionAreaDto,
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
    path: "/regions/:region_id/areas",
    alias: "GetRegionAreaLinks",
    description: `List region area links`,
    requestFormat: "json",
    response: RegionAreaLinksDto,
  },
  {
    method: "put",
    path: "/regions/:region_id/areas",
    alias: "SetRegionAreaLinks",
    description: `Replace region area links`,
    requestFormat: "json",
    parameters: [
      {
        name: "body",
        type: "Body",
        schema: RegionAreaLinksRequestDto,
      },
    ],
    response: RegionAreaLinksDto,
  },
  {
    method: "post",
    path: "/regions/:region_id/areas/:area_id",
    alias: "AddRegionAreaLink",
    description: `Add region area link`,
    requestFormat: "json",
    response: RegionAreaLinksDto,
  },
  {
    method: "delete",
    path: "/regions/:region_id/areas/:area_id",
    alias: "RemoveRegionAreaLink",
    description: `Remove region area link`,
    requestFormat: "json",
    response: RegionAreaLinksDto,
  },
  {
    method: "get",
    path: "/regions/:region_id/metadata",
    alias: "GetRegionMetadata",
    description: `Get region metadata`,
    requestFormat: "json",
    response: z.unknown(),
  },
  {
    method: "patch",
    path: "/regions/:region_id/metadata",
    alias: "PatchRegionMetadata",
    description: `Merge a JSON object into region.metadata at the top level using jsonb concatenation`,
    requestFormat: "json",
    parameters: [
      {
        name: "body",
        type: "Body",
        schema: z.unknown(),
      },
    ],
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
    method: "put",
    path: "/tools/polis/config",
    alias: "PolisUpdateConfig",
    description: `Proxies topic, description, strict_moderation and is_active to the Polis conversation via the server-side admin session. Only provided fields are written.`,
    requestFormat: "json",
    parameters: [
      {
        name: "body",
        type: "Body",
        schema: UpdatePolisConfigRequest,
      },
    ],
    response: WikiPoll,
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
    response: WikiPollReport,
  },
  {
    method: "post",
    path: "/tools/polis/seed",
    alias: "PolisPostSeed",
    description: `Posts a moderator-authored seed statement (is_seed) to the active Polis poll via the server-side admin session. Re-sync to surface it in the local statement_aux table.`,
    requestFormat: "json",
    parameters: [
      {
        name: "body",
        type: "Body",
        schema: PostSeedRequest,
      },
    ],
    response: z.object({ polis_statement_id: z.string() }).passthrough(),
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
    path: "/tools/polis/statement_aux/:id/moderate",
    alias: "PolisModerateStatementAux",
    description: `Forwards a moderation decision (accept/reject) to the Polis server using the admin account, then updates the polis_statement_aux row&#x27;s moderation_status and moderation_reason`,
    requestFormat: "json",
    parameters: [
      {
        name: "body",
        type: "Body",
        schema: ModerateStatementAuxRequest,
      },
    ],
    response: PolisStatementAux,
  },
  {
    method: "post",
    path: "/tools/polis/statement_aux/:id/split",
    alias: "PolisSplitStatement",
    description: `Posts one or more admin-authored replacement statements as non-seed (is_seed: false), auto-accepts them, rejects the original statement, and records lineage (original_statement_id) on each replacement. The replacements are real, votable statements, never host seeds. Returns the now-rejected original and the derived replacements.`,
    requestFormat: "json",
    parameters: [
      {
        name: "body",
        type: "Body",
        schema: SplitStatementRequest,
      },
    ],
    response: SplitStatementResponse,
  },
  {
    method: "post",
    path: "/tools/polis/statement_aux/:id/themes",
    alias: "PolisAddStatementAuxTheme",
    description: `Adds a theme to the statement&#x27;s themes array. Idempotent: adding a theme that is already present is a no-op. Caller must be the owner of the conversation the statement belongs to.`,
    requestFormat: "json",
    parameters: [
      {
        name: "body",
        type: "Body",
        schema: z.object({ theme: z.string() }).passthrough(),
      },
    ],
    response: PolisStatementAux,
  },
  {
    method: "delete",
    path: "/tools/polis/statement_aux/:id/themes",
    alias: "PolisRemoveStatementAuxTheme",
    description: `Removes a theme from the statement&#x27;s themes array. Idempotent: removing a theme that is not present is a no-op. Caller must be the owner of the conversation the statement belongs to.`,
    requestFormat: "json",
    parameters: [
      {
        name: "body",
        type: "Body",
        schema: z.object({ theme: z.string() }).passthrough(),
      },
    ],
    response: PolisStatementAux,
  },
  {
    method: "post",
    path: "/tools/polis/statement_aux/moderate_batch",
    alias: "PolisModerateStatementAuxBatch",
    description: `Forwards an accept/reject decision for many polis_statement_aux rows to Polis using a single admin login, then bulk-updates the rows that succeeded. All ids must belong to the same workflow step. Returns the updated rows plus any per-row failures.`,
    requestFormat: "json",
    parameters: [
      {
        name: "body",
        type: "Body",
        schema: ModerateStatementAuxBatchRequest,
      },
    ],
    response: ModerateStatementAuxBatchResponse,
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
    path: "/tools/prioritization/insights",
    alias: "GetPrioritizationInsights",
    description: `Insights reporting data for prioritization tool step`,
    requestFormat: "json",
    parameters: [
      {
        name: "workflow_step_id",
        type: "Query",
        schema: z.string().uuid(),
      },
    ],
    response: PrioritizationInsightsResponse,
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
    method: "post",
    path: "/tools/prioritization/proposals/:proposal_id/sections",
    alias: "CreateProposalSection",
    description: `Append a section to a prioritization tool proposal`,
    requestFormat: "json",
    parameters: [
      {
        name: "body",
        type: "Body",
        schema: CreateSectionRequest,
      },
    ],
    response: ProposalSectionDto,
  },
  {
    method: "delete",
    path: "/tools/prioritization/proposals/:proposal_id/sections/:section_id",
    alias: "DeleteProposalSection",
    description: `Delete a section from a prioritization tool proposal`,
    requestFormat: "json",
    response: ProposalSectionDto,
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
    method: "get",
    path: "/tools/survey_tool/workflow_step/:workflow_step_id/form",
    alias: "HeyFormGetForm",
    description: `Fetches the form for the HeyForm tool attached to a workflow step`,
    requestFormat: "json",
    response: Form,
  },
  {
    method: "get",
    path: "/tools/survey_tool/workflow_step/:workflow_step_id/form_report",
    alias: "HeyFormGetFormReport",
    description: `Fetches the form report for the HeyForm tool attached to a workflow step`,
    requestFormat: "json",
    response: FormReport,
  },
  {
    method: "get",
    path: "/tools/survey_tool/workflow_step/:workflow_step_id/insights",
    alias: "HeyFormGetInsights",
    description: `Combines the HeyForm form definition with its aggregate report to produce a per-question breakdown with human-readable question titles and choice labels resolved from the form schema.`,
    requestFormat: "json",
    response: SurveyInsights,
  },
  {
    method: "get",
    path: "/tools/survey_tool/workflow_step/:workflow_step_id/submissions",
    alias: "HeyFormGetSubmissions",
    description: `Fetches the form submissions for the HeyForm tool attached to a workflow step`,
    requestFormat: "json",
    parameters: [
      {
        name: "category",
        type: "Query",
        schema: created_after,
      },
    ],
    response: Submissions,
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
    path: "/tools/thinking_space/insights",
    alias: "GetThinkingSpaceInsights",
    description: `Get thinking space insights data`,
    requestFormat: "json",
    parameters: [
      {
        name: "workflow_step_id",
        type: "Query",
        schema: z.string().uuid(),
      },
    ],
    response: ThinkingSpaceInsightsResponse,
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
        name: "is_shared_with_organizer",
        type: "Query",
        schema: is_complete,
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
    path: "/user/organizations",
    alias: "GetUserOrganizations",
    description: `Gets the organizations associated with the current user and those they can manage`,
    requestFormat: "json",
    response: UserOrganizationsResponse,
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
    path: "/user/permitted_conversations",
    alias: "GetPermittedConversations",
    description: `Gets a list of the conversations a user is permitted access to`,
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

// Axios error for failing request
export interface ApiError {
  // Standard
  message: string;
  name: string;
  // Microsoft
  description: string;
  number: number;
  // Mozilla
  fileName: string;
  lineNumber: number;
  columnNumber: number;
  stack: string;
  // Axios
  config: {
    adapter: string[];
    allowAbsoluteUrls: boolean;
    baseURL: string;
    data: undefined;
    env: object;
    headers: object;
    maxBodyLength: number;
    maxContentLength: number;
    method: string;
    params: object;
    timeout: number;
    transformRequest: string[];
    transformResponse: string[];
    transitional: {
      silentJSONParsing: boolean;
      forcedJSONParsing: boolean;
      clarifyTimeoutError: boolean;
    };
    url: string;
    validateStatus: (status: string) => void;
    withCredentials: true;
    xsrfCookieName: string;
    xsrfHeaderName: string;
  };
  code: string;
  status: number;
  response: Response;
  request: Request;
}

export const api: ZodiosInstance<typeof endpoints> = new Zodios(endpoints);
export type ApiClient = typeof api;

export function createApiClient(
  baseUrl: string,
  options?: ZodiosOptions
): ZodiosInstance<typeof endpoints> {
  return new Zodios(baseUrl, endpoints, options);
}

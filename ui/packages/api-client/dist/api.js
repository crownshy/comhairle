import { makeApi, Zodios, } from "@zodios/core";
import { z } from "zod";
const _GuestLoginRequest = z.object({ guest_code: z.string() }).passthrough();
export const GuestLoginRequest = _GuestLoginRequest;
const _UserAuthType = z.enum([
    "guest",
    "email_password",
    "otp",
    "scot_account",
]);
export const UserAuthType = _UserAuthType;
const _UserDto = z
    .object({
    authType: UserAuthType,
    avatarUrl: z.union([z.string(), z.null()]).optional(),
    email: z.union([z.string(), z.null()]).optional(),
    emailVerified: z.boolean(),
    guestCode: z.union([z.string(), z.null()]).optional(),
    id: z.string().uuid(),
    organizationId: z.union([z.string(), z.null()]).optional(),
    username: z.union([z.string(), z.null()]).optional(),
})
    .passthrough();
export const UserDto = _UserDto;
const _LoginRequest = z
    .object({ email: z.string(), password: z.string() })
    .passthrough();
export const LoginRequest = _LoginRequest;
const _OtpLoginRequest = z
    .object({ code: z.string(), email: z.string() })
    .passthrough();
export const OtpLoginRequest = _OtpLoginRequest;
const _SignupRequest = z
    .object({
    avatar_url: z.union([z.string(), z.null()]).optional(),
    email: z.string(),
    password: z.string(),
    username: z.string(),
})
    .passthrough();
export const SignupRequest = _SignupRequest;
const _OtpSignupRequest = z
    .object({
    email: z.string(),
    username: z.union([z.string(), z.null()]).optional(),
})
    .passthrough();
export const OtpSignupRequest = _OtpSignupRequest;
const _CreateOtpRequest = z
    .object({
    email: z.string(),
    redirect_url: z.union([z.string(), z.null()]).optional(),
})
    .passthrough();
export const CreateOtpRequest = _CreateOtpRequest;
const _VerifyOtpTokenRequest = z.object({ token: z.string() }).passthrough();
export const VerifyOtpTokenRequest = _VerifyOtpTokenRequest;
const _VerifyEmailTokenRequest = z.object({ token: z.string() }).passthrough();
export const VerifyEmailTokenRequest = _VerifyEmailTokenRequest;
const _ResendVerificationEmailRequest = z
    .object({ id: z.string() })
    .passthrough();
export const ResendVerificationEmailRequest = _ResendVerificationEmailRequest;
const _CreatePasswordResetRequest = z
    .object({ email: z.string() })
    .passthrough();
export const CreatePasswordResetRequest = _CreatePasswordResetRequest;
const _PasswordResetUpdateRequest = z
    .object({
    confirm_password: z.string(),
    password: z.string(),
    token: z.string(),
})
    .passthrough();
export const PasswordResetUpdateRequest = _PasswordResetUpdateRequest;
const _ResourceType = z.union([
    z.literal("Site"),
    z.object({ Conversation: z.string().uuid() }),
]);
export const ResourceType = _ResourceType;
const _ResourceRole = z.enum(["Admin", "SuperAdmin"]);
export const ResourceRole = _ResourceRole;
const _UserRoles = z
    .object({ resource: ResourceType, roles: z.array(ResourceRole) })
    .passthrough();
export const UserRoles = _UserRoles;
const _LocalizedConversationDto = z
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
export const LocalizedConversationDto = _LocalizedConversationDto;
const _created_after = z.union([z.string(), z.null()]).optional();
export const created_after = _created_after;
const _is_complete = z.union([z.boolean(), z.null()]).optional();
export const is_complete = _is_complete;
const _limit = z.union([z.number(), z.null()]).optional();
export const limit = _limit;
const _PaginatedResults_for_LocalizedConversationDto = z
    .object({
    records: z.array(LocalizedConversationDto),
    total: z.number().int(),
})
    .passthrough();
export const PaginatedResults_for_LocalizedConversationDto = _PaginatedResults_for_LocalizedConversationDto;
const _OrganizationType = z.enum(["non_profit", "governmental", "other"]);
export const OrganizationType = _OrganizationType;
const _LocalizedOrganizationDto = z
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
export const LocalizedOrganizationDto = _LocalizedOrganizationDto;
const _UserOrganizationAccess = z
    .object({
    canDelete: z.boolean(),
    canManageTeam: z.boolean(),
    canUpdate: z.boolean(),
    isAssociated: z.boolean(),
    organization: LocalizedOrganizationDto,
})
    .passthrough();
export const UserOrganizationAccess = _UserOrganizationAccess;
const _UserOrganizationsResponse = z
    .object({
    canCreateOrganization: z.boolean(),
    organizations: z.array(UserOrganizationAccess),
})
    .passthrough();
export const UserOrganizationsResponse = _UserOrganizationsResponse;
const _UpdateUserRequest = z
    .object({
    email_verified: z.union([z.boolean(), z.null()]),
    organization_id: z.union([z.string(), z.null()]),
    password: z.union([z.string(), z.null()]),
    username: z.union([z.string(), z.null()]),
})
    .partial()
    .passthrough();
export const UpdateUserRequest = _UpdateUserRequest;
const _UpgradeAccountRequest = z
    .object({ email: z.string(), password: z.string(), username: z.string() })
    .passthrough();
export const UpgradeAccountRequest = _UpgradeAccountRequest;
const _UserConversationPreferencesDto = z
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
export const UserConversationPreferencesDto = _UserConversationPreferencesDto;
const _UpdateUserConversationPreferences = z
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
export const UpdateUserConversationPreferences = _UpdateUserConversationPreferences;
const _UserProfileDto = z
    .object({
    consented: z.boolean(),
    createdAt: z.string().datetime({ offset: true }),
    id: z.string().uuid(),
    updatedAt: z.string().datetime({ offset: true }),
    userId: z.string().uuid(),
})
    .passthrough();
export const UserProfileDto = _UserProfileDto;
const _UpsertUserProfileRequest = z
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
export const UpsertUserProfileRequest = _UpsertUserProfileRequest;
const _DeliveryMethod = z.enum(["in_app", "email"]);
export const DeliveryMethod = _DeliveryMethod;
const _NotificationContextType = z.enum(["site", "conversation"]);
export const NotificationContextType = _NotificationContextType;
const _NotificationType = z.enum(["info", "warning", "error", "success"]);
export const NotificationType = _NotificationType;
const _NotificationDto = z
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
export const NotificationDto = _NotificationDto;
const _NotificationWithDelivery = z
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
export const NotificationWithDelivery = _NotificationWithDelivery;
const _PaginatedResults_for_NotificationWithDelivery = z
    .object({
    records: z.array(NotificationWithDelivery),
    total: z.number().int(),
})
    .passthrough();
export const PaginatedResults_for_NotificationWithDelivery = _PaginatedResults_for_NotificationWithDelivery;
const _UnreadCount = z.object({ count: z.number().int() }).passthrough();
export const UnreadCount = _UnreadCount;
const _NotificationDelivery = z
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
export const NotificationDelivery = _NotificationDelivery;
const _TextFormat = z.union([
    z.literal("plain"),
    z.literal("markdown"),
    z.literal("rich"),
]);
export const TextFormat = _TextFormat;
const _CreateTextContentRequest = z
    .object({
    content: z.string(),
    format: TextFormat,
    primary_locale: z.string(),
})
    .passthrough();
export const CreateTextContentRequest = _CreateTextContentRequest;
const _TextContentDto = z
    .object({
    format: TextFormat,
    id: z.string().uuid(),
    primaryLocale: z.string(),
})
    .passthrough();
export const TextContentDto = _TextContentDto;
const _TextTranslationDto = z
    .object({
    aiGenerated: z.boolean(),
    content: z.string(),
    contentId: z.string().uuid(),
    id: z.string().uuid(),
    locale: z.string(),
    requiresValidation: z.boolean(),
})
    .passthrough();
export const TextTranslationDto = _TextTranslationDto;
const _TextContentWithTranslations = z
    .object({
    format: TextFormat,
    id: z.string().uuid(),
    primaryLocale: z.string(),
    translations: z.array(TextTranslationDto),
})
    .passthrough();
export const TextContentWithTranslations = _TextContentWithTranslations;
const _UpdateTextContent = z
    .object({
    format: z.union([TextFormat, z.null()]),
    primary_locale: z.union([z.string(), z.null()]),
})
    .partial()
    .passthrough();
export const UpdateTextContent = _UpdateTextContent;
const _UpdateTextTranslation = z
    .object({
    ai_generated: z.union([z.boolean(), z.null()]),
    content: z.union([z.string(), z.null()]),
    locale: z.union([z.string(), z.null()]),
    requires_validation: z.union([z.boolean(), z.null()]),
})
    .partial()
    .passthrough();
export const UpdateTextTranslation = _UpdateTextTranslation;
const _CreateOrUpdateTextTranslationRequest = z
    .object({
    ai_generated: z.union([z.boolean(), z.null()]).optional(),
    content: z.string(),
    requires_validation: z.union([z.boolean(), z.null()]).optional(),
})
    .passthrough();
export const CreateOrUpdateTextTranslationRequest = _CreateOrUpdateTextTranslationRequest;
const _GroupVoteCounts = z
    .object({
    agrees: z.number().int().gte(0),
    disagrees: z.number().int().gte(0),
    group_id: z.number().int().gte(0),
    passes: z.number().int().gte(0),
})
    .passthrough();
export const GroupVoteCounts = _GroupVoteCounts;
const _VoteCounts = z
    .object({
    agrees: z.number().int().gte(0),
    disagrees: z.number().int().gte(0),
    passes: z.number().int().gte(0),
})
    .passthrough();
export const VoteCounts = _VoteCounts;
const _CommentReportData = z
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
export const CommentReportData = _CommentReportData;
const _RepresentativeComment = z
    .object({ text: z.string(), tid: z.number().int().gte(0) })
    .passthrough();
export const RepresentativeComment = _RepresentativeComment;
const _GroupReportData = z
    .object({
    group_id: z.number().int().gte(0),
    members: z.array(z.number().int().gte(0)),
    representative_comments: z.array(RepresentativeComment),
    total_members: z.number().int().gte(0),
})
    .passthrough();
export const GroupReportData = _GroupReportData;
const _PcaPosition = z.object({ x: z.number(), y: z.number() }).passthrough();
export const PcaPosition = _PcaPosition;
const _ParticipantReportData = z
    .object({
    group_id: z.union([z.number(), z.null()]).optional(),
    pca_position: z.union([PcaPosition, z.null()]).optional(),
    pid: z.number().int().gte(0),
})
    .passthrough();
export const ParticipantReportData = _ParticipantReportData;
const _WikiPollReport = z
    .object({
    comments: z.array(CommentReportData),
    groups: z.array(GroupReportData),
    participants: z.array(ParticipantReportData),
})
    .passthrough();
export const WikiPollReport = _WikiPollReport;
const _VoteCountResponse = z
    .object({ vote_count: z.number().int().gte(0) })
    .passthrough();
export const VoteCountResponse = _VoteCountResponse;
const _UpdatePolisConfigRequest = z
    .object({
    description: z.union([z.string(), z.null()]).optional(),
    is_active: z.union([z.boolean(), z.null()]).optional(),
    strict_moderation: z.union([z.boolean(), z.null()]).optional(),
    topic: z.union([z.string(), z.null()]).optional(),
    workflow_step_id: z.string().uuid(),
})
    .passthrough();
export const UpdatePolisConfigRequest = _UpdatePolisConfigRequest;
const _WikiPoll = z
    .object({
    is_active: z.union([z.boolean(), z.null()]).optional(),
    poll_id: z.string(),
})
    .passthrough();
export const WikiPoll = _WikiPoll;
const _PostSeedRequest = z
    .object({ statement_text: z.string(), workflow_step_id: z.string().uuid() })
    .passthrough();
export const PostSeedRequest = _PostSeedRequest;
const _PostSeedResponse = z
    .object({ polis_statement_id: z.string() })
    .passthrough();
export const PostSeedResponse = _PostSeedResponse;
const _ModerationStatus = z.enum(["accepted", "rejected", "pending"]);
export const ModerationStatus = _ModerationStatus;
const _PolisStatementAux = z
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
export const PolisStatementAux = _PolisStatementAux;
const _CreatePolisStatementAux = z
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
export const CreatePolisStatementAux = _CreatePolisStatementAux;
const _UpdatePolisStatementAux = z
    .object({
    moderation_reason: z.union([z.string(), z.null()]),
    moderation_status: z.union([ModerationStatus, z.null()]),
    statement_text: z.union([z.string(), z.null()]),
    themes: z.union([z.array(z.string()), z.null()]),
    visible_statement_when_submitted: z.union([z.string(), z.null()]),
})
    .partial()
    .passthrough();
export const UpdatePolisStatementAux = _UpdatePolisStatementAux;
const _SyncStatementAuxRequest = z
    .object({ workflow_step_id: z.string().uuid() })
    .passthrough();
export const SyncStatementAuxRequest = _SyncStatementAuxRequest;
const _SyncStatementAuxResponse = z
    .object({
    skipped_invalid_xid: z.number().int().gte(0),
    statements: z.array(PolisStatementAux),
    synced: z.number().int().gte(0),
})
    .passthrough();
export const SyncStatementAuxResponse = _SyncStatementAuxResponse;
const _ThemeStatistic = z
    .object({ count: z.number().int(), theme: z.string() })
    .passthrough();
export const ThemeStatistic = _ThemeStatistic;
const _ThemeRequest = z.object({ theme: z.string() }).passthrough();
export const ThemeRequest = _ThemeRequest;
const _ModerationDecisionRequest = z.enum(["accept", "reject"]);
export const ModerationDecisionRequest = _ModerationDecisionRequest;
const _ModerateStatementAuxRequest = z
    .object({
    decision: ModerationDecisionRequest,
    moderation_reason: z.union([z.string(), z.null()]).optional(),
})
    .passthrough();
export const ModerateStatementAuxRequest = _ModerateStatementAuxRequest;
const _ModerateStatementAuxBatchRequest = z
    .object({
    decision: ModerationDecisionRequest,
    ids: z.array(z.string().uuid()),
    moderation_reason: z.union([z.string(), z.null()]).optional(),
})
    .passthrough();
export const ModerateStatementAuxBatchRequest = _ModerateStatementAuxBatchRequest;
const _ModerateBatchFailure = z
    .object({ error: z.string(), id: z.string().uuid() })
    .passthrough();
export const ModerateBatchFailure = _ModerateBatchFailure;
const _ModerateStatementAuxBatchResponse = z
    .object({
    failed: z.array(ModerateBatchFailure),
    succeeded: z.array(PolisStatementAux),
})
    .passthrough();
export const ModerateStatementAuxBatchResponse = _ModerateStatementAuxBatchResponse;
const _SplitStatementRequest = z
    .object({ replacements: z.array(z.string()) })
    .passthrough();
export const SplitStatementRequest = _SplitStatementRequest;
const _SplitStatementResponse = z
    .object({
    original: PolisStatementAux,
    replacements: z.array(PolisStatementAux),
})
    .passthrough();
export const SplitStatementResponse = _SplitStatementResponse;
const _FormField = z
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
export const FormField = _FormField;
const _FormSettings = z
    .object({
    active: z.union([z.boolean(), z.null()]),
    allowArchive: z.union([z.boolean(), z.null()]),
    enableQuestionList: z.union([z.boolean(), z.null()]),
    locale: z.union([z.string(), z.null()]),
    published: z.union([z.boolean(), z.null()]),
})
    .partial()
    .passthrough();
export const FormSettings = _FormSettings;
const _FormTheme = z
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
export const FormTheme = _FormTheme;
const _ThemeSettings = z
    .object({ theme: z.union([FormTheme, z.null()]) })
    .partial()
    .passthrough();
export const ThemeSettings = _ThemeSettings;
const _Form = z
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
export const Form = _Form;
const _FormReportResponse = z
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
export const FormReportResponse = _FormReportResponse;
const _FormReportAnswer = z
    .object({
    endAt: z.number().int(),
    kind: z.string(),
    submissionId: z.string(),
    value: z.unknown().optional(),
})
    .passthrough();
export const FormReportAnswer = _FormReportAnswer;
const _FormReportSubmission = z
    .object({ _id: z.string(), answers: z.array(FormReportAnswer) })
    .passthrough();
export const FormReportSubmission = _FormReportSubmission;
const _FormReport = z
    .object({
    responses: z.array(FormReportResponse),
    submissions: z.array(FormReportSubmission),
})
    .passthrough();
export const FormReport = _FormReport;
const _SubmissionCategory = z.enum(["inbox", "spam", "starred", "archive"]);
export const SubmissionCategory = _SubmissionCategory;
const _HiddenFieldAnswer = z
    .object({
    id: z.string(),
    name: z.string(),
    value: z.union([z.string(), z.null()]).optional(),
})
    .passthrough();
export const HiddenFieldAnswer = _HiddenFieldAnswer;
const _Submission = z
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
export const Submission = _Submission;
const _Submissions = z
    .object({ submissions: z.array(Submission), total: z.number().int().gte(0) })
    .passthrough();
export const Submissions = _Submissions;
const _InsightChoice = z
    .object({ count: z.number().int(), id: z.string(), label: z.string() })
    .passthrough();
export const InsightChoice = _InsightChoice;
const _InsightSubmission = z
    .object({
    submission_id: z.string(),
    submitted_at: z.union([z.number(), z.null()]).optional(),
    value: z.unknown(),
})
    .passthrough();
export const InsightSubmission = _InsightSubmission;
const _InsightQuestion = z
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
export const InsightQuestion = _InsightQuestion;
const _SurveyInsights = z
    .object({ questions: z.array(InsightQuestion) })
    .passthrough();
export const SurveyInsights = _SurveyInsights;
const _Story = z
    .object({
    id: z.string().uuid(),
    transcript_id: z.union([z.string(), z.null()]).optional(),
    user_id: z.string().uuid(),
    video_id: z.string().uuid(),
    workflow_step_id: z.string().uuid(),
})
    .passthrough();
export const Story = _Story;
const _ComhairleMessageReference = z
    .object({
    content: z.string(),
    dataset_id: z.string(),
    document_id: z.string(),
    document_name: z.string(),
    id: z.string(),
    positions: z.union([z.array(z.array(z.number())), z.null()]).optional(),
})
    .passthrough();
export const ComhairleMessageReference = _ComhairleMessageReference;
const _ComhairleSessionMessage = z
    .object({
    content: z.string(),
    id: z.string(),
    reference: z
        .union([z.array(ComhairleMessageReference), z.null()])
        .optional(),
    role: z.string(),
})
    .passthrough();
export const ComhairleSessionMessage = _ComhairleSessionMessage;
const _ComhairleAgentSession = z
    .object({
    agent_id: z.string(),
    configuration: z.unknown(),
    id: z.string(),
    messages: z.array(ComhairleSessionMessage),
})
    .passthrough();
export const ComhairleAgentSession = _ComhairleAgentSession;
const _ConversationRequest = z.object({ question: z.string() }).passthrough();
export const ConversationRequest = _ConversationRequest;
const _Translation2 = z
    .object({
    textContent: TextContentDto,
    textTranslations: z.array(TextTranslationDto),
})
    .passthrough();
export const Translation2 = _Translation2;
const _SectionWithTranslationsDto = z
    .object({
    body: z.string(),
    bodyTranslations: Translation2,
    id: z.string().uuid(),
    position: z.number().int(),
})
    .passthrough();
export const SectionWithTranslationsDto = _SectionWithTranslationsDto;
const _Translation = z
    .object({
    textContent: TextContentDto,
    textTranslations: z.array(TextTranslationDto),
})
    .passthrough();
export const Translation = _Translation;
const _ProposalWithTranslationsDto = z
    .object({
    id: z.string().uuid(),
    sections: z.array(SectionWithTranslationsDto),
    title: z.string(),
    titleTranslations: Translation,
    workflowStepId: z.string().uuid(),
})
    .passthrough();
export const ProposalWithTranslationsDto = _ProposalWithTranslationsDto;
const _LocalizedProposalSectionDto = z
    .object({
    body: z.string(),
    id: z.string().uuid(),
    position: z.number().int(),
})
    .passthrough();
export const LocalizedProposalSectionDto = _LocalizedProposalSectionDto;
const _LocalizedProposalDto = z
    .object({
    id: z.string().uuid(),
    sections: z.array(LocalizedProposalSectionDto),
    title: z.string(),
    workflowStepId: z.string().uuid(),
})
    .passthrough();
export const LocalizedProposalDto = _LocalizedProposalDto;
const _ProposalsListResponse = z.union([
    z.array(ProposalWithTranslationsDto),
    z.array(LocalizedProposalDto),
]);
export const ProposalsListResponse = _ProposalsListResponse;
const _CreateProposalRequest = z
    .object({
    sections: z.array(z.string()).optional().default([]),
    title: z.string(),
    workflow_step_id: z.string().uuid(),
})
    .passthrough();
export const CreateProposalRequest = _CreateProposalRequest;
const _ProposalSectionDto = z
    .object({
    body: z.string().uuid(),
    id: z.string().uuid(),
    position: z.number().int(),
})
    .passthrough();
export const ProposalSectionDto = _ProposalSectionDto;
const _ProposalDto = z
    .object({
    id: z.string().uuid(),
    sections: z.array(ProposalSectionDto),
    title: z.string().uuid(),
    workflowStepId: z.string().uuid(),
})
    .passthrough();
export const ProposalDto = _ProposalDto;
const _CreateSectionRequest = z
    .object({
    body: z.string(),
    position: z.union([z.number(), z.null()]).optional().default(null),
})
    .passthrough();
export const CreateSectionRequest = _CreateSectionRequest;
const _ResponseValue = z.union([z.number(), z.string()]);
export const ResponseValue = _ResponseValue;
const _Response = z
    .object({
    question_id: z.string().uuid(),
    section_id: z.union([z.string(), z.null()]).optional(),
    value: ResponseValue,
})
    .passthrough();
export const Response = _Response;
const _QuestionResponses = z.array(Response);
export const QuestionResponses = _QuestionResponses;
const _ProposalResponseDto = z
    .object({
    id: z.string().uuid(),
    proposalId: z.string().uuid(),
    response: QuestionResponses,
    userId: z.string().uuid(),
})
    .passthrough();
export const ProposalResponseDto = _ProposalResponseDto;
const _CreateResponse = z
    .object({ question_responses: z.array(Response) })
    .passthrough();
export const CreateResponse = _CreateResponse;
const _RankedProposal = z
    .object({
    alignmentRating: z.number(),
    id: z.string().uuid(),
    responses: z.array(ProposalResponseDto),
    sections: z.array(LocalizedProposalSectionDto),
    title: z.string(),
    workflowStepId: z.string().uuid(),
})
    .passthrough();
export const RankedProposal = _RankedProposal;
const _PrioritizationInsightsResponse = z
    .object({ rankedProposals: z.array(RankedProposal) })
    .passthrough();
export const PrioritizationInsightsResponse = _PrioritizationInsightsResponse;
const _ConversationRequest2 = z
    .object({
    history: z.string(),
    question_intent: z.string(),
    starting_question: z.string(),
    workflow_step_id: z.string().uuid(),
})
    .passthrough();
export const ConversationRequest2 = _ConversationRequest2;
const _AnswerStatus = z.enum(["pending", "approved", "declined"]);
export const AnswerStatus = _AnswerStatus;
const _status = z.union([AnswerStatus, z.null()]).optional();
export const status = _status;
const _ThinkingSpaceAnswerDto = z
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
export const ThinkingSpaceAnswerDto = _ThinkingSpaceAnswerDto;
const _CreateAnswerRequest = z
    .object({
    answer: z.string(),
    is_follow_up: z.union([z.boolean(), z.null()]).optional(),
    other_questions: z.union([z.array(z.string()), z.null()]).optional(),
    question: z.string(),
    root_question_id: z.union([z.string(), z.null()]).optional(),
    workflow_step_id: z.string().uuid(),
})
    .passthrough();
export const CreateAnswerRequest = _CreateAnswerRequest;
const _UpdateAnswer = z
    .object({
    answer: z.union([z.string(), z.null()]),
    status: z.union([AnswerStatus, z.null()]),
})
    .partial()
    .passthrough();
export const UpdateAnswer = _UpdateAnswer;
const _GenerateThinkingSpaceSummary = z
    .object({ workflow_step_id: z.string().uuid() })
    .passthrough();
export const GenerateThinkingSpaceSummary = _GenerateThinkingSpaceSummary;
const _ThinkingSpaceSummaryDto = z
    .object({
    aiGeneratedSummary: z.union([z.string(), z.null()]).optional(),
    id: z.string().uuid(),
    isAiGenerated: z.boolean(),
    summary: z.string(),
    userId: z.string().uuid(),
    workflowStepId: z.string().uuid(),
})
    .passthrough();
export const ThinkingSpaceSummaryDto = _ThinkingSpaceSummaryDto;
const _UpdateCreateThinkingSpace = z
    .object({
    summary: z.string(),
    summary_id: z.union([z.string(), z.null()]).optional(),
    workflow_step_id: z.string().uuid(),
})
    .passthrough();
export const UpdateCreateThinkingSpace = _UpdateCreateThinkingSpace;
const _ThinkingSpaceFollowUpQuestionDto = z
    .object({
    followUpQuestions: z.array(z.string()),
    id: z.string().uuid(),
    rootQuestionId: z.string().uuid(),
    userId: z.string().uuid(),
    workflowStepId: z.string().uuid(),
})
    .passthrough();
export const ThinkingSpaceFollowUpQuestionDto = _ThinkingSpaceFollowUpQuestionDto;
const _CreateFollowUpQuestions = z
    .object({
    follow_up_questions: z.array(z.string()),
    root_question_id: z.string().uuid(),
    workflow_step_id: z.string().uuid(),
})
    .passthrough();
export const CreateFollowUpQuestions = _CreateFollowUpQuestions;
const _UpdateFollowUpQuestions = z
    .object({ follow_up_questions: z.array(z.string()) })
    .passthrough();
export const UpdateFollowUpQuestions = _UpdateFollowUpQuestions;
const _AnswersByRoot = z
    .object({
    followUps: z.array(ThinkingSpaceAnswerDto),
    root: ThinkingSpaceAnswerDto,
})
    .passthrough();
export const AnswersByRoot = _AnswersByRoot;
const _ThinkingSpaceUserInsights = z
    .object({
    answers: z.array(AnswersByRoot),
    summary: ThinkingSpaceSummaryDto,
    userId: z.string().uuid(),
})
    .passthrough();
export const ThinkingSpaceUserInsights = _ThinkingSpaceUserInsights;
const _ThinkingSpaceInsightsResponse = z
    .object({ users: z.array(ThinkingSpaceUserInsights) })
    .passthrough();
export const ThinkingSpaceInsightsResponse = _ThinkingSpaceInsightsResponse;
const _CreateConversation = z
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
export const CreateConversation = _CreateConversation;
const _ConversationDto = z
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
export const ConversationDto = _ConversationDto;
const _Translation3 = z
    .object({
    textContent: TextContentDto,
    textTranslations: z.array(TextTranslationDto),
})
    .passthrough();
export const Translation3 = _Translation3;
const _ConversationTranslations = z
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
export const ConversationTranslations = _ConversationTranslations;
const _ConversationWithTranslations = z
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
export const ConversationWithTranslations = _ConversationWithTranslations;
const _ConversationResponse = z.union([
    ConversationWithTranslations,
    LocalizedConversationDto,
]);
export const ConversationResponse = _ConversationResponse;
const _PartialConversation = z
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
export const PartialConversation = _PartialConversation;
const _OrganizationWithPermissionDto = z
    .object({ id: z.string().uuid(), name: z.string(), roleName: z.string() })
    .passthrough();
export const OrganizationWithPermissionDto = _OrganizationWithPermissionDto;
const _CohostInfo = z
    .object({ organization_id: z.string().uuid() })
    .passthrough();
export const CohostInfo = _CohostInfo;
const _SendNotificationRequest = z
    .object({
    content: z.string(),
    delivery_method: z.union([DeliveryMethod, z.null()]).optional(),
    html_content: z.union([z.string(), z.null()]).optional(),
    notification_type: z.union([NotificationType, z.null()]).optional(),
    test_email_recipient: z.union([z.string(), z.null()]).optional(),
    title: z.string(),
})
    .passthrough();
export const SendNotificationRequest = _SendNotificationRequest;
const _SendEmailNotificationResponse = z
    .object({
    failedRecipients: z.array(z.string()).optional().default([]),
    message: z.string(),
    notificationId: z.string().uuid(),
    participantsNotified: z.number().int(),
})
    .passthrough();
export const SendEmailNotificationResponse = _SendEmailNotificationResponse;
const _NotificationRecipientsResponse = z
    .object({
    emailRecipientCount: z.number().int(),
    emailRecipients: z.array(z.string()),
    participantCount: z.number().int(),
})
    .passthrough();
export const NotificationRecipientsResponse = _NotificationRecipientsResponse;
const _RegisterEmailRequest = z
    .object({
    email: z.string(),
    receive_similar_conversation_updates_by_email: z.boolean(),
    receive_updates_by_email: z.boolean(),
})
    .passthrough();
export const RegisterEmailRequest = _RegisterEmailRequest;
const _RegisterEmailResponse = z
    .object({
    conversationId: z.string().uuid(),
    email: z.string(),
    id: z.string().uuid(),
    message: z.string(),
})
    .passthrough();
export const RegisterEmailResponse = _RegisterEmailResponse;
const _WorkflowDto = z
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
export const WorkflowDto = _WorkflowDto;
const _CreateWorkflow = z
    .object({
    auto_login: z.boolean(),
    description: z.string(),
    is_active: z.boolean(),
    is_public: z.boolean(),
    name: z.string(),
    region_id: z.union([z.string(), z.null()]).optional(),
})
    .passthrough();
export const CreateWorkflow = _CreateWorkflow;
const _PartialWorkflow = z
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
export const PartialWorkflow = _PartialWorkflow;
const _ActivationRule = z.literal("manual");
export const ActivationRule = _ActivationRule;
const _LearnPage = z
    .object({ text_content_id: z.string().uuid() })
    .passthrough();
export const LearnPage = _LearnPage;
const _LocalizedPage = z
    .object({ content: z.string(), type: z.literal("markdown") })
    .passthrough();
export const LocalizedPage = _LocalizedPage;
const _LearnPageEntry = z.union([LearnPage, z.array(LocalizedPage)]);
export const LearnPageEntry = _LearnPageEntry;
const _Category = z
    .object({ label: z.string().uuid(), value: z.number() })
    .passthrough();
export const Category = _Category;
const _QuestionType = z.union([
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
export const QuestionType = _QuestionType;
const _Question = z
    .object({
    id: z.string().uuid(),
    text: z.string().uuid(),
    type: QuestionType,
})
    .passthrough();
export const Question = _Question;
const _ThinkingSpaceQuestion = z
    .object({
    id: z.string().uuid(),
    intent: z.string().uuid(),
    text: z.string().uuid(),
})
    .passthrough();
export const ThinkingSpaceQuestion = _ThinkingSpaceQuestion;
const _ToolConfig = z.union([
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
export const ToolConfig = _ToolConfig;
const _WorkflowStep = z
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
export const WorkflowStep = _WorkflowStep;
const _DailySignupStats = z
    .object({
    day: z.string().datetime({ offset: true }),
    users: z.number().int(),
})
    .passthrough();
export const DailySignupStats = _DailySignupStats;
const _WorkflowStepStats = z
    .object({
    completed: z.number().int(),
    id: z.string().uuid(),
    started: z.number().int(),
})
    .passthrough();
export const WorkflowStepStats = _WorkflowStepStats;
const _WorkflowStats = z
    .object({
    signupStats: z.array(DailySignupStats),
    stepStats: z.array(WorkflowStepStats),
    totalUsers: z.number().int(),
})
    .passthrough();
export const WorkflowStats = _WorkflowStats;
const _DemographicCount = z
    .object({
    count: z.number().int(),
    displayName: z.string(),
    value: z.string(),
})
    .passthrough();
export const DemographicCount = _DemographicCount;
const _DemographicReport = z
    .object({
    categories: z.record(z.array(DemographicCount)),
    totalParticipants: z.number().int(),
})
    .passthrough();
export const DemographicReport = _DemographicReport;
const _UserParticipation = z
    .object({
    created_at: z.string().datetime({ offset: true }),
    id: z.string().uuid(),
    updated_at: z.string().datetime({ offset: true }),
    user_id: z.string().uuid(),
    workflow_id: z.string().uuid(),
})
    .passthrough();
export const UserParticipation = _UserParticipation;
const _UserParticipationDto = z
    .object({
    created_at: z.string().datetime({ offset: true }),
    id: z.string().uuid(),
    sealed: z.boolean(),
    updated_at: z.string().datetime({ offset: true }),
    user_id: z.string().uuid(),
    workflow_id: z.string().uuid(),
})
    .passthrough();
export const UserParticipationDto = _UserParticipationDto;
const _TranslationDto = z
    .object({
    textContent: TextContentDto,
    textTranslations: z.array(TextTranslationDto),
})
    .passthrough();
export const TranslationDto = _TranslationDto;
const _JsonFieldWithTranslations = z
    .object({ localized: z.string(), translations: TranslationDto })
    .passthrough();
export const JsonFieldWithTranslations = _JsonFieldWithTranslations;
const _CategoryWithTranslations = z
    .object({ label: JsonFieldWithTranslations, value: z.number() })
    .passthrough();
export const CategoryWithTranslations = _CategoryWithTranslations;
const _QuestionTypeWithTranslations = z.union([
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
export const QuestionTypeWithTranslations = _QuestionTypeWithTranslations;
const _QuestionWithTranslations = z
    .object({
    id: z.string().uuid(),
    text: JsonFieldWithTranslations,
    type: QuestionTypeWithTranslations,
})
    .passthrough();
export const QuestionWithTranslations = _QuestionWithTranslations;
const _ThinkingSpaceQuestionWithTranslations = z
    .object({
    id: z.string().uuid(),
    intent: JsonFieldWithTranslations,
    text: JsonFieldWithTranslations,
})
    .passthrough();
export const ThinkingSpaceQuestionWithTranslations = _ThinkingSpaceQuestionWithTranslations;
const _ToolConfigWithTranslations = z.union([
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
export const ToolConfigWithTranslations = _ToolConfigWithTranslations;
const _Translation4 = z
    .object({
    textContent: TextContentDto,
    textTranslations: z.array(TextTranslationDto),
})
    .passthrough();
export const Translation4 = _Translation4;
const _WorkflowStepTranslations = z
    .object({ description: Translation4, name: Translation4 })
    .passthrough();
export const WorkflowStepTranslations = _WorkflowStepTranslations;
const _WorkflowStepWithTranslationsDto = z
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
export const WorkflowStepWithTranslationsDto = _WorkflowStepWithTranslationsDto;
const _LocalizedCategory = z
    .object({ label: z.string(), value: z.number() })
    .passthrough();
export const LocalizedCategory = _LocalizedCategory;
const _LocalizedQuestionType = z.union([
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
export const LocalizedQuestionType = _LocalizedQuestionType;
const _LocalizedQuestion = z
    .object({
    id: z.string().uuid(),
    text: z.string(),
    type: LocalizedQuestionType,
})
    .passthrough();
export const LocalizedQuestion = _LocalizedQuestion;
const _LocalizedThinkingSpaceQuestion = z
    .object({ id: z.string().uuid(), intent: z.string(), text: z.string() })
    .passthrough();
export const LocalizedThinkingSpaceQuestion = _LocalizedThinkingSpaceQuestion;
const _LocalizedToolConfig = z.union([
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
export const LocalizedToolConfig = _LocalizedToolConfig;
const _ProgressStatus = z.enum(["not_started", "in_progress", "done"]);
export const ProgressStatus = _ProgressStatus;
const _LocalizedWorkflowStepWithProgressDto = z
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
export const LocalizedWorkflowStepWithProgressDto = _LocalizedWorkflowStepWithProgressDto;
const _LocalizedWorkflowStepDto = z
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
export const LocalizedWorkflowStepDto = _LocalizedWorkflowStepDto;
const _WorkflowStepsListResponse = z.union([
    z.array(WorkflowStepWithTranslationsDto),
    z.array(LocalizedWorkflowStepWithProgressDto),
    z.array(LocalizedWorkflowStepDto),
]);
export const WorkflowStepsListResponse = _WorkflowStepsListResponse;
const _SetupCategory = z
    .object({ label: z.string(), value: z.number() })
    .passthrough();
export const SetupCategory = _SetupCategory;
const _SetupQuestionType = z.union([
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
export const SetupQuestionType = _SetupQuestionType;
const _SetupQuestion = z
    .object({ text: z.string(), type: SetupQuestionType })
    .passthrough();
export const SetupQuestion = _SetupQuestion;
const _ThinkingSpaceSetupQuestion = z
    .object({ intent: z.string(), text: z.string() })
    .passthrough();
export const ThinkingSpaceSetupQuestion = _ThinkingSpaceSetupQuestion;
const _ToolSetup = z.union([
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
export const ToolSetup = _ToolSetup;
const _CreateWorkflowStep = z
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
export const CreateWorkflowStep = _CreateWorkflowStep;
const _WorkflowStepDto = z
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
export const WorkflowStepDto = _WorkflowStepDto;
const _PartialWorkflowStep = z
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
export const PartialWorkflowStep = _PartialWorkflowStep;
const _UserProgressDto = z
    .object({
    id: z.string().uuid(),
    permissionToShareWithOrganizers: z.boolean(),
    status: ProgressStatus,
    userId: z.string().uuid(),
    workflowStepId: z.string().uuid(),
})
    .passthrough();
export const UserProgressDto = _UserProgressDto;
const _UpdateUserProgress = z
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
export const UpdateUserProgress = _UpdateUserProgress;
const _RecruitmentTargetDto = z
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
export const RecruitmentTargetDto = _RecruitmentTargetDto;
const _CreateRecruitmentTarget = z
    .object({
    bucket: z.string(),
    metric: z.string(),
    target_count: z.number().int(),
})
    .passthrough();
export const CreateRecruitmentTarget = _CreateRecruitmentTarget;
const _PartialRecruitmentTarget = z
    .object({
    bucket: z.union([z.string(), z.null()]),
    metric: z.union([z.string(), z.null()]),
    target_count: z.union([z.number(), z.null()]),
})
    .partial()
    .passthrough();
export const PartialRecruitmentTarget = _PartialRecruitmentTarget;
const _InviteType = z.union([
    z.object({ email: z.string() }),
    z.object({ user: z.string().uuid() }),
    z.literal("singleuse"),
    z.literal("open"),
]);
export const InviteType = _InviteType;
const _LoginBehaviour = z.union([
    z.literal("manual"),
    z.literal("auto_create_guest"),
]);
export const LoginBehaviour = _LoginBehaviour;
const _InviteStatus = z.union([
    z.literal("pending"),
    z.literal("open"),
    z.literal("accepted"),
    z.literal("rejected"),
    z.literal("expired"),
]);
export const InviteStatus = _InviteStatus;
const _InviteDto = z
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
export const InviteDto = _InviteDto;
const _CreateInviteDTO = z
    .object({
    event_id: z.union([z.string(), z.null()]).optional(),
    expires_at: z.union([z.string(), z.null()]).optional(),
    invite_type: InviteType,
    label: z.union([z.string(), z.null()]).optional(),
    login_behaviour: LoginBehaviour.optional(),
})
    .passthrough();
export const CreateInviteDTO = _CreateInviteDTO;
const _PartialInvite = z
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
export const PartialInvite = _PartialInvite;
const _DailyResponseStats = z
    .object({
    accept: z.number().int(),
    day: z.string().datetime({ offset: true }),
    reject: z.number().int(),
})
    .passthrough();
export const DailyResponseStats = _DailyResponseStats;
const _PolisReport = z.null();
export const PolisReport = _PolisReport;
const _HeyFormReport = z.null();
export const HeyFormReport = _HeyFormReport;
const _LearnReport = z.null();
export const LearnReport = _LearnReport;
const _StoriesReport = z.null();
export const StoriesReport = _StoriesReport;
const _ElicitationBotReport = z.null();
export const ElicitationBotReport = _ElicitationBotReport;
const _PrioritizationReport = z.null();
export const PrioritizationReport = _PrioritizationReport;
const _ThinkingSpaceReport = z.null();
export const ThinkingSpaceReport = _ThinkingSpaceReport;
const _ReportConfig = z.union([
    z.object({ Polis: PolisReport }),
    z.object({ HeyForm: HeyFormReport }),
    z.object({ Learn: LearnReport }),
    z.object({ Stories: StoriesReport }),
    z.object({ ElicitationBot: ElicitationBotReport }),
    z.object({ Prioritization: PrioritizationReport }),
    z.object({ ThinkingSpace: ThinkingSpaceReport }),
]);
export const ReportConfig = _ReportConfig;
const _ReportSectionConfig = z
    .object({
    ai_generated: z.boolean(),
    config: ReportConfig,
    verified: z.boolean(),
    workflow_step_id: z.string().uuid(),
})
    .passthrough();
export const ReportSectionConfig = _ReportSectionConfig;
const _ReportSectionConfigs = z.array(ReportSectionConfig);
export const ReportSectionConfigs = _ReportSectionConfigs;
const _Translation5 = z
    .object({
    textContent: TextContentDto,
    textTranslations: z.array(TextTranslationDto),
})
    .passthrough();
export const Translation5 = _Translation5;
const _ReportTranslations = z.object({ summary: Translation5 }).passthrough();
export const ReportTranslations = _ReportTranslations;
const _ReportWithTranslations = z
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
export const ReportWithTranslations = _ReportWithTranslations;
const _LocalizedReportDto = z
    .object({
    conversationId: z.string().uuid(),
    createdAt: z.string().datetime({ offset: true }),
    id: z.string().uuid(),
    isPublic: z.boolean(),
    sectionConfigs: ReportSectionConfigs,
    summary: z.string(),
})
    .passthrough();
export const LocalizedReportDto = _LocalizedReportDto;
const _FullReportDto = z.union([ReportWithTranslations, LocalizedReportDto]);
export const FullReportDto = _FullReportDto;
const _PartialReport = z
    .object({
    conversation_id: z.union([z.string(), z.null()]),
    is_public: z.union([z.boolean(), z.null()]),
    section_configs: z.union([ReportSectionConfigs, z.null()]),
})
    .partial()
    .passthrough();
export const PartialReport = _PartialReport;
const _ReportDto = z
    .object({
    conversationId: z.string().uuid(),
    createdAt: z.string().datetime({ offset: true }),
    id: z.string().uuid(),
    isPublic: z.boolean(),
    sectionConfigs: ReportSectionConfigs,
    summary: z.string().uuid(),
})
    .passthrough();
export const ReportDto = _ReportDto;
const _ReportImpactDto = z
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
export const ReportImpactDto = _ReportImpactDto;
const _PartialReportImpact = z
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
export const PartialReportImpact = _PartialReportImpact;
const _CreateImpactDTO = z
    .object({ details: z.string(), kind: z.string(), title: z.string() })
    .passthrough();
export const CreateImpactDTO = _CreateImpactDTO;
const _FeedbackDto = z
    .object({
    content: z.string(),
    conversationId: z.string().uuid(),
    id: z.string().uuid(),
})
    .passthrough();
export const FeedbackDto = _FeedbackDto;
const _CreateFeedbackDTO = z.object({ content: z.string() }).passthrough();
export const CreateFeedbackDTO = _CreateFeedbackDTO;
const _PartialFeedback = z
    .object({ content: z.union([z.string(), z.null()]) })
    .partial()
    .passthrough();
export const PartialFeedback = _PartialFeedback;
const _ComhairleLlm = z
    .object({ model_name: z.union([z.string(), z.null()]) })
    .partial()
    .passthrough();
export const ComhairleLlm = _ComhairleLlm;
const _ComhairlePrompt = z
    .object({
    cross_languages: z.union([z.array(z.string()), z.null()]),
    empty_response: z.union([z.string(), z.null()]),
    llm_prompt: z.union([z.string(), z.null()]),
    opener: z.union([z.string(), z.null()]),
})
    .partial()
    .passthrough();
export const ComhairlePrompt = _ComhairlePrompt;
const _ComhairleChat = z
    .object({
    id: z.string(),
    knowledge_base_ids: z.array(z.string()),
    llm_model: z.union([ComhairleLlm, z.null()]).optional(),
    name: z.string(),
    prompt: z.union([ComhairlePrompt, z.null()]).optional(),
})
    .passthrough();
export const ComhairleChat = _ComhairleChat;
const _UpdateChatRequest = z
    .object({
    knowledge_base_ids: z.union([z.array(z.string()), z.null()]),
    llm_model: z.union([ComhairleLlm, z.null()]),
    name: z.union([z.string(), z.null()]),
    prompt: z.union([ComhairlePrompt, z.null()]),
})
    .partial()
    .passthrough();
export const UpdateChatRequest = _UpdateChatRequest;
const _ComhairleChatSession = z
    .object({
    chat_id: z.string(),
    id: z.string(),
    messages: z.array(ComhairleSessionMessage),
    name: z.union([z.string(), z.null()]).optional(),
})
    .passthrough();
export const ComhairleChatSession = _ComhairleChatSession;
const _ChatConversationRequest = z
    .object({ question: z.string() })
    .passthrough();
export const ChatConversationRequest = _ChatConversationRequest;
const _page_size = z.union([z.number(), z.null()]).optional().default(400);
export const page_size = _page_size;
const _ComhairleDocument = z
    .object({
    id: z.string(),
    name: z.string(),
    parse_progress: z.number(),
    parse_status: z.string(),
    size: z.number().int(),
})
    .passthrough();
export const ComhairleDocument = _ComhairleDocument;
const _UploadFileResponse = z
    .object({
    document: ComhairleDocument,
    job_id: z.string().uuid(),
    message: z.string(),
})
    .passthrough();
export const UploadFileResponse = _UploadFileResponse;
const _SyncLearningContentResponse = z
    .object({
    document: z.union([ComhairleDocument, z.null()]).optional(),
    job_id: z.union([z.string(), z.null()]).optional(),
    message: z.string(),
})
    .passthrough();
export const SyncLearningContentResponse = _SyncLearningContentResponse;
const _LearnContentPage = z
    .object({ content: z.string(), is_rich: z.boolean() })
    .passthrough();
export const LearnContentPage = _LearnContentPage;
const _LearnContentSection = z
    .object({ heading: z.string(), pages: z.array(LearnContentPage) })
    .passthrough();
export const LearnContentSection = _LearnContentSection;
const _LearnContentResponse = z
    .object({ sections: z.array(LearnContentSection) })
    .passthrough();
export const LearnContentResponse = _LearnContentResponse;
const _Order = z.enum(["asc", "desc"]);
export const Order = _Order;
const _created_at = z.union([Order, z.null()]).optional();
export const created_at = _created_at;
const _CapacityStatus = z.enum(["full", "available"]);
export const CapacityStatus = _CapacityStatus;
const _capacity_status = z.union([CapacityStatus, z.null()]).optional();
export const capacity_status = _capacity_status;
const _TimeStatus = z.enum(["past", "future"]);
export const TimeStatus = _TimeStatus;
const _time_status = z.union([TimeStatus, z.null()]).optional();
export const time_status = _time_status;
const _BasicEventAgendaItem = z
    .object({
    description: z.string(),
    estimated_time: z.number().int().gte(0),
    title: z.string(),
})
    .passthrough();
export const BasicEventAgendaItem = _BasicEventAgendaItem;
const _BreakoutRoomAgendaItem = z
    .object({
    estimated_time: z.number().int().gte(0),
    instructions: z.string(),
    max_per_room: z.union([z.number(), z.null()]).optional(),
    prompt: z.string(),
    time_limit: z.union([z.number(), z.null()]).optional(),
})
    .passthrough();
export const BreakoutRoomAgendaItem = _BreakoutRoomAgendaItem;
const _EventAgendaItem = z.union([
    z.object({ Basic: BasicEventAgendaItem }),
    z.object({ BreakoutRoom: BreakoutRoomAgendaItem }),
]);
export const EventAgendaItem = _EventAgendaItem;
const _EventFormat = z.enum(["online", "in_person"]);
export const EventFormat = _EventFormat;
const _EventLocation = z
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
export const EventLocation = _EventLocation;
const _LocalizedEventDto = z
    .object({
    agenda: z.array(EventAgendaItem),
    capacity: z.union([z.number(), z.null()]).optional(),
    conversationId: z.string().uuid(),
    createdAt: z.string().datetime({ offset: true }),
    currentAttendance: z.union([z.number(), z.null()]).optional(),
    customEventLink: z.union([z.string(), z.null()]).optional(),
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
export const LocalizedEventDto = _LocalizedEventDto;
const _PaginatedResults_for_LocalizedEventDto = z
    .object({ records: z.array(LocalizedEventDto), total: z.number().int() })
    .passthrough();
export const PaginatedResults_for_LocalizedEventDto = _PaginatedResults_for_LocalizedEventDto;
const _CreateEvent = z
    .object({
    agenda: z.union([z.array(EventAgendaItem), z.null()]).optional(),
    capacity: z.union([z.number(), z.null()]).optional(),
    custom_event_link: z.union([z.string(), z.null()]).optional(),
    default_time_zone: z.union([z.string(), z.null()]).optional(),
    description: z.string(),
    end_time: z.string().datetime({ offset: true }),
    location: z.union([EventLocation, z.null()]).optional(),
    name: z.string(),
    signup_mode: z.string(),
    start_time: z.string().datetime({ offset: true }),
})
    .passthrough();
export const CreateEvent = _CreateEvent;
const _EventDto = z
    .object({
    agenda: z.array(EventAgendaItem),
    capacity: z.union([z.number(), z.null()]).optional(),
    conversationId: z.string().uuid(),
    createdAt: z.string().datetime({ offset: true }),
    customEventLink: z.union([z.string(), z.null()]).optional(),
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
export const EventDto = _EventDto;
const _BreakoutSeat = z
    .object({
    invite_id: z.union([z.string(), z.null()]),
    is_moderator: z.boolean().default(false),
    user_id: z.union([z.string(), z.null()]),
})
    .partial()
    .passthrough();
export const BreakoutSeat = _BreakoutSeat;
const _BreakoutPlanRoom = z
    .object({ seats: z.array(BreakoutSeat).default([]) })
    .partial()
    .passthrough();
export const BreakoutPlanRoom = _BreakoutPlanRoom;
const _Translation6 = z
    .object({
    textContent: TextContentDto,
    textTranslations: z.array(TextTranslationDto),
})
    .passthrough();
export const Translation6 = _Translation6;
const _EventTranslations = z
    .object({ description: Translation6, name: Translation6 })
    .passthrough();
export const EventTranslations = _EventTranslations;
const _EventWithTranslations = z
    .object({
    agenda: z.array(EventAgendaItem),
    breakoutPlan: z.array(BreakoutPlanRoom),
    capacity: z.union([z.number(), z.null()]).optional(),
    conversationId: z.string().uuid(),
    createdAt: z.string().datetime({ offset: true }),
    customEventLink: z.union([z.string(), z.null()]).optional(),
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
export const EventWithTranslations = _EventWithTranslations;
const _EventResponse = z.union([LocalizedEventDto, EventWithTranslations]);
export const EventResponse = _EventResponse;
const _PartialEvent = z
    .object({
    agenda: z.union([z.array(EventAgendaItem), z.null()]).default(null),
    capacity: z.union([z.number(), z.null()]),
    custom_event_link: z.union([z.string(), z.null()]),
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
export const PartialEvent = _PartialEvent;
const _JwtResponse = z
    .object({ isModerator: z.boolean(), jwt: z.string() })
    .passthrough();
export const JwtResponse = _JwtResponse;
const _BreakoutSeatDto = z
    .object({
    inviteId: z.union([z.string(), z.null()]).optional(),
    isModerator: z.boolean(),
    label: z.string(),
    pending: z.boolean(),
    userId: z.union([z.string(), z.null()]).optional(),
})
    .passthrough();
export const BreakoutSeatDto = _BreakoutSeatDto;
const _BreakoutRoomDto = z
    .object({ seats: z.array(BreakoutSeatDto) })
    .passthrough();
export const BreakoutRoomDto = _BreakoutRoomDto;
const _BreakoutPlanDto = z
    .object({ rooms: z.array(BreakoutRoomDto) })
    .passthrough();
export const BreakoutPlanDto = _BreakoutPlanDto;
const _SaveBreakoutPlanRequest = z
    .object({ rooms: z.array(BreakoutPlanRoom) })
    .passthrough();
export const SaveBreakoutPlanRequest = _SaveBreakoutPlanRequest;
const _EventAttendanceEtx = z
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
export const EventAttendanceEtx = _EventAttendanceEtx;
const _PaginatedResults_for_EventAttendanceEtx = z
    .object({ records: z.array(EventAttendanceEtx), total: z.number().int() })
    .passthrough();
export const PaginatedResults_for_EventAttendanceEtx = _PaginatedResults_for_EventAttendanceEtx;
const _CreateEventAttendanceRequest = z
    .object({
    role: z.string(),
    user_email: z.union([z.string(), z.null()]).optional(),
})
    .passthrough();
export const CreateEventAttendanceRequest = _CreateEventAttendanceRequest;
const _EventAttendanceDto = z
    .object({
    createdAt: z.string().datetime({ offset: true }),
    eventId: z.string().uuid(),
    id: z.string().uuid(),
    role: z.string(),
    userId: z.string().uuid(),
})
    .passthrough();
export const EventAttendanceDto = _EventAttendanceDto;
const _UpdateEventAttendanceRequest = z
    .object({ role: z.union([z.string(), z.null()]) })
    .partial()
    .passthrough();
export const UpdateEventAttendanceRequest = _UpdateEventAttendanceRequest;
const _CreateFacilitatorRequest = z.object({ email: z.string() }).passthrough();
export const CreateFacilitatorRequest = _CreateFacilitatorRequest;
const _AudioFormat = z.enum([
    "wav",
    "mp3",
    "m4a",
    "mp4",
    "ogg",
    "flac",
    "webm",
]);
export const AudioFormat = _AudioFormat;
const _AudioRecordingStatus = z.union([
    z.literal("awaiting_upload"),
    z.literal("transcribing"),
    z.literal("categorizing"),
    z.literal("complete"),
    z.literal("transcription_failed"),
    z.literal("categorization_failed"),
]);
export const AudioRecordingStatus = _AudioRecordingStatus;
const _AudioRecordingDto = z
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
export const AudioRecordingDto = _AudioRecordingDto;
const _CreateRecordingRequest = z
    .object({ fileExtension: AudioFormat, name: z.string() })
    .passthrough();
export const CreateRecordingRequest = _CreateRecordingRequest;
const _CreateRecordingResponse = z
    .object({ recording: AudioRecordingDto, uploadUrl: z.string() })
    .passthrough();
export const CreateRecordingResponse = _CreateRecordingResponse;
const _RecordingDownloadUrls = z
    .object({
    recordingUrl: z.string(),
    reportUrl: z.string(),
    transcriptUrl: z.string(),
})
    .passthrough();
export const RecordingDownloadUrls = _RecordingDownloadUrls;
const _RecordingDetailResponse = z
    .object({ downloads: RecordingDownloadUrls, recording: AudioRecordingDto })
    .passthrough();
export const RecordingDetailResponse = _RecordingDetailResponse;
const _DeleteRecordingResponse = z
    .object({ recording: AudioRecordingDto })
    .passthrough();
export const DeleteRecordingResponse = _DeleteRecordingResponse;
const _ProcessRecordingResponse = z
    .object({ jobId: z.string().uuid(), message: z.string() })
    .passthrough();
export const ProcessRecordingResponse = _ProcessRecordingResponse;
const _SubmitReportResponse = z
    .object({ success: z.boolean(), url: z.string() })
    .passthrough();
export const SubmitReportResponse = _SubmitReportResponse;
const _WebSocketStats = z
    .object({
    connected_users: z.array(z.string().uuid()),
    total_connections: z.number().int().gte(0),
})
    .passthrough();
export const WebSocketStats = _WebSocketStats;
const _BroadcastMessage = z
    .object({
    authenticated_only: z.union([z.boolean(), z.null()]).optional(),
    message: z.string(),
})
    .passthrough();
export const BroadcastMessage = _BroadcastMessage;
const _BroadcastResponse = z
    .object({ message: z.string(), sent_to: z.number().int().gte(0) })
    .passthrough();
export const BroadcastResponse = _BroadcastResponse;
const _SendToUserMessage = z
    .object({ message: z.string(), user_id: z.string().uuid() })
    .passthrough();
export const SendToUserMessage = _SendToUserMessage;
const _PaginatedResults_for_LocalizedOrganizationDto = z
    .object({
    records: z.array(LocalizedOrganizationDto),
    total: z.number().int(),
})
    .passthrough();
export const PaginatedResults_for_LocalizedOrganizationDto = _PaginatedResults_for_LocalizedOrganizationDto;
const _CreateOrganization = z
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
export const CreateOrganization = _CreateOrganization;
const _OrganizationDto = z
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
export const OrganizationDto = _OrganizationDto;
const _UpdateOrganizationBody = z
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
export const UpdateOrganizationBody = _UpdateOrganizationBody;
const _OrganizationTeamRole = z.enum(["member", "admin"]);
export const OrganizationTeamRole = _OrganizationTeamRole;
const _OrganizationTeamUserDto = z
    .object({
    email: z.union([z.string(), z.null()]).optional(),
    id: z.string().uuid(),
    role: OrganizationTeamRole,
    username: z.union([z.string(), z.null()]).optional(),
})
    .passthrough();
export const OrganizationTeamUserDto = _OrganizationTeamUserDto;
const _OrganizationTeamResponseDto = z
    .object({ members: z.array(OrganizationTeamUserDto) })
    .passthrough();
export const OrganizationTeamResponseDto = _OrganizationTeamResponseDto;
const _UpsertOrganizationUserBody = z
    .object({
    allow_create_user: z.union([z.boolean(), z.null()]).optional(),
    email: z.string(),
    role: z.union([OrganizationTeamRole, z.null()]).optional(),
})
    .passthrough();
export const UpsertOrganizationUserBody = _UpsertOrganizationUserBody;
const _UpsertOrganizationUserResponseDto = z
    .object({
    createdAccount: z.boolean(),
    emailed: z.boolean(),
    user: OrganizationTeamUserDto,
})
    .passthrough();
export const UpsertOrganizationUserResponseDto = _UpsertOrganizationUserResponseDto;
const _UpdateOrganizationMemberRoleBody = z
    .object({ role: OrganizationTeamRole })
    .passthrough();
export const UpdateOrganizationMemberRoleBody = _UpdateOrganizationMemberRoleBody;
const _RegionType = z.enum(["custom", "official"]);
export const RegionType = _RegionType;
const _LocalizedRegionDto = z
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
export const LocalizedRegionDto = _LocalizedRegionDto;
const _PaginatedResults_for_LocalizedRegionDto = z
    .object({ records: z.array(LocalizedRegionDto), total: z.number().int() })
    .passthrough();
export const PaginatedResults_for_LocalizedRegionDto = _PaginatedResults_for_LocalizedRegionDto;
const _CreateRegion = z
    .object({
    description: z.string(),
    name: z.string(),
    official_id: z.union([z.string(), z.null()]).optional(),
    region_type: RegionType,
})
    .passthrough();
export const CreateRegion = _CreateRegion;
const _RegionDto = z
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
export const RegionDto = _RegionDto;
const _PartialRegion = z
    .object({
    metadata: z.unknown(),
    official_id: z.union([z.string(), z.null()]),
    region_type: z.union([RegionType, z.null()]),
})
    .partial()
    .passthrough();
export const PartialRegion = _PartialRegion;
const _RegionAreaLinksDto = z
    .object({
    area_ids: z.array(z.string().uuid()),
    region_id: z.string().uuid(),
})
    .passthrough();
export const RegionAreaLinksDto = _RegionAreaLinksDto;
const _RegionAreaLinksRequestDto = z
    .object({ area_ids: z.array(z.string().uuid()) })
    .passthrough();
export const RegionAreaLinksRequestDto = _RegionAreaLinksRequestDto;
const _RegionAreaDto = z
    .object({
    createdAt: z.string().datetime({ offset: true }),
    id: z.string().uuid(),
    zipPrefix: z.string(),
})
    .passthrough();
export const RegionAreaDto = _RegionAreaDto;
const _CreateRegionArea = z.object({ zip_prefix: z.string() }).passthrough();
export const CreateRegionArea = _CreateRegionArea;
const _PartialRegionArea = z
    .object({ zip_prefix: z.union([z.string(), z.null()]) })
    .partial()
    .passthrough();
export const PartialRegionArea = _PartialRegionArea;
const _MediaContentType = z.enum([
    "image/jpeg",
    "image/png",
    "image/gif",
    "image/webp",
    "video/mp4",
    "video/mpeg",
    "video/webm",
    "audio/mpeg",
    "audio/mp4",
    "audio/webm",
    "audio/wav",
    "audio/ogg",
]);
export const MediaContentType = _MediaContentType;
const _content_type = z.union([MediaContentType, z.null()]).optional();
export const content_type = _content_type;
const _MediaDto = z
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
export const MediaDto = _MediaDto;
const _PaginatedResults_for_MediaDto = z
    .object({ records: z.array(MediaDto), total: z.number().int() })
    .passthrough();
export const PaginatedResults_for_MediaDto = _PaginatedResults_for_MediaDto;
const _MediaEditableFields = z
    .object({
    alt: z.union([z.string(), z.null()]),
    name: z.union([z.string(), z.null()]),
})
    .partial()
    .passthrough();
export const MediaEditableFields = _MediaEditableFields;
const _Job = z
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
export const Job = _Job;
const _PaginatedResults_for_Job = z
    .object({ records: z.array(Job), total: z.number().int() })
    .passthrough();
export const PaginatedResults_for_Job = _PaginatedResults_for_Job;
const _CreateJob = z
    .object({
    progress: z.union([z.number(), z.null()]),
    step: z.union([z.string(), z.null()]),
})
    .partial()
    .passthrough();
export const CreateJob = _CreateJob;
const _ComhairleServices = z
    .object({ botService: z.boolean(), translationService: z.boolean() })
    .passthrough();
export const ComhairleServices = _ComhairleServices;
const _CreateApiKeyRequest = z
    .object({ name: z.string(), prefix: z.string() })
    .passthrough();
export const CreateApiKeyRequest = _CreateApiKeyRequest;
const _CreateResponse2 = z.object({ key: z.string() }).passthrough();
export const CreateResponse2 = _CreateResponse2;
const _EmailType = z.enum([
    "conversation_invite",
    "event_registration_invite",
    "event_registration_confirmation",
    "event_reminder",
]);
export const EmailType = _EmailType;
const _email_type = z.union([EmailType, z.null()]).optional();
export const email_type = _email_type;
const _EmailTemplateSlots = z.union([
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
export const EmailTemplateSlots = _EmailTemplateSlots;
const _EmailTemplateConfigDto = z
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
export const EmailTemplateConfigDto = _EmailTemplateConfigDto;
const _CreateEmailTemplateConfig = z
    .object({
    slots: EmailTemplateSlots,
    subject: z.union([z.string(), z.null()]).optional(),
})
    .passthrough();
export const CreateEmailTemplateConfig = _CreateEmailTemplateConfig;
const _UpdateEmailTemplateConfig = z
    .object({
    slots: z.union([EmailTemplateSlots, z.null()]),
    subject: z.union([z.string(), z.null()]),
})
    .partial()
    .passthrough();
export const UpdateEmailTemplateConfig = _UpdateEmailTemplateConfig;
const _ContentType = z.enum(["plain_text", "rich_text"]);
export const ContentType = _ContentType;
const _SlotSchemaDefinition = z
    .object({
    content_type: ContentType,
    default_content: z.string(),
    hint: z.string(),
    key: z.string(),
    label: z.string(),
})
    .passthrough();
export const SlotSchemaDefinition = _SlotSchemaDefinition;
const _EmailTypeSchema = z
    .object({
    default_subject: z.string(),
    email_type: EmailType,
    slots: z.array(SlotSchemaDefinition),
    template: z.string(),
    variables: z.array(z.string()),
})
    .passthrough();
export const EmailTypeSchema = _EmailTypeSchema;
const _PreviewEmailTemplateConfigRequest = z
    .object({ slots: EmailTemplateSlots })
    .passthrough();
export const PreviewEmailTemplateConfigRequest = _PreviewEmailTemplateConfigRequest;
const _PreviewEmailTemplateConfigResponse = z
    .object({ html: z.string() })
    .passthrough();
export const PreviewEmailTemplateConfigResponse = _PreviewEmailTemplateConfigResponse;
const _ResourcePermission = z
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
export const ResourcePermission = _ResourcePermission;
const _PaginatedResults_for_ResourcePermission = z
    .object({ records: z.array(ResourcePermission), total: z.number().int() })
    .passthrough();
export const PaginatedResults_for_ResourcePermission = _PaginatedResults_for_ResourcePermission;
const _GrantPermissionBody = z
    .object({
    grant_reason: z.string(),
    organization_id: z.union([z.string(), z.null()]).optional(),
    role_name: z.string(),
    user_email: z.union([z.string(), z.null()]).optional(),
    user_id: z.union([z.string(), z.null()]).optional(),
})
    .passthrough();
export const GrantPermissionBody = _GrantPermissionBody;
const _UserWithPermissionDto = z
    .object({
    email: z.union([z.string(), z.null()]).optional(),
    id: z.string().uuid(),
    roleName: z.string(),
    username: z.union([z.string(), z.null()]).optional(),
})
    .passthrough();
export const UserWithPermissionDto = _UserWithPermissionDto;
const _ConversationDemographics = z
    .object({ conversationId: z.string().uuid(), questionSlug: z.string() })
    .passthrough();
export const ConversationDemographics = _ConversationDemographics;
const _PaginatedResults_for_ConversationDemographics = z
    .object({
    records: z.array(ConversationDemographics),
    total: z.number().int(),
})
    .passthrough();
export const PaginatedResults_for_ConversationDemographics = _PaginatedResults_for_ConversationDemographics;
const _CreateConversationDemographics = z
    .object({ conversationId: z.string().uuid(), questionSlug: z.string() })
    .passthrough();
export const CreateConversationDemographics = _CreateConversationDemographics;
const _NumericBucket = z
    .object({
    label: z.string(),
    max: z.union([z.number(), z.null()]).optional(),
    min: z.union([z.number(), z.null()]).optional(),
})
    .passthrough();
export const NumericBucket = _NumericBucket;
const _TextBucket = z
    .object({
    label: z.string(),
    values: z.union([z.array(z.string()), z.null()]).optional(),
})
    .passthrough();
export const TextBucket = _TextBucket;
const _ValueBuckets = z.union([
    z
        .object({ buckets: z.array(NumericBucket), type: z.literal("numeric") })
        .passthrough(),
    z
        .object({ buckets: z.array(TextBucket), type: z.literal("text") })
        .passthrough(),
]);
export const ValueBuckets = _ValueBuckets;
const _DemographicsQuestionResponseType = z.enum(["string", "number"]);
export const DemographicsQuestionResponseType = _DemographicsQuestionResponseType;
const _DemographicsQuestion = z
    .object({
    bucketConfig: z.union([z.array(ValueBuckets), z.null()]).optional(),
    displayName: z.string(),
    responseType: DemographicsQuestionResponseType,
    slug: z.string(),
})
    .passthrough();
export const DemographicsQuestion = _DemographicsQuestion;
const _PaginatedResults_for_DemographicsQuestion = z
    .object({ records: z.array(DemographicsQuestion), total: z.number().int() })
    .passthrough();
export const PaginatedResults_for_DemographicsQuestion = _PaginatedResults_for_DemographicsQuestion;
const _CreateDemographicsQuestion = z
    .object({
    bucketConfig: z.union([z.array(ValueBuckets), z.null()]).optional(),
    displayName: z.string(),
    responseType: DemographicsQuestionResponseType,
    slug: z.string(),
})
    .passthrough();
export const CreateDemographicsQuestion = _CreateDemographicsQuestion;
const _PartialDemographicsQuestion = z
    .object({
    bucketConfig: z.union([z.array(ValueBuckets), z.null()]),
    displayName: z.union([z.string(), z.null()]),
    responseType: z.union([DemographicsQuestionResponseType, z.null()]),
})
    .partial()
    .passthrough();
export const PartialDemographicsQuestion = _PartialDemographicsQuestion;
const _DemographicsResponse = z
    .object({
    id: z.string().uuid(),
    questionSlug: z.string(),
    userId: z.union([z.string(), z.null()]).optional(),
    value: z.string(),
})
    .passthrough();
export const DemographicsResponse = _DemographicsResponse;
const _PaginatedResults_for_DemographicsResponse = z
    .object({ records: z.array(DemographicsResponse), total: z.number().int() })
    .passthrough();
export const PaginatedResults_for_DemographicsResponse = _PaginatedResults_for_DemographicsResponse;
const _CreateDemographicsResponse = z
    .object({
    questionSlug: z.string(),
    userId: z.string().uuid(),
    value: z.string(),
})
    .passthrough();
export const CreateDemographicsResponse = _CreateDemographicsResponse;
const _PartialDemographicsResponse = z
    .object({ value: z.union([z.string(), z.null()]) })
    .partial()
    .passthrough();
export const PartialDemographicsResponse = _PartialDemographicsResponse;
export const schemas = {
    GuestLoginRequest,
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
    VoteCountResponse,
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
    DemographicCount,
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
    ComhairleLlm,
    ComhairlePrompt,
    ComhairleChat,
    UpdateChatRequest,
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
    ConversationDemographics,
    PaginatedResults_for_ConversationDemographics,
    CreateConversationDemographics,
    NumericBucket,
    TextBucket,
    ValueBuckets,
    DemographicsQuestionResponseType,
    DemographicsQuestion,
    PaginatedResults_for_DemographicsQuestion,
    CreateDemographicsQuestion,
    PartialDemographicsQuestion,
    DemographicsResponse,
    PaginatedResults_for_DemographicsResponse,
    CreateDemographicsResponse,
    PartialDemographicsResponse,
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
        path: "/auth/login_guest",
        alias: "LoginGuestUser",
        requestFormat: "json",
        parameters: [
            {
                name: "body",
                description: `Expected payload for an guest login request`,
                type: "Body",
                schema: z.object({ guest_code: z.string() }).passthrough(),
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
        path: "/auth/signup_guest",
        alias: "SignupGuestUser",
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
        path: "/conversation/:conversation_id/chats",
        alias: "GetChat",
        description: `Get a conversation&#x27;s bot service chat`,
        requestFormat: "json",
        response: ComhairleChat,
    },
    {
        method: "put",
        path: "/conversation/:conversation_id/chats",
        alias: "UpdateChat",
        description: `Update a conversation&#x27;s bot service chat`,
        requestFormat: "json",
        parameters: [
            {
                name: "body",
                type: "Body",
                schema: UpdateChatRequest,
            },
        ],
        response: ComhairleChat,
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
        path: "/demographics/conversations_questions",
        alias: "GetConversationDemographics",
        description: `Retrieve demographics responses for a specific conversation and question`,
        requestFormat: "json",
        parameters: [
            {
                name: "conversation_id",
                type: "Query",
                schema: created_after,
            },
            {
                name: "question_slug",
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
        response: PaginatedResults_for_ConversationDemographics,
    },
    {
        method: "post",
        path: "/demographics/conversations_questions",
        alias: "CreateConversationDemographics",
        description: `Create a new demographics response for a specific conversation and question`,
        requestFormat: "json",
        parameters: [
            {
                name: "body",
                type: "Body",
                schema: CreateConversationDemographics,
            },
        ],
        response: ConversationDemographics,
    },
    {
        method: "delete",
        path: "/demographics/conversations_questions/:conversation_id/:question_slug/",
        alias: "DeleteConversationDemographicsByQuestion",
        description: `Delete demographics responses for a specific conversation and question`,
        requestFormat: "json",
        parameters: [
            {
                name: "body",
                type: "Body",
                schema: z.array(z.unknown()).min(2).max(2),
            },
        ],
        response: PaginatedResults_for_ConversationDemographics,
    },
    {
        method: "get",
        path: "/demographics/questions",
        alias: "GetDemographicsQuestions",
        description: `Paginated list of demographics questions with optional filtering and ordering`,
        requestFormat: "json",
        parameters: [
            {
                name: "conversation_id",
                type: "Query",
                schema: created_after,
            },
            {
                name: "question_slug",
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
        response: PaginatedResults_for_DemographicsQuestion,
    },
    {
        method: "post",
        path: "/demographics/questions",
        alias: "CreateDemographicsQuestion",
        description: `Create a new demographics question`,
        requestFormat: "json",
        parameters: [
            {
                name: "body",
                type: "Body",
                schema: CreateDemographicsQuestion,
            },
        ],
        response: DemographicsQuestion,
    },
    {
        method: "put",
        path: "/demographics/questions/:question_slug",
        alias: "UpdateDemographicsQuestion",
        description: `Update a specific demographics question`,
        requestFormat: "json",
        parameters: [
            {
                name: "body",
                description: `Represents a demographics question.`,
                type: "Body",
                schema: PartialDemographicsQuestion,
            },
        ],
        response: DemographicsQuestion,
    },
    {
        method: "delete",
        path: "/demographics/questions/:question_slug",
        alias: "DeleteDemographicsQuestion",
        description: `Delete a specific demographics question`,
        requestFormat: "json",
        response: z.union([DemographicsQuestion, z.null()]),
    },
    {
        method: "get",
        path: "/demographics/responses",
        alias: "GetDemographicsResponses",
        description: `Paginated list of demographics responses with optional filtering and ordering`,
        requestFormat: "json",
        parameters: [
            {
                name: "conversation_id",
                type: "Query",
                schema: created_after,
            },
            {
                name: "question_slug",
                type: "Query",
                schema: created_after,
            },
            {
                name: "user_id",
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
        response: PaginatedResults_for_DemographicsResponse,
    },
    {
        method: "post",
        path: "/demographics/responses",
        alias: "CreateDemographicsResponse",
        description: `Create a new response for a specific demographics question and user`,
        requestFormat: "json",
        parameters: [
            {
                name: "body",
                type: "Body",
                schema: CreateDemographicsResponse,
            },
        ],
        response: DemographicsResponse,
    },
    {
        method: "put",
        path: "/demographics/responses/:question_slug/:user_id",
        alias: "UpdateDemographicsResponse",
        description: `Update a response for a specific demographics question and user`,
        requestFormat: "json",
        parameters: [
            {
                name: "body",
                description: `Represents a demographics response from a user to a specific demographics question.`,
                type: "Body",
                schema: PartialDemographicsResponse,
            },
        ],
        response: DemographicsResponse,
    },
    {
        method: "delete",
        path: "/demographics/responses/:question_slug/:user_id",
        alias: "DeleteDemographicsResponse",
        description: `Delete a response for a specific demographics question and user`,
        requestFormat: "json",
        response: z.union([DemographicsResponse, z.null()]),
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
        path: "/tools/polis/vote_count",
        alias: "PolisGetUserVoteCount",
        description: `Counts the votes the authenticated participant has cast in the Polis poll for the given workflow step, mapping their comhairle user id to the Polis participant via xids. Used to seed the required-votes progress from server data.`,
        requestFormat: "json",
        parameters: [
            {
                name: "workflow_step_id",
                type: "Query",
                schema: z.string().uuid(),
            },
        ],
        response: z.object({ vote_count: z.number().int().gte(0) }).passthrough(),
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
]);
export const api = new Zodios(endpoints);
export function createApiClient(baseUrl, options) {
    return new Zodios(baseUrl, endpoints, options);
}

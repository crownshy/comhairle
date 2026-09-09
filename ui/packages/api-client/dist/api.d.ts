import { type ZodiosOptions, type ZodiosInstance, type ZodiosEndpointDefinitions } from "@zodios/core";
import { z } from "zod";
declare const _GuestLoginRequest: z.ZodObject<{
    guest_code: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    guest_code: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    guest_code: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type GuestLoginRequest = z.infer<typeof _GuestLoginRequest>;
export declare const GuestLoginRequest: z.ZodType<GuestLoginRequest, z.ZodTypeDef, any>;
declare const _UserAuthType: z.ZodEnum<["guest", "email_password", "otp", "scot_account"]>;
export type UserAuthType = z.infer<typeof _UserAuthType>;
export declare const UserAuthType: z.ZodType<UserAuthType, z.ZodTypeDef, any>;
declare const _UserDto: z.ZodObject<{
    authType: z.ZodType<"guest" | "email_password" | "otp" | "scot_account", z.ZodTypeDef, any>;
    avatarUrl: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    email: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    emailVerified: z.ZodBoolean;
    guestCode: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    id: z.ZodString;
    organizationId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    username: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    authType: z.ZodType<"guest" | "email_password" | "otp" | "scot_account", z.ZodTypeDef, any>;
    avatarUrl: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    email: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    emailVerified: z.ZodBoolean;
    guestCode: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    id: z.ZodString;
    organizationId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    username: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    authType: z.ZodType<"guest" | "email_password" | "otp" | "scot_account", z.ZodTypeDef, any>;
    avatarUrl: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    email: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    emailVerified: z.ZodBoolean;
    guestCode: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    id: z.ZodString;
    organizationId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    username: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">>;
export type UserDto = z.infer<typeof _UserDto>;
export declare const UserDto: z.ZodType<UserDto, z.ZodTypeDef, any>;
declare const _LoginRequest: z.ZodObject<{
    email: z.ZodString;
    password: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    email: z.ZodString;
    password: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    email: z.ZodString;
    password: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type LoginRequest = z.infer<typeof _LoginRequest>;
export declare const LoginRequest: z.ZodType<LoginRequest, z.ZodTypeDef, any>;
declare const _OtpLoginRequest: z.ZodObject<{
    code: z.ZodString;
    email: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    code: z.ZodString;
    email: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    code: z.ZodString;
    email: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type OtpLoginRequest = z.infer<typeof _OtpLoginRequest>;
export declare const OtpLoginRequest: z.ZodType<OtpLoginRequest, z.ZodTypeDef, any>;
declare const _SignupRequest: z.ZodObject<{
    avatar_url: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    email: z.ZodString;
    password: z.ZodString;
    username: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    avatar_url: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    email: z.ZodString;
    password: z.ZodString;
    username: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    avatar_url: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    email: z.ZodString;
    password: z.ZodString;
    username: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type SignupRequest = z.infer<typeof _SignupRequest>;
export declare const SignupRequest: z.ZodType<SignupRequest, z.ZodTypeDef, any>;
declare const _OtpSignupRequest: z.ZodObject<{
    email: z.ZodString;
    username: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    email: z.ZodString;
    username: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    email: z.ZodString;
    username: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">>;
export type OtpSignupRequest = z.infer<typeof _OtpSignupRequest>;
export declare const OtpSignupRequest: z.ZodType<OtpSignupRequest, z.ZodTypeDef, any>;
declare const _CreateOtpRequest: z.ZodObject<{
    email: z.ZodString;
    redirect_url: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    email: z.ZodString;
    redirect_url: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    email: z.ZodString;
    redirect_url: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">>;
export type CreateOtpRequest = z.infer<typeof _CreateOtpRequest>;
export declare const CreateOtpRequest: z.ZodType<CreateOtpRequest, z.ZodTypeDef, any>;
declare const _VerifyOtpTokenRequest: z.ZodObject<{
    token: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    token: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    token: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type VerifyOtpTokenRequest = z.infer<typeof _VerifyOtpTokenRequest>;
export declare const VerifyOtpTokenRequest: z.ZodType<VerifyOtpTokenRequest, z.ZodTypeDef, any>;
declare const _VerifyEmailTokenRequest: z.ZodObject<{
    token: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    token: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    token: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type VerifyEmailTokenRequest = z.infer<typeof _VerifyEmailTokenRequest>;
export declare const VerifyEmailTokenRequest: z.ZodType<VerifyEmailTokenRequest, z.ZodTypeDef, any>;
declare const _ResendVerificationEmailRequest: z.ZodObject<{
    id: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    id: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    id: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type ResendVerificationEmailRequest = z.infer<typeof _ResendVerificationEmailRequest>;
export declare const ResendVerificationEmailRequest: z.ZodType<ResendVerificationEmailRequest, z.ZodTypeDef, any>;
declare const _CreatePasswordResetRequest: z.ZodObject<{
    email: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    email: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    email: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type CreatePasswordResetRequest = z.infer<typeof _CreatePasswordResetRequest>;
export declare const CreatePasswordResetRequest: z.ZodType<CreatePasswordResetRequest, z.ZodTypeDef, any>;
declare const _PasswordResetUpdateRequest: z.ZodObject<{
    confirm_password: z.ZodString;
    password: z.ZodString;
    token: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    confirm_password: z.ZodString;
    password: z.ZodString;
    token: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    confirm_password: z.ZodString;
    password: z.ZodString;
    token: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type PasswordResetUpdateRequest = z.infer<typeof _PasswordResetUpdateRequest>;
export declare const PasswordResetUpdateRequest: z.ZodType<PasswordResetUpdateRequest, z.ZodTypeDef, any>;
declare const _ResourceType: z.ZodUnion<[z.ZodLiteral<"Site">, z.ZodObject<{
    Conversation: z.ZodString;
}, "strip", z.ZodTypeAny, {
    Conversation: string;
}, {
    Conversation: string;
}>]>;
export type ResourceType = z.infer<typeof _ResourceType>;
export declare const ResourceType: z.ZodType<ResourceType, z.ZodTypeDef, any>;
declare const _ResourceRole: z.ZodEnum<["Admin", "SuperAdmin"]>;
export type ResourceRole = z.infer<typeof _ResourceRole>;
export declare const ResourceRole: z.ZodType<ResourceRole, z.ZodTypeDef, any>;
declare const _UserRoles: z.ZodObject<{
    resource: z.ZodType<"Site" | {
        Conversation: string;
    }, z.ZodTypeDef, any>;
    roles: z.ZodArray<z.ZodType<"Admin" | "SuperAdmin", z.ZodTypeDef, any>, "many">;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    resource: z.ZodType<"Site" | {
        Conversation: string;
    }, z.ZodTypeDef, any>;
    roles: z.ZodArray<z.ZodType<"Admin" | "SuperAdmin", z.ZodTypeDef, any>, "many">;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    resource: z.ZodType<"Site" | {
        Conversation: string;
    }, z.ZodTypeDef, any>;
    roles: z.ZodArray<z.ZodType<"Admin" | "SuperAdmin", z.ZodTypeDef, any>, "many">;
}, z.ZodTypeAny, "passthrough">>;
export type UserRoles = z.infer<typeof _UserRoles>;
export declare const UserRoles: z.ZodType<UserRoles, z.ZodTypeDef, any>;
declare const _LocalizedConversationDto: z.ZodObject<{
    allowRevisitAfterFinishing: z.ZodBoolean;
    callToAction: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    chatBotId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    description: z.ZodString;
    enableQaChatBot: z.ZodBoolean;
    enableSignupPrompts: z.ZodBoolean;
    faqs: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    id: z.ZodString;
    imageUrl: z.ZodString;
    isComplete: z.ZodBoolean;
    isInviteOnly: z.ZodBoolean;
    isLive: z.ZodBoolean;
    isPublic: z.ZodBoolean;
    knowledgeBaseId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    metadata: z.ZodUnknown;
    organizationId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    primaryLocale: z.ZodString;
    privacyPolicy: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    shortDescription: z.ZodString;
    shortPrivacyPolicy: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    showThankYouPageAnnonInstructions: z.ZodBoolean;
    showThankyouPageFeedbackButton: z.ZodBoolean;
    slug: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    supportedLanguages: z.ZodArray<z.ZodString, "many">;
    tags: z.ZodArray<z.ZodString, "many">;
    thankYouMessage: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    title: z.ZodString;
    videoUrl: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    allowRevisitAfterFinishing: z.ZodBoolean;
    callToAction: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    chatBotId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    description: z.ZodString;
    enableQaChatBot: z.ZodBoolean;
    enableSignupPrompts: z.ZodBoolean;
    faqs: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    id: z.ZodString;
    imageUrl: z.ZodString;
    isComplete: z.ZodBoolean;
    isInviteOnly: z.ZodBoolean;
    isLive: z.ZodBoolean;
    isPublic: z.ZodBoolean;
    knowledgeBaseId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    metadata: z.ZodUnknown;
    organizationId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    primaryLocale: z.ZodString;
    privacyPolicy: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    shortDescription: z.ZodString;
    shortPrivacyPolicy: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    showThankYouPageAnnonInstructions: z.ZodBoolean;
    showThankyouPageFeedbackButton: z.ZodBoolean;
    slug: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    supportedLanguages: z.ZodArray<z.ZodString, "many">;
    tags: z.ZodArray<z.ZodString, "many">;
    thankYouMessage: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    title: z.ZodString;
    videoUrl: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    allowRevisitAfterFinishing: z.ZodBoolean;
    callToAction: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    chatBotId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    description: z.ZodString;
    enableQaChatBot: z.ZodBoolean;
    enableSignupPrompts: z.ZodBoolean;
    faqs: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    id: z.ZodString;
    imageUrl: z.ZodString;
    isComplete: z.ZodBoolean;
    isInviteOnly: z.ZodBoolean;
    isLive: z.ZodBoolean;
    isPublic: z.ZodBoolean;
    knowledgeBaseId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    metadata: z.ZodUnknown;
    organizationId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    primaryLocale: z.ZodString;
    privacyPolicy: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    shortDescription: z.ZodString;
    shortPrivacyPolicy: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    showThankYouPageAnnonInstructions: z.ZodBoolean;
    showThankyouPageFeedbackButton: z.ZodBoolean;
    slug: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    supportedLanguages: z.ZodArray<z.ZodString, "many">;
    tags: z.ZodArray<z.ZodString, "many">;
    thankYouMessage: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    title: z.ZodString;
    videoUrl: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">>;
export type LocalizedConversationDto = z.infer<typeof _LocalizedConversationDto>;
export declare const LocalizedConversationDto: z.ZodType<LocalizedConversationDto, z.ZodTypeDef, any>;
declare const _created_after: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
export type created_after = z.infer<typeof _created_after>;
export declare const created_after: z.ZodType<created_after, z.ZodTypeDef, any>;
declare const _is_complete: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
export type is_complete = z.infer<typeof _is_complete>;
export declare const is_complete: z.ZodType<is_complete, z.ZodTypeDef, any>;
declare const _limit: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
export type limit = z.infer<typeof _limit>;
export declare const limit: z.ZodType<limit, z.ZodTypeDef, any>;
declare const _PaginatedResults_for_LocalizedConversationDto: z.ZodObject<{
    records: z.ZodArray<z.ZodType<z.objectOutputType<{
        allowRevisitAfterFinishing: z.ZodBoolean;
        callToAction: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        chatBotId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        description: z.ZodString;
        enableQaChatBot: z.ZodBoolean;
        enableSignupPrompts: z.ZodBoolean;
        faqs: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        id: z.ZodString;
        imageUrl: z.ZodString;
        isComplete: z.ZodBoolean;
        isInviteOnly: z.ZodBoolean;
        isLive: z.ZodBoolean;
        isPublic: z.ZodBoolean;
        knowledgeBaseId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        metadata: z.ZodUnknown;
        organizationId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        primaryLocale: z.ZodString;
        privacyPolicy: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        shortDescription: z.ZodString;
        shortPrivacyPolicy: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        showThankYouPageAnnonInstructions: z.ZodBoolean;
        showThankyouPageFeedbackButton: z.ZodBoolean;
        slug: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        supportedLanguages: z.ZodArray<z.ZodString, "many">;
        tags: z.ZodArray<z.ZodString, "many">;
        thankYouMessage: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        title: z.ZodString;
        videoUrl: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    total: z.ZodNumber;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    records: z.ZodArray<z.ZodType<z.objectOutputType<{
        allowRevisitAfterFinishing: z.ZodBoolean;
        callToAction: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        chatBotId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        description: z.ZodString;
        enableQaChatBot: z.ZodBoolean;
        enableSignupPrompts: z.ZodBoolean;
        faqs: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        id: z.ZodString;
        imageUrl: z.ZodString;
        isComplete: z.ZodBoolean;
        isInviteOnly: z.ZodBoolean;
        isLive: z.ZodBoolean;
        isPublic: z.ZodBoolean;
        knowledgeBaseId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        metadata: z.ZodUnknown;
        organizationId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        primaryLocale: z.ZodString;
        privacyPolicy: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        shortDescription: z.ZodString;
        shortPrivacyPolicy: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        showThankYouPageAnnonInstructions: z.ZodBoolean;
        showThankyouPageFeedbackButton: z.ZodBoolean;
        slug: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        supportedLanguages: z.ZodArray<z.ZodString, "many">;
        tags: z.ZodArray<z.ZodString, "many">;
        thankYouMessage: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        title: z.ZodString;
        videoUrl: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    total: z.ZodNumber;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    records: z.ZodArray<z.ZodType<z.objectOutputType<{
        allowRevisitAfterFinishing: z.ZodBoolean;
        callToAction: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        chatBotId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        description: z.ZodString;
        enableQaChatBot: z.ZodBoolean;
        enableSignupPrompts: z.ZodBoolean;
        faqs: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        id: z.ZodString;
        imageUrl: z.ZodString;
        isComplete: z.ZodBoolean;
        isInviteOnly: z.ZodBoolean;
        isLive: z.ZodBoolean;
        isPublic: z.ZodBoolean;
        knowledgeBaseId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        metadata: z.ZodUnknown;
        organizationId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        primaryLocale: z.ZodString;
        privacyPolicy: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        shortDescription: z.ZodString;
        shortPrivacyPolicy: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        showThankYouPageAnnonInstructions: z.ZodBoolean;
        showThankyouPageFeedbackButton: z.ZodBoolean;
        slug: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        supportedLanguages: z.ZodArray<z.ZodString, "many">;
        tags: z.ZodArray<z.ZodString, "many">;
        thankYouMessage: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        title: z.ZodString;
        videoUrl: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    total: z.ZodNumber;
}, z.ZodTypeAny, "passthrough">>;
export type PaginatedResults_for_LocalizedConversationDto = z.infer<typeof _PaginatedResults_for_LocalizedConversationDto>;
export declare const PaginatedResults_for_LocalizedConversationDto: z.ZodType<PaginatedResults_for_LocalizedConversationDto, z.ZodTypeDef, any>;
declare const _OrganizationType: z.ZodEnum<["non_profit", "governmental", "other"]>;
export type OrganizationType = z.infer<typeof _OrganizationType>;
export declare const OrganizationType: z.ZodType<OrganizationType, z.ZodTypeDef, any>;
declare const _LocalizedOrganizationDto: z.ZodObject<{
    contactEmail: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    createdAt: z.ZodString;
    description: z.ZodString;
    externalUrl: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    id: z.ZodString;
    metadata: z.ZodOptional<z.ZodUnknown>;
    mission: z.ZodString;
    name: z.ZodString;
    orgType: z.ZodType<"non_profit" | "governmental" | "other", z.ZodTypeDef, any>;
    regions: z.ZodArray<z.ZodString, "many">;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    contactEmail: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    createdAt: z.ZodString;
    description: z.ZodString;
    externalUrl: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    id: z.ZodString;
    metadata: z.ZodOptional<z.ZodUnknown>;
    mission: z.ZodString;
    name: z.ZodString;
    orgType: z.ZodType<"non_profit" | "governmental" | "other", z.ZodTypeDef, any>;
    regions: z.ZodArray<z.ZodString, "many">;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    contactEmail: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    createdAt: z.ZodString;
    description: z.ZodString;
    externalUrl: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    id: z.ZodString;
    metadata: z.ZodOptional<z.ZodUnknown>;
    mission: z.ZodString;
    name: z.ZodString;
    orgType: z.ZodType<"non_profit" | "governmental" | "other", z.ZodTypeDef, any>;
    regions: z.ZodArray<z.ZodString, "many">;
}, z.ZodTypeAny, "passthrough">>;
export type LocalizedOrganizationDto = z.infer<typeof _LocalizedOrganizationDto>;
export declare const LocalizedOrganizationDto: z.ZodType<LocalizedOrganizationDto, z.ZodTypeDef, any>;
declare const _UserOrganizationAccess: z.ZodObject<{
    canDelete: z.ZodBoolean;
    canManageTeam: z.ZodBoolean;
    canUpdate: z.ZodBoolean;
    isAssociated: z.ZodBoolean;
    organization: z.ZodType<z.objectOutputType<{
        contactEmail: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        createdAt: z.ZodString;
        description: z.ZodString;
        externalUrl: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        id: z.ZodString;
        metadata: z.ZodOptional<z.ZodUnknown>;
        mission: z.ZodString;
        name: z.ZodString;
        orgType: z.ZodType<"non_profit" | "governmental" | "other", z.ZodTypeDef, any>;
        regions: z.ZodArray<z.ZodString, "many">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    canDelete: z.ZodBoolean;
    canManageTeam: z.ZodBoolean;
    canUpdate: z.ZodBoolean;
    isAssociated: z.ZodBoolean;
    organization: z.ZodType<z.objectOutputType<{
        contactEmail: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        createdAt: z.ZodString;
        description: z.ZodString;
        externalUrl: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        id: z.ZodString;
        metadata: z.ZodOptional<z.ZodUnknown>;
        mission: z.ZodString;
        name: z.ZodString;
        orgType: z.ZodType<"non_profit" | "governmental" | "other", z.ZodTypeDef, any>;
        regions: z.ZodArray<z.ZodString, "many">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    canDelete: z.ZodBoolean;
    canManageTeam: z.ZodBoolean;
    canUpdate: z.ZodBoolean;
    isAssociated: z.ZodBoolean;
    organization: z.ZodType<z.objectOutputType<{
        contactEmail: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        createdAt: z.ZodString;
        description: z.ZodString;
        externalUrl: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        id: z.ZodString;
        metadata: z.ZodOptional<z.ZodUnknown>;
        mission: z.ZodString;
        name: z.ZodString;
        orgType: z.ZodType<"non_profit" | "governmental" | "other", z.ZodTypeDef, any>;
        regions: z.ZodArray<z.ZodString, "many">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
}, z.ZodTypeAny, "passthrough">>;
export type UserOrganizationAccess = z.infer<typeof _UserOrganizationAccess>;
export declare const UserOrganizationAccess: z.ZodType<UserOrganizationAccess, z.ZodTypeDef, any>;
declare const _UserOrganizationsResponse: z.ZodObject<{
    canCreateOrganization: z.ZodBoolean;
    organizations: z.ZodArray<z.ZodType<z.objectOutputType<{
        canDelete: z.ZodBoolean;
        canManageTeam: z.ZodBoolean;
        canUpdate: z.ZodBoolean;
        isAssociated: z.ZodBoolean;
        organization: z.ZodType<z.objectOutputType<{
            contactEmail: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
            createdAt: z.ZodString;
            description: z.ZodString;
            externalUrl: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
            id: z.ZodString;
            metadata: z.ZodOptional<z.ZodUnknown>;
            mission: z.ZodString;
            name: z.ZodString;
            orgType: z.ZodType<"non_profit" | "governmental" | "other", z.ZodTypeDef, any>;
            regions: z.ZodArray<z.ZodString, "many">;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    canCreateOrganization: z.ZodBoolean;
    organizations: z.ZodArray<z.ZodType<z.objectOutputType<{
        canDelete: z.ZodBoolean;
        canManageTeam: z.ZodBoolean;
        canUpdate: z.ZodBoolean;
        isAssociated: z.ZodBoolean;
        organization: z.ZodType<z.objectOutputType<{
            contactEmail: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
            createdAt: z.ZodString;
            description: z.ZodString;
            externalUrl: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
            id: z.ZodString;
            metadata: z.ZodOptional<z.ZodUnknown>;
            mission: z.ZodString;
            name: z.ZodString;
            orgType: z.ZodType<"non_profit" | "governmental" | "other", z.ZodTypeDef, any>;
            regions: z.ZodArray<z.ZodString, "many">;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    canCreateOrganization: z.ZodBoolean;
    organizations: z.ZodArray<z.ZodType<z.objectOutputType<{
        canDelete: z.ZodBoolean;
        canManageTeam: z.ZodBoolean;
        canUpdate: z.ZodBoolean;
        isAssociated: z.ZodBoolean;
        organization: z.ZodType<z.objectOutputType<{
            contactEmail: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
            createdAt: z.ZodString;
            description: z.ZodString;
            externalUrl: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
            id: z.ZodString;
            metadata: z.ZodOptional<z.ZodUnknown>;
            mission: z.ZodString;
            name: z.ZodString;
            orgType: z.ZodType<"non_profit" | "governmental" | "other", z.ZodTypeDef, any>;
            regions: z.ZodArray<z.ZodString, "many">;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
}, z.ZodTypeAny, "passthrough">>;
export type UserOrganizationsResponse = z.infer<typeof _UserOrganizationsResponse>;
export declare const UserOrganizationsResponse: z.ZodType<UserOrganizationsResponse, z.ZodTypeDef, any>;
declare const _UpdateUserRequest: z.ZodObject<{
    email_verified: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    organization_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    password: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    username: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    email_verified: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    organization_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    password: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    username: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    email_verified: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    organization_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    password: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    username: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">>;
export type UpdateUserRequest = z.infer<typeof _UpdateUserRequest>;
export declare const UpdateUserRequest: z.ZodType<UpdateUserRequest, z.ZodTypeDef, any>;
declare const _UpgradeAccountRequest: z.ZodObject<{
    email: z.ZodString;
    password: z.ZodString;
    username: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    email: z.ZodString;
    password: z.ZodString;
    username: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    email: z.ZodString;
    password: z.ZodString;
    username: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type UpgradeAccountRequest = z.infer<typeof _UpgradeAccountRequest>;
export declare const UpgradeAccountRequest: z.ZodType<UpgradeAccountRequest, z.ZodTypeDef, any>;
declare const _UserConversationPreferencesDto: z.ZodObject<{
    conversationId: z.ZodString;
    id: z.ZodString;
    receiveSimilarConversationUpdatesByEmail: z.ZodBoolean;
    receiveSimilarConversationUpdatesByNotification: z.ZodBoolean;
    receiveUpdatesByEmail: z.ZodBoolean;
    receiveUpdatesByNotification: z.ZodBoolean;
    userId: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    conversationId: z.ZodString;
    id: z.ZodString;
    receiveSimilarConversationUpdatesByEmail: z.ZodBoolean;
    receiveSimilarConversationUpdatesByNotification: z.ZodBoolean;
    receiveUpdatesByEmail: z.ZodBoolean;
    receiveUpdatesByNotification: z.ZodBoolean;
    userId: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    conversationId: z.ZodString;
    id: z.ZodString;
    receiveSimilarConversationUpdatesByEmail: z.ZodBoolean;
    receiveSimilarConversationUpdatesByNotification: z.ZodBoolean;
    receiveUpdatesByEmail: z.ZodBoolean;
    receiveUpdatesByNotification: z.ZodBoolean;
    userId: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type UserConversationPreferencesDto = z.infer<typeof _UserConversationPreferencesDto>;
export declare const UserConversationPreferencesDto: z.ZodType<UserConversationPreferencesDto, z.ZodTypeDef, any>;
declare const _UpdateUserConversationPreferences: z.ZodObject<{
    receiveSimilarConversationUpdatesByEmail: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    receiveSimilarConversationUpdatesByNotification: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    receiveUpdatesByEmail: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    receiveUpdatesByNotification: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    receiveSimilarConversationUpdatesByEmail: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    receiveSimilarConversationUpdatesByNotification: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    receiveUpdatesByEmail: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    receiveUpdatesByNotification: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    receiveSimilarConversationUpdatesByEmail: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    receiveSimilarConversationUpdatesByNotification: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    receiveUpdatesByEmail: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    receiveUpdatesByNotification: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">>;
export type UpdateUserConversationPreferences = z.infer<typeof _UpdateUserConversationPreferences>;
export declare const UpdateUserConversationPreferences: z.ZodType<UpdateUserConversationPreferences, z.ZodTypeDef, any>;
declare const _UserProfileDto: z.ZodObject<{
    consented: z.ZodBoolean;
    createdAt: z.ZodString;
    id: z.ZodString;
    updatedAt: z.ZodString;
    userId: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    consented: z.ZodBoolean;
    createdAt: z.ZodString;
    id: z.ZodString;
    updatedAt: z.ZodString;
    userId: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    consented: z.ZodBoolean;
    createdAt: z.ZodString;
    id: z.ZodString;
    updatedAt: z.ZodString;
    userId: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type UserProfileDto = z.infer<typeof _UserProfileDto>;
export declare const UserProfileDto: z.ZodType<UserProfileDto, z.ZodTypeDef, any>;
declare const _UpsertUserProfileRequest: z.ZodObject<{
    age: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
    consented: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    ethnicity: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    gender: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    politicalParty: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    zipcode: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    age: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
    consented: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    ethnicity: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    gender: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    politicalParty: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    zipcode: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    age: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
    consented: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    ethnicity: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    gender: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    politicalParty: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    zipcode: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">>;
export type UpsertUserProfileRequest = z.infer<typeof _UpsertUserProfileRequest>;
export declare const UpsertUserProfileRequest: z.ZodType<UpsertUserProfileRequest, z.ZodTypeDef, any>;
declare const _DeliveryMethod: z.ZodEnum<["in_app", "email"]>;
export type DeliveryMethod = z.infer<typeof _DeliveryMethod>;
export declare const DeliveryMethod: z.ZodType<DeliveryMethod, z.ZodTypeDef, any>;
declare const _NotificationContextType: z.ZodEnum<["site", "conversation"]>;
export type NotificationContextType = z.infer<typeof _NotificationContextType>;
export declare const NotificationContextType: z.ZodType<NotificationContextType, z.ZodTypeDef, any>;
declare const _NotificationType: z.ZodEnum<["info", "warning", "error", "success"]>;
export type NotificationType = z.infer<typeof _NotificationType>;
export declare const NotificationType: z.ZodType<NotificationType, z.ZodTypeDef, any>;
declare const _NotificationDto: z.ZodObject<{
    content: z.ZodString;
    contextId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    contextType: z.ZodType<"site" | "conversation", z.ZodTypeDef, any>;
    createdAt: z.ZodString;
    id: z.ZodString;
    notificationType: z.ZodType<"info" | "warning" | "error" | "success", z.ZodTypeDef, any>;
    title: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    content: z.ZodString;
    contextId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    contextType: z.ZodType<"site" | "conversation", z.ZodTypeDef, any>;
    createdAt: z.ZodString;
    id: z.ZodString;
    notificationType: z.ZodType<"info" | "warning" | "error" | "success", z.ZodTypeDef, any>;
    title: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    content: z.ZodString;
    contextId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    contextType: z.ZodType<"site" | "conversation", z.ZodTypeDef, any>;
    createdAt: z.ZodString;
    id: z.ZodString;
    notificationType: z.ZodType<"info" | "warning" | "error" | "success", z.ZodTypeDef, any>;
    title: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type NotificationDto = z.infer<typeof _NotificationDto>;
export declare const NotificationDto: z.ZodType<NotificationDto, z.ZodTypeDef, any>;
declare const _NotificationWithDelivery: z.ZodObject<{
    createdAt: z.ZodString;
    deliveredAt: z.ZodString;
    deliveryMethod: z.ZodType<"email" | "in_app", z.ZodTypeDef, any>;
    id: z.ZodString;
    notification: z.ZodType<z.objectOutputType<{
        content: z.ZodString;
        contextId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        contextType: z.ZodType<"site" | "conversation", z.ZodTypeDef, any>;
        createdAt: z.ZodString;
        id: z.ZodString;
        notificationType: z.ZodType<"info" | "warning" | "error" | "success", z.ZodTypeDef, any>;
        title: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    notificationId: z.ZodString;
    readAt: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    userId: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    createdAt: z.ZodString;
    deliveredAt: z.ZodString;
    deliveryMethod: z.ZodType<"email" | "in_app", z.ZodTypeDef, any>;
    id: z.ZodString;
    notification: z.ZodType<z.objectOutputType<{
        content: z.ZodString;
        contextId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        contextType: z.ZodType<"site" | "conversation", z.ZodTypeDef, any>;
        createdAt: z.ZodString;
        id: z.ZodString;
        notificationType: z.ZodType<"info" | "warning" | "error" | "success", z.ZodTypeDef, any>;
        title: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    notificationId: z.ZodString;
    readAt: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    userId: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    createdAt: z.ZodString;
    deliveredAt: z.ZodString;
    deliveryMethod: z.ZodType<"email" | "in_app", z.ZodTypeDef, any>;
    id: z.ZodString;
    notification: z.ZodType<z.objectOutputType<{
        content: z.ZodString;
        contextId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        contextType: z.ZodType<"site" | "conversation", z.ZodTypeDef, any>;
        createdAt: z.ZodString;
        id: z.ZodString;
        notificationType: z.ZodType<"info" | "warning" | "error" | "success", z.ZodTypeDef, any>;
        title: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    notificationId: z.ZodString;
    readAt: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    userId: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type NotificationWithDelivery = z.infer<typeof _NotificationWithDelivery>;
export declare const NotificationWithDelivery: z.ZodType<NotificationWithDelivery, z.ZodTypeDef, any>;
declare const _PaginatedResults_for_NotificationWithDelivery: z.ZodObject<{
    records: z.ZodArray<z.ZodType<z.objectOutputType<{
        createdAt: z.ZodString;
        deliveredAt: z.ZodString;
        deliveryMethod: z.ZodType<"email" | "in_app", z.ZodTypeDef, any>;
        id: z.ZodString;
        notification: z.ZodType<z.objectOutputType<{
            content: z.ZodString;
            contextId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
            contextType: z.ZodType<"site" | "conversation", z.ZodTypeDef, any>;
            createdAt: z.ZodString;
            id: z.ZodString;
            notificationType: z.ZodType<"info" | "warning" | "error" | "success", z.ZodTypeDef, any>;
            title: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        notificationId: z.ZodString;
        readAt: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        userId: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    total: z.ZodNumber;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    records: z.ZodArray<z.ZodType<z.objectOutputType<{
        createdAt: z.ZodString;
        deliveredAt: z.ZodString;
        deliveryMethod: z.ZodType<"email" | "in_app", z.ZodTypeDef, any>;
        id: z.ZodString;
        notification: z.ZodType<z.objectOutputType<{
            content: z.ZodString;
            contextId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
            contextType: z.ZodType<"site" | "conversation", z.ZodTypeDef, any>;
            createdAt: z.ZodString;
            id: z.ZodString;
            notificationType: z.ZodType<"info" | "warning" | "error" | "success", z.ZodTypeDef, any>;
            title: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        notificationId: z.ZodString;
        readAt: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        userId: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    total: z.ZodNumber;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    records: z.ZodArray<z.ZodType<z.objectOutputType<{
        createdAt: z.ZodString;
        deliveredAt: z.ZodString;
        deliveryMethod: z.ZodType<"email" | "in_app", z.ZodTypeDef, any>;
        id: z.ZodString;
        notification: z.ZodType<z.objectOutputType<{
            content: z.ZodString;
            contextId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
            contextType: z.ZodType<"site" | "conversation", z.ZodTypeDef, any>;
            createdAt: z.ZodString;
            id: z.ZodString;
            notificationType: z.ZodType<"info" | "warning" | "error" | "success", z.ZodTypeDef, any>;
            title: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        notificationId: z.ZodString;
        readAt: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        userId: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    total: z.ZodNumber;
}, z.ZodTypeAny, "passthrough">>;
export type PaginatedResults_for_NotificationWithDelivery = z.infer<typeof _PaginatedResults_for_NotificationWithDelivery>;
export declare const PaginatedResults_for_NotificationWithDelivery: z.ZodType<PaginatedResults_for_NotificationWithDelivery, z.ZodTypeDef, any>;
declare const _UnreadCount: z.ZodObject<{
    count: z.ZodNumber;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    count: z.ZodNumber;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    count: z.ZodNumber;
}, z.ZodTypeAny, "passthrough">>;
export type UnreadCount = z.infer<typeof _UnreadCount>;
export declare const UnreadCount: z.ZodType<UnreadCount, z.ZodTypeDef, any>;
declare const _NotificationDelivery: z.ZodObject<{
    created_at: z.ZodString;
    delivered_at: z.ZodString;
    delivery_method: z.ZodType<"email" | "in_app", z.ZodTypeDef, any>;
    id: z.ZodString;
    notification_id: z.ZodString;
    read_at: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    updated_at: z.ZodString;
    user_id: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    created_at: z.ZodString;
    delivered_at: z.ZodString;
    delivery_method: z.ZodType<"email" | "in_app", z.ZodTypeDef, any>;
    id: z.ZodString;
    notification_id: z.ZodString;
    read_at: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    updated_at: z.ZodString;
    user_id: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    created_at: z.ZodString;
    delivered_at: z.ZodString;
    delivery_method: z.ZodType<"email" | "in_app", z.ZodTypeDef, any>;
    id: z.ZodString;
    notification_id: z.ZodString;
    read_at: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    updated_at: z.ZodString;
    user_id: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type NotificationDelivery = z.infer<typeof _NotificationDelivery>;
export declare const NotificationDelivery: z.ZodType<NotificationDelivery, z.ZodTypeDef, any>;
declare const _TextFormat: z.ZodUnion<[z.ZodLiteral<"plain">, z.ZodLiteral<"markdown">, z.ZodLiteral<"rich">]>;
export type TextFormat = z.infer<typeof _TextFormat>;
export declare const TextFormat: z.ZodType<TextFormat, z.ZodTypeDef, any>;
declare const _CreateTextContentRequest: z.ZodObject<{
    content: z.ZodString;
    format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
    primary_locale: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    content: z.ZodString;
    format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
    primary_locale: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    content: z.ZodString;
    format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
    primary_locale: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type CreateTextContentRequest = z.infer<typeof _CreateTextContentRequest>;
export declare const CreateTextContentRequest: z.ZodType<CreateTextContentRequest, z.ZodTypeDef, any>;
declare const _TextContentDto: z.ZodObject<{
    format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
    id: z.ZodString;
    primaryLocale: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
    id: z.ZodString;
    primaryLocale: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
    id: z.ZodString;
    primaryLocale: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type TextContentDto = z.infer<typeof _TextContentDto>;
export declare const TextContentDto: z.ZodType<TextContentDto, z.ZodTypeDef, any>;
declare const _TextTranslationDto: z.ZodObject<{
    aiGenerated: z.ZodBoolean;
    content: z.ZodString;
    contentId: z.ZodString;
    id: z.ZodString;
    locale: z.ZodString;
    requiresValidation: z.ZodBoolean;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    aiGenerated: z.ZodBoolean;
    content: z.ZodString;
    contentId: z.ZodString;
    id: z.ZodString;
    locale: z.ZodString;
    requiresValidation: z.ZodBoolean;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    aiGenerated: z.ZodBoolean;
    content: z.ZodString;
    contentId: z.ZodString;
    id: z.ZodString;
    locale: z.ZodString;
    requiresValidation: z.ZodBoolean;
}, z.ZodTypeAny, "passthrough">>;
export type TextTranslationDto = z.infer<typeof _TextTranslationDto>;
export declare const TextTranslationDto: z.ZodType<TextTranslationDto, z.ZodTypeDef, any>;
declare const _TextContentWithTranslations: z.ZodObject<{
    format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
    id: z.ZodString;
    primaryLocale: z.ZodString;
    translations: z.ZodArray<z.ZodType<z.objectOutputType<{
        aiGenerated: z.ZodBoolean;
        content: z.ZodString;
        contentId: z.ZodString;
        id: z.ZodString;
        locale: z.ZodString;
        requiresValidation: z.ZodBoolean;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
    id: z.ZodString;
    primaryLocale: z.ZodString;
    translations: z.ZodArray<z.ZodType<z.objectOutputType<{
        aiGenerated: z.ZodBoolean;
        content: z.ZodString;
        contentId: z.ZodString;
        id: z.ZodString;
        locale: z.ZodString;
        requiresValidation: z.ZodBoolean;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
    id: z.ZodString;
    primaryLocale: z.ZodString;
    translations: z.ZodArray<z.ZodType<z.objectOutputType<{
        aiGenerated: z.ZodBoolean;
        content: z.ZodString;
        contentId: z.ZodString;
        id: z.ZodString;
        locale: z.ZodString;
        requiresValidation: z.ZodBoolean;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
}, z.ZodTypeAny, "passthrough">>;
export type TextContentWithTranslations = z.infer<typeof _TextContentWithTranslations>;
export declare const TextContentWithTranslations: z.ZodType<TextContentWithTranslations, z.ZodTypeDef, any>;
declare const _UpdateTextContent: z.ZodObject<{
    format: z.ZodOptional<z.ZodUnion<[z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>, z.ZodNull]>>;
    primary_locale: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    format: z.ZodOptional<z.ZodUnion<[z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>, z.ZodNull]>>;
    primary_locale: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    format: z.ZodOptional<z.ZodUnion<[z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>, z.ZodNull]>>;
    primary_locale: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">>;
export type UpdateTextContent = z.infer<typeof _UpdateTextContent>;
export declare const UpdateTextContent: z.ZodType<UpdateTextContent, z.ZodTypeDef, any>;
declare const _UpdateTextTranslation: z.ZodObject<{
    ai_generated: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    content: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    locale: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    requires_validation: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    ai_generated: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    content: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    locale: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    requires_validation: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    ai_generated: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    content: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    locale: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    requires_validation: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">>;
export type UpdateTextTranslation = z.infer<typeof _UpdateTextTranslation>;
export declare const UpdateTextTranslation: z.ZodType<UpdateTextTranslation, z.ZodTypeDef, any>;
declare const _CreateOrUpdateTextTranslationRequest: z.ZodObject<{
    ai_generated: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    content: z.ZodString;
    requires_validation: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    ai_generated: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    content: z.ZodString;
    requires_validation: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    ai_generated: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    content: z.ZodString;
    requires_validation: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">>;
export type CreateOrUpdateTextTranslationRequest = z.infer<typeof _CreateOrUpdateTextTranslationRequest>;
export declare const CreateOrUpdateTextTranslationRequest: z.ZodType<CreateOrUpdateTextTranslationRequest, z.ZodTypeDef, any>;
declare const _GroupVoteCounts: z.ZodObject<{
    agrees: z.ZodNumber;
    disagrees: z.ZodNumber;
    group_id: z.ZodNumber;
    passes: z.ZodNumber;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    agrees: z.ZodNumber;
    disagrees: z.ZodNumber;
    group_id: z.ZodNumber;
    passes: z.ZodNumber;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    agrees: z.ZodNumber;
    disagrees: z.ZodNumber;
    group_id: z.ZodNumber;
    passes: z.ZodNumber;
}, z.ZodTypeAny, "passthrough">>;
export type GroupVoteCounts = z.infer<typeof _GroupVoteCounts>;
export declare const GroupVoteCounts: z.ZodType<GroupVoteCounts, z.ZodTypeDef, any>;
declare const _VoteCounts: z.ZodObject<{
    agrees: z.ZodNumber;
    disagrees: z.ZodNumber;
    passes: z.ZodNumber;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    agrees: z.ZodNumber;
    disagrees: z.ZodNumber;
    passes: z.ZodNumber;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    agrees: z.ZodNumber;
    disagrees: z.ZodNumber;
    passes: z.ZodNumber;
}, z.ZodTypeAny, "passthrough">>;
export type VoteCounts = z.infer<typeof _VoteCounts>;
export declare const VoteCounts: z.ZodType<VoteCounts, z.ZodTypeDef, any>;
declare const _CommentReportData: z.ZodObject<{
    divisiveness: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
    group_informed_consensus: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
    group_votes: z.ZodArray<z.ZodType<z.objectOutputType<{
        agrees: z.ZodNumber;
        disagrees: z.ZodNumber;
        group_id: z.ZodNumber;
        passes: z.ZodNumber;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    is_seed: z.ZodBoolean;
    overall_votes: z.ZodType<z.objectOutputType<{
        agrees: z.ZodNumber;
        disagrees: z.ZodNumber;
        passes: z.ZodNumber;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    text: z.ZodString;
    tid: z.ZodNumber;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    divisiveness: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
    group_informed_consensus: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
    group_votes: z.ZodArray<z.ZodType<z.objectOutputType<{
        agrees: z.ZodNumber;
        disagrees: z.ZodNumber;
        group_id: z.ZodNumber;
        passes: z.ZodNumber;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    is_seed: z.ZodBoolean;
    overall_votes: z.ZodType<z.objectOutputType<{
        agrees: z.ZodNumber;
        disagrees: z.ZodNumber;
        passes: z.ZodNumber;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    text: z.ZodString;
    tid: z.ZodNumber;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    divisiveness: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
    group_informed_consensus: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
    group_votes: z.ZodArray<z.ZodType<z.objectOutputType<{
        agrees: z.ZodNumber;
        disagrees: z.ZodNumber;
        group_id: z.ZodNumber;
        passes: z.ZodNumber;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    is_seed: z.ZodBoolean;
    overall_votes: z.ZodType<z.objectOutputType<{
        agrees: z.ZodNumber;
        disagrees: z.ZodNumber;
        passes: z.ZodNumber;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    text: z.ZodString;
    tid: z.ZodNumber;
}, z.ZodTypeAny, "passthrough">>;
export type CommentReportData = z.infer<typeof _CommentReportData>;
export declare const CommentReportData: z.ZodType<CommentReportData, z.ZodTypeDef, any>;
declare const _RepresentativeComment: z.ZodObject<{
    text: z.ZodString;
    tid: z.ZodNumber;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    text: z.ZodString;
    tid: z.ZodNumber;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    text: z.ZodString;
    tid: z.ZodNumber;
}, z.ZodTypeAny, "passthrough">>;
export type RepresentativeComment = z.infer<typeof _RepresentativeComment>;
export declare const RepresentativeComment: z.ZodType<RepresentativeComment, z.ZodTypeDef, any>;
declare const _GroupReportData: z.ZodObject<{
    group_id: z.ZodNumber;
    members: z.ZodArray<z.ZodNumber, "many">;
    representative_comments: z.ZodArray<z.ZodType<z.objectOutputType<{
        text: z.ZodString;
        tid: z.ZodNumber;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    total_members: z.ZodNumber;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    group_id: z.ZodNumber;
    members: z.ZodArray<z.ZodNumber, "many">;
    representative_comments: z.ZodArray<z.ZodType<z.objectOutputType<{
        text: z.ZodString;
        tid: z.ZodNumber;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    total_members: z.ZodNumber;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    group_id: z.ZodNumber;
    members: z.ZodArray<z.ZodNumber, "many">;
    representative_comments: z.ZodArray<z.ZodType<z.objectOutputType<{
        text: z.ZodString;
        tid: z.ZodNumber;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    total_members: z.ZodNumber;
}, z.ZodTypeAny, "passthrough">>;
export type GroupReportData = z.infer<typeof _GroupReportData>;
export declare const GroupReportData: z.ZodType<GroupReportData, z.ZodTypeDef, any>;
declare const _PcaPosition: z.ZodObject<{
    x: z.ZodNumber;
    y: z.ZodNumber;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    x: z.ZodNumber;
    y: z.ZodNumber;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    x: z.ZodNumber;
    y: z.ZodNumber;
}, z.ZodTypeAny, "passthrough">>;
export type PcaPosition = z.infer<typeof _PcaPosition>;
export declare const PcaPosition: z.ZodType<PcaPosition, z.ZodTypeDef, any>;
declare const _ParticipantReportData: z.ZodObject<{
    group_id: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
    pca_position: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
        x: z.ZodNumber;
        y: z.ZodNumber;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
    pid: z.ZodNumber;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    group_id: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
    pca_position: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
        x: z.ZodNumber;
        y: z.ZodNumber;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
    pid: z.ZodNumber;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    group_id: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
    pca_position: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
        x: z.ZodNumber;
        y: z.ZodNumber;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
    pid: z.ZodNumber;
}, z.ZodTypeAny, "passthrough">>;
export type ParticipantReportData = z.infer<typeof _ParticipantReportData>;
export declare const ParticipantReportData: z.ZodType<ParticipantReportData, z.ZodTypeDef, any>;
declare const _WikiPollReport: z.ZodObject<{
    comments: z.ZodArray<z.ZodType<z.objectOutputType<{
        divisiveness: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        group_informed_consensus: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        group_votes: z.ZodArray<z.ZodType<z.objectOutputType<{
            agrees: z.ZodNumber;
            disagrees: z.ZodNumber;
            group_id: z.ZodNumber;
            passes: z.ZodNumber;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        is_seed: z.ZodBoolean;
        overall_votes: z.ZodType<z.objectOutputType<{
            agrees: z.ZodNumber;
            disagrees: z.ZodNumber;
            passes: z.ZodNumber;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        text: z.ZodString;
        tid: z.ZodNumber;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    groups: z.ZodArray<z.ZodType<z.objectOutputType<{
        group_id: z.ZodNumber;
        members: z.ZodArray<z.ZodNumber, "many">;
        representative_comments: z.ZodArray<z.ZodType<z.objectOutputType<{
            text: z.ZodString;
            tid: z.ZodNumber;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        total_members: z.ZodNumber;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    participants: z.ZodArray<z.ZodType<z.objectOutputType<{
        group_id: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        pca_position: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
            x: z.ZodNumber;
            y: z.ZodNumber;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
        pid: z.ZodNumber;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    comments: z.ZodArray<z.ZodType<z.objectOutputType<{
        divisiveness: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        group_informed_consensus: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        group_votes: z.ZodArray<z.ZodType<z.objectOutputType<{
            agrees: z.ZodNumber;
            disagrees: z.ZodNumber;
            group_id: z.ZodNumber;
            passes: z.ZodNumber;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        is_seed: z.ZodBoolean;
        overall_votes: z.ZodType<z.objectOutputType<{
            agrees: z.ZodNumber;
            disagrees: z.ZodNumber;
            passes: z.ZodNumber;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        text: z.ZodString;
        tid: z.ZodNumber;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    groups: z.ZodArray<z.ZodType<z.objectOutputType<{
        group_id: z.ZodNumber;
        members: z.ZodArray<z.ZodNumber, "many">;
        representative_comments: z.ZodArray<z.ZodType<z.objectOutputType<{
            text: z.ZodString;
            tid: z.ZodNumber;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        total_members: z.ZodNumber;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    participants: z.ZodArray<z.ZodType<z.objectOutputType<{
        group_id: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        pca_position: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
            x: z.ZodNumber;
            y: z.ZodNumber;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
        pid: z.ZodNumber;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    comments: z.ZodArray<z.ZodType<z.objectOutputType<{
        divisiveness: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        group_informed_consensus: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        group_votes: z.ZodArray<z.ZodType<z.objectOutputType<{
            agrees: z.ZodNumber;
            disagrees: z.ZodNumber;
            group_id: z.ZodNumber;
            passes: z.ZodNumber;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        is_seed: z.ZodBoolean;
        overall_votes: z.ZodType<z.objectOutputType<{
            agrees: z.ZodNumber;
            disagrees: z.ZodNumber;
            passes: z.ZodNumber;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        text: z.ZodString;
        tid: z.ZodNumber;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    groups: z.ZodArray<z.ZodType<z.objectOutputType<{
        group_id: z.ZodNumber;
        members: z.ZodArray<z.ZodNumber, "many">;
        representative_comments: z.ZodArray<z.ZodType<z.objectOutputType<{
            text: z.ZodString;
            tid: z.ZodNumber;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        total_members: z.ZodNumber;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    participants: z.ZodArray<z.ZodType<z.objectOutputType<{
        group_id: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        pca_position: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
            x: z.ZodNumber;
            y: z.ZodNumber;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
        pid: z.ZodNumber;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
}, z.ZodTypeAny, "passthrough">>;
export type WikiPollReport = z.infer<typeof _WikiPollReport>;
export declare const WikiPollReport: z.ZodType<WikiPollReport, z.ZodTypeDef, any>;
declare const _VoteCountResponse: z.ZodObject<{
    vote_count: z.ZodNumber;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    vote_count: z.ZodNumber;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    vote_count: z.ZodNumber;
}, z.ZodTypeAny, "passthrough">>;
export type VoteCountResponse = z.infer<typeof _VoteCountResponse>;
export declare const VoteCountResponse: z.ZodType<VoteCountResponse, z.ZodTypeDef, any>;
declare const _UpdatePolisConfigRequest: z.ZodObject<{
    description: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    is_active: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    strict_moderation: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    topic: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    workflow_step_id: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    description: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    is_active: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    strict_moderation: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    topic: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    workflow_step_id: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    description: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    is_active: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    strict_moderation: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    topic: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    workflow_step_id: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type UpdatePolisConfigRequest = z.infer<typeof _UpdatePolisConfigRequest>;
export declare const UpdatePolisConfigRequest: z.ZodType<UpdatePolisConfigRequest, z.ZodTypeDef, any>;
declare const _WikiPoll: z.ZodObject<{
    is_active: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    poll_id: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    is_active: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    poll_id: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    is_active: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    poll_id: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type WikiPoll = z.infer<typeof _WikiPoll>;
export declare const WikiPoll: z.ZodType<WikiPoll, z.ZodTypeDef, any>;
declare const _PostSeedRequest: z.ZodObject<{
    statement_text: z.ZodString;
    workflow_step_id: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    statement_text: z.ZodString;
    workflow_step_id: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    statement_text: z.ZodString;
    workflow_step_id: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type PostSeedRequest = z.infer<typeof _PostSeedRequest>;
export declare const PostSeedRequest: z.ZodType<PostSeedRequest, z.ZodTypeDef, any>;
declare const _PostSeedResponse: z.ZodObject<{
    polis_statement_id: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    polis_statement_id: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    polis_statement_id: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type PostSeedResponse = z.infer<typeof _PostSeedResponse>;
export declare const PostSeedResponse: z.ZodType<PostSeedResponse, z.ZodTypeDef, any>;
declare const _ModerationStatus: z.ZodEnum<["accepted", "rejected", "pending"]>;
export type ModerationStatus = z.infer<typeof _ModerationStatus>;
export declare const ModerationStatus: z.ZodType<ModerationStatus, z.ZodTypeDef, any>;
declare const _PolisStatementAux: z.ZodObject<{
    created_at: z.ZodString;
    id: z.ZodString;
    is_seed: z.ZodBoolean;
    moderation_reason: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    moderation_status: z.ZodType<"accepted" | "rejected" | "pending", z.ZodTypeDef, any>;
    original_statement_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    polis_conversation_id: z.ZodString;
    polis_statement_id: z.ZodNumber;
    statement_text: z.ZodString;
    themes: z.ZodArray<z.ZodString, "many">;
    updated_at: z.ZodString;
    user_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    visible_statement_when_submitted: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    workflow_step_id: z.ZodString;
    zid: z.ZodNumber;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    created_at: z.ZodString;
    id: z.ZodString;
    is_seed: z.ZodBoolean;
    moderation_reason: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    moderation_status: z.ZodType<"accepted" | "rejected" | "pending", z.ZodTypeDef, any>;
    original_statement_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    polis_conversation_id: z.ZodString;
    polis_statement_id: z.ZodNumber;
    statement_text: z.ZodString;
    themes: z.ZodArray<z.ZodString, "many">;
    updated_at: z.ZodString;
    user_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    visible_statement_when_submitted: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    workflow_step_id: z.ZodString;
    zid: z.ZodNumber;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    created_at: z.ZodString;
    id: z.ZodString;
    is_seed: z.ZodBoolean;
    moderation_reason: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    moderation_status: z.ZodType<"accepted" | "rejected" | "pending", z.ZodTypeDef, any>;
    original_statement_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    polis_conversation_id: z.ZodString;
    polis_statement_id: z.ZodNumber;
    statement_text: z.ZodString;
    themes: z.ZodArray<z.ZodString, "many">;
    updated_at: z.ZodString;
    user_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    visible_statement_when_submitted: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    workflow_step_id: z.ZodString;
    zid: z.ZodNumber;
}, z.ZodTypeAny, "passthrough">>;
export type PolisStatementAux = z.infer<typeof _PolisStatementAux>;
export declare const PolisStatementAux: z.ZodType<PolisStatementAux, z.ZodTypeDef, any>;
declare const _CreatePolisStatementAux: z.ZodObject<{
    is_seed: z.ZodBoolean;
    moderation_reason: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    moderation_status: z.ZodOptional<z.ZodType<"accepted" | "rejected" | "pending", z.ZodTypeDef, any>>;
    polis_conversation_id: z.ZodString;
    polis_statement_id: z.ZodNumber;
    statement_text: z.ZodString;
    themes: z.ZodArray<z.ZodString, "many">;
    visible_statement_when_submitted: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    workflow_step_id: z.ZodString;
    zid: z.ZodNumber;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    is_seed: z.ZodBoolean;
    moderation_reason: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    moderation_status: z.ZodOptional<z.ZodType<"accepted" | "rejected" | "pending", z.ZodTypeDef, any>>;
    polis_conversation_id: z.ZodString;
    polis_statement_id: z.ZodNumber;
    statement_text: z.ZodString;
    themes: z.ZodArray<z.ZodString, "many">;
    visible_statement_when_submitted: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    workflow_step_id: z.ZodString;
    zid: z.ZodNumber;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    is_seed: z.ZodBoolean;
    moderation_reason: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    moderation_status: z.ZodOptional<z.ZodType<"accepted" | "rejected" | "pending", z.ZodTypeDef, any>>;
    polis_conversation_id: z.ZodString;
    polis_statement_id: z.ZodNumber;
    statement_text: z.ZodString;
    themes: z.ZodArray<z.ZodString, "many">;
    visible_statement_when_submitted: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    workflow_step_id: z.ZodString;
    zid: z.ZodNumber;
}, z.ZodTypeAny, "passthrough">>;
export type CreatePolisStatementAux = z.infer<typeof _CreatePolisStatementAux>;
export declare const CreatePolisStatementAux: z.ZodType<CreatePolisStatementAux, z.ZodTypeDef, any>;
declare const _UpdatePolisStatementAux: z.ZodObject<{
    moderation_reason: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    moderation_status: z.ZodOptional<z.ZodUnion<[z.ZodType<"accepted" | "rejected" | "pending", z.ZodTypeDef, any>, z.ZodNull]>>;
    statement_text: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    themes: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodString, "many">, z.ZodNull]>>;
    visible_statement_when_submitted: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    moderation_reason: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    moderation_status: z.ZodOptional<z.ZodUnion<[z.ZodType<"accepted" | "rejected" | "pending", z.ZodTypeDef, any>, z.ZodNull]>>;
    statement_text: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    themes: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodString, "many">, z.ZodNull]>>;
    visible_statement_when_submitted: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    moderation_reason: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    moderation_status: z.ZodOptional<z.ZodUnion<[z.ZodType<"accepted" | "rejected" | "pending", z.ZodTypeDef, any>, z.ZodNull]>>;
    statement_text: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    themes: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodString, "many">, z.ZodNull]>>;
    visible_statement_when_submitted: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">>;
export type UpdatePolisStatementAux = z.infer<typeof _UpdatePolisStatementAux>;
export declare const UpdatePolisStatementAux: z.ZodType<UpdatePolisStatementAux, z.ZodTypeDef, any>;
declare const _SyncStatementAuxRequest: z.ZodObject<{
    workflow_step_id: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    workflow_step_id: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    workflow_step_id: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type SyncStatementAuxRequest = z.infer<typeof _SyncStatementAuxRequest>;
export declare const SyncStatementAuxRequest: z.ZodType<SyncStatementAuxRequest, z.ZodTypeDef, any>;
declare const _SyncStatementAuxResponse: z.ZodObject<{
    skipped_invalid_xid: z.ZodNumber;
    statements: z.ZodArray<z.ZodType<z.objectOutputType<{
        created_at: z.ZodString;
        id: z.ZodString;
        is_seed: z.ZodBoolean;
        moderation_reason: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        moderation_status: z.ZodType<"accepted" | "rejected" | "pending", z.ZodTypeDef, any>;
        original_statement_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        polis_conversation_id: z.ZodString;
        polis_statement_id: z.ZodNumber;
        statement_text: z.ZodString;
        themes: z.ZodArray<z.ZodString, "many">;
        updated_at: z.ZodString;
        user_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        visible_statement_when_submitted: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        workflow_step_id: z.ZodString;
        zid: z.ZodNumber;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    synced: z.ZodNumber;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    skipped_invalid_xid: z.ZodNumber;
    statements: z.ZodArray<z.ZodType<z.objectOutputType<{
        created_at: z.ZodString;
        id: z.ZodString;
        is_seed: z.ZodBoolean;
        moderation_reason: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        moderation_status: z.ZodType<"accepted" | "rejected" | "pending", z.ZodTypeDef, any>;
        original_statement_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        polis_conversation_id: z.ZodString;
        polis_statement_id: z.ZodNumber;
        statement_text: z.ZodString;
        themes: z.ZodArray<z.ZodString, "many">;
        updated_at: z.ZodString;
        user_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        visible_statement_when_submitted: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        workflow_step_id: z.ZodString;
        zid: z.ZodNumber;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    synced: z.ZodNumber;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    skipped_invalid_xid: z.ZodNumber;
    statements: z.ZodArray<z.ZodType<z.objectOutputType<{
        created_at: z.ZodString;
        id: z.ZodString;
        is_seed: z.ZodBoolean;
        moderation_reason: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        moderation_status: z.ZodType<"accepted" | "rejected" | "pending", z.ZodTypeDef, any>;
        original_statement_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        polis_conversation_id: z.ZodString;
        polis_statement_id: z.ZodNumber;
        statement_text: z.ZodString;
        themes: z.ZodArray<z.ZodString, "many">;
        updated_at: z.ZodString;
        user_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        visible_statement_when_submitted: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        workflow_step_id: z.ZodString;
        zid: z.ZodNumber;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    synced: z.ZodNumber;
}, z.ZodTypeAny, "passthrough">>;
export type SyncStatementAuxResponse = z.infer<typeof _SyncStatementAuxResponse>;
export declare const SyncStatementAuxResponse: z.ZodType<SyncStatementAuxResponse, z.ZodTypeDef, any>;
declare const _ThemeStatistic: z.ZodObject<{
    count: z.ZodNumber;
    theme: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    count: z.ZodNumber;
    theme: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    count: z.ZodNumber;
    theme: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type ThemeStatistic = z.infer<typeof _ThemeStatistic>;
export declare const ThemeStatistic: z.ZodType<ThemeStatistic, z.ZodTypeDef, any>;
declare const _ThemeRequest: z.ZodObject<{
    theme: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    theme: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    theme: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type ThemeRequest = z.infer<typeof _ThemeRequest>;
export declare const ThemeRequest: z.ZodType<ThemeRequest, z.ZodTypeDef, any>;
declare const _ModerationDecisionRequest: z.ZodEnum<["accept", "reject"]>;
export type ModerationDecisionRequest = z.infer<typeof _ModerationDecisionRequest>;
export declare const ModerationDecisionRequest: z.ZodType<ModerationDecisionRequest, z.ZodTypeDef, any>;
declare const _ModerateStatementAuxRequest: z.ZodObject<{
    decision: z.ZodType<"accept" | "reject", z.ZodTypeDef, any>;
    moderation_reason: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    decision: z.ZodType<"accept" | "reject", z.ZodTypeDef, any>;
    moderation_reason: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    decision: z.ZodType<"accept" | "reject", z.ZodTypeDef, any>;
    moderation_reason: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">>;
export type ModerateStatementAuxRequest = z.infer<typeof _ModerateStatementAuxRequest>;
export declare const ModerateStatementAuxRequest: z.ZodType<ModerateStatementAuxRequest, z.ZodTypeDef, any>;
declare const _ModerateStatementAuxBatchRequest: z.ZodObject<{
    decision: z.ZodType<"accept" | "reject", z.ZodTypeDef, any>;
    ids: z.ZodArray<z.ZodString, "many">;
    moderation_reason: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    decision: z.ZodType<"accept" | "reject", z.ZodTypeDef, any>;
    ids: z.ZodArray<z.ZodString, "many">;
    moderation_reason: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    decision: z.ZodType<"accept" | "reject", z.ZodTypeDef, any>;
    ids: z.ZodArray<z.ZodString, "many">;
    moderation_reason: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">>;
export type ModerateStatementAuxBatchRequest = z.infer<typeof _ModerateStatementAuxBatchRequest>;
export declare const ModerateStatementAuxBatchRequest: z.ZodType<ModerateStatementAuxBatchRequest, z.ZodTypeDef, any>;
declare const _ModerateBatchFailure: z.ZodObject<{
    error: z.ZodString;
    id: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    error: z.ZodString;
    id: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    error: z.ZodString;
    id: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type ModerateBatchFailure = z.infer<typeof _ModerateBatchFailure>;
export declare const ModerateBatchFailure: z.ZodType<ModerateBatchFailure, z.ZodTypeDef, any>;
declare const _ModerateStatementAuxBatchResponse: z.ZodObject<{
    failed: z.ZodArray<z.ZodType<z.objectOutputType<{
        error: z.ZodString;
        id: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    succeeded: z.ZodArray<z.ZodType<z.objectOutputType<{
        created_at: z.ZodString;
        id: z.ZodString;
        is_seed: z.ZodBoolean;
        moderation_reason: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        moderation_status: z.ZodType<"accepted" | "rejected" | "pending", z.ZodTypeDef, any>;
        original_statement_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        polis_conversation_id: z.ZodString;
        polis_statement_id: z.ZodNumber;
        statement_text: z.ZodString;
        themes: z.ZodArray<z.ZodString, "many">;
        updated_at: z.ZodString;
        user_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        visible_statement_when_submitted: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        workflow_step_id: z.ZodString;
        zid: z.ZodNumber;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    failed: z.ZodArray<z.ZodType<z.objectOutputType<{
        error: z.ZodString;
        id: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    succeeded: z.ZodArray<z.ZodType<z.objectOutputType<{
        created_at: z.ZodString;
        id: z.ZodString;
        is_seed: z.ZodBoolean;
        moderation_reason: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        moderation_status: z.ZodType<"accepted" | "rejected" | "pending", z.ZodTypeDef, any>;
        original_statement_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        polis_conversation_id: z.ZodString;
        polis_statement_id: z.ZodNumber;
        statement_text: z.ZodString;
        themes: z.ZodArray<z.ZodString, "many">;
        updated_at: z.ZodString;
        user_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        visible_statement_when_submitted: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        workflow_step_id: z.ZodString;
        zid: z.ZodNumber;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    failed: z.ZodArray<z.ZodType<z.objectOutputType<{
        error: z.ZodString;
        id: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    succeeded: z.ZodArray<z.ZodType<z.objectOutputType<{
        created_at: z.ZodString;
        id: z.ZodString;
        is_seed: z.ZodBoolean;
        moderation_reason: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        moderation_status: z.ZodType<"accepted" | "rejected" | "pending", z.ZodTypeDef, any>;
        original_statement_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        polis_conversation_id: z.ZodString;
        polis_statement_id: z.ZodNumber;
        statement_text: z.ZodString;
        themes: z.ZodArray<z.ZodString, "many">;
        updated_at: z.ZodString;
        user_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        visible_statement_when_submitted: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        workflow_step_id: z.ZodString;
        zid: z.ZodNumber;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
}, z.ZodTypeAny, "passthrough">>;
export type ModerateStatementAuxBatchResponse = z.infer<typeof _ModerateStatementAuxBatchResponse>;
export declare const ModerateStatementAuxBatchResponse: z.ZodType<ModerateStatementAuxBatchResponse, z.ZodTypeDef, any>;
declare const _SplitStatementRequest: z.ZodObject<{
    replacements: z.ZodArray<z.ZodString, "many">;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    replacements: z.ZodArray<z.ZodString, "many">;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    replacements: z.ZodArray<z.ZodString, "many">;
}, z.ZodTypeAny, "passthrough">>;
export type SplitStatementRequest = z.infer<typeof _SplitStatementRequest>;
export declare const SplitStatementRequest: z.ZodType<SplitStatementRequest, z.ZodTypeDef, any>;
declare const _SplitStatementResponse: z.ZodObject<{
    original: z.ZodType<z.objectOutputType<{
        created_at: z.ZodString;
        id: z.ZodString;
        is_seed: z.ZodBoolean;
        moderation_reason: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        moderation_status: z.ZodType<"accepted" | "rejected" | "pending", z.ZodTypeDef, any>;
        original_statement_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        polis_conversation_id: z.ZodString;
        polis_statement_id: z.ZodNumber;
        statement_text: z.ZodString;
        themes: z.ZodArray<z.ZodString, "many">;
        updated_at: z.ZodString;
        user_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        visible_statement_when_submitted: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        workflow_step_id: z.ZodString;
        zid: z.ZodNumber;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    replacements: z.ZodArray<z.ZodType<z.objectOutputType<{
        created_at: z.ZodString;
        id: z.ZodString;
        is_seed: z.ZodBoolean;
        moderation_reason: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        moderation_status: z.ZodType<"accepted" | "rejected" | "pending", z.ZodTypeDef, any>;
        original_statement_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        polis_conversation_id: z.ZodString;
        polis_statement_id: z.ZodNumber;
        statement_text: z.ZodString;
        themes: z.ZodArray<z.ZodString, "many">;
        updated_at: z.ZodString;
        user_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        visible_statement_when_submitted: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        workflow_step_id: z.ZodString;
        zid: z.ZodNumber;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    original: z.ZodType<z.objectOutputType<{
        created_at: z.ZodString;
        id: z.ZodString;
        is_seed: z.ZodBoolean;
        moderation_reason: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        moderation_status: z.ZodType<"accepted" | "rejected" | "pending", z.ZodTypeDef, any>;
        original_statement_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        polis_conversation_id: z.ZodString;
        polis_statement_id: z.ZodNumber;
        statement_text: z.ZodString;
        themes: z.ZodArray<z.ZodString, "many">;
        updated_at: z.ZodString;
        user_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        visible_statement_when_submitted: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        workflow_step_id: z.ZodString;
        zid: z.ZodNumber;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    replacements: z.ZodArray<z.ZodType<z.objectOutputType<{
        created_at: z.ZodString;
        id: z.ZodString;
        is_seed: z.ZodBoolean;
        moderation_reason: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        moderation_status: z.ZodType<"accepted" | "rejected" | "pending", z.ZodTypeDef, any>;
        original_statement_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        polis_conversation_id: z.ZodString;
        polis_statement_id: z.ZodNumber;
        statement_text: z.ZodString;
        themes: z.ZodArray<z.ZodString, "many">;
        updated_at: z.ZodString;
        user_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        visible_statement_when_submitted: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        workflow_step_id: z.ZodString;
        zid: z.ZodNumber;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    original: z.ZodType<z.objectOutputType<{
        created_at: z.ZodString;
        id: z.ZodString;
        is_seed: z.ZodBoolean;
        moderation_reason: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        moderation_status: z.ZodType<"accepted" | "rejected" | "pending", z.ZodTypeDef, any>;
        original_statement_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        polis_conversation_id: z.ZodString;
        polis_statement_id: z.ZodNumber;
        statement_text: z.ZodString;
        themes: z.ZodArray<z.ZodString, "many">;
        updated_at: z.ZodString;
        user_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        visible_statement_when_submitted: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        workflow_step_id: z.ZodString;
        zid: z.ZodNumber;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    replacements: z.ZodArray<z.ZodType<z.objectOutputType<{
        created_at: z.ZodString;
        id: z.ZodString;
        is_seed: z.ZodBoolean;
        moderation_reason: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        moderation_status: z.ZodType<"accepted" | "rejected" | "pending", z.ZodTypeDef, any>;
        original_statement_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        polis_conversation_id: z.ZodString;
        polis_statement_id: z.ZodNumber;
        statement_text: z.ZodString;
        themes: z.ZodArray<z.ZodString, "many">;
        updated_at: z.ZodString;
        user_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        visible_statement_when_submitted: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        workflow_step_id: z.ZodString;
        zid: z.ZodNumber;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
}, z.ZodTypeAny, "passthrough">>;
export type SplitStatementResponse = z.infer<typeof _SplitStatementResponse>;
export declare const SplitStatementResponse: z.ZodType<SplitStatementResponse, z.ZodTypeDef, any>;
declare const _FormField: z.ZodObject<{
    description: z.ZodOptional<z.ZodUnknown>;
    frozen: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    hide: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    id: z.ZodString;
    kind: z.ZodString;
    layout: z.ZodOptional<z.ZodUnion<[z.ZodObject<{}, "passthrough", z.ZodTypeAny, z.objectOutputType<{}, z.ZodTypeAny, "passthrough">, z.objectInputType<{}, z.ZodTypeAny, "passthrough">>, z.ZodNull]>>;
    properties: z.ZodOptional<z.ZodUnion<[z.ZodObject<{}, "passthrough", z.ZodTypeAny, z.objectOutputType<{}, z.ZodTypeAny, "passthrough">, z.objectInputType<{}, z.ZodTypeAny, "passthrough">>, z.ZodNull]>>;
    title: z.ZodOptional<z.ZodUnknown>;
    validations: z.ZodOptional<z.ZodUnion<[z.ZodObject<{}, "passthrough", z.ZodTypeAny, z.objectOutputType<{}, z.ZodTypeAny, "passthrough">, z.objectInputType<{}, z.ZodTypeAny, "passthrough">>, z.ZodNull]>>;
    width: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    description: z.ZodOptional<z.ZodUnknown>;
    frozen: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    hide: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    id: z.ZodString;
    kind: z.ZodString;
    layout: z.ZodOptional<z.ZodUnion<[z.ZodObject<{}, "passthrough", z.ZodTypeAny, z.objectOutputType<{}, z.ZodTypeAny, "passthrough">, z.objectInputType<{}, z.ZodTypeAny, "passthrough">>, z.ZodNull]>>;
    properties: z.ZodOptional<z.ZodUnion<[z.ZodObject<{}, "passthrough", z.ZodTypeAny, z.objectOutputType<{}, z.ZodTypeAny, "passthrough">, z.objectInputType<{}, z.ZodTypeAny, "passthrough">>, z.ZodNull]>>;
    title: z.ZodOptional<z.ZodUnknown>;
    validations: z.ZodOptional<z.ZodUnion<[z.ZodObject<{}, "passthrough", z.ZodTypeAny, z.objectOutputType<{}, z.ZodTypeAny, "passthrough">, z.objectInputType<{}, z.ZodTypeAny, "passthrough">>, z.ZodNull]>>;
    width: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    description: z.ZodOptional<z.ZodUnknown>;
    frozen: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    hide: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    id: z.ZodString;
    kind: z.ZodString;
    layout: z.ZodOptional<z.ZodUnion<[z.ZodObject<{}, "passthrough", z.ZodTypeAny, z.objectOutputType<{}, z.ZodTypeAny, "passthrough">, z.objectInputType<{}, z.ZodTypeAny, "passthrough">>, z.ZodNull]>>;
    properties: z.ZodOptional<z.ZodUnion<[z.ZodObject<{}, "passthrough", z.ZodTypeAny, z.objectOutputType<{}, z.ZodTypeAny, "passthrough">, z.objectInputType<{}, z.ZodTypeAny, "passthrough">>, z.ZodNull]>>;
    title: z.ZodOptional<z.ZodUnknown>;
    validations: z.ZodOptional<z.ZodUnion<[z.ZodObject<{}, "passthrough", z.ZodTypeAny, z.objectOutputType<{}, z.ZodTypeAny, "passthrough">, z.objectInputType<{}, z.ZodTypeAny, "passthrough">>, z.ZodNull]>>;
    width: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">>;
export type FormField = z.infer<typeof _FormField>;
export declare const FormField: z.ZodType<FormField, z.ZodTypeDef, any>;
declare const _FormSettings: z.ZodObject<{
    active: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    allowArchive: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    enableQuestionList: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    locale: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    published: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    active: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    allowArchive: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    enableQuestionList: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    locale: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    published: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    active: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    allowArchive: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    enableQuestionList: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    locale: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    published: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">>;
export type FormSettings = z.infer<typeof _FormSettings>;
export declare const FormSettings: z.ZodType<FormSettings, z.ZodTypeDef, any>;
declare const _FormTheme: z.ZodObject<{
    answerTextColor: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    backgroundBrightness: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
    backgroundColor: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    backgroundImage: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    buttonBackground: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    buttonTextColor: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    customCSS: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    fontFamily: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    logo: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    questionTextColor: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    answerTextColor: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    backgroundBrightness: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
    backgroundColor: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    backgroundImage: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    buttonBackground: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    buttonTextColor: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    customCSS: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    fontFamily: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    logo: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    questionTextColor: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    answerTextColor: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    backgroundBrightness: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
    backgroundColor: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    backgroundImage: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    buttonBackground: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    buttonTextColor: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    customCSS: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    fontFamily: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    logo: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    questionTextColor: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">>;
export type FormTheme = z.infer<typeof _FormTheme>;
export declare const FormTheme: z.ZodType<FormTheme, z.ZodTypeDef, any>;
declare const _ThemeSettings: z.ZodObject<{
    theme: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
        answerTextColor: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        backgroundBrightness: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        backgroundColor: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        backgroundImage: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        buttonBackground: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        buttonTextColor: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        customCSS: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        fontFamily: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        logo: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        questionTextColor: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    theme: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
        answerTextColor: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        backgroundBrightness: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        backgroundColor: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        backgroundImage: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        buttonBackground: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        buttonTextColor: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        customCSS: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        fontFamily: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        logo: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        questionTextColor: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    theme: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
        answerTextColor: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        backgroundBrightness: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        backgroundColor: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        backgroundImage: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        buttonBackground: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        buttonTextColor: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        customCSS: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        fontFamily: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        logo: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        questionTextColor: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">>;
export type ThemeSettings = z.infer<typeof _ThemeSettings>;
export declare const ThemeSettings: z.ZodType<ThemeSettings, z.ZodTypeDef, any>;
declare const _Form: z.ZodObject<{
    description: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    draft: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    fields: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodType<z.objectOutputType<{
        description: z.ZodOptional<z.ZodUnknown>;
        frozen: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
        hide: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
        id: z.ZodString;
        kind: z.ZodString;
        layout: z.ZodOptional<z.ZodUnion<[z.ZodObject<{}, "passthrough", z.ZodTypeAny, z.objectOutputType<{}, z.ZodTypeAny, "passthrough">, z.objectInputType<{}, z.ZodTypeAny, "passthrough">>, z.ZodNull]>>;
        properties: z.ZodOptional<z.ZodUnion<[z.ZodObject<{}, "passthrough", z.ZodTypeAny, z.objectOutputType<{}, z.ZodTypeAny, "passthrough">, z.objectInputType<{}, z.ZodTypeAny, "passthrough">>, z.ZodNull]>>;
        title: z.ZodOptional<z.ZodUnknown>;
        validations: z.ZodOptional<z.ZodUnion<[z.ZodObject<{}, "passthrough", z.ZodTypeAny, z.objectOutputType<{}, z.ZodTypeAny, "passthrough">, z.objectInputType<{}, z.ZodTypeAny, "passthrough">>, z.ZodNull]>>;
        width: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">, z.ZodNull]>>;
    id: z.ZodString;
    interactiveMode: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
    kind: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
    name: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    projectId: z.ZodString;
    settings: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
        active: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
        allowArchive: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
        enableQuestionList: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
        locale: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        published: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
    status: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
    teamId: z.ZodString;
    themeSettings: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
        theme: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
            answerTextColor: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
            backgroundBrightness: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
            backgroundColor: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
            backgroundImage: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
            buttonBackground: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
            buttonTextColor: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
            customCSS: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
            fontFamily: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
            logo: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
            questionTextColor: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    description: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    draft: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    fields: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodType<z.objectOutputType<{
        description: z.ZodOptional<z.ZodUnknown>;
        frozen: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
        hide: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
        id: z.ZodString;
        kind: z.ZodString;
        layout: z.ZodOptional<z.ZodUnion<[z.ZodObject<{}, "passthrough", z.ZodTypeAny, z.objectOutputType<{}, z.ZodTypeAny, "passthrough">, z.objectInputType<{}, z.ZodTypeAny, "passthrough">>, z.ZodNull]>>;
        properties: z.ZodOptional<z.ZodUnion<[z.ZodObject<{}, "passthrough", z.ZodTypeAny, z.objectOutputType<{}, z.ZodTypeAny, "passthrough">, z.objectInputType<{}, z.ZodTypeAny, "passthrough">>, z.ZodNull]>>;
        title: z.ZodOptional<z.ZodUnknown>;
        validations: z.ZodOptional<z.ZodUnion<[z.ZodObject<{}, "passthrough", z.ZodTypeAny, z.objectOutputType<{}, z.ZodTypeAny, "passthrough">, z.objectInputType<{}, z.ZodTypeAny, "passthrough">>, z.ZodNull]>>;
        width: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">, z.ZodNull]>>;
    id: z.ZodString;
    interactiveMode: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
    kind: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
    name: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    projectId: z.ZodString;
    settings: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
        active: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
        allowArchive: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
        enableQuestionList: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
        locale: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        published: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
    status: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
    teamId: z.ZodString;
    themeSettings: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
        theme: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
            answerTextColor: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
            backgroundBrightness: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
            backgroundColor: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
            backgroundImage: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
            buttonBackground: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
            buttonTextColor: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
            customCSS: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
            fontFamily: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
            logo: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
            questionTextColor: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    description: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    draft: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    fields: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodType<z.objectOutputType<{
        description: z.ZodOptional<z.ZodUnknown>;
        frozen: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
        hide: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
        id: z.ZodString;
        kind: z.ZodString;
        layout: z.ZodOptional<z.ZodUnion<[z.ZodObject<{}, "passthrough", z.ZodTypeAny, z.objectOutputType<{}, z.ZodTypeAny, "passthrough">, z.objectInputType<{}, z.ZodTypeAny, "passthrough">>, z.ZodNull]>>;
        properties: z.ZodOptional<z.ZodUnion<[z.ZodObject<{}, "passthrough", z.ZodTypeAny, z.objectOutputType<{}, z.ZodTypeAny, "passthrough">, z.objectInputType<{}, z.ZodTypeAny, "passthrough">>, z.ZodNull]>>;
        title: z.ZodOptional<z.ZodUnknown>;
        validations: z.ZodOptional<z.ZodUnion<[z.ZodObject<{}, "passthrough", z.ZodTypeAny, z.objectOutputType<{}, z.ZodTypeAny, "passthrough">, z.objectInputType<{}, z.ZodTypeAny, "passthrough">>, z.ZodNull]>>;
        width: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">, z.ZodNull]>>;
    id: z.ZodString;
    interactiveMode: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
    kind: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
    name: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    projectId: z.ZodString;
    settings: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
        active: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
        allowArchive: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
        enableQuestionList: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
        locale: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        published: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
    status: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
    teamId: z.ZodString;
    themeSettings: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
        theme: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
            answerTextColor: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
            backgroundBrightness: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
            backgroundColor: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
            backgroundImage: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
            buttonBackground: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
            buttonTextColor: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
            customCSS: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
            fontFamily: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
            logo: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
            questionTextColor: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">>;
export type Form = z.infer<typeof _Form>;
export declare const Form: z.ZodType<Form, z.ZodTypeDef, any>;
declare const _FormReportResponse: z.ZodObject<{
    average: z.ZodNumber;
    chooses: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodUnknown, "many">, z.ZodNull]>>;
    count: z.ZodNumber;
    id: z.ZodString;
    kind: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    title: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    total: z.ZodNumber;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    average: z.ZodNumber;
    chooses: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodUnknown, "many">, z.ZodNull]>>;
    count: z.ZodNumber;
    id: z.ZodString;
    kind: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    title: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    total: z.ZodNumber;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    average: z.ZodNumber;
    chooses: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodUnknown, "many">, z.ZodNull]>>;
    count: z.ZodNumber;
    id: z.ZodString;
    kind: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    title: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    total: z.ZodNumber;
}, z.ZodTypeAny, "passthrough">>;
export type FormReportResponse = z.infer<typeof _FormReportResponse>;
export declare const FormReportResponse: z.ZodType<FormReportResponse, z.ZodTypeDef, any>;
declare const _FormReportAnswer: z.ZodObject<{
    endAt: z.ZodNumber;
    kind: z.ZodString;
    submissionId: z.ZodString;
    value: z.ZodOptional<z.ZodUnknown>;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    endAt: z.ZodNumber;
    kind: z.ZodString;
    submissionId: z.ZodString;
    value: z.ZodOptional<z.ZodUnknown>;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    endAt: z.ZodNumber;
    kind: z.ZodString;
    submissionId: z.ZodString;
    value: z.ZodOptional<z.ZodUnknown>;
}, z.ZodTypeAny, "passthrough">>;
export type FormReportAnswer = z.infer<typeof _FormReportAnswer>;
export declare const FormReportAnswer: z.ZodType<FormReportAnswer, z.ZodTypeDef, any>;
declare const _FormReportSubmission: z.ZodObject<{
    _id: z.ZodString;
    answers: z.ZodArray<z.ZodType<z.objectOutputType<{
        endAt: z.ZodNumber;
        kind: z.ZodString;
        submissionId: z.ZodString;
        value: z.ZodOptional<z.ZodUnknown>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    _id: z.ZodString;
    answers: z.ZodArray<z.ZodType<z.objectOutputType<{
        endAt: z.ZodNumber;
        kind: z.ZodString;
        submissionId: z.ZodString;
        value: z.ZodOptional<z.ZodUnknown>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    _id: z.ZodString;
    answers: z.ZodArray<z.ZodType<z.objectOutputType<{
        endAt: z.ZodNumber;
        kind: z.ZodString;
        submissionId: z.ZodString;
        value: z.ZodOptional<z.ZodUnknown>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
}, z.ZodTypeAny, "passthrough">>;
export type FormReportSubmission = z.infer<typeof _FormReportSubmission>;
export declare const FormReportSubmission: z.ZodType<FormReportSubmission, z.ZodTypeDef, any>;
declare const _FormReport: z.ZodObject<{
    responses: z.ZodArray<z.ZodType<z.objectOutputType<{
        average: z.ZodNumber;
        chooses: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodUnknown, "many">, z.ZodNull]>>;
        count: z.ZodNumber;
        id: z.ZodString;
        kind: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        title: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        total: z.ZodNumber;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    submissions: z.ZodArray<z.ZodType<z.objectOutputType<{
        _id: z.ZodString;
        answers: z.ZodArray<z.ZodType<z.objectOutputType<{
            endAt: z.ZodNumber;
            kind: z.ZodString;
            submissionId: z.ZodString;
            value: z.ZodOptional<z.ZodUnknown>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    responses: z.ZodArray<z.ZodType<z.objectOutputType<{
        average: z.ZodNumber;
        chooses: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodUnknown, "many">, z.ZodNull]>>;
        count: z.ZodNumber;
        id: z.ZodString;
        kind: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        title: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        total: z.ZodNumber;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    submissions: z.ZodArray<z.ZodType<z.objectOutputType<{
        _id: z.ZodString;
        answers: z.ZodArray<z.ZodType<z.objectOutputType<{
            endAt: z.ZodNumber;
            kind: z.ZodString;
            submissionId: z.ZodString;
            value: z.ZodOptional<z.ZodUnknown>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    responses: z.ZodArray<z.ZodType<z.objectOutputType<{
        average: z.ZodNumber;
        chooses: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodUnknown, "many">, z.ZodNull]>>;
        count: z.ZodNumber;
        id: z.ZodString;
        kind: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        title: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        total: z.ZodNumber;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    submissions: z.ZodArray<z.ZodType<z.objectOutputType<{
        _id: z.ZodString;
        answers: z.ZodArray<z.ZodType<z.objectOutputType<{
            endAt: z.ZodNumber;
            kind: z.ZodString;
            submissionId: z.ZodString;
            value: z.ZodOptional<z.ZodUnknown>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
}, z.ZodTypeAny, "passthrough">>;
export type FormReport = z.infer<typeof _FormReport>;
export declare const FormReport: z.ZodType<FormReport, z.ZodTypeDef, any>;
declare const _SubmissionCategory: z.ZodEnum<["inbox", "spam", "starred", "archive"]>;
export type SubmissionCategory = z.infer<typeof _SubmissionCategory>;
export declare const SubmissionCategory: z.ZodType<SubmissionCategory, z.ZodTypeDef, any>;
declare const _HiddenFieldAnswer: z.ZodObject<{
    id: z.ZodString;
    name: z.ZodString;
    value: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    id: z.ZodString;
    name: z.ZodString;
    value: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    id: z.ZodString;
    name: z.ZodString;
    value: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">>;
export type HiddenFieldAnswer = z.infer<typeof _HiddenFieldAnswer>;
export declare const HiddenFieldAnswer: z.ZodType<HiddenFieldAnswer, z.ZodTypeDef, any>;
declare const _Submission: z.ZodObject<{
    answers: z.ZodArray<z.ZodObject<{}, "passthrough", z.ZodTypeAny, z.objectOutputType<{}, z.ZodTypeAny, "passthrough">, z.objectInputType<{}, z.ZodTypeAny, "passthrough">>, "many">;
    category: z.ZodOptional<z.ZodUnion<[z.ZodType<"inbox" | "spam" | "starred" | "archive", z.ZodTypeDef, any>, z.ZodNull]>>;
    endAt: z.ZodNumber;
    hiddenFields: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodType<z.objectOutputType<{
        id: z.ZodString;
        name: z.ZodString;
        value: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">, z.ZodNull]>>;
    id: z.ZodString;
    title: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    variables: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodUnknown, "many">, z.ZodNull]>>;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    answers: z.ZodArray<z.ZodObject<{}, "passthrough", z.ZodTypeAny, z.objectOutputType<{}, z.ZodTypeAny, "passthrough">, z.objectInputType<{}, z.ZodTypeAny, "passthrough">>, "many">;
    category: z.ZodOptional<z.ZodUnion<[z.ZodType<"inbox" | "spam" | "starred" | "archive", z.ZodTypeDef, any>, z.ZodNull]>>;
    endAt: z.ZodNumber;
    hiddenFields: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodType<z.objectOutputType<{
        id: z.ZodString;
        name: z.ZodString;
        value: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">, z.ZodNull]>>;
    id: z.ZodString;
    title: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    variables: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodUnknown, "many">, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    answers: z.ZodArray<z.ZodObject<{}, "passthrough", z.ZodTypeAny, z.objectOutputType<{}, z.ZodTypeAny, "passthrough">, z.objectInputType<{}, z.ZodTypeAny, "passthrough">>, "many">;
    category: z.ZodOptional<z.ZodUnion<[z.ZodType<"inbox" | "spam" | "starred" | "archive", z.ZodTypeDef, any>, z.ZodNull]>>;
    endAt: z.ZodNumber;
    hiddenFields: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodType<z.objectOutputType<{
        id: z.ZodString;
        name: z.ZodString;
        value: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">, z.ZodNull]>>;
    id: z.ZodString;
    title: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    variables: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodUnknown, "many">, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">>;
export type Submission = z.infer<typeof _Submission>;
export declare const Submission: z.ZodType<Submission, z.ZodTypeDef, any>;
declare const _Submissions: z.ZodObject<{
    submissions: z.ZodArray<z.ZodType<z.objectOutputType<{
        answers: z.ZodArray<z.ZodObject<{}, "passthrough", z.ZodTypeAny, z.objectOutputType<{}, z.ZodTypeAny, "passthrough">, z.objectInputType<{}, z.ZodTypeAny, "passthrough">>, "many">;
        category: z.ZodOptional<z.ZodUnion<[z.ZodType<"inbox" | "spam" | "starred" | "archive", z.ZodTypeDef, any>, z.ZodNull]>>;
        endAt: z.ZodNumber;
        hiddenFields: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            name: z.ZodString;
            value: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">, z.ZodNull]>>;
        id: z.ZodString;
        title: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        variables: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodUnknown, "many">, z.ZodNull]>>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    total: z.ZodNumber;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    submissions: z.ZodArray<z.ZodType<z.objectOutputType<{
        answers: z.ZodArray<z.ZodObject<{}, "passthrough", z.ZodTypeAny, z.objectOutputType<{}, z.ZodTypeAny, "passthrough">, z.objectInputType<{}, z.ZodTypeAny, "passthrough">>, "many">;
        category: z.ZodOptional<z.ZodUnion<[z.ZodType<"inbox" | "spam" | "starred" | "archive", z.ZodTypeDef, any>, z.ZodNull]>>;
        endAt: z.ZodNumber;
        hiddenFields: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            name: z.ZodString;
            value: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">, z.ZodNull]>>;
        id: z.ZodString;
        title: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        variables: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodUnknown, "many">, z.ZodNull]>>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    total: z.ZodNumber;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    submissions: z.ZodArray<z.ZodType<z.objectOutputType<{
        answers: z.ZodArray<z.ZodObject<{}, "passthrough", z.ZodTypeAny, z.objectOutputType<{}, z.ZodTypeAny, "passthrough">, z.objectInputType<{}, z.ZodTypeAny, "passthrough">>, "many">;
        category: z.ZodOptional<z.ZodUnion<[z.ZodType<"inbox" | "spam" | "starred" | "archive", z.ZodTypeDef, any>, z.ZodNull]>>;
        endAt: z.ZodNumber;
        hiddenFields: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            name: z.ZodString;
            value: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">, z.ZodNull]>>;
        id: z.ZodString;
        title: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        variables: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodUnknown, "many">, z.ZodNull]>>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    total: z.ZodNumber;
}, z.ZodTypeAny, "passthrough">>;
export type Submissions = z.infer<typeof _Submissions>;
export declare const Submissions: z.ZodType<Submissions, z.ZodTypeDef, any>;
declare const _InsightChoice: z.ZodObject<{
    count: z.ZodNumber;
    id: z.ZodString;
    label: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    count: z.ZodNumber;
    id: z.ZodString;
    label: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    count: z.ZodNumber;
    id: z.ZodString;
    label: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type InsightChoice = z.infer<typeof _InsightChoice>;
export declare const InsightChoice: z.ZodType<InsightChoice, z.ZodTypeDef, any>;
declare const _InsightSubmission: z.ZodObject<{
    submission_id: z.ZodString;
    submitted_at: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
    value: z.ZodUnknown;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    submission_id: z.ZodString;
    submitted_at: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
    value: z.ZodUnknown;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    submission_id: z.ZodString;
    submitted_at: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
    value: z.ZodUnknown;
}, z.ZodTypeAny, "passthrough">>;
export type InsightSubmission = z.infer<typeof _InsightSubmission>;
export declare const InsightSubmission: z.ZodType<InsightSubmission, z.ZodTypeDef, any>;
declare const _InsightQuestion: z.ZodObject<{
    answered: z.ZodNumber;
    choices: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodType<z.objectOutputType<{
        count: z.ZodNumber;
        id: z.ZodString;
        label: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">, z.ZodNull]>>;
    id: z.ZodString;
    kind: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    properties: z.ZodOptional<z.ZodUnion<[z.ZodObject<{}, "passthrough", z.ZodTypeAny, z.objectOutputType<{}, z.ZodTypeAny, "passthrough">, z.objectInputType<{}, z.ZodTypeAny, "passthrough">>, z.ZodNull]>>;
    submissions: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodType<z.objectOutputType<{
        submission_id: z.ZodString;
        submitted_at: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        value: z.ZodUnknown;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">, z.ZodNull]>>;
    title: z.ZodString;
    total: z.ZodNumber;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    answered: z.ZodNumber;
    choices: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodType<z.objectOutputType<{
        count: z.ZodNumber;
        id: z.ZodString;
        label: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">, z.ZodNull]>>;
    id: z.ZodString;
    kind: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    properties: z.ZodOptional<z.ZodUnion<[z.ZodObject<{}, "passthrough", z.ZodTypeAny, z.objectOutputType<{}, z.ZodTypeAny, "passthrough">, z.objectInputType<{}, z.ZodTypeAny, "passthrough">>, z.ZodNull]>>;
    submissions: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodType<z.objectOutputType<{
        submission_id: z.ZodString;
        submitted_at: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        value: z.ZodUnknown;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">, z.ZodNull]>>;
    title: z.ZodString;
    total: z.ZodNumber;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    answered: z.ZodNumber;
    choices: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodType<z.objectOutputType<{
        count: z.ZodNumber;
        id: z.ZodString;
        label: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">, z.ZodNull]>>;
    id: z.ZodString;
    kind: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    properties: z.ZodOptional<z.ZodUnion<[z.ZodObject<{}, "passthrough", z.ZodTypeAny, z.objectOutputType<{}, z.ZodTypeAny, "passthrough">, z.objectInputType<{}, z.ZodTypeAny, "passthrough">>, z.ZodNull]>>;
    submissions: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodType<z.objectOutputType<{
        submission_id: z.ZodString;
        submitted_at: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        value: z.ZodUnknown;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">, z.ZodNull]>>;
    title: z.ZodString;
    total: z.ZodNumber;
}, z.ZodTypeAny, "passthrough">>;
export type InsightQuestion = z.infer<typeof _InsightQuestion>;
export declare const InsightQuestion: z.ZodType<InsightQuestion, z.ZodTypeDef, any>;
declare const _SurveyInsights: z.ZodObject<{
    questions: z.ZodArray<z.ZodType<z.objectOutputType<{
        answered: z.ZodNumber;
        choices: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodType<z.objectOutputType<{
            count: z.ZodNumber;
            id: z.ZodString;
            label: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">, z.ZodNull]>>;
        id: z.ZodString;
        kind: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        properties: z.ZodOptional<z.ZodUnion<[z.ZodObject<{}, "passthrough", z.ZodTypeAny, z.objectOutputType<{}, z.ZodTypeAny, "passthrough">, z.objectInputType<{}, z.ZodTypeAny, "passthrough">>, z.ZodNull]>>;
        submissions: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodType<z.objectOutputType<{
            submission_id: z.ZodString;
            submitted_at: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
            value: z.ZodUnknown;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">, z.ZodNull]>>;
        title: z.ZodString;
        total: z.ZodNumber;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    questions: z.ZodArray<z.ZodType<z.objectOutputType<{
        answered: z.ZodNumber;
        choices: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodType<z.objectOutputType<{
            count: z.ZodNumber;
            id: z.ZodString;
            label: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">, z.ZodNull]>>;
        id: z.ZodString;
        kind: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        properties: z.ZodOptional<z.ZodUnion<[z.ZodObject<{}, "passthrough", z.ZodTypeAny, z.objectOutputType<{}, z.ZodTypeAny, "passthrough">, z.objectInputType<{}, z.ZodTypeAny, "passthrough">>, z.ZodNull]>>;
        submissions: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodType<z.objectOutputType<{
            submission_id: z.ZodString;
            submitted_at: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
            value: z.ZodUnknown;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">, z.ZodNull]>>;
        title: z.ZodString;
        total: z.ZodNumber;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    questions: z.ZodArray<z.ZodType<z.objectOutputType<{
        answered: z.ZodNumber;
        choices: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodType<z.objectOutputType<{
            count: z.ZodNumber;
            id: z.ZodString;
            label: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">, z.ZodNull]>>;
        id: z.ZodString;
        kind: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        properties: z.ZodOptional<z.ZodUnion<[z.ZodObject<{}, "passthrough", z.ZodTypeAny, z.objectOutputType<{}, z.ZodTypeAny, "passthrough">, z.objectInputType<{}, z.ZodTypeAny, "passthrough">>, z.ZodNull]>>;
        submissions: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodType<z.objectOutputType<{
            submission_id: z.ZodString;
            submitted_at: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
            value: z.ZodUnknown;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">, z.ZodNull]>>;
        title: z.ZodString;
        total: z.ZodNumber;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
}, z.ZodTypeAny, "passthrough">>;
export type SurveyInsights = z.infer<typeof _SurveyInsights>;
export declare const SurveyInsights: z.ZodType<SurveyInsights, z.ZodTypeDef, any>;
declare const _Story: z.ZodObject<{
    id: z.ZodString;
    transcript_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    user_id: z.ZodString;
    video_id: z.ZodString;
    workflow_step_id: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    id: z.ZodString;
    transcript_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    user_id: z.ZodString;
    video_id: z.ZodString;
    workflow_step_id: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    id: z.ZodString;
    transcript_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    user_id: z.ZodString;
    video_id: z.ZodString;
    workflow_step_id: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type Story = z.infer<typeof _Story>;
export declare const Story: z.ZodType<Story, z.ZodTypeDef, any>;
declare const _ComhairleMessageReference: z.ZodObject<{
    content: z.ZodString;
    dataset_id: z.ZodString;
    document_id: z.ZodString;
    document_name: z.ZodString;
    id: z.ZodString;
    positions: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodArray<z.ZodNumber, "many">, "many">, z.ZodNull]>>;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    content: z.ZodString;
    dataset_id: z.ZodString;
    document_id: z.ZodString;
    document_name: z.ZodString;
    id: z.ZodString;
    positions: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodArray<z.ZodNumber, "many">, "many">, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    content: z.ZodString;
    dataset_id: z.ZodString;
    document_id: z.ZodString;
    document_name: z.ZodString;
    id: z.ZodString;
    positions: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodArray<z.ZodNumber, "many">, "many">, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">>;
export type ComhairleMessageReference = z.infer<typeof _ComhairleMessageReference>;
export declare const ComhairleMessageReference: z.ZodType<ComhairleMessageReference, z.ZodTypeDef, any>;
declare const _ComhairleSessionMessage: z.ZodObject<{
    content: z.ZodString;
    id: z.ZodString;
    reference: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodType<z.objectOutputType<{
        content: z.ZodString;
        dataset_id: z.ZodString;
        document_id: z.ZodString;
        document_name: z.ZodString;
        id: z.ZodString;
        positions: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodArray<z.ZodNumber, "many">, "many">, z.ZodNull]>>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">, z.ZodNull]>>;
    role: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    content: z.ZodString;
    id: z.ZodString;
    reference: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodType<z.objectOutputType<{
        content: z.ZodString;
        dataset_id: z.ZodString;
        document_id: z.ZodString;
        document_name: z.ZodString;
        id: z.ZodString;
        positions: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodArray<z.ZodNumber, "many">, "many">, z.ZodNull]>>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">, z.ZodNull]>>;
    role: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    content: z.ZodString;
    id: z.ZodString;
    reference: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodType<z.objectOutputType<{
        content: z.ZodString;
        dataset_id: z.ZodString;
        document_id: z.ZodString;
        document_name: z.ZodString;
        id: z.ZodString;
        positions: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodArray<z.ZodNumber, "many">, "many">, z.ZodNull]>>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">, z.ZodNull]>>;
    role: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type ComhairleSessionMessage = z.infer<typeof _ComhairleSessionMessage>;
export declare const ComhairleSessionMessage: z.ZodType<ComhairleSessionMessage, z.ZodTypeDef, any>;
declare const _ComhairleAgentSession: z.ZodObject<{
    agent_id: z.ZodString;
    configuration: z.ZodUnknown;
    id: z.ZodString;
    messages: z.ZodArray<z.ZodType<z.objectOutputType<{
        content: z.ZodString;
        id: z.ZodString;
        reference: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodType<z.objectOutputType<{
            content: z.ZodString;
            dataset_id: z.ZodString;
            document_id: z.ZodString;
            document_name: z.ZodString;
            id: z.ZodString;
            positions: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodArray<z.ZodNumber, "many">, "many">, z.ZodNull]>>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">, z.ZodNull]>>;
        role: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    agent_id: z.ZodString;
    configuration: z.ZodUnknown;
    id: z.ZodString;
    messages: z.ZodArray<z.ZodType<z.objectOutputType<{
        content: z.ZodString;
        id: z.ZodString;
        reference: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodType<z.objectOutputType<{
            content: z.ZodString;
            dataset_id: z.ZodString;
            document_id: z.ZodString;
            document_name: z.ZodString;
            id: z.ZodString;
            positions: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodArray<z.ZodNumber, "many">, "many">, z.ZodNull]>>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">, z.ZodNull]>>;
        role: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    agent_id: z.ZodString;
    configuration: z.ZodUnknown;
    id: z.ZodString;
    messages: z.ZodArray<z.ZodType<z.objectOutputType<{
        content: z.ZodString;
        id: z.ZodString;
        reference: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodType<z.objectOutputType<{
            content: z.ZodString;
            dataset_id: z.ZodString;
            document_id: z.ZodString;
            document_name: z.ZodString;
            id: z.ZodString;
            positions: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodArray<z.ZodNumber, "many">, "many">, z.ZodNull]>>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">, z.ZodNull]>>;
        role: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
}, z.ZodTypeAny, "passthrough">>;
export type ComhairleAgentSession = z.infer<typeof _ComhairleAgentSession>;
export declare const ComhairleAgentSession: z.ZodType<ComhairleAgentSession, z.ZodTypeDef, any>;
declare const _ConversationRequest: z.ZodObject<{
    question: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    question: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    question: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type ConversationRequest = z.infer<typeof _ConversationRequest>;
export declare const ConversationRequest: z.ZodType<ConversationRequest, z.ZodTypeDef, any>;
declare const _Translation2: z.ZodObject<{
    textContent: z.ZodType<z.objectOutputType<{
        format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
        id: z.ZodString;
        primaryLocale: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
        aiGenerated: z.ZodBoolean;
        content: z.ZodString;
        contentId: z.ZodString;
        id: z.ZodString;
        locale: z.ZodString;
        requiresValidation: z.ZodBoolean;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    textContent: z.ZodType<z.objectOutputType<{
        format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
        id: z.ZodString;
        primaryLocale: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
        aiGenerated: z.ZodBoolean;
        content: z.ZodString;
        contentId: z.ZodString;
        id: z.ZodString;
        locale: z.ZodString;
        requiresValidation: z.ZodBoolean;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    textContent: z.ZodType<z.objectOutputType<{
        format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
        id: z.ZodString;
        primaryLocale: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
        aiGenerated: z.ZodBoolean;
        content: z.ZodString;
        contentId: z.ZodString;
        id: z.ZodString;
        locale: z.ZodString;
        requiresValidation: z.ZodBoolean;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
}, z.ZodTypeAny, "passthrough">>;
export type Translation2 = z.infer<typeof _Translation2>;
export declare const Translation2: z.ZodType<Translation2, z.ZodTypeDef, any>;
declare const _SectionWithTranslationsDto: z.ZodObject<{
    body: z.ZodString;
    bodyTranslations: z.ZodType<z.objectOutputType<{
        textContent: z.ZodType<z.objectOutputType<{
            format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
            id: z.ZodString;
            primaryLocale: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
            aiGenerated: z.ZodBoolean;
            content: z.ZodString;
            contentId: z.ZodString;
            id: z.ZodString;
            locale: z.ZodString;
            requiresValidation: z.ZodBoolean;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    id: z.ZodString;
    position: z.ZodNumber;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    body: z.ZodString;
    bodyTranslations: z.ZodType<z.objectOutputType<{
        textContent: z.ZodType<z.objectOutputType<{
            format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
            id: z.ZodString;
            primaryLocale: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
            aiGenerated: z.ZodBoolean;
            content: z.ZodString;
            contentId: z.ZodString;
            id: z.ZodString;
            locale: z.ZodString;
            requiresValidation: z.ZodBoolean;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    id: z.ZodString;
    position: z.ZodNumber;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    body: z.ZodString;
    bodyTranslations: z.ZodType<z.objectOutputType<{
        textContent: z.ZodType<z.objectOutputType<{
            format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
            id: z.ZodString;
            primaryLocale: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
            aiGenerated: z.ZodBoolean;
            content: z.ZodString;
            contentId: z.ZodString;
            id: z.ZodString;
            locale: z.ZodString;
            requiresValidation: z.ZodBoolean;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    id: z.ZodString;
    position: z.ZodNumber;
}, z.ZodTypeAny, "passthrough">>;
export type SectionWithTranslationsDto = z.infer<typeof _SectionWithTranslationsDto>;
export declare const SectionWithTranslationsDto: z.ZodType<SectionWithTranslationsDto, z.ZodTypeDef, any>;
declare const _Translation: z.ZodObject<{
    textContent: z.ZodType<z.objectOutputType<{
        format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
        id: z.ZodString;
        primaryLocale: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
        aiGenerated: z.ZodBoolean;
        content: z.ZodString;
        contentId: z.ZodString;
        id: z.ZodString;
        locale: z.ZodString;
        requiresValidation: z.ZodBoolean;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    textContent: z.ZodType<z.objectOutputType<{
        format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
        id: z.ZodString;
        primaryLocale: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
        aiGenerated: z.ZodBoolean;
        content: z.ZodString;
        contentId: z.ZodString;
        id: z.ZodString;
        locale: z.ZodString;
        requiresValidation: z.ZodBoolean;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    textContent: z.ZodType<z.objectOutputType<{
        format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
        id: z.ZodString;
        primaryLocale: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
        aiGenerated: z.ZodBoolean;
        content: z.ZodString;
        contentId: z.ZodString;
        id: z.ZodString;
        locale: z.ZodString;
        requiresValidation: z.ZodBoolean;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
}, z.ZodTypeAny, "passthrough">>;
export type Translation = z.infer<typeof _Translation>;
export declare const Translation: z.ZodType<Translation, z.ZodTypeDef, any>;
declare const _ProposalWithTranslationsDto: z.ZodObject<{
    id: z.ZodString;
    sections: z.ZodArray<z.ZodType<z.objectOutputType<{
        body: z.ZodString;
        bodyTranslations: z.ZodType<z.objectOutputType<{
            textContent: z.ZodType<z.objectOutputType<{
                format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                id: z.ZodString;
                primaryLocale: z.ZodString;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                aiGenerated: z.ZodBoolean;
                content: z.ZodString;
                contentId: z.ZodString;
                id: z.ZodString;
                locale: z.ZodString;
                requiresValidation: z.ZodBoolean;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        id: z.ZodString;
        position: z.ZodNumber;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    title: z.ZodString;
    titleTranslations: z.ZodType<z.objectOutputType<{
        textContent: z.ZodType<z.objectOutputType<{
            format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
            id: z.ZodString;
            primaryLocale: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
            aiGenerated: z.ZodBoolean;
            content: z.ZodString;
            contentId: z.ZodString;
            id: z.ZodString;
            locale: z.ZodString;
            requiresValidation: z.ZodBoolean;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    workflowStepId: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    id: z.ZodString;
    sections: z.ZodArray<z.ZodType<z.objectOutputType<{
        body: z.ZodString;
        bodyTranslations: z.ZodType<z.objectOutputType<{
            textContent: z.ZodType<z.objectOutputType<{
                format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                id: z.ZodString;
                primaryLocale: z.ZodString;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                aiGenerated: z.ZodBoolean;
                content: z.ZodString;
                contentId: z.ZodString;
                id: z.ZodString;
                locale: z.ZodString;
                requiresValidation: z.ZodBoolean;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        id: z.ZodString;
        position: z.ZodNumber;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    title: z.ZodString;
    titleTranslations: z.ZodType<z.objectOutputType<{
        textContent: z.ZodType<z.objectOutputType<{
            format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
            id: z.ZodString;
            primaryLocale: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
            aiGenerated: z.ZodBoolean;
            content: z.ZodString;
            contentId: z.ZodString;
            id: z.ZodString;
            locale: z.ZodString;
            requiresValidation: z.ZodBoolean;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    workflowStepId: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    id: z.ZodString;
    sections: z.ZodArray<z.ZodType<z.objectOutputType<{
        body: z.ZodString;
        bodyTranslations: z.ZodType<z.objectOutputType<{
            textContent: z.ZodType<z.objectOutputType<{
                format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                id: z.ZodString;
                primaryLocale: z.ZodString;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                aiGenerated: z.ZodBoolean;
                content: z.ZodString;
                contentId: z.ZodString;
                id: z.ZodString;
                locale: z.ZodString;
                requiresValidation: z.ZodBoolean;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        id: z.ZodString;
        position: z.ZodNumber;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    title: z.ZodString;
    titleTranslations: z.ZodType<z.objectOutputType<{
        textContent: z.ZodType<z.objectOutputType<{
            format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
            id: z.ZodString;
            primaryLocale: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
            aiGenerated: z.ZodBoolean;
            content: z.ZodString;
            contentId: z.ZodString;
            id: z.ZodString;
            locale: z.ZodString;
            requiresValidation: z.ZodBoolean;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    workflowStepId: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type ProposalWithTranslationsDto = z.infer<typeof _ProposalWithTranslationsDto>;
export declare const ProposalWithTranslationsDto: z.ZodType<ProposalWithTranslationsDto, z.ZodTypeDef, any>;
declare const _LocalizedProposalSectionDto: z.ZodObject<{
    body: z.ZodString;
    id: z.ZodString;
    position: z.ZodNumber;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    body: z.ZodString;
    id: z.ZodString;
    position: z.ZodNumber;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    body: z.ZodString;
    id: z.ZodString;
    position: z.ZodNumber;
}, z.ZodTypeAny, "passthrough">>;
export type LocalizedProposalSectionDto = z.infer<typeof _LocalizedProposalSectionDto>;
export declare const LocalizedProposalSectionDto: z.ZodType<LocalizedProposalSectionDto, z.ZodTypeDef, any>;
declare const _LocalizedProposalDto: z.ZodObject<{
    id: z.ZodString;
    sections: z.ZodArray<z.ZodType<z.objectOutputType<{
        body: z.ZodString;
        id: z.ZodString;
        position: z.ZodNumber;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    title: z.ZodString;
    workflowStepId: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    id: z.ZodString;
    sections: z.ZodArray<z.ZodType<z.objectOutputType<{
        body: z.ZodString;
        id: z.ZodString;
        position: z.ZodNumber;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    title: z.ZodString;
    workflowStepId: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    id: z.ZodString;
    sections: z.ZodArray<z.ZodType<z.objectOutputType<{
        body: z.ZodString;
        id: z.ZodString;
        position: z.ZodNumber;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    title: z.ZodString;
    workflowStepId: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type LocalizedProposalDto = z.infer<typeof _LocalizedProposalDto>;
export declare const LocalizedProposalDto: z.ZodType<LocalizedProposalDto, z.ZodTypeDef, any>;
declare const _ProposalsListResponse: z.ZodUnion<[z.ZodArray<z.ZodType<z.objectOutputType<{
    id: z.ZodString;
    sections: z.ZodArray<z.ZodType<z.objectOutputType<{
        body: z.ZodString;
        bodyTranslations: z.ZodType<z.objectOutputType<{
            textContent: z.ZodType<z.objectOutputType<{
                format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                id: z.ZodString;
                primaryLocale: z.ZodString;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                aiGenerated: z.ZodBoolean;
                content: z.ZodString;
                contentId: z.ZodString;
                id: z.ZodString;
                locale: z.ZodString;
                requiresValidation: z.ZodBoolean;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        id: z.ZodString;
        position: z.ZodNumber;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    title: z.ZodString;
    titleTranslations: z.ZodType<z.objectOutputType<{
        textContent: z.ZodType<z.objectOutputType<{
            format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
            id: z.ZodString;
            primaryLocale: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
            aiGenerated: z.ZodBoolean;
            content: z.ZodString;
            contentId: z.ZodString;
            id: z.ZodString;
            locale: z.ZodString;
            requiresValidation: z.ZodBoolean;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    workflowStepId: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">, z.ZodArray<z.ZodType<z.objectOutputType<{
    id: z.ZodString;
    sections: z.ZodArray<z.ZodType<z.objectOutputType<{
        body: z.ZodString;
        id: z.ZodString;
        position: z.ZodNumber;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    title: z.ZodString;
    workflowStepId: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">]>;
export type ProposalsListResponse = z.infer<typeof _ProposalsListResponse>;
export declare const ProposalsListResponse: z.ZodType<ProposalsListResponse, z.ZodTypeDef, any>;
declare const _CreateProposalRequest: z.ZodObject<{
    sections: z.ZodDefault<z.ZodOptional<z.ZodArray<z.ZodString, "many">>>;
    title: z.ZodString;
    workflow_step_id: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    sections: z.ZodDefault<z.ZodOptional<z.ZodArray<z.ZodString, "many">>>;
    title: z.ZodString;
    workflow_step_id: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    sections: z.ZodDefault<z.ZodOptional<z.ZodArray<z.ZodString, "many">>>;
    title: z.ZodString;
    workflow_step_id: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type CreateProposalRequest = z.infer<typeof _CreateProposalRequest>;
export declare const CreateProposalRequest: z.ZodType<CreateProposalRequest, z.ZodTypeDef, any>;
declare const _ProposalSectionDto: z.ZodObject<{
    body: z.ZodString;
    id: z.ZodString;
    position: z.ZodNumber;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    body: z.ZodString;
    id: z.ZodString;
    position: z.ZodNumber;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    body: z.ZodString;
    id: z.ZodString;
    position: z.ZodNumber;
}, z.ZodTypeAny, "passthrough">>;
export type ProposalSectionDto = z.infer<typeof _ProposalSectionDto>;
export declare const ProposalSectionDto: z.ZodType<ProposalSectionDto, z.ZodTypeDef, any>;
declare const _ProposalDto: z.ZodObject<{
    id: z.ZodString;
    sections: z.ZodArray<z.ZodType<z.objectOutputType<{
        body: z.ZodString;
        id: z.ZodString;
        position: z.ZodNumber;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    title: z.ZodString;
    workflowStepId: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    id: z.ZodString;
    sections: z.ZodArray<z.ZodType<z.objectOutputType<{
        body: z.ZodString;
        id: z.ZodString;
        position: z.ZodNumber;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    title: z.ZodString;
    workflowStepId: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    id: z.ZodString;
    sections: z.ZodArray<z.ZodType<z.objectOutputType<{
        body: z.ZodString;
        id: z.ZodString;
        position: z.ZodNumber;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    title: z.ZodString;
    workflowStepId: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type ProposalDto = z.infer<typeof _ProposalDto>;
export declare const ProposalDto: z.ZodType<ProposalDto, z.ZodTypeDef, any>;
declare const _CreateSectionRequest: z.ZodObject<{
    body: z.ZodString;
    position: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>>;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    body: z.ZodString;
    position: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>>;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    body: z.ZodString;
    position: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>>;
}, z.ZodTypeAny, "passthrough">>;
export type CreateSectionRequest = z.infer<typeof _CreateSectionRequest>;
export declare const CreateSectionRequest: z.ZodType<CreateSectionRequest, z.ZodTypeDef, any>;
declare const _ResponseValue: z.ZodUnion<[z.ZodNumber, z.ZodString]>;
export type ResponseValue = z.infer<typeof _ResponseValue>;
export declare const ResponseValue: z.ZodType<ResponseValue, z.ZodTypeDef, any>;
declare const _Response: z.ZodObject<{
    question_id: z.ZodString;
    section_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    value: z.ZodType<string | number, z.ZodTypeDef, any>;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    question_id: z.ZodString;
    section_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    value: z.ZodType<string | number, z.ZodTypeDef, any>;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    question_id: z.ZodString;
    section_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    value: z.ZodType<string | number, z.ZodTypeDef, any>;
}, z.ZodTypeAny, "passthrough">>;
export type Response = z.infer<typeof _Response>;
export declare const Response: z.ZodType<Response, z.ZodTypeDef, any>;
declare const _QuestionResponses: z.ZodArray<z.ZodType<z.objectOutputType<{
    question_id: z.ZodString;
    section_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    value: z.ZodType<string | number, z.ZodTypeDef, any>;
}, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
export type QuestionResponses = z.infer<typeof _QuestionResponses>;
export declare const QuestionResponses: z.ZodType<QuestionResponses, z.ZodTypeDef, any>;
declare const _ProposalResponseDto: z.ZodObject<{
    id: z.ZodString;
    proposalId: z.ZodString;
    response: z.ZodType<z.objectOutputType<{
        question_id: z.ZodString;
        section_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        value: z.ZodType<string | number, z.ZodTypeDef, any>;
    }, z.ZodTypeAny, "passthrough">[], z.ZodTypeDef, any>;
    userId: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    id: z.ZodString;
    proposalId: z.ZodString;
    response: z.ZodType<z.objectOutputType<{
        question_id: z.ZodString;
        section_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        value: z.ZodType<string | number, z.ZodTypeDef, any>;
    }, z.ZodTypeAny, "passthrough">[], z.ZodTypeDef, any>;
    userId: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    id: z.ZodString;
    proposalId: z.ZodString;
    response: z.ZodType<z.objectOutputType<{
        question_id: z.ZodString;
        section_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        value: z.ZodType<string | number, z.ZodTypeDef, any>;
    }, z.ZodTypeAny, "passthrough">[], z.ZodTypeDef, any>;
    userId: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type ProposalResponseDto = z.infer<typeof _ProposalResponseDto>;
export declare const ProposalResponseDto: z.ZodType<ProposalResponseDto, z.ZodTypeDef, any>;
declare const _CreateResponse: z.ZodObject<{
    question_responses: z.ZodArray<z.ZodType<z.objectOutputType<{
        question_id: z.ZodString;
        section_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        value: z.ZodType<string | number, z.ZodTypeDef, any>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    question_responses: z.ZodArray<z.ZodType<z.objectOutputType<{
        question_id: z.ZodString;
        section_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        value: z.ZodType<string | number, z.ZodTypeDef, any>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    question_responses: z.ZodArray<z.ZodType<z.objectOutputType<{
        question_id: z.ZodString;
        section_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        value: z.ZodType<string | number, z.ZodTypeDef, any>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
}, z.ZodTypeAny, "passthrough">>;
export type CreateResponse = z.infer<typeof _CreateResponse>;
export declare const CreateResponse: z.ZodType<CreateResponse, z.ZodTypeDef, any>;
declare const _RankedProposal: z.ZodObject<{
    alignmentRating: z.ZodNumber;
    id: z.ZodString;
    responses: z.ZodArray<z.ZodType<z.objectOutputType<{
        id: z.ZodString;
        proposalId: z.ZodString;
        response: z.ZodType<z.objectOutputType<{
            question_id: z.ZodString;
            section_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
            value: z.ZodType<string | number, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">[], z.ZodTypeDef, any>;
        userId: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    sections: z.ZodArray<z.ZodType<z.objectOutputType<{
        body: z.ZodString;
        id: z.ZodString;
        position: z.ZodNumber;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    title: z.ZodString;
    workflowStepId: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    alignmentRating: z.ZodNumber;
    id: z.ZodString;
    responses: z.ZodArray<z.ZodType<z.objectOutputType<{
        id: z.ZodString;
        proposalId: z.ZodString;
        response: z.ZodType<z.objectOutputType<{
            question_id: z.ZodString;
            section_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
            value: z.ZodType<string | number, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">[], z.ZodTypeDef, any>;
        userId: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    sections: z.ZodArray<z.ZodType<z.objectOutputType<{
        body: z.ZodString;
        id: z.ZodString;
        position: z.ZodNumber;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    title: z.ZodString;
    workflowStepId: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    alignmentRating: z.ZodNumber;
    id: z.ZodString;
    responses: z.ZodArray<z.ZodType<z.objectOutputType<{
        id: z.ZodString;
        proposalId: z.ZodString;
        response: z.ZodType<z.objectOutputType<{
            question_id: z.ZodString;
            section_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
            value: z.ZodType<string | number, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">[], z.ZodTypeDef, any>;
        userId: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    sections: z.ZodArray<z.ZodType<z.objectOutputType<{
        body: z.ZodString;
        id: z.ZodString;
        position: z.ZodNumber;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    title: z.ZodString;
    workflowStepId: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type RankedProposal = z.infer<typeof _RankedProposal>;
export declare const RankedProposal: z.ZodType<RankedProposal, z.ZodTypeDef, any>;
declare const _PrioritizationInsightsResponse: z.ZodObject<{
    rankedProposals: z.ZodArray<z.ZodType<z.objectOutputType<{
        alignmentRating: z.ZodNumber;
        id: z.ZodString;
        responses: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            proposalId: z.ZodString;
            response: z.ZodType<z.objectOutputType<{
                question_id: z.ZodString;
                section_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
                value: z.ZodType<string | number, z.ZodTypeDef, any>;
            }, z.ZodTypeAny, "passthrough">[], z.ZodTypeDef, any>;
            userId: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        sections: z.ZodArray<z.ZodType<z.objectOutputType<{
            body: z.ZodString;
            id: z.ZodString;
            position: z.ZodNumber;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        title: z.ZodString;
        workflowStepId: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    rankedProposals: z.ZodArray<z.ZodType<z.objectOutputType<{
        alignmentRating: z.ZodNumber;
        id: z.ZodString;
        responses: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            proposalId: z.ZodString;
            response: z.ZodType<z.objectOutputType<{
                question_id: z.ZodString;
                section_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
                value: z.ZodType<string | number, z.ZodTypeDef, any>;
            }, z.ZodTypeAny, "passthrough">[], z.ZodTypeDef, any>;
            userId: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        sections: z.ZodArray<z.ZodType<z.objectOutputType<{
            body: z.ZodString;
            id: z.ZodString;
            position: z.ZodNumber;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        title: z.ZodString;
        workflowStepId: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    rankedProposals: z.ZodArray<z.ZodType<z.objectOutputType<{
        alignmentRating: z.ZodNumber;
        id: z.ZodString;
        responses: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            proposalId: z.ZodString;
            response: z.ZodType<z.objectOutputType<{
                question_id: z.ZodString;
                section_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
                value: z.ZodType<string | number, z.ZodTypeDef, any>;
            }, z.ZodTypeAny, "passthrough">[], z.ZodTypeDef, any>;
            userId: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        sections: z.ZodArray<z.ZodType<z.objectOutputType<{
            body: z.ZodString;
            id: z.ZodString;
            position: z.ZodNumber;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        title: z.ZodString;
        workflowStepId: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
}, z.ZodTypeAny, "passthrough">>;
export type PrioritizationInsightsResponse = z.infer<typeof _PrioritizationInsightsResponse>;
export declare const PrioritizationInsightsResponse: z.ZodType<PrioritizationInsightsResponse, z.ZodTypeDef, any>;
declare const _ConversationRequest2: z.ZodObject<{
    history: z.ZodString;
    question_intent: z.ZodString;
    starting_question: z.ZodString;
    workflow_step_id: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    history: z.ZodString;
    question_intent: z.ZodString;
    starting_question: z.ZodString;
    workflow_step_id: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    history: z.ZodString;
    question_intent: z.ZodString;
    starting_question: z.ZodString;
    workflow_step_id: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type ConversationRequest2 = z.infer<typeof _ConversationRequest2>;
export declare const ConversationRequest2: z.ZodType<ConversationRequest2, z.ZodTypeDef, any>;
declare const _AnswerStatus: z.ZodEnum<["pending", "approved", "declined"]>;
export type AnswerStatus = z.infer<typeof _AnswerStatus>;
export declare const AnswerStatus: z.ZodType<AnswerStatus, z.ZodTypeDef, any>;
declare const _status: z.ZodOptional<z.ZodUnion<[z.ZodType<"pending" | "approved" | "declined", z.ZodTypeDef, any>, z.ZodNull]>>;
export type status = z.infer<typeof _status>;
export declare const status: z.ZodType<status, z.ZodTypeDef, any>;
declare const _ThinkingSpaceAnswerDto: z.ZodObject<{
    answer: z.ZodString;
    id: z.ZodString;
    isFollowUp: z.ZodBoolean;
    otherQuestions: z.ZodArray<z.ZodString, "many">;
    question: z.ZodString;
    rootQuestionId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    status: z.ZodType<"pending" | "approved" | "declined", z.ZodTypeDef, any>;
    workflowStepId: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    answer: z.ZodString;
    id: z.ZodString;
    isFollowUp: z.ZodBoolean;
    otherQuestions: z.ZodArray<z.ZodString, "many">;
    question: z.ZodString;
    rootQuestionId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    status: z.ZodType<"pending" | "approved" | "declined", z.ZodTypeDef, any>;
    workflowStepId: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    answer: z.ZodString;
    id: z.ZodString;
    isFollowUp: z.ZodBoolean;
    otherQuestions: z.ZodArray<z.ZodString, "many">;
    question: z.ZodString;
    rootQuestionId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    status: z.ZodType<"pending" | "approved" | "declined", z.ZodTypeDef, any>;
    workflowStepId: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type ThinkingSpaceAnswerDto = z.infer<typeof _ThinkingSpaceAnswerDto>;
export declare const ThinkingSpaceAnswerDto: z.ZodType<ThinkingSpaceAnswerDto, z.ZodTypeDef, any>;
declare const _CreateAnswerRequest: z.ZodObject<{
    answer: z.ZodString;
    is_follow_up: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    other_questions: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodString, "many">, z.ZodNull]>>;
    question: z.ZodString;
    root_question_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    workflow_step_id: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    answer: z.ZodString;
    is_follow_up: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    other_questions: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodString, "many">, z.ZodNull]>>;
    question: z.ZodString;
    root_question_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    workflow_step_id: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    answer: z.ZodString;
    is_follow_up: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    other_questions: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodString, "many">, z.ZodNull]>>;
    question: z.ZodString;
    root_question_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    workflow_step_id: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type CreateAnswerRequest = z.infer<typeof _CreateAnswerRequest>;
export declare const CreateAnswerRequest: z.ZodType<CreateAnswerRequest, z.ZodTypeDef, any>;
declare const _UpdateAnswer: z.ZodObject<{
    answer: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    status: z.ZodOptional<z.ZodUnion<[z.ZodType<"pending" | "approved" | "declined", z.ZodTypeDef, any>, z.ZodNull]>>;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    answer: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    status: z.ZodOptional<z.ZodUnion<[z.ZodType<"pending" | "approved" | "declined", z.ZodTypeDef, any>, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    answer: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    status: z.ZodOptional<z.ZodUnion<[z.ZodType<"pending" | "approved" | "declined", z.ZodTypeDef, any>, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">>;
export type UpdateAnswer = z.infer<typeof _UpdateAnswer>;
export declare const UpdateAnswer: z.ZodType<UpdateAnswer, z.ZodTypeDef, any>;
declare const _GenerateThinkingSpaceSummary: z.ZodObject<{
    workflow_step_id: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    workflow_step_id: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    workflow_step_id: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type GenerateThinkingSpaceSummary = z.infer<typeof _GenerateThinkingSpaceSummary>;
export declare const GenerateThinkingSpaceSummary: z.ZodType<GenerateThinkingSpaceSummary, z.ZodTypeDef, any>;
declare const _ThinkingSpaceSummaryDto: z.ZodObject<{
    aiGeneratedSummary: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    id: z.ZodString;
    isAiGenerated: z.ZodBoolean;
    summary: z.ZodString;
    userId: z.ZodString;
    workflowStepId: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    aiGeneratedSummary: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    id: z.ZodString;
    isAiGenerated: z.ZodBoolean;
    summary: z.ZodString;
    userId: z.ZodString;
    workflowStepId: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    aiGeneratedSummary: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    id: z.ZodString;
    isAiGenerated: z.ZodBoolean;
    summary: z.ZodString;
    userId: z.ZodString;
    workflowStepId: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type ThinkingSpaceSummaryDto = z.infer<typeof _ThinkingSpaceSummaryDto>;
export declare const ThinkingSpaceSummaryDto: z.ZodType<ThinkingSpaceSummaryDto, z.ZodTypeDef, any>;
declare const _UpdateCreateThinkingSpace: z.ZodObject<{
    summary: z.ZodString;
    summary_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    workflow_step_id: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    summary: z.ZodString;
    summary_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    workflow_step_id: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    summary: z.ZodString;
    summary_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    workflow_step_id: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type UpdateCreateThinkingSpace = z.infer<typeof _UpdateCreateThinkingSpace>;
export declare const UpdateCreateThinkingSpace: z.ZodType<UpdateCreateThinkingSpace, z.ZodTypeDef, any>;
declare const _ThinkingSpaceFollowUpQuestionDto: z.ZodObject<{
    followUpQuestions: z.ZodArray<z.ZodString, "many">;
    id: z.ZodString;
    rootQuestionId: z.ZodString;
    userId: z.ZodString;
    workflowStepId: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    followUpQuestions: z.ZodArray<z.ZodString, "many">;
    id: z.ZodString;
    rootQuestionId: z.ZodString;
    userId: z.ZodString;
    workflowStepId: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    followUpQuestions: z.ZodArray<z.ZodString, "many">;
    id: z.ZodString;
    rootQuestionId: z.ZodString;
    userId: z.ZodString;
    workflowStepId: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type ThinkingSpaceFollowUpQuestionDto = z.infer<typeof _ThinkingSpaceFollowUpQuestionDto>;
export declare const ThinkingSpaceFollowUpQuestionDto: z.ZodType<ThinkingSpaceFollowUpQuestionDto, z.ZodTypeDef, any>;
declare const _CreateFollowUpQuestions: z.ZodObject<{
    follow_up_questions: z.ZodArray<z.ZodString, "many">;
    root_question_id: z.ZodString;
    workflow_step_id: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    follow_up_questions: z.ZodArray<z.ZodString, "many">;
    root_question_id: z.ZodString;
    workflow_step_id: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    follow_up_questions: z.ZodArray<z.ZodString, "many">;
    root_question_id: z.ZodString;
    workflow_step_id: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type CreateFollowUpQuestions = z.infer<typeof _CreateFollowUpQuestions>;
export declare const CreateFollowUpQuestions: z.ZodType<CreateFollowUpQuestions, z.ZodTypeDef, any>;
declare const _UpdateFollowUpQuestions: z.ZodObject<{
    follow_up_questions: z.ZodArray<z.ZodString, "many">;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    follow_up_questions: z.ZodArray<z.ZodString, "many">;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    follow_up_questions: z.ZodArray<z.ZodString, "many">;
}, z.ZodTypeAny, "passthrough">>;
export type UpdateFollowUpQuestions = z.infer<typeof _UpdateFollowUpQuestions>;
export declare const UpdateFollowUpQuestions: z.ZodType<UpdateFollowUpQuestions, z.ZodTypeDef, any>;
declare const _AnswersByRoot: z.ZodObject<{
    followUps: z.ZodArray<z.ZodType<z.objectOutputType<{
        answer: z.ZodString;
        id: z.ZodString;
        isFollowUp: z.ZodBoolean;
        otherQuestions: z.ZodArray<z.ZodString, "many">;
        question: z.ZodString;
        rootQuestionId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        status: z.ZodType<"pending" | "approved" | "declined", z.ZodTypeDef, any>;
        workflowStepId: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    root: z.ZodType<z.objectOutputType<{
        answer: z.ZodString;
        id: z.ZodString;
        isFollowUp: z.ZodBoolean;
        otherQuestions: z.ZodArray<z.ZodString, "many">;
        question: z.ZodString;
        rootQuestionId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        status: z.ZodType<"pending" | "approved" | "declined", z.ZodTypeDef, any>;
        workflowStepId: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    followUps: z.ZodArray<z.ZodType<z.objectOutputType<{
        answer: z.ZodString;
        id: z.ZodString;
        isFollowUp: z.ZodBoolean;
        otherQuestions: z.ZodArray<z.ZodString, "many">;
        question: z.ZodString;
        rootQuestionId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        status: z.ZodType<"pending" | "approved" | "declined", z.ZodTypeDef, any>;
        workflowStepId: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    root: z.ZodType<z.objectOutputType<{
        answer: z.ZodString;
        id: z.ZodString;
        isFollowUp: z.ZodBoolean;
        otherQuestions: z.ZodArray<z.ZodString, "many">;
        question: z.ZodString;
        rootQuestionId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        status: z.ZodType<"pending" | "approved" | "declined", z.ZodTypeDef, any>;
        workflowStepId: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    followUps: z.ZodArray<z.ZodType<z.objectOutputType<{
        answer: z.ZodString;
        id: z.ZodString;
        isFollowUp: z.ZodBoolean;
        otherQuestions: z.ZodArray<z.ZodString, "many">;
        question: z.ZodString;
        rootQuestionId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        status: z.ZodType<"pending" | "approved" | "declined", z.ZodTypeDef, any>;
        workflowStepId: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    root: z.ZodType<z.objectOutputType<{
        answer: z.ZodString;
        id: z.ZodString;
        isFollowUp: z.ZodBoolean;
        otherQuestions: z.ZodArray<z.ZodString, "many">;
        question: z.ZodString;
        rootQuestionId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        status: z.ZodType<"pending" | "approved" | "declined", z.ZodTypeDef, any>;
        workflowStepId: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
}, z.ZodTypeAny, "passthrough">>;
export type AnswersByRoot = z.infer<typeof _AnswersByRoot>;
export declare const AnswersByRoot: z.ZodType<AnswersByRoot, z.ZodTypeDef, any>;
declare const _ThinkingSpaceUserInsights: z.ZodObject<{
    answers: z.ZodArray<z.ZodType<z.objectOutputType<{
        followUps: z.ZodArray<z.ZodType<z.objectOutputType<{
            answer: z.ZodString;
            id: z.ZodString;
            isFollowUp: z.ZodBoolean;
            otherQuestions: z.ZodArray<z.ZodString, "many">;
            question: z.ZodString;
            rootQuestionId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
            status: z.ZodType<"pending" | "approved" | "declined", z.ZodTypeDef, any>;
            workflowStepId: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        root: z.ZodType<z.objectOutputType<{
            answer: z.ZodString;
            id: z.ZodString;
            isFollowUp: z.ZodBoolean;
            otherQuestions: z.ZodArray<z.ZodString, "many">;
            question: z.ZodString;
            rootQuestionId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
            status: z.ZodType<"pending" | "approved" | "declined", z.ZodTypeDef, any>;
            workflowStepId: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    summary: z.ZodType<z.objectOutputType<{
        aiGeneratedSummary: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        id: z.ZodString;
        isAiGenerated: z.ZodBoolean;
        summary: z.ZodString;
        userId: z.ZodString;
        workflowStepId: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    userId: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    answers: z.ZodArray<z.ZodType<z.objectOutputType<{
        followUps: z.ZodArray<z.ZodType<z.objectOutputType<{
            answer: z.ZodString;
            id: z.ZodString;
            isFollowUp: z.ZodBoolean;
            otherQuestions: z.ZodArray<z.ZodString, "many">;
            question: z.ZodString;
            rootQuestionId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
            status: z.ZodType<"pending" | "approved" | "declined", z.ZodTypeDef, any>;
            workflowStepId: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        root: z.ZodType<z.objectOutputType<{
            answer: z.ZodString;
            id: z.ZodString;
            isFollowUp: z.ZodBoolean;
            otherQuestions: z.ZodArray<z.ZodString, "many">;
            question: z.ZodString;
            rootQuestionId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
            status: z.ZodType<"pending" | "approved" | "declined", z.ZodTypeDef, any>;
            workflowStepId: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    summary: z.ZodType<z.objectOutputType<{
        aiGeneratedSummary: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        id: z.ZodString;
        isAiGenerated: z.ZodBoolean;
        summary: z.ZodString;
        userId: z.ZodString;
        workflowStepId: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    userId: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    answers: z.ZodArray<z.ZodType<z.objectOutputType<{
        followUps: z.ZodArray<z.ZodType<z.objectOutputType<{
            answer: z.ZodString;
            id: z.ZodString;
            isFollowUp: z.ZodBoolean;
            otherQuestions: z.ZodArray<z.ZodString, "many">;
            question: z.ZodString;
            rootQuestionId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
            status: z.ZodType<"pending" | "approved" | "declined", z.ZodTypeDef, any>;
            workflowStepId: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        root: z.ZodType<z.objectOutputType<{
            answer: z.ZodString;
            id: z.ZodString;
            isFollowUp: z.ZodBoolean;
            otherQuestions: z.ZodArray<z.ZodString, "many">;
            question: z.ZodString;
            rootQuestionId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
            status: z.ZodType<"pending" | "approved" | "declined", z.ZodTypeDef, any>;
            workflowStepId: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    summary: z.ZodType<z.objectOutputType<{
        aiGeneratedSummary: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        id: z.ZodString;
        isAiGenerated: z.ZodBoolean;
        summary: z.ZodString;
        userId: z.ZodString;
        workflowStepId: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    userId: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type ThinkingSpaceUserInsights = z.infer<typeof _ThinkingSpaceUserInsights>;
export declare const ThinkingSpaceUserInsights: z.ZodType<ThinkingSpaceUserInsights, z.ZodTypeDef, any>;
declare const _ThinkingSpaceInsightsResponse: z.ZodObject<{
    users: z.ZodArray<z.ZodType<z.objectOutputType<{
        answers: z.ZodArray<z.ZodType<z.objectOutputType<{
            followUps: z.ZodArray<z.ZodType<z.objectOutputType<{
                answer: z.ZodString;
                id: z.ZodString;
                isFollowUp: z.ZodBoolean;
                otherQuestions: z.ZodArray<z.ZodString, "many">;
                question: z.ZodString;
                rootQuestionId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
                status: z.ZodType<"pending" | "approved" | "declined", z.ZodTypeDef, any>;
                workflowStepId: z.ZodString;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
            root: z.ZodType<z.objectOutputType<{
                answer: z.ZodString;
                id: z.ZodString;
                isFollowUp: z.ZodBoolean;
                otherQuestions: z.ZodArray<z.ZodString, "many">;
                question: z.ZodString;
                rootQuestionId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
                status: z.ZodType<"pending" | "approved" | "declined", z.ZodTypeDef, any>;
                workflowStepId: z.ZodString;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        summary: z.ZodType<z.objectOutputType<{
            aiGeneratedSummary: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
            id: z.ZodString;
            isAiGenerated: z.ZodBoolean;
            summary: z.ZodString;
            userId: z.ZodString;
            workflowStepId: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        userId: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    users: z.ZodArray<z.ZodType<z.objectOutputType<{
        answers: z.ZodArray<z.ZodType<z.objectOutputType<{
            followUps: z.ZodArray<z.ZodType<z.objectOutputType<{
                answer: z.ZodString;
                id: z.ZodString;
                isFollowUp: z.ZodBoolean;
                otherQuestions: z.ZodArray<z.ZodString, "many">;
                question: z.ZodString;
                rootQuestionId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
                status: z.ZodType<"pending" | "approved" | "declined", z.ZodTypeDef, any>;
                workflowStepId: z.ZodString;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
            root: z.ZodType<z.objectOutputType<{
                answer: z.ZodString;
                id: z.ZodString;
                isFollowUp: z.ZodBoolean;
                otherQuestions: z.ZodArray<z.ZodString, "many">;
                question: z.ZodString;
                rootQuestionId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
                status: z.ZodType<"pending" | "approved" | "declined", z.ZodTypeDef, any>;
                workflowStepId: z.ZodString;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        summary: z.ZodType<z.objectOutputType<{
            aiGeneratedSummary: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
            id: z.ZodString;
            isAiGenerated: z.ZodBoolean;
            summary: z.ZodString;
            userId: z.ZodString;
            workflowStepId: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        userId: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    users: z.ZodArray<z.ZodType<z.objectOutputType<{
        answers: z.ZodArray<z.ZodType<z.objectOutputType<{
            followUps: z.ZodArray<z.ZodType<z.objectOutputType<{
                answer: z.ZodString;
                id: z.ZodString;
                isFollowUp: z.ZodBoolean;
                otherQuestions: z.ZodArray<z.ZodString, "many">;
                question: z.ZodString;
                rootQuestionId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
                status: z.ZodType<"pending" | "approved" | "declined", z.ZodTypeDef, any>;
                workflowStepId: z.ZodString;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
            root: z.ZodType<z.objectOutputType<{
                answer: z.ZodString;
                id: z.ZodString;
                isFollowUp: z.ZodBoolean;
                otherQuestions: z.ZodArray<z.ZodString, "many">;
                question: z.ZodString;
                rootQuestionId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
                status: z.ZodType<"pending" | "approved" | "declined", z.ZodTypeDef, any>;
                workflowStepId: z.ZodString;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        summary: z.ZodType<z.objectOutputType<{
            aiGeneratedSummary: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
            id: z.ZodString;
            isAiGenerated: z.ZodBoolean;
            summary: z.ZodString;
            userId: z.ZodString;
            workflowStepId: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        userId: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
}, z.ZodTypeAny, "passthrough">>;
export type ThinkingSpaceInsightsResponse = z.infer<typeof _ThinkingSpaceInsightsResponse>;
export declare const ThinkingSpaceInsightsResponse: z.ZodType<ThinkingSpaceInsightsResponse, z.ZodTypeDef, any>;
declare const _CreateConversation: z.ZodObject<{
    default_workflow_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    description: z.ZodString;
    enable_qa_chat_bot: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    image: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    is_invite_only: z.ZodBoolean;
    is_live: z.ZodBoolean;
    is_public: z.ZodBoolean;
    primary_locale: z.ZodString;
    short_description: z.ZodString;
    slug: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    supported_languages: z.ZodArray<z.ZodString, "many">;
    tags: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodString, "many">, z.ZodNull]>>;
    title: z.ZodString;
    video_url: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    default_workflow_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    description: z.ZodString;
    enable_qa_chat_bot: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    image: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    is_invite_only: z.ZodBoolean;
    is_live: z.ZodBoolean;
    is_public: z.ZodBoolean;
    primary_locale: z.ZodString;
    short_description: z.ZodString;
    slug: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    supported_languages: z.ZodArray<z.ZodString, "many">;
    tags: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodString, "many">, z.ZodNull]>>;
    title: z.ZodString;
    video_url: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    default_workflow_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    description: z.ZodString;
    enable_qa_chat_bot: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    image: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    is_invite_only: z.ZodBoolean;
    is_live: z.ZodBoolean;
    is_public: z.ZodBoolean;
    primary_locale: z.ZodString;
    short_description: z.ZodString;
    slug: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    supported_languages: z.ZodArray<z.ZodString, "many">;
    tags: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodString, "many">, z.ZodNull]>>;
    title: z.ZodString;
    video_url: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">>;
export type CreateConversation = z.infer<typeof _CreateConversation>;
export declare const CreateConversation: z.ZodType<CreateConversation, z.ZodTypeDef, any>;
declare const _ConversationDto: z.ZodObject<{
    allowRevisitAfterFinishing: z.ZodBoolean;
    callToAction: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    chatBotId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    description: z.ZodString;
    enableQaChatBot: z.ZodBoolean;
    enableSignupPrompts: z.ZodBoolean;
    faqs: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    id: z.ZodString;
    image: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    isComplete: z.ZodBoolean;
    isInviteOnly: z.ZodBoolean;
    isLive: z.ZodBoolean;
    isPublic: z.ZodBoolean;
    knowledgeBaseId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    metadata: z.ZodUnknown;
    organizationId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    primaryLocale: z.ZodString;
    privacyPolicy: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    shortDescription: z.ZodString;
    shortPrivacyPolicy: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    showThankYouPageAnnonInstructions: z.ZodBoolean;
    showThankyouPageFeedbackButton: z.ZodBoolean;
    slug: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    supportedLanguages: z.ZodArray<z.ZodString, "many">;
    tags: z.ZodArray<z.ZodString, "many">;
    thankYouMessage: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    title: z.ZodString;
    videoUrl: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    allowRevisitAfterFinishing: z.ZodBoolean;
    callToAction: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    chatBotId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    description: z.ZodString;
    enableQaChatBot: z.ZodBoolean;
    enableSignupPrompts: z.ZodBoolean;
    faqs: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    id: z.ZodString;
    image: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    isComplete: z.ZodBoolean;
    isInviteOnly: z.ZodBoolean;
    isLive: z.ZodBoolean;
    isPublic: z.ZodBoolean;
    knowledgeBaseId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    metadata: z.ZodUnknown;
    organizationId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    primaryLocale: z.ZodString;
    privacyPolicy: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    shortDescription: z.ZodString;
    shortPrivacyPolicy: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    showThankYouPageAnnonInstructions: z.ZodBoolean;
    showThankyouPageFeedbackButton: z.ZodBoolean;
    slug: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    supportedLanguages: z.ZodArray<z.ZodString, "many">;
    tags: z.ZodArray<z.ZodString, "many">;
    thankYouMessage: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    title: z.ZodString;
    videoUrl: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    allowRevisitAfterFinishing: z.ZodBoolean;
    callToAction: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    chatBotId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    description: z.ZodString;
    enableQaChatBot: z.ZodBoolean;
    enableSignupPrompts: z.ZodBoolean;
    faqs: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    id: z.ZodString;
    image: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    isComplete: z.ZodBoolean;
    isInviteOnly: z.ZodBoolean;
    isLive: z.ZodBoolean;
    isPublic: z.ZodBoolean;
    knowledgeBaseId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    metadata: z.ZodUnknown;
    organizationId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    primaryLocale: z.ZodString;
    privacyPolicy: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    shortDescription: z.ZodString;
    shortPrivacyPolicy: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    showThankYouPageAnnonInstructions: z.ZodBoolean;
    showThankyouPageFeedbackButton: z.ZodBoolean;
    slug: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    supportedLanguages: z.ZodArray<z.ZodString, "many">;
    tags: z.ZodArray<z.ZodString, "many">;
    thankYouMessage: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    title: z.ZodString;
    videoUrl: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">>;
export type ConversationDto = z.infer<typeof _ConversationDto>;
export declare const ConversationDto: z.ZodType<ConversationDto, z.ZodTypeDef, any>;
declare const _Translation3: z.ZodObject<{
    textContent: z.ZodType<z.objectOutputType<{
        format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
        id: z.ZodString;
        primaryLocale: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
        aiGenerated: z.ZodBoolean;
        content: z.ZodString;
        contentId: z.ZodString;
        id: z.ZodString;
        locale: z.ZodString;
        requiresValidation: z.ZodBoolean;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    textContent: z.ZodType<z.objectOutputType<{
        format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
        id: z.ZodString;
        primaryLocale: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
        aiGenerated: z.ZodBoolean;
        content: z.ZodString;
        contentId: z.ZodString;
        id: z.ZodString;
        locale: z.ZodString;
        requiresValidation: z.ZodBoolean;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    textContent: z.ZodType<z.objectOutputType<{
        format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
        id: z.ZodString;
        primaryLocale: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
        aiGenerated: z.ZodBoolean;
        content: z.ZodString;
        contentId: z.ZodString;
        id: z.ZodString;
        locale: z.ZodString;
        requiresValidation: z.ZodBoolean;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
}, z.ZodTypeAny, "passthrough">>;
export type Translation3 = z.infer<typeof _Translation3>;
export declare const Translation3: z.ZodType<Translation3, z.ZodTypeDef, any>;
declare const _ConversationTranslations: z.ZodObject<{
    callToAction: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
        textContent: z.ZodType<z.objectOutputType<{
            format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
            id: z.ZodString;
            primaryLocale: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
            aiGenerated: z.ZodBoolean;
            content: z.ZodString;
            contentId: z.ZodString;
            id: z.ZodString;
            locale: z.ZodString;
            requiresValidation: z.ZodBoolean;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
    description: z.ZodType<z.objectOutputType<{
        textContent: z.ZodType<z.objectOutputType<{
            format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
            id: z.ZodString;
            primaryLocale: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
            aiGenerated: z.ZodBoolean;
            content: z.ZodString;
            contentId: z.ZodString;
            id: z.ZodString;
            locale: z.ZodString;
            requiresValidation: z.ZodBoolean;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    faqs: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
        textContent: z.ZodType<z.objectOutputType<{
            format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
            id: z.ZodString;
            primaryLocale: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
            aiGenerated: z.ZodBoolean;
            content: z.ZodString;
            contentId: z.ZodString;
            id: z.ZodString;
            locale: z.ZodString;
            requiresValidation: z.ZodBoolean;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
    privacyPolicy: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
        textContent: z.ZodType<z.objectOutputType<{
            format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
            id: z.ZodString;
            primaryLocale: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
            aiGenerated: z.ZodBoolean;
            content: z.ZodString;
            contentId: z.ZodString;
            id: z.ZodString;
            locale: z.ZodString;
            requiresValidation: z.ZodBoolean;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
    shortDescription: z.ZodType<z.objectOutputType<{
        textContent: z.ZodType<z.objectOutputType<{
            format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
            id: z.ZodString;
            primaryLocale: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
            aiGenerated: z.ZodBoolean;
            content: z.ZodString;
            contentId: z.ZodString;
            id: z.ZodString;
            locale: z.ZodString;
            requiresValidation: z.ZodBoolean;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    shortPrivacyPolicy: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
        textContent: z.ZodType<z.objectOutputType<{
            format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
            id: z.ZodString;
            primaryLocale: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
            aiGenerated: z.ZodBoolean;
            content: z.ZodString;
            contentId: z.ZodString;
            id: z.ZodString;
            locale: z.ZodString;
            requiresValidation: z.ZodBoolean;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
    thankYouMessage: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
        textContent: z.ZodType<z.objectOutputType<{
            format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
            id: z.ZodString;
            primaryLocale: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
            aiGenerated: z.ZodBoolean;
            content: z.ZodString;
            contentId: z.ZodString;
            id: z.ZodString;
            locale: z.ZodString;
            requiresValidation: z.ZodBoolean;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
    title: z.ZodType<z.objectOutputType<{
        textContent: z.ZodType<z.objectOutputType<{
            format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
            id: z.ZodString;
            primaryLocale: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
            aiGenerated: z.ZodBoolean;
            content: z.ZodString;
            contentId: z.ZodString;
            id: z.ZodString;
            locale: z.ZodString;
            requiresValidation: z.ZodBoolean;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    callToAction: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
        textContent: z.ZodType<z.objectOutputType<{
            format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
            id: z.ZodString;
            primaryLocale: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
            aiGenerated: z.ZodBoolean;
            content: z.ZodString;
            contentId: z.ZodString;
            id: z.ZodString;
            locale: z.ZodString;
            requiresValidation: z.ZodBoolean;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
    description: z.ZodType<z.objectOutputType<{
        textContent: z.ZodType<z.objectOutputType<{
            format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
            id: z.ZodString;
            primaryLocale: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
            aiGenerated: z.ZodBoolean;
            content: z.ZodString;
            contentId: z.ZodString;
            id: z.ZodString;
            locale: z.ZodString;
            requiresValidation: z.ZodBoolean;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    faqs: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
        textContent: z.ZodType<z.objectOutputType<{
            format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
            id: z.ZodString;
            primaryLocale: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
            aiGenerated: z.ZodBoolean;
            content: z.ZodString;
            contentId: z.ZodString;
            id: z.ZodString;
            locale: z.ZodString;
            requiresValidation: z.ZodBoolean;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
    privacyPolicy: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
        textContent: z.ZodType<z.objectOutputType<{
            format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
            id: z.ZodString;
            primaryLocale: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
            aiGenerated: z.ZodBoolean;
            content: z.ZodString;
            contentId: z.ZodString;
            id: z.ZodString;
            locale: z.ZodString;
            requiresValidation: z.ZodBoolean;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
    shortDescription: z.ZodType<z.objectOutputType<{
        textContent: z.ZodType<z.objectOutputType<{
            format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
            id: z.ZodString;
            primaryLocale: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
            aiGenerated: z.ZodBoolean;
            content: z.ZodString;
            contentId: z.ZodString;
            id: z.ZodString;
            locale: z.ZodString;
            requiresValidation: z.ZodBoolean;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    shortPrivacyPolicy: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
        textContent: z.ZodType<z.objectOutputType<{
            format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
            id: z.ZodString;
            primaryLocale: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
            aiGenerated: z.ZodBoolean;
            content: z.ZodString;
            contentId: z.ZodString;
            id: z.ZodString;
            locale: z.ZodString;
            requiresValidation: z.ZodBoolean;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
    thankYouMessage: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
        textContent: z.ZodType<z.objectOutputType<{
            format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
            id: z.ZodString;
            primaryLocale: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
            aiGenerated: z.ZodBoolean;
            content: z.ZodString;
            contentId: z.ZodString;
            id: z.ZodString;
            locale: z.ZodString;
            requiresValidation: z.ZodBoolean;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
    title: z.ZodType<z.objectOutputType<{
        textContent: z.ZodType<z.objectOutputType<{
            format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
            id: z.ZodString;
            primaryLocale: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
            aiGenerated: z.ZodBoolean;
            content: z.ZodString;
            contentId: z.ZodString;
            id: z.ZodString;
            locale: z.ZodString;
            requiresValidation: z.ZodBoolean;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    callToAction: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
        textContent: z.ZodType<z.objectOutputType<{
            format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
            id: z.ZodString;
            primaryLocale: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
            aiGenerated: z.ZodBoolean;
            content: z.ZodString;
            contentId: z.ZodString;
            id: z.ZodString;
            locale: z.ZodString;
            requiresValidation: z.ZodBoolean;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
    description: z.ZodType<z.objectOutputType<{
        textContent: z.ZodType<z.objectOutputType<{
            format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
            id: z.ZodString;
            primaryLocale: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
            aiGenerated: z.ZodBoolean;
            content: z.ZodString;
            contentId: z.ZodString;
            id: z.ZodString;
            locale: z.ZodString;
            requiresValidation: z.ZodBoolean;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    faqs: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
        textContent: z.ZodType<z.objectOutputType<{
            format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
            id: z.ZodString;
            primaryLocale: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
            aiGenerated: z.ZodBoolean;
            content: z.ZodString;
            contentId: z.ZodString;
            id: z.ZodString;
            locale: z.ZodString;
            requiresValidation: z.ZodBoolean;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
    privacyPolicy: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
        textContent: z.ZodType<z.objectOutputType<{
            format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
            id: z.ZodString;
            primaryLocale: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
            aiGenerated: z.ZodBoolean;
            content: z.ZodString;
            contentId: z.ZodString;
            id: z.ZodString;
            locale: z.ZodString;
            requiresValidation: z.ZodBoolean;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
    shortDescription: z.ZodType<z.objectOutputType<{
        textContent: z.ZodType<z.objectOutputType<{
            format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
            id: z.ZodString;
            primaryLocale: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
            aiGenerated: z.ZodBoolean;
            content: z.ZodString;
            contentId: z.ZodString;
            id: z.ZodString;
            locale: z.ZodString;
            requiresValidation: z.ZodBoolean;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    shortPrivacyPolicy: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
        textContent: z.ZodType<z.objectOutputType<{
            format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
            id: z.ZodString;
            primaryLocale: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
            aiGenerated: z.ZodBoolean;
            content: z.ZodString;
            contentId: z.ZodString;
            id: z.ZodString;
            locale: z.ZodString;
            requiresValidation: z.ZodBoolean;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
    thankYouMessage: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
        textContent: z.ZodType<z.objectOutputType<{
            format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
            id: z.ZodString;
            primaryLocale: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
            aiGenerated: z.ZodBoolean;
            content: z.ZodString;
            contentId: z.ZodString;
            id: z.ZodString;
            locale: z.ZodString;
            requiresValidation: z.ZodBoolean;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
    title: z.ZodType<z.objectOutputType<{
        textContent: z.ZodType<z.objectOutputType<{
            format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
            id: z.ZodString;
            primaryLocale: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
            aiGenerated: z.ZodBoolean;
            content: z.ZodString;
            contentId: z.ZodString;
            id: z.ZodString;
            locale: z.ZodString;
            requiresValidation: z.ZodBoolean;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
}, z.ZodTypeAny, "passthrough">>;
export type ConversationTranslations = z.infer<typeof _ConversationTranslations>;
export declare const ConversationTranslations: z.ZodType<ConversationTranslations, z.ZodTypeDef, any>;
declare const _ConversationWithTranslations: z.ZodObject<{
    allowRevisitAfterFinishing: z.ZodBoolean;
    callToAction: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    chatBotId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    createdAt: z.ZodString;
    defaultWorkflowId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    description: z.ZodString;
    enableQaChatBot: z.ZodBoolean;
    enableSignupPrompts: z.ZodBoolean;
    faqs: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    id: z.ZodString;
    image: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    isComplete: z.ZodBoolean;
    isInviteOnly: z.ZodBoolean;
    isLive: z.ZodBoolean;
    isPublic: z.ZodBoolean;
    knowledgeBaseId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    metadata: z.ZodUnknown;
    organizationId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    ownerId: z.ZodString;
    primaryLocale: z.ZodString;
    privacyPolicy: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    shortDescription: z.ZodString;
    shortPrivacyPolicy: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    showThankYouPageAnnonInstructions: z.ZodBoolean;
    showThankyouPageFeedbackButton: z.ZodBoolean;
    slug: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    supportedLanguages: z.ZodArray<z.ZodString, "many">;
    tags: z.ZodArray<z.ZodString, "many">;
    thankYouMessage: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    title: z.ZodString;
    translations: z.ZodType<z.objectOutputType<{
        callToAction: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
            textContent: z.ZodType<z.objectOutputType<{
                format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                id: z.ZodString;
                primaryLocale: z.ZodString;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                aiGenerated: z.ZodBoolean;
                content: z.ZodString;
                contentId: z.ZodString;
                id: z.ZodString;
                locale: z.ZodString;
                requiresValidation: z.ZodBoolean;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
        description: z.ZodType<z.objectOutputType<{
            textContent: z.ZodType<z.objectOutputType<{
                format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                id: z.ZodString;
                primaryLocale: z.ZodString;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                aiGenerated: z.ZodBoolean;
                content: z.ZodString;
                contentId: z.ZodString;
                id: z.ZodString;
                locale: z.ZodString;
                requiresValidation: z.ZodBoolean;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        faqs: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
            textContent: z.ZodType<z.objectOutputType<{
                format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                id: z.ZodString;
                primaryLocale: z.ZodString;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                aiGenerated: z.ZodBoolean;
                content: z.ZodString;
                contentId: z.ZodString;
                id: z.ZodString;
                locale: z.ZodString;
                requiresValidation: z.ZodBoolean;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
        privacyPolicy: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
            textContent: z.ZodType<z.objectOutputType<{
                format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                id: z.ZodString;
                primaryLocale: z.ZodString;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                aiGenerated: z.ZodBoolean;
                content: z.ZodString;
                contentId: z.ZodString;
                id: z.ZodString;
                locale: z.ZodString;
                requiresValidation: z.ZodBoolean;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
        shortDescription: z.ZodType<z.objectOutputType<{
            textContent: z.ZodType<z.objectOutputType<{
                format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                id: z.ZodString;
                primaryLocale: z.ZodString;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                aiGenerated: z.ZodBoolean;
                content: z.ZodString;
                contentId: z.ZodString;
                id: z.ZodString;
                locale: z.ZodString;
                requiresValidation: z.ZodBoolean;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        shortPrivacyPolicy: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
            textContent: z.ZodType<z.objectOutputType<{
                format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                id: z.ZodString;
                primaryLocale: z.ZodString;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                aiGenerated: z.ZodBoolean;
                content: z.ZodString;
                contentId: z.ZodString;
                id: z.ZodString;
                locale: z.ZodString;
                requiresValidation: z.ZodBoolean;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
        thankYouMessage: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
            textContent: z.ZodType<z.objectOutputType<{
                format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                id: z.ZodString;
                primaryLocale: z.ZodString;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                aiGenerated: z.ZodBoolean;
                content: z.ZodString;
                contentId: z.ZodString;
                id: z.ZodString;
                locale: z.ZodString;
                requiresValidation: z.ZodBoolean;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
        title: z.ZodType<z.objectOutputType<{
            textContent: z.ZodType<z.objectOutputType<{
                format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                id: z.ZodString;
                primaryLocale: z.ZodString;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                aiGenerated: z.ZodBoolean;
                content: z.ZodString;
                contentId: z.ZodString;
                id: z.ZodString;
                locale: z.ZodString;
                requiresValidation: z.ZodBoolean;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    updatedAt: z.ZodString;
    videoUrl: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    allowRevisitAfterFinishing: z.ZodBoolean;
    callToAction: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    chatBotId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    createdAt: z.ZodString;
    defaultWorkflowId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    description: z.ZodString;
    enableQaChatBot: z.ZodBoolean;
    enableSignupPrompts: z.ZodBoolean;
    faqs: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    id: z.ZodString;
    image: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    isComplete: z.ZodBoolean;
    isInviteOnly: z.ZodBoolean;
    isLive: z.ZodBoolean;
    isPublic: z.ZodBoolean;
    knowledgeBaseId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    metadata: z.ZodUnknown;
    organizationId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    ownerId: z.ZodString;
    primaryLocale: z.ZodString;
    privacyPolicy: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    shortDescription: z.ZodString;
    shortPrivacyPolicy: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    showThankYouPageAnnonInstructions: z.ZodBoolean;
    showThankyouPageFeedbackButton: z.ZodBoolean;
    slug: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    supportedLanguages: z.ZodArray<z.ZodString, "many">;
    tags: z.ZodArray<z.ZodString, "many">;
    thankYouMessage: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    title: z.ZodString;
    translations: z.ZodType<z.objectOutputType<{
        callToAction: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
            textContent: z.ZodType<z.objectOutputType<{
                format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                id: z.ZodString;
                primaryLocale: z.ZodString;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                aiGenerated: z.ZodBoolean;
                content: z.ZodString;
                contentId: z.ZodString;
                id: z.ZodString;
                locale: z.ZodString;
                requiresValidation: z.ZodBoolean;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
        description: z.ZodType<z.objectOutputType<{
            textContent: z.ZodType<z.objectOutputType<{
                format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                id: z.ZodString;
                primaryLocale: z.ZodString;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                aiGenerated: z.ZodBoolean;
                content: z.ZodString;
                contentId: z.ZodString;
                id: z.ZodString;
                locale: z.ZodString;
                requiresValidation: z.ZodBoolean;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        faqs: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
            textContent: z.ZodType<z.objectOutputType<{
                format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                id: z.ZodString;
                primaryLocale: z.ZodString;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                aiGenerated: z.ZodBoolean;
                content: z.ZodString;
                contentId: z.ZodString;
                id: z.ZodString;
                locale: z.ZodString;
                requiresValidation: z.ZodBoolean;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
        privacyPolicy: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
            textContent: z.ZodType<z.objectOutputType<{
                format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                id: z.ZodString;
                primaryLocale: z.ZodString;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                aiGenerated: z.ZodBoolean;
                content: z.ZodString;
                contentId: z.ZodString;
                id: z.ZodString;
                locale: z.ZodString;
                requiresValidation: z.ZodBoolean;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
        shortDescription: z.ZodType<z.objectOutputType<{
            textContent: z.ZodType<z.objectOutputType<{
                format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                id: z.ZodString;
                primaryLocale: z.ZodString;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                aiGenerated: z.ZodBoolean;
                content: z.ZodString;
                contentId: z.ZodString;
                id: z.ZodString;
                locale: z.ZodString;
                requiresValidation: z.ZodBoolean;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        shortPrivacyPolicy: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
            textContent: z.ZodType<z.objectOutputType<{
                format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                id: z.ZodString;
                primaryLocale: z.ZodString;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                aiGenerated: z.ZodBoolean;
                content: z.ZodString;
                contentId: z.ZodString;
                id: z.ZodString;
                locale: z.ZodString;
                requiresValidation: z.ZodBoolean;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
        thankYouMessage: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
            textContent: z.ZodType<z.objectOutputType<{
                format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                id: z.ZodString;
                primaryLocale: z.ZodString;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                aiGenerated: z.ZodBoolean;
                content: z.ZodString;
                contentId: z.ZodString;
                id: z.ZodString;
                locale: z.ZodString;
                requiresValidation: z.ZodBoolean;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
        title: z.ZodType<z.objectOutputType<{
            textContent: z.ZodType<z.objectOutputType<{
                format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                id: z.ZodString;
                primaryLocale: z.ZodString;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                aiGenerated: z.ZodBoolean;
                content: z.ZodString;
                contentId: z.ZodString;
                id: z.ZodString;
                locale: z.ZodString;
                requiresValidation: z.ZodBoolean;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    updatedAt: z.ZodString;
    videoUrl: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    allowRevisitAfterFinishing: z.ZodBoolean;
    callToAction: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    chatBotId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    createdAt: z.ZodString;
    defaultWorkflowId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    description: z.ZodString;
    enableQaChatBot: z.ZodBoolean;
    enableSignupPrompts: z.ZodBoolean;
    faqs: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    id: z.ZodString;
    image: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    isComplete: z.ZodBoolean;
    isInviteOnly: z.ZodBoolean;
    isLive: z.ZodBoolean;
    isPublic: z.ZodBoolean;
    knowledgeBaseId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    metadata: z.ZodUnknown;
    organizationId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    ownerId: z.ZodString;
    primaryLocale: z.ZodString;
    privacyPolicy: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    shortDescription: z.ZodString;
    shortPrivacyPolicy: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    showThankYouPageAnnonInstructions: z.ZodBoolean;
    showThankyouPageFeedbackButton: z.ZodBoolean;
    slug: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    supportedLanguages: z.ZodArray<z.ZodString, "many">;
    tags: z.ZodArray<z.ZodString, "many">;
    thankYouMessage: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    title: z.ZodString;
    translations: z.ZodType<z.objectOutputType<{
        callToAction: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
            textContent: z.ZodType<z.objectOutputType<{
                format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                id: z.ZodString;
                primaryLocale: z.ZodString;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                aiGenerated: z.ZodBoolean;
                content: z.ZodString;
                contentId: z.ZodString;
                id: z.ZodString;
                locale: z.ZodString;
                requiresValidation: z.ZodBoolean;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
        description: z.ZodType<z.objectOutputType<{
            textContent: z.ZodType<z.objectOutputType<{
                format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                id: z.ZodString;
                primaryLocale: z.ZodString;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                aiGenerated: z.ZodBoolean;
                content: z.ZodString;
                contentId: z.ZodString;
                id: z.ZodString;
                locale: z.ZodString;
                requiresValidation: z.ZodBoolean;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        faqs: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
            textContent: z.ZodType<z.objectOutputType<{
                format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                id: z.ZodString;
                primaryLocale: z.ZodString;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                aiGenerated: z.ZodBoolean;
                content: z.ZodString;
                contentId: z.ZodString;
                id: z.ZodString;
                locale: z.ZodString;
                requiresValidation: z.ZodBoolean;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
        privacyPolicy: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
            textContent: z.ZodType<z.objectOutputType<{
                format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                id: z.ZodString;
                primaryLocale: z.ZodString;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                aiGenerated: z.ZodBoolean;
                content: z.ZodString;
                contentId: z.ZodString;
                id: z.ZodString;
                locale: z.ZodString;
                requiresValidation: z.ZodBoolean;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
        shortDescription: z.ZodType<z.objectOutputType<{
            textContent: z.ZodType<z.objectOutputType<{
                format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                id: z.ZodString;
                primaryLocale: z.ZodString;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                aiGenerated: z.ZodBoolean;
                content: z.ZodString;
                contentId: z.ZodString;
                id: z.ZodString;
                locale: z.ZodString;
                requiresValidation: z.ZodBoolean;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        shortPrivacyPolicy: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
            textContent: z.ZodType<z.objectOutputType<{
                format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                id: z.ZodString;
                primaryLocale: z.ZodString;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                aiGenerated: z.ZodBoolean;
                content: z.ZodString;
                contentId: z.ZodString;
                id: z.ZodString;
                locale: z.ZodString;
                requiresValidation: z.ZodBoolean;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
        thankYouMessage: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
            textContent: z.ZodType<z.objectOutputType<{
                format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                id: z.ZodString;
                primaryLocale: z.ZodString;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                aiGenerated: z.ZodBoolean;
                content: z.ZodString;
                contentId: z.ZodString;
                id: z.ZodString;
                locale: z.ZodString;
                requiresValidation: z.ZodBoolean;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
        title: z.ZodType<z.objectOutputType<{
            textContent: z.ZodType<z.objectOutputType<{
                format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                id: z.ZodString;
                primaryLocale: z.ZodString;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                aiGenerated: z.ZodBoolean;
                content: z.ZodString;
                contentId: z.ZodString;
                id: z.ZodString;
                locale: z.ZodString;
                requiresValidation: z.ZodBoolean;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    updatedAt: z.ZodString;
    videoUrl: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">>;
export type ConversationWithTranslations = z.infer<typeof _ConversationWithTranslations>;
export declare const ConversationWithTranslations: z.ZodType<ConversationWithTranslations, z.ZodTypeDef, any>;
declare const _ConversationResponse: z.ZodUnion<[z.ZodType<z.objectOutputType<{
    allowRevisitAfterFinishing: z.ZodBoolean;
    callToAction: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    chatBotId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    createdAt: z.ZodString;
    defaultWorkflowId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    description: z.ZodString;
    enableQaChatBot: z.ZodBoolean;
    enableSignupPrompts: z.ZodBoolean;
    faqs: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    id: z.ZodString;
    image: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    isComplete: z.ZodBoolean;
    isInviteOnly: z.ZodBoolean;
    isLive: z.ZodBoolean;
    isPublic: z.ZodBoolean;
    knowledgeBaseId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    metadata: z.ZodUnknown;
    organizationId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    ownerId: z.ZodString;
    primaryLocale: z.ZodString;
    privacyPolicy: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    shortDescription: z.ZodString;
    shortPrivacyPolicy: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    showThankYouPageAnnonInstructions: z.ZodBoolean;
    showThankyouPageFeedbackButton: z.ZodBoolean;
    slug: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    supportedLanguages: z.ZodArray<z.ZodString, "many">;
    tags: z.ZodArray<z.ZodString, "many">;
    thankYouMessage: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    title: z.ZodString;
    translations: z.ZodType<z.objectOutputType<{
        callToAction: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
            textContent: z.ZodType<z.objectOutputType<{
                format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                id: z.ZodString;
                primaryLocale: z.ZodString;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                aiGenerated: z.ZodBoolean;
                content: z.ZodString;
                contentId: z.ZodString;
                id: z.ZodString;
                locale: z.ZodString;
                requiresValidation: z.ZodBoolean;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
        description: z.ZodType<z.objectOutputType<{
            textContent: z.ZodType<z.objectOutputType<{
                format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                id: z.ZodString;
                primaryLocale: z.ZodString;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                aiGenerated: z.ZodBoolean;
                content: z.ZodString;
                contentId: z.ZodString;
                id: z.ZodString;
                locale: z.ZodString;
                requiresValidation: z.ZodBoolean;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        faqs: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
            textContent: z.ZodType<z.objectOutputType<{
                format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                id: z.ZodString;
                primaryLocale: z.ZodString;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                aiGenerated: z.ZodBoolean;
                content: z.ZodString;
                contentId: z.ZodString;
                id: z.ZodString;
                locale: z.ZodString;
                requiresValidation: z.ZodBoolean;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
        privacyPolicy: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
            textContent: z.ZodType<z.objectOutputType<{
                format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                id: z.ZodString;
                primaryLocale: z.ZodString;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                aiGenerated: z.ZodBoolean;
                content: z.ZodString;
                contentId: z.ZodString;
                id: z.ZodString;
                locale: z.ZodString;
                requiresValidation: z.ZodBoolean;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
        shortDescription: z.ZodType<z.objectOutputType<{
            textContent: z.ZodType<z.objectOutputType<{
                format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                id: z.ZodString;
                primaryLocale: z.ZodString;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                aiGenerated: z.ZodBoolean;
                content: z.ZodString;
                contentId: z.ZodString;
                id: z.ZodString;
                locale: z.ZodString;
                requiresValidation: z.ZodBoolean;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        shortPrivacyPolicy: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
            textContent: z.ZodType<z.objectOutputType<{
                format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                id: z.ZodString;
                primaryLocale: z.ZodString;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                aiGenerated: z.ZodBoolean;
                content: z.ZodString;
                contentId: z.ZodString;
                id: z.ZodString;
                locale: z.ZodString;
                requiresValidation: z.ZodBoolean;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
        thankYouMessage: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
            textContent: z.ZodType<z.objectOutputType<{
                format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                id: z.ZodString;
                primaryLocale: z.ZodString;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                aiGenerated: z.ZodBoolean;
                content: z.ZodString;
                contentId: z.ZodString;
                id: z.ZodString;
                locale: z.ZodString;
                requiresValidation: z.ZodBoolean;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
        title: z.ZodType<z.objectOutputType<{
            textContent: z.ZodType<z.objectOutputType<{
                format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                id: z.ZodString;
                primaryLocale: z.ZodString;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                aiGenerated: z.ZodBoolean;
                content: z.ZodString;
                contentId: z.ZodString;
                id: z.ZodString;
                locale: z.ZodString;
                requiresValidation: z.ZodBoolean;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    updatedAt: z.ZodString;
    videoUrl: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodType<z.objectOutputType<{
    allowRevisitAfterFinishing: z.ZodBoolean;
    callToAction: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    chatBotId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    description: z.ZodString;
    enableQaChatBot: z.ZodBoolean;
    enableSignupPrompts: z.ZodBoolean;
    faqs: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    id: z.ZodString;
    imageUrl: z.ZodString;
    isComplete: z.ZodBoolean;
    isInviteOnly: z.ZodBoolean;
    isLive: z.ZodBoolean;
    isPublic: z.ZodBoolean;
    knowledgeBaseId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    metadata: z.ZodUnknown;
    organizationId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    primaryLocale: z.ZodString;
    privacyPolicy: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    shortDescription: z.ZodString;
    shortPrivacyPolicy: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    showThankYouPageAnnonInstructions: z.ZodBoolean;
    showThankyouPageFeedbackButton: z.ZodBoolean;
    slug: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    supportedLanguages: z.ZodArray<z.ZodString, "many">;
    tags: z.ZodArray<z.ZodString, "many">;
    thankYouMessage: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    title: z.ZodString;
    videoUrl: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>]>;
export type ConversationResponse = z.infer<typeof _ConversationResponse>;
export declare const ConversationResponse: z.ZodType<ConversationResponse, z.ZodTypeDef, any>;
declare const _PartialConversation: z.ZodObject<{
    allow_revisit_after_finishing: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    call_to_action: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    chat_bot_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    default_workflow_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    description: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    enable_qa_chat_bot: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    enable_signup_prompts: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    faqs: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    image: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    is_complete: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    is_invite_only: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    is_live: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    is_public: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    knowledge_base_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    metadata: z.ZodOptional<z.ZodUnknown>;
    organization_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    primary_locale: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    privacy_policy: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    short_description: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    short_privacy_policy: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    show_thank_you_page_annon_instructions: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    show_thankyou_page_feedback_button: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    slug: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    supported_languages: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodString, "many">, z.ZodNull]>>;
    tags: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodString, "many">, z.ZodNull]>>;
    thank_you_message: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    title: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    video_url: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    allow_revisit_after_finishing: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    call_to_action: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    chat_bot_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    default_workflow_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    description: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    enable_qa_chat_bot: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    enable_signup_prompts: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    faqs: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    image: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    is_complete: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    is_invite_only: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    is_live: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    is_public: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    knowledge_base_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    metadata: z.ZodOptional<z.ZodUnknown>;
    organization_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    primary_locale: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    privacy_policy: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    short_description: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    short_privacy_policy: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    show_thank_you_page_annon_instructions: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    show_thankyou_page_feedback_button: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    slug: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    supported_languages: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodString, "many">, z.ZodNull]>>;
    tags: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodString, "many">, z.ZodNull]>>;
    thank_you_message: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    title: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    video_url: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    allow_revisit_after_finishing: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    call_to_action: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    chat_bot_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    default_workflow_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    description: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    enable_qa_chat_bot: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    enable_signup_prompts: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    faqs: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    image: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    is_complete: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    is_invite_only: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    is_live: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    is_public: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    knowledge_base_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    metadata: z.ZodOptional<z.ZodUnknown>;
    organization_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    primary_locale: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    privacy_policy: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    short_description: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    short_privacy_policy: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    show_thank_you_page_annon_instructions: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    show_thankyou_page_feedback_button: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    slug: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    supported_languages: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodString, "many">, z.ZodNull]>>;
    tags: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodString, "many">, z.ZodNull]>>;
    thank_you_message: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    title: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    video_url: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">>;
export type PartialConversation = z.infer<typeof _PartialConversation>;
export declare const PartialConversation: z.ZodType<PartialConversation, z.ZodTypeDef, any>;
declare const _OrganizationWithPermissionDto: z.ZodObject<{
    id: z.ZodString;
    name: z.ZodString;
    roleName: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    id: z.ZodString;
    name: z.ZodString;
    roleName: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    id: z.ZodString;
    name: z.ZodString;
    roleName: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type OrganizationWithPermissionDto = z.infer<typeof _OrganizationWithPermissionDto>;
export declare const OrganizationWithPermissionDto: z.ZodType<OrganizationWithPermissionDto, z.ZodTypeDef, any>;
declare const _CohostInfo: z.ZodObject<{
    organization_id: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    organization_id: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    organization_id: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type CohostInfo = z.infer<typeof _CohostInfo>;
export declare const CohostInfo: z.ZodType<CohostInfo, z.ZodTypeDef, any>;
declare const _SendNotificationRequest: z.ZodObject<{
    content: z.ZodString;
    delivery_method: z.ZodOptional<z.ZodUnion<[z.ZodType<"email" | "in_app", z.ZodTypeDef, any>, z.ZodNull]>>;
    html_content: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    notification_type: z.ZodOptional<z.ZodUnion<[z.ZodType<"info" | "warning" | "error" | "success", z.ZodTypeDef, any>, z.ZodNull]>>;
    test_email_recipient: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    title: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    content: z.ZodString;
    delivery_method: z.ZodOptional<z.ZodUnion<[z.ZodType<"email" | "in_app", z.ZodTypeDef, any>, z.ZodNull]>>;
    html_content: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    notification_type: z.ZodOptional<z.ZodUnion<[z.ZodType<"info" | "warning" | "error" | "success", z.ZodTypeDef, any>, z.ZodNull]>>;
    test_email_recipient: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    title: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    content: z.ZodString;
    delivery_method: z.ZodOptional<z.ZodUnion<[z.ZodType<"email" | "in_app", z.ZodTypeDef, any>, z.ZodNull]>>;
    html_content: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    notification_type: z.ZodOptional<z.ZodUnion<[z.ZodType<"info" | "warning" | "error" | "success", z.ZodTypeDef, any>, z.ZodNull]>>;
    test_email_recipient: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    title: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type SendNotificationRequest = z.infer<typeof _SendNotificationRequest>;
export declare const SendNotificationRequest: z.ZodType<SendNotificationRequest, z.ZodTypeDef, any>;
declare const _SendEmailNotificationResponse: z.ZodObject<{
    failedRecipients: z.ZodDefault<z.ZodOptional<z.ZodArray<z.ZodString, "many">>>;
    message: z.ZodString;
    notificationId: z.ZodString;
    participantsNotified: z.ZodNumber;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    failedRecipients: z.ZodDefault<z.ZodOptional<z.ZodArray<z.ZodString, "many">>>;
    message: z.ZodString;
    notificationId: z.ZodString;
    participantsNotified: z.ZodNumber;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    failedRecipients: z.ZodDefault<z.ZodOptional<z.ZodArray<z.ZodString, "many">>>;
    message: z.ZodString;
    notificationId: z.ZodString;
    participantsNotified: z.ZodNumber;
}, z.ZodTypeAny, "passthrough">>;
export type SendEmailNotificationResponse = z.infer<typeof _SendEmailNotificationResponse>;
export declare const SendEmailNotificationResponse: z.ZodType<SendEmailNotificationResponse, z.ZodTypeDef, any>;
declare const _NotificationRecipientsResponse: z.ZodObject<{
    emailRecipientCount: z.ZodNumber;
    emailRecipients: z.ZodArray<z.ZodString, "many">;
    participantCount: z.ZodNumber;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    emailRecipientCount: z.ZodNumber;
    emailRecipients: z.ZodArray<z.ZodString, "many">;
    participantCount: z.ZodNumber;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    emailRecipientCount: z.ZodNumber;
    emailRecipients: z.ZodArray<z.ZodString, "many">;
    participantCount: z.ZodNumber;
}, z.ZodTypeAny, "passthrough">>;
export type NotificationRecipientsResponse = z.infer<typeof _NotificationRecipientsResponse>;
export declare const NotificationRecipientsResponse: z.ZodType<NotificationRecipientsResponse, z.ZodTypeDef, any>;
declare const _RegisterEmailRequest: z.ZodObject<{
    email: z.ZodString;
    receive_similar_conversation_updates_by_email: z.ZodBoolean;
    receive_updates_by_email: z.ZodBoolean;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    email: z.ZodString;
    receive_similar_conversation_updates_by_email: z.ZodBoolean;
    receive_updates_by_email: z.ZodBoolean;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    email: z.ZodString;
    receive_similar_conversation_updates_by_email: z.ZodBoolean;
    receive_updates_by_email: z.ZodBoolean;
}, z.ZodTypeAny, "passthrough">>;
export type RegisterEmailRequest = z.infer<typeof _RegisterEmailRequest>;
export declare const RegisterEmailRequest: z.ZodType<RegisterEmailRequest, z.ZodTypeDef, any>;
declare const _RegisterEmailResponse: z.ZodObject<{
    conversationId: z.ZodString;
    email: z.ZodString;
    id: z.ZodString;
    message: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    conversationId: z.ZodString;
    email: z.ZodString;
    id: z.ZodString;
    message: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    conversationId: z.ZodString;
    email: z.ZodString;
    id: z.ZodString;
    message: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type RegisterEmailResponse = z.infer<typeof _RegisterEmailResponse>;
export declare const RegisterEmailResponse: z.ZodType<RegisterEmailResponse, z.ZodTypeDef, any>;
declare const _WorkflowDto: z.ZodObject<{
    autoLogin: z.ZodBoolean;
    conversationId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    createdAt: z.ZodString;
    description: z.ZodString;
    eventId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    id: z.ZodString;
    isActive: z.ZodBoolean;
    isPublic: z.ZodBoolean;
    name: z.ZodString;
    regionId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    autoLogin: z.ZodBoolean;
    conversationId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    createdAt: z.ZodString;
    description: z.ZodString;
    eventId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    id: z.ZodString;
    isActive: z.ZodBoolean;
    isPublic: z.ZodBoolean;
    name: z.ZodString;
    regionId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    autoLogin: z.ZodBoolean;
    conversationId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    createdAt: z.ZodString;
    description: z.ZodString;
    eventId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    id: z.ZodString;
    isActive: z.ZodBoolean;
    isPublic: z.ZodBoolean;
    name: z.ZodString;
    regionId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">>;
export type WorkflowDto = z.infer<typeof _WorkflowDto>;
export declare const WorkflowDto: z.ZodType<WorkflowDto, z.ZodTypeDef, any>;
declare const _CreateWorkflow: z.ZodObject<{
    auto_login: z.ZodBoolean;
    description: z.ZodString;
    is_active: z.ZodBoolean;
    is_public: z.ZodBoolean;
    name: z.ZodString;
    region_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    auto_login: z.ZodBoolean;
    description: z.ZodString;
    is_active: z.ZodBoolean;
    is_public: z.ZodBoolean;
    name: z.ZodString;
    region_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    auto_login: z.ZodBoolean;
    description: z.ZodString;
    is_active: z.ZodBoolean;
    is_public: z.ZodBoolean;
    name: z.ZodString;
    region_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">>;
export type CreateWorkflow = z.infer<typeof _CreateWorkflow>;
export declare const CreateWorkflow: z.ZodType<CreateWorkflow, z.ZodTypeDef, any>;
declare const _PartialWorkflow: z.ZodObject<{
    auto_login: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    description: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    event_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    is_active: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    is_public: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    name: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    region_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    auto_login: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    description: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    event_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    is_active: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    is_public: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    name: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    region_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    auto_login: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    description: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    event_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    is_active: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    is_public: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    name: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    region_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">>;
export type PartialWorkflow = z.infer<typeof _PartialWorkflow>;
export declare const PartialWorkflow: z.ZodType<PartialWorkflow, z.ZodTypeDef, any>;
declare const _ActivationRule: z.ZodLiteral<"manual">;
export type ActivationRule = z.infer<typeof _ActivationRule>;
export declare const ActivationRule: z.ZodType<ActivationRule, z.ZodTypeDef, any>;
declare const _LearnPage: z.ZodObject<{
    text_content_id: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    text_content_id: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    text_content_id: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type LearnPage = z.infer<typeof _LearnPage>;
export declare const LearnPage: z.ZodType<LearnPage, z.ZodTypeDef, any>;
declare const _LocalizedPage: z.ZodObject<{
    content: z.ZodString;
    type: z.ZodLiteral<"markdown">;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    content: z.ZodString;
    type: z.ZodLiteral<"markdown">;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    content: z.ZodString;
    type: z.ZodLiteral<"markdown">;
}, z.ZodTypeAny, "passthrough">>;
export type LocalizedPage = z.infer<typeof _LocalizedPage>;
export declare const LocalizedPage: z.ZodType<LocalizedPage, z.ZodTypeDef, any>;
declare const _LearnPageEntry: z.ZodUnion<[z.ZodType<z.objectOutputType<{
    text_content_id: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodArray<z.ZodType<z.objectOutputType<{
    content: z.ZodString;
    type: z.ZodLiteral<"markdown">;
}, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">]>;
export type LearnPageEntry = z.infer<typeof _LearnPageEntry>;
export declare const LearnPageEntry: z.ZodType<LearnPageEntry, z.ZodTypeDef, any>;
declare const _Category: z.ZodObject<{
    label: z.ZodString;
    value: z.ZodNumber;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    label: z.ZodString;
    value: z.ZodNumber;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    label: z.ZodString;
    value: z.ZodNumber;
}, z.ZodTypeAny, "passthrough">>;
export type Category = z.infer<typeof _Category>;
export declare const Category: z.ZodType<Category, z.ZodTypeDef, any>;
declare const _QuestionType: z.ZodUnion<[z.ZodLiteral<"text">, z.ZodObject<{
    likert_scale: z.ZodObject<{
        categories: z.ZodArray<z.ZodType<z.objectOutputType<{
            label: z.ZodString;
            value: z.ZodNumber;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    }, "passthrough", z.ZodTypeAny, z.objectOutputType<{
        categories: z.ZodArray<z.ZodType<z.objectOutputType<{
            label: z.ZodString;
            value: z.ZodNumber;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    }, z.ZodTypeAny, "passthrough">, z.objectInputType<{
        categories: z.ZodArray<z.ZodType<z.objectOutputType<{
            label: z.ZodString;
            value: z.ZodNumber;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    }, z.ZodTypeAny, "passthrough">>;
}, "strip", z.ZodTypeAny, {
    likert_scale: {
        categories: z.objectOutputType<{
            label: z.ZodString;
            value: z.ZodNumber;
        }, z.ZodTypeAny, "passthrough">[];
    } & {
        [k: string]: unknown;
    };
}, {
    likert_scale: {
        categories: any[];
    } & {
        [k: string]: unknown;
    };
}>, z.ZodObject<{
    continuous: z.ZodObject<{
        max_label: z.ZodString;
        max_value: z.ZodDefault<z.ZodOptional<z.ZodNumber>>;
        min_label: z.ZodString;
        min_value: z.ZodDefault<z.ZodOptional<z.ZodNumber>>;
        sub_steps: z.ZodDefault<z.ZodOptional<z.ZodNumber>>;
    }, "passthrough", z.ZodTypeAny, z.objectOutputType<{
        max_label: z.ZodString;
        max_value: z.ZodDefault<z.ZodOptional<z.ZodNumber>>;
        min_label: z.ZodString;
        min_value: z.ZodDefault<z.ZodOptional<z.ZodNumber>>;
        sub_steps: z.ZodDefault<z.ZodOptional<z.ZodNumber>>;
    }, z.ZodTypeAny, "passthrough">, z.objectInputType<{
        max_label: z.ZodString;
        max_value: z.ZodDefault<z.ZodOptional<z.ZodNumber>>;
        min_label: z.ZodString;
        min_value: z.ZodDefault<z.ZodOptional<z.ZodNumber>>;
        sub_steps: z.ZodDefault<z.ZodOptional<z.ZodNumber>>;
    }, z.ZodTypeAny, "passthrough">>;
}, "strip", z.ZodTypeAny, {
    continuous: {
        max_label: string;
        max_value: number;
        min_label: string;
        min_value: number;
        sub_steps: number;
    } & {
        [k: string]: unknown;
    };
}, {
    continuous: {
        max_label: string;
        min_label: string;
        max_value?: number | undefined;
        min_value?: number | undefined;
        sub_steps?: number | undefined;
    } & {
        [k: string]: unknown;
    };
}>]>;
export type QuestionType = z.infer<typeof _QuestionType>;
export declare const QuestionType: z.ZodType<QuestionType, z.ZodTypeDef, any>;
declare const _Question: z.ZodObject<{
    id: z.ZodString;
    text: z.ZodString;
    type: z.ZodType<"text" | {
        likert_scale: {
            categories: z.objectOutputType<{
                label: z.ZodString;
                value: z.ZodNumber;
            }, z.ZodTypeAny, "passthrough">[];
        } & {
            [k: string]: unknown;
        };
    } | {
        continuous: {
            max_label: string;
            max_value: number;
            min_label: string;
            min_value: number;
            sub_steps: number;
        } & {
            [k: string]: unknown;
        };
    }, z.ZodTypeDef, any>;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    id: z.ZodString;
    text: z.ZodString;
    type: z.ZodType<"text" | {
        likert_scale: {
            categories: z.objectOutputType<{
                label: z.ZodString;
                value: z.ZodNumber;
            }, z.ZodTypeAny, "passthrough">[];
        } & {
            [k: string]: unknown;
        };
    } | {
        continuous: {
            max_label: string;
            max_value: number;
            min_label: string;
            min_value: number;
            sub_steps: number;
        } & {
            [k: string]: unknown;
        };
    }, z.ZodTypeDef, any>;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    id: z.ZodString;
    text: z.ZodString;
    type: z.ZodType<"text" | {
        likert_scale: {
            categories: z.objectOutputType<{
                label: z.ZodString;
                value: z.ZodNumber;
            }, z.ZodTypeAny, "passthrough">[];
        } & {
            [k: string]: unknown;
        };
    } | {
        continuous: {
            max_label: string;
            max_value: number;
            min_label: string;
            min_value: number;
            sub_steps: number;
        } & {
            [k: string]: unknown;
        };
    }, z.ZodTypeDef, any>;
}, z.ZodTypeAny, "passthrough">>;
export type Question = z.infer<typeof _Question>;
export declare const Question: z.ZodType<Question, z.ZodTypeDef, any>;
declare const _ThinkingSpaceQuestion: z.ZodObject<{
    id: z.ZodString;
    intent: z.ZodString;
    text: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    id: z.ZodString;
    intent: z.ZodString;
    text: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    id: z.ZodString;
    intent: z.ZodString;
    text: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type ThinkingSpaceQuestion = z.infer<typeof _ThinkingSpaceQuestion>;
export declare const ThinkingSpaceQuestion: z.ZodType<ThinkingSpaceQuestion, z.ZodTypeDef, any>;
declare const _ToolConfig: z.ZodUnion<[z.ZodObject<{
    admin_password: z.ZodString;
    admin_user: z.ZodString;
    description: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
    is_active: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
    label_seeds_as_conversation_starter: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
    poll_id: z.ZodString;
    required_votes: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
    server_url: z.ZodString;
    show_remaining_statements: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
    strict_moderation: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
    topic: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
    type: z.ZodLiteral<"polis">;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    admin_password: z.ZodString;
    admin_user: z.ZodString;
    description: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
    is_active: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
    label_seeds_as_conversation_starter: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
    poll_id: z.ZodString;
    required_votes: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
    server_url: z.ZodString;
    show_remaining_statements: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
    strict_moderation: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
    topic: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
    type: z.ZodLiteral<"polis">;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    admin_password: z.ZodString;
    admin_user: z.ZodString;
    description: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
    is_active: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
    label_seeds_as_conversation_starter: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
    poll_id: z.ZodString;
    required_votes: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
    server_url: z.ZodString;
    show_remaining_statements: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
    strict_moderation: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
    topic: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
    type: z.ZodLiteral<"polis">;
}, z.ZodTypeAny, "passthrough">>, z.ZodObject<{
    pages: z.ZodArray<z.ZodType<z.objectOutputType<{
        text_content_id: z.ZodString;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        content: z.ZodString;
        type: z.ZodLiteral<"markdown">;
    }, z.ZodTypeAny, "passthrough">[], z.ZodTypeDef, any>, "many">;
    type: z.ZodLiteral<"learn">;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    pages: z.ZodArray<z.ZodType<z.objectOutputType<{
        text_content_id: z.ZodString;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        content: z.ZodString;
        type: z.ZodLiteral<"markdown">;
    }, z.ZodTypeAny, "passthrough">[], z.ZodTypeDef, any>, "many">;
    type: z.ZodLiteral<"learn">;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    pages: z.ZodArray<z.ZodType<z.objectOutputType<{
        text_content_id: z.ZodString;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        content: z.ZodString;
        type: z.ZodLiteral<"markdown">;
    }, z.ZodTypeAny, "passthrough">[], z.ZodTypeDef, any>, "many">;
    type: z.ZodLiteral<"learn">;
}, z.ZodTypeAny, "passthrough">>, z.ZodObject<{
    admin_password: z.ZodString;
    admin_user: z.ZodString;
    project_id: z.ZodString;
    server_url: z.ZodDefault<z.ZodOptional<z.ZodString>>;
    survey_id: z.ZodString;
    survey_url: z.ZodString;
    type: z.ZodLiteral<"heyform">;
    workspace_id: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    admin_password: z.ZodString;
    admin_user: z.ZodString;
    project_id: z.ZodString;
    server_url: z.ZodDefault<z.ZodOptional<z.ZodString>>;
    survey_id: z.ZodString;
    survey_url: z.ZodString;
    type: z.ZodLiteral<"heyform">;
    workspace_id: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    admin_password: z.ZodString;
    admin_user: z.ZodString;
    project_id: z.ZodString;
    server_url: z.ZodDefault<z.ZodOptional<z.ZodString>>;
    survey_id: z.ZodString;
    survey_url: z.ZodString;
    type: z.ZodLiteral<"heyform">;
    workspace_id: z.ZodString;
}, z.ZodTypeAny, "passthrough">>, z.ZodObject<{
    max_time: z.ZodNumber;
    to_see: z.ZodNumber;
    type: z.ZodLiteral<"stories">;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    max_time: z.ZodNumber;
    to_see: z.ZodNumber;
    type: z.ZodLiteral<"stories">;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    max_time: z.ZodNumber;
    to_see: z.ZodNumber;
    type: z.ZodLiteral<"stories">;
}, z.ZodTypeAny, "passthrough">>, z.ZodObject<{
    topic: z.ZodString;
    type: z.ZodLiteral<"elicitationbot">;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    topic: z.ZodString;
    type: z.ZodLiteral<"elicitationbot">;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    topic: z.ZodString;
    type: z.ZodLiteral<"elicitationbot">;
}, z.ZodTypeAny, "passthrough">>, z.ZodObject<{
    alignment_question_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    questions: z.ZodArray<z.ZodType<z.objectOutputType<{
        id: z.ZodString;
        text: z.ZodString;
        type: z.ZodType<"text" | {
            likert_scale: {
                categories: z.objectOutputType<{
                    label: z.ZodString;
                    value: z.ZodNumber;
                }, z.ZodTypeAny, "passthrough">[];
            } & {
                [k: string]: unknown;
            };
        } | {
            continuous: {
                max_label: string;
                max_value: number;
                min_label: string;
                min_value: number;
                sub_steps: number;
            } & {
                [k: string]: unknown;
            };
        }, z.ZodTypeDef, any>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    randomize_order: z.ZodBoolean;
    required_reviews: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>>;
    section_questions: z.ZodDefault<z.ZodOptional<z.ZodArray<z.ZodType<z.objectOutputType<{
        id: z.ZodString;
        text: z.ZodString;
        type: z.ZodType<"text" | {
            likert_scale: {
                categories: z.objectOutputType<{
                    label: z.ZodString;
                    value: z.ZodNumber;
                }, z.ZodTypeAny, "passthrough">[];
            } & {
                [k: string]: unknown;
            };
        } | {
            continuous: {
                max_label: string;
                max_value: number;
                min_label: string;
                min_value: number;
                sub_steps: number;
            } & {
                [k: string]: unknown;
            };
        }, z.ZodTypeDef, any>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">>>;
    type: z.ZodLiteral<"prioritization">;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    alignment_question_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    questions: z.ZodArray<z.ZodType<z.objectOutputType<{
        id: z.ZodString;
        text: z.ZodString;
        type: z.ZodType<"text" | {
            likert_scale: {
                categories: z.objectOutputType<{
                    label: z.ZodString;
                    value: z.ZodNumber;
                }, z.ZodTypeAny, "passthrough">[];
            } & {
                [k: string]: unknown;
            };
        } | {
            continuous: {
                max_label: string;
                max_value: number;
                min_label: string;
                min_value: number;
                sub_steps: number;
            } & {
                [k: string]: unknown;
            };
        }, z.ZodTypeDef, any>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    randomize_order: z.ZodBoolean;
    required_reviews: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>>;
    section_questions: z.ZodDefault<z.ZodOptional<z.ZodArray<z.ZodType<z.objectOutputType<{
        id: z.ZodString;
        text: z.ZodString;
        type: z.ZodType<"text" | {
            likert_scale: {
                categories: z.objectOutputType<{
                    label: z.ZodString;
                    value: z.ZodNumber;
                }, z.ZodTypeAny, "passthrough">[];
            } & {
                [k: string]: unknown;
            };
        } | {
            continuous: {
                max_label: string;
                max_value: number;
                min_label: string;
                min_value: number;
                sub_steps: number;
            } & {
                [k: string]: unknown;
            };
        }, z.ZodTypeDef, any>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">>>;
    type: z.ZodLiteral<"prioritization">;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    alignment_question_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    questions: z.ZodArray<z.ZodType<z.objectOutputType<{
        id: z.ZodString;
        text: z.ZodString;
        type: z.ZodType<"text" | {
            likert_scale: {
                categories: z.objectOutputType<{
                    label: z.ZodString;
                    value: z.ZodNumber;
                }, z.ZodTypeAny, "passthrough">[];
            } & {
                [k: string]: unknown;
            };
        } | {
            continuous: {
                max_label: string;
                max_value: number;
                min_label: string;
                min_value: number;
                sub_steps: number;
            } & {
                [k: string]: unknown;
            };
        }, z.ZodTypeDef, any>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    randomize_order: z.ZodBoolean;
    required_reviews: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>>;
    section_questions: z.ZodDefault<z.ZodOptional<z.ZodArray<z.ZodType<z.objectOutputType<{
        id: z.ZodString;
        text: z.ZodString;
        type: z.ZodType<"text" | {
            likert_scale: {
                categories: z.objectOutputType<{
                    label: z.ZodString;
                    value: z.ZodNumber;
                }, z.ZodTypeAny, "passthrough">[];
            } & {
                [k: string]: unknown;
            };
        } | {
            continuous: {
                max_label: string;
                max_value: number;
                min_label: string;
                min_value: number;
                sub_steps: number;
            } & {
                [k: string]: unknown;
            };
        }, z.ZodTypeDef, any>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">>>;
    type: z.ZodLiteral<"prioritization">;
}, z.ZodTypeAny, "passthrough">>, z.ZodObject<{
    follow_up_rounds_count: z.ZodNumber;
    root_questions: z.ZodArray<z.ZodType<z.objectOutputType<{
        id: z.ZodString;
        intent: z.ZodString;
        text: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    topic: z.ZodString;
    type: z.ZodLiteral<"thinkingspace">;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    follow_up_rounds_count: z.ZodNumber;
    root_questions: z.ZodArray<z.ZodType<z.objectOutputType<{
        id: z.ZodString;
        intent: z.ZodString;
        text: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    topic: z.ZodString;
    type: z.ZodLiteral<"thinkingspace">;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    follow_up_rounds_count: z.ZodNumber;
    root_questions: z.ZodArray<z.ZodType<z.objectOutputType<{
        id: z.ZodString;
        intent: z.ZodString;
        text: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    topic: z.ZodString;
    type: z.ZodLiteral<"thinkingspace">;
}, z.ZodTypeAny, "passthrough">>]>;
export type ToolConfig = z.infer<typeof _ToolConfig>;
export declare const ToolConfig: z.ZodType<ToolConfig, z.ZodTypeDef, any>;
declare const _WorkflowStep: z.ZodObject<{
    activation_rule: z.ZodType<"manual", z.ZodTypeDef, any>;
    can_revisit: z.ZodBoolean;
    created_at: z.ZodString;
    description: z.ZodString;
    id: z.ZodString;
    is_offline: z.ZodBoolean;
    name: z.ZodString;
    preview_tool_config: z.ZodType<z.objectOutputType<{
        admin_password: z.ZodString;
        admin_user: z.ZodString;
        description: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
        is_active: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
        label_seeds_as_conversation_starter: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
        poll_id: z.ZodString;
        required_votes: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        server_url: z.ZodString;
        show_remaining_statements: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
        strict_moderation: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
        topic: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
        type: z.ZodLiteral<"polis">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        pages: z.ZodArray<z.ZodType<z.objectOutputType<{
            text_content_id: z.ZodString;
        }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
            content: z.ZodString;
            type: z.ZodLiteral<"markdown">;
        }, z.ZodTypeAny, "passthrough">[], z.ZodTypeDef, any>, "many">;
        type: z.ZodLiteral<"learn">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        admin_password: z.ZodString;
        admin_user: z.ZodString;
        project_id: z.ZodString;
        server_url: z.ZodDefault<z.ZodOptional<z.ZodString>>;
        survey_id: z.ZodString;
        survey_url: z.ZodString;
        type: z.ZodLiteral<"heyform">;
        workspace_id: z.ZodString;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        max_time: z.ZodNumber;
        to_see: z.ZodNumber;
        type: z.ZodLiteral<"stories">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        topic: z.ZodString;
        type: z.ZodLiteral<"elicitationbot">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        alignment_question_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            text: z.ZodString;
            type: z.ZodType<"text" | {
                likert_scale: {
                    categories: z.objectOutputType<{
                        label: z.ZodString;
                        value: z.ZodNumber;
                    }, z.ZodTypeAny, "passthrough">[];
                } & {
                    [k: string]: unknown;
                };
            } | {
                continuous: {
                    max_label: string;
                    max_value: number;
                    min_label: string;
                    min_value: number;
                    sub_steps: number;
                } & {
                    [k: string]: unknown;
                };
            }, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        randomize_order: z.ZodBoolean;
        required_reviews: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>>;
        section_questions: z.ZodDefault<z.ZodOptional<z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            text: z.ZodString;
            type: z.ZodType<"text" | {
                likert_scale: {
                    categories: z.objectOutputType<{
                        label: z.ZodString;
                        value: z.ZodNumber;
                    }, z.ZodTypeAny, "passthrough">[];
                } & {
                    [k: string]: unknown;
                };
            } | {
                continuous: {
                    max_label: string;
                    max_value: number;
                    min_label: string;
                    min_value: number;
                    sub_steps: number;
                } & {
                    [k: string]: unknown;
                };
            }, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">>>;
        type: z.ZodLiteral<"prioritization">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        follow_up_rounds_count: z.ZodNumber;
        root_questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            intent: z.ZodString;
            text: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        topic: z.ZodString;
        type: z.ZodLiteral<"thinkingspace">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    request_user_share_permission: z.ZodBoolean;
    required: z.ZodBoolean;
    step_order: z.ZodNumber;
    tool_config: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
        admin_password: z.ZodString;
        admin_user: z.ZodString;
        description: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
        is_active: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
        label_seeds_as_conversation_starter: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
        poll_id: z.ZodString;
        required_votes: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        server_url: z.ZodString;
        show_remaining_statements: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
        strict_moderation: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
        topic: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
        type: z.ZodLiteral<"polis">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        pages: z.ZodArray<z.ZodType<z.objectOutputType<{
            text_content_id: z.ZodString;
        }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
            content: z.ZodString;
            type: z.ZodLiteral<"markdown">;
        }, z.ZodTypeAny, "passthrough">[], z.ZodTypeDef, any>, "many">;
        type: z.ZodLiteral<"learn">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        admin_password: z.ZodString;
        admin_user: z.ZodString;
        project_id: z.ZodString;
        server_url: z.ZodDefault<z.ZodOptional<z.ZodString>>;
        survey_id: z.ZodString;
        survey_url: z.ZodString;
        type: z.ZodLiteral<"heyform">;
        workspace_id: z.ZodString;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        max_time: z.ZodNumber;
        to_see: z.ZodNumber;
        type: z.ZodLiteral<"stories">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        topic: z.ZodString;
        type: z.ZodLiteral<"elicitationbot">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        alignment_question_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            text: z.ZodString;
            type: z.ZodType<"text" | {
                likert_scale: {
                    categories: z.objectOutputType<{
                        label: z.ZodString;
                        value: z.ZodNumber;
                    }, z.ZodTypeAny, "passthrough">[];
                } & {
                    [k: string]: unknown;
                };
            } | {
                continuous: {
                    max_label: string;
                    max_value: number;
                    min_label: string;
                    min_value: number;
                    sub_steps: number;
                } & {
                    [k: string]: unknown;
                };
            }, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        randomize_order: z.ZodBoolean;
        required_reviews: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>>;
        section_questions: z.ZodDefault<z.ZodOptional<z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            text: z.ZodString;
            type: z.ZodType<"text" | {
                likert_scale: {
                    categories: z.objectOutputType<{
                        label: z.ZodString;
                        value: z.ZodNumber;
                    }, z.ZodTypeAny, "passthrough">[];
                } & {
                    [k: string]: unknown;
                };
            } | {
                continuous: {
                    max_label: string;
                    max_value: number;
                    min_label: string;
                    min_value: number;
                    sub_steps: number;
                } & {
                    [k: string]: unknown;
                };
            }, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">>>;
        type: z.ZodLiteral<"prioritization">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        follow_up_rounds_count: z.ZodNumber;
        root_questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            intent: z.ZodString;
            text: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        topic: z.ZodString;
        type: z.ZodLiteral<"thinkingspace">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
    updated_at: z.ZodString;
    workflow_id: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    activation_rule: z.ZodType<"manual", z.ZodTypeDef, any>;
    can_revisit: z.ZodBoolean;
    created_at: z.ZodString;
    description: z.ZodString;
    id: z.ZodString;
    is_offline: z.ZodBoolean;
    name: z.ZodString;
    preview_tool_config: z.ZodType<z.objectOutputType<{
        admin_password: z.ZodString;
        admin_user: z.ZodString;
        description: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
        is_active: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
        label_seeds_as_conversation_starter: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
        poll_id: z.ZodString;
        required_votes: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        server_url: z.ZodString;
        show_remaining_statements: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
        strict_moderation: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
        topic: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
        type: z.ZodLiteral<"polis">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        pages: z.ZodArray<z.ZodType<z.objectOutputType<{
            text_content_id: z.ZodString;
        }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
            content: z.ZodString;
            type: z.ZodLiteral<"markdown">;
        }, z.ZodTypeAny, "passthrough">[], z.ZodTypeDef, any>, "many">;
        type: z.ZodLiteral<"learn">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        admin_password: z.ZodString;
        admin_user: z.ZodString;
        project_id: z.ZodString;
        server_url: z.ZodDefault<z.ZodOptional<z.ZodString>>;
        survey_id: z.ZodString;
        survey_url: z.ZodString;
        type: z.ZodLiteral<"heyform">;
        workspace_id: z.ZodString;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        max_time: z.ZodNumber;
        to_see: z.ZodNumber;
        type: z.ZodLiteral<"stories">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        topic: z.ZodString;
        type: z.ZodLiteral<"elicitationbot">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        alignment_question_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            text: z.ZodString;
            type: z.ZodType<"text" | {
                likert_scale: {
                    categories: z.objectOutputType<{
                        label: z.ZodString;
                        value: z.ZodNumber;
                    }, z.ZodTypeAny, "passthrough">[];
                } & {
                    [k: string]: unknown;
                };
            } | {
                continuous: {
                    max_label: string;
                    max_value: number;
                    min_label: string;
                    min_value: number;
                    sub_steps: number;
                } & {
                    [k: string]: unknown;
                };
            }, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        randomize_order: z.ZodBoolean;
        required_reviews: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>>;
        section_questions: z.ZodDefault<z.ZodOptional<z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            text: z.ZodString;
            type: z.ZodType<"text" | {
                likert_scale: {
                    categories: z.objectOutputType<{
                        label: z.ZodString;
                        value: z.ZodNumber;
                    }, z.ZodTypeAny, "passthrough">[];
                } & {
                    [k: string]: unknown;
                };
            } | {
                continuous: {
                    max_label: string;
                    max_value: number;
                    min_label: string;
                    min_value: number;
                    sub_steps: number;
                } & {
                    [k: string]: unknown;
                };
            }, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">>>;
        type: z.ZodLiteral<"prioritization">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        follow_up_rounds_count: z.ZodNumber;
        root_questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            intent: z.ZodString;
            text: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        topic: z.ZodString;
        type: z.ZodLiteral<"thinkingspace">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    request_user_share_permission: z.ZodBoolean;
    required: z.ZodBoolean;
    step_order: z.ZodNumber;
    tool_config: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
        admin_password: z.ZodString;
        admin_user: z.ZodString;
        description: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
        is_active: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
        label_seeds_as_conversation_starter: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
        poll_id: z.ZodString;
        required_votes: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        server_url: z.ZodString;
        show_remaining_statements: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
        strict_moderation: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
        topic: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
        type: z.ZodLiteral<"polis">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        pages: z.ZodArray<z.ZodType<z.objectOutputType<{
            text_content_id: z.ZodString;
        }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
            content: z.ZodString;
            type: z.ZodLiteral<"markdown">;
        }, z.ZodTypeAny, "passthrough">[], z.ZodTypeDef, any>, "many">;
        type: z.ZodLiteral<"learn">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        admin_password: z.ZodString;
        admin_user: z.ZodString;
        project_id: z.ZodString;
        server_url: z.ZodDefault<z.ZodOptional<z.ZodString>>;
        survey_id: z.ZodString;
        survey_url: z.ZodString;
        type: z.ZodLiteral<"heyform">;
        workspace_id: z.ZodString;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        max_time: z.ZodNumber;
        to_see: z.ZodNumber;
        type: z.ZodLiteral<"stories">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        topic: z.ZodString;
        type: z.ZodLiteral<"elicitationbot">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        alignment_question_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            text: z.ZodString;
            type: z.ZodType<"text" | {
                likert_scale: {
                    categories: z.objectOutputType<{
                        label: z.ZodString;
                        value: z.ZodNumber;
                    }, z.ZodTypeAny, "passthrough">[];
                } & {
                    [k: string]: unknown;
                };
            } | {
                continuous: {
                    max_label: string;
                    max_value: number;
                    min_label: string;
                    min_value: number;
                    sub_steps: number;
                } & {
                    [k: string]: unknown;
                };
            }, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        randomize_order: z.ZodBoolean;
        required_reviews: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>>;
        section_questions: z.ZodDefault<z.ZodOptional<z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            text: z.ZodString;
            type: z.ZodType<"text" | {
                likert_scale: {
                    categories: z.objectOutputType<{
                        label: z.ZodString;
                        value: z.ZodNumber;
                    }, z.ZodTypeAny, "passthrough">[];
                } & {
                    [k: string]: unknown;
                };
            } | {
                continuous: {
                    max_label: string;
                    max_value: number;
                    min_label: string;
                    min_value: number;
                    sub_steps: number;
                } & {
                    [k: string]: unknown;
                };
            }, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">>>;
        type: z.ZodLiteral<"prioritization">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        follow_up_rounds_count: z.ZodNumber;
        root_questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            intent: z.ZodString;
            text: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        topic: z.ZodString;
        type: z.ZodLiteral<"thinkingspace">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
    updated_at: z.ZodString;
    workflow_id: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    activation_rule: z.ZodType<"manual", z.ZodTypeDef, any>;
    can_revisit: z.ZodBoolean;
    created_at: z.ZodString;
    description: z.ZodString;
    id: z.ZodString;
    is_offline: z.ZodBoolean;
    name: z.ZodString;
    preview_tool_config: z.ZodType<z.objectOutputType<{
        admin_password: z.ZodString;
        admin_user: z.ZodString;
        description: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
        is_active: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
        label_seeds_as_conversation_starter: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
        poll_id: z.ZodString;
        required_votes: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        server_url: z.ZodString;
        show_remaining_statements: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
        strict_moderation: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
        topic: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
        type: z.ZodLiteral<"polis">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        pages: z.ZodArray<z.ZodType<z.objectOutputType<{
            text_content_id: z.ZodString;
        }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
            content: z.ZodString;
            type: z.ZodLiteral<"markdown">;
        }, z.ZodTypeAny, "passthrough">[], z.ZodTypeDef, any>, "many">;
        type: z.ZodLiteral<"learn">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        admin_password: z.ZodString;
        admin_user: z.ZodString;
        project_id: z.ZodString;
        server_url: z.ZodDefault<z.ZodOptional<z.ZodString>>;
        survey_id: z.ZodString;
        survey_url: z.ZodString;
        type: z.ZodLiteral<"heyform">;
        workspace_id: z.ZodString;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        max_time: z.ZodNumber;
        to_see: z.ZodNumber;
        type: z.ZodLiteral<"stories">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        topic: z.ZodString;
        type: z.ZodLiteral<"elicitationbot">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        alignment_question_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            text: z.ZodString;
            type: z.ZodType<"text" | {
                likert_scale: {
                    categories: z.objectOutputType<{
                        label: z.ZodString;
                        value: z.ZodNumber;
                    }, z.ZodTypeAny, "passthrough">[];
                } & {
                    [k: string]: unknown;
                };
            } | {
                continuous: {
                    max_label: string;
                    max_value: number;
                    min_label: string;
                    min_value: number;
                    sub_steps: number;
                } & {
                    [k: string]: unknown;
                };
            }, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        randomize_order: z.ZodBoolean;
        required_reviews: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>>;
        section_questions: z.ZodDefault<z.ZodOptional<z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            text: z.ZodString;
            type: z.ZodType<"text" | {
                likert_scale: {
                    categories: z.objectOutputType<{
                        label: z.ZodString;
                        value: z.ZodNumber;
                    }, z.ZodTypeAny, "passthrough">[];
                } & {
                    [k: string]: unknown;
                };
            } | {
                continuous: {
                    max_label: string;
                    max_value: number;
                    min_label: string;
                    min_value: number;
                    sub_steps: number;
                } & {
                    [k: string]: unknown;
                };
            }, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">>>;
        type: z.ZodLiteral<"prioritization">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        follow_up_rounds_count: z.ZodNumber;
        root_questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            intent: z.ZodString;
            text: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        topic: z.ZodString;
        type: z.ZodLiteral<"thinkingspace">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    request_user_share_permission: z.ZodBoolean;
    required: z.ZodBoolean;
    step_order: z.ZodNumber;
    tool_config: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
        admin_password: z.ZodString;
        admin_user: z.ZodString;
        description: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
        is_active: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
        label_seeds_as_conversation_starter: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
        poll_id: z.ZodString;
        required_votes: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        server_url: z.ZodString;
        show_remaining_statements: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
        strict_moderation: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
        topic: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
        type: z.ZodLiteral<"polis">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        pages: z.ZodArray<z.ZodType<z.objectOutputType<{
            text_content_id: z.ZodString;
        }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
            content: z.ZodString;
            type: z.ZodLiteral<"markdown">;
        }, z.ZodTypeAny, "passthrough">[], z.ZodTypeDef, any>, "many">;
        type: z.ZodLiteral<"learn">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        admin_password: z.ZodString;
        admin_user: z.ZodString;
        project_id: z.ZodString;
        server_url: z.ZodDefault<z.ZodOptional<z.ZodString>>;
        survey_id: z.ZodString;
        survey_url: z.ZodString;
        type: z.ZodLiteral<"heyform">;
        workspace_id: z.ZodString;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        max_time: z.ZodNumber;
        to_see: z.ZodNumber;
        type: z.ZodLiteral<"stories">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        topic: z.ZodString;
        type: z.ZodLiteral<"elicitationbot">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        alignment_question_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            text: z.ZodString;
            type: z.ZodType<"text" | {
                likert_scale: {
                    categories: z.objectOutputType<{
                        label: z.ZodString;
                        value: z.ZodNumber;
                    }, z.ZodTypeAny, "passthrough">[];
                } & {
                    [k: string]: unknown;
                };
            } | {
                continuous: {
                    max_label: string;
                    max_value: number;
                    min_label: string;
                    min_value: number;
                    sub_steps: number;
                } & {
                    [k: string]: unknown;
                };
            }, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        randomize_order: z.ZodBoolean;
        required_reviews: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>>;
        section_questions: z.ZodDefault<z.ZodOptional<z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            text: z.ZodString;
            type: z.ZodType<"text" | {
                likert_scale: {
                    categories: z.objectOutputType<{
                        label: z.ZodString;
                        value: z.ZodNumber;
                    }, z.ZodTypeAny, "passthrough">[];
                } & {
                    [k: string]: unknown;
                };
            } | {
                continuous: {
                    max_label: string;
                    max_value: number;
                    min_label: string;
                    min_value: number;
                    sub_steps: number;
                } & {
                    [k: string]: unknown;
                };
            }, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">>>;
        type: z.ZodLiteral<"prioritization">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        follow_up_rounds_count: z.ZodNumber;
        root_questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            intent: z.ZodString;
            text: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        topic: z.ZodString;
        type: z.ZodLiteral<"thinkingspace">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
    updated_at: z.ZodString;
    workflow_id: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type WorkflowStep = z.infer<typeof _WorkflowStep>;
export declare const WorkflowStep: z.ZodType<WorkflowStep, z.ZodTypeDef, any>;
declare const _DailySignupStats: z.ZodObject<{
    day: z.ZodString;
    users: z.ZodNumber;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    day: z.ZodString;
    users: z.ZodNumber;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    day: z.ZodString;
    users: z.ZodNumber;
}, z.ZodTypeAny, "passthrough">>;
export type DailySignupStats = z.infer<typeof _DailySignupStats>;
export declare const DailySignupStats: z.ZodType<DailySignupStats, z.ZodTypeDef, any>;
declare const _WorkflowStepStats: z.ZodObject<{
    completed: z.ZodNumber;
    id: z.ZodString;
    started: z.ZodNumber;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    completed: z.ZodNumber;
    id: z.ZodString;
    started: z.ZodNumber;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    completed: z.ZodNumber;
    id: z.ZodString;
    started: z.ZodNumber;
}, z.ZodTypeAny, "passthrough">>;
export type WorkflowStepStats = z.infer<typeof _WorkflowStepStats>;
export declare const WorkflowStepStats: z.ZodType<WorkflowStepStats, z.ZodTypeDef, any>;
declare const _WorkflowStats: z.ZodObject<{
    signupStats: z.ZodArray<z.ZodType<z.objectOutputType<{
        day: z.ZodString;
        users: z.ZodNumber;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    stepStats: z.ZodArray<z.ZodType<z.objectOutputType<{
        completed: z.ZodNumber;
        id: z.ZodString;
        started: z.ZodNumber;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    totalUsers: z.ZodNumber;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    signupStats: z.ZodArray<z.ZodType<z.objectOutputType<{
        day: z.ZodString;
        users: z.ZodNumber;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    stepStats: z.ZodArray<z.ZodType<z.objectOutputType<{
        completed: z.ZodNumber;
        id: z.ZodString;
        started: z.ZodNumber;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    totalUsers: z.ZodNumber;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    signupStats: z.ZodArray<z.ZodType<z.objectOutputType<{
        day: z.ZodString;
        users: z.ZodNumber;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    stepStats: z.ZodArray<z.ZodType<z.objectOutputType<{
        completed: z.ZodNumber;
        id: z.ZodString;
        started: z.ZodNumber;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    totalUsers: z.ZodNumber;
}, z.ZodTypeAny, "passthrough">>;
export type WorkflowStats = z.infer<typeof _WorkflowStats>;
export declare const WorkflowStats: z.ZodType<WorkflowStats, z.ZodTypeDef, any>;
declare const _DemographicCount: z.ZodObject<{
    count: z.ZodNumber;
    displayName: z.ZodString;
    value: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    count: z.ZodNumber;
    displayName: z.ZodString;
    value: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    count: z.ZodNumber;
    displayName: z.ZodString;
    value: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type DemographicCount = z.infer<typeof _DemographicCount>;
export declare const DemographicCount: z.ZodType<DemographicCount, z.ZodTypeDef, any>;
declare const _DemographicReport: z.ZodObject<{
    categories: z.ZodRecord<z.ZodString, z.ZodArray<z.ZodType<z.objectOutputType<{
        count: z.ZodNumber;
        displayName: z.ZodString;
        value: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">>;
    totalParticipants: z.ZodNumber;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    categories: z.ZodRecord<z.ZodString, z.ZodArray<z.ZodType<z.objectOutputType<{
        count: z.ZodNumber;
        displayName: z.ZodString;
        value: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">>;
    totalParticipants: z.ZodNumber;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    categories: z.ZodRecord<z.ZodString, z.ZodArray<z.ZodType<z.objectOutputType<{
        count: z.ZodNumber;
        displayName: z.ZodString;
        value: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">>;
    totalParticipants: z.ZodNumber;
}, z.ZodTypeAny, "passthrough">>;
export type DemographicReport = z.infer<typeof _DemographicReport>;
export declare const DemographicReport: z.ZodType<DemographicReport, z.ZodTypeDef, any>;
declare const _UserParticipation: z.ZodObject<{
    created_at: z.ZodString;
    id: z.ZodString;
    updated_at: z.ZodString;
    user_id: z.ZodString;
    workflow_id: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    created_at: z.ZodString;
    id: z.ZodString;
    updated_at: z.ZodString;
    user_id: z.ZodString;
    workflow_id: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    created_at: z.ZodString;
    id: z.ZodString;
    updated_at: z.ZodString;
    user_id: z.ZodString;
    workflow_id: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type UserParticipation = z.infer<typeof _UserParticipation>;
export declare const UserParticipation: z.ZodType<UserParticipation, z.ZodTypeDef, any>;
declare const _UserParticipationDto: z.ZodObject<{
    created_at: z.ZodString;
    id: z.ZodString;
    sealed: z.ZodBoolean;
    updated_at: z.ZodString;
    user_id: z.ZodString;
    workflow_id: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    created_at: z.ZodString;
    id: z.ZodString;
    sealed: z.ZodBoolean;
    updated_at: z.ZodString;
    user_id: z.ZodString;
    workflow_id: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    created_at: z.ZodString;
    id: z.ZodString;
    sealed: z.ZodBoolean;
    updated_at: z.ZodString;
    user_id: z.ZodString;
    workflow_id: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type UserParticipationDto = z.infer<typeof _UserParticipationDto>;
export declare const UserParticipationDto: z.ZodType<UserParticipationDto, z.ZodTypeDef, any>;
declare const _TranslationDto: z.ZodObject<{
    textContent: z.ZodType<z.objectOutputType<{
        format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
        id: z.ZodString;
        primaryLocale: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
        aiGenerated: z.ZodBoolean;
        content: z.ZodString;
        contentId: z.ZodString;
        id: z.ZodString;
        locale: z.ZodString;
        requiresValidation: z.ZodBoolean;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    textContent: z.ZodType<z.objectOutputType<{
        format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
        id: z.ZodString;
        primaryLocale: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
        aiGenerated: z.ZodBoolean;
        content: z.ZodString;
        contentId: z.ZodString;
        id: z.ZodString;
        locale: z.ZodString;
        requiresValidation: z.ZodBoolean;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    textContent: z.ZodType<z.objectOutputType<{
        format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
        id: z.ZodString;
        primaryLocale: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
        aiGenerated: z.ZodBoolean;
        content: z.ZodString;
        contentId: z.ZodString;
        id: z.ZodString;
        locale: z.ZodString;
        requiresValidation: z.ZodBoolean;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
}, z.ZodTypeAny, "passthrough">>;
export type TranslationDto = z.infer<typeof _TranslationDto>;
export declare const TranslationDto: z.ZodType<TranslationDto, z.ZodTypeDef, any>;
declare const _JsonFieldWithTranslations: z.ZodObject<{
    localized: z.ZodString;
    translations: z.ZodType<z.objectOutputType<{
        textContent: z.ZodType<z.objectOutputType<{
            format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
            id: z.ZodString;
            primaryLocale: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
            aiGenerated: z.ZodBoolean;
            content: z.ZodString;
            contentId: z.ZodString;
            id: z.ZodString;
            locale: z.ZodString;
            requiresValidation: z.ZodBoolean;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    localized: z.ZodString;
    translations: z.ZodType<z.objectOutputType<{
        textContent: z.ZodType<z.objectOutputType<{
            format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
            id: z.ZodString;
            primaryLocale: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
            aiGenerated: z.ZodBoolean;
            content: z.ZodString;
            contentId: z.ZodString;
            id: z.ZodString;
            locale: z.ZodString;
            requiresValidation: z.ZodBoolean;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    localized: z.ZodString;
    translations: z.ZodType<z.objectOutputType<{
        textContent: z.ZodType<z.objectOutputType<{
            format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
            id: z.ZodString;
            primaryLocale: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
            aiGenerated: z.ZodBoolean;
            content: z.ZodString;
            contentId: z.ZodString;
            id: z.ZodString;
            locale: z.ZodString;
            requiresValidation: z.ZodBoolean;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
}, z.ZodTypeAny, "passthrough">>;
export type JsonFieldWithTranslations = z.infer<typeof _JsonFieldWithTranslations>;
export declare const JsonFieldWithTranslations: z.ZodType<JsonFieldWithTranslations, z.ZodTypeDef, any>;
declare const _CategoryWithTranslations: z.ZodObject<{
    label: z.ZodType<z.objectOutputType<{
        localized: z.ZodString;
        translations: z.ZodType<z.objectOutputType<{
            textContent: z.ZodType<z.objectOutputType<{
                format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                id: z.ZodString;
                primaryLocale: z.ZodString;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                aiGenerated: z.ZodBoolean;
                content: z.ZodString;
                contentId: z.ZodString;
                id: z.ZodString;
                locale: z.ZodString;
                requiresValidation: z.ZodBoolean;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    value: z.ZodNumber;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    label: z.ZodType<z.objectOutputType<{
        localized: z.ZodString;
        translations: z.ZodType<z.objectOutputType<{
            textContent: z.ZodType<z.objectOutputType<{
                format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                id: z.ZodString;
                primaryLocale: z.ZodString;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                aiGenerated: z.ZodBoolean;
                content: z.ZodString;
                contentId: z.ZodString;
                id: z.ZodString;
                locale: z.ZodString;
                requiresValidation: z.ZodBoolean;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    value: z.ZodNumber;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    label: z.ZodType<z.objectOutputType<{
        localized: z.ZodString;
        translations: z.ZodType<z.objectOutputType<{
            textContent: z.ZodType<z.objectOutputType<{
                format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                id: z.ZodString;
                primaryLocale: z.ZodString;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                aiGenerated: z.ZodBoolean;
                content: z.ZodString;
                contentId: z.ZodString;
                id: z.ZodString;
                locale: z.ZodString;
                requiresValidation: z.ZodBoolean;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    value: z.ZodNumber;
}, z.ZodTypeAny, "passthrough">>;
export type CategoryWithTranslations = z.infer<typeof _CategoryWithTranslations>;
export declare const CategoryWithTranslations: z.ZodType<CategoryWithTranslations, z.ZodTypeDef, any>;
declare const _QuestionTypeWithTranslations: z.ZodUnion<[z.ZodLiteral<"text">, z.ZodObject<{
    likert_scale: z.ZodObject<{
        categories: z.ZodArray<z.ZodType<z.objectOutputType<{
            label: z.ZodType<z.objectOutputType<{
                localized: z.ZodString;
                translations: z.ZodType<z.objectOutputType<{
                    textContent: z.ZodType<z.objectOutputType<{
                        format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                        id: z.ZodString;
                        primaryLocale: z.ZodString;
                    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                    textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                        aiGenerated: z.ZodBoolean;
                        content: z.ZodString;
                        contentId: z.ZodString;
                        id: z.ZodString;
                        locale: z.ZodString;
                        requiresValidation: z.ZodBoolean;
                    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            value: z.ZodNumber;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    }, "passthrough", z.ZodTypeAny, z.objectOutputType<{
        categories: z.ZodArray<z.ZodType<z.objectOutputType<{
            label: z.ZodType<z.objectOutputType<{
                localized: z.ZodString;
                translations: z.ZodType<z.objectOutputType<{
                    textContent: z.ZodType<z.objectOutputType<{
                        format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                        id: z.ZodString;
                        primaryLocale: z.ZodString;
                    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                    textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                        aiGenerated: z.ZodBoolean;
                        content: z.ZodString;
                        contentId: z.ZodString;
                        id: z.ZodString;
                        locale: z.ZodString;
                        requiresValidation: z.ZodBoolean;
                    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            value: z.ZodNumber;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    }, z.ZodTypeAny, "passthrough">, z.objectInputType<{
        categories: z.ZodArray<z.ZodType<z.objectOutputType<{
            label: z.ZodType<z.objectOutputType<{
                localized: z.ZodString;
                translations: z.ZodType<z.objectOutputType<{
                    textContent: z.ZodType<z.objectOutputType<{
                        format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                        id: z.ZodString;
                        primaryLocale: z.ZodString;
                    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                    textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                        aiGenerated: z.ZodBoolean;
                        content: z.ZodString;
                        contentId: z.ZodString;
                        id: z.ZodString;
                        locale: z.ZodString;
                        requiresValidation: z.ZodBoolean;
                    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            value: z.ZodNumber;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    }, z.ZodTypeAny, "passthrough">>;
}, "strip", z.ZodTypeAny, {
    likert_scale: {
        categories: z.objectOutputType<{
            label: z.ZodType<z.objectOutputType<{
                localized: z.ZodString;
                translations: z.ZodType<z.objectOutputType<{
                    textContent: z.ZodType<z.objectOutputType<{
                        format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                        id: z.ZodString;
                        primaryLocale: z.ZodString;
                    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                    textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                        aiGenerated: z.ZodBoolean;
                        content: z.ZodString;
                        contentId: z.ZodString;
                        id: z.ZodString;
                        locale: z.ZodString;
                        requiresValidation: z.ZodBoolean;
                    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            value: z.ZodNumber;
        }, z.ZodTypeAny, "passthrough">[];
    } & {
        [k: string]: unknown;
    };
}, {
    likert_scale: {
        categories: any[];
    } & {
        [k: string]: unknown;
    };
}>, z.ZodObject<{
    continuous: z.ZodObject<{
        max_label: z.ZodType<z.objectOutputType<{
            localized: z.ZodString;
            translations: z.ZodType<z.objectOutputType<{
                textContent: z.ZodType<z.objectOutputType<{
                    format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                    id: z.ZodString;
                    primaryLocale: z.ZodString;
                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                    aiGenerated: z.ZodBoolean;
                    content: z.ZodString;
                    contentId: z.ZodString;
                    id: z.ZodString;
                    locale: z.ZodString;
                    requiresValidation: z.ZodBoolean;
                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        max_value: z.ZodNumber;
        min_label: z.ZodType<z.objectOutputType<{
            localized: z.ZodString;
            translations: z.ZodType<z.objectOutputType<{
                textContent: z.ZodType<z.objectOutputType<{
                    format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                    id: z.ZodString;
                    primaryLocale: z.ZodString;
                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                    aiGenerated: z.ZodBoolean;
                    content: z.ZodString;
                    contentId: z.ZodString;
                    id: z.ZodString;
                    locale: z.ZodString;
                    requiresValidation: z.ZodBoolean;
                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        min_value: z.ZodNumber;
        sub_steps: z.ZodNumber;
    }, "passthrough", z.ZodTypeAny, z.objectOutputType<{
        max_label: z.ZodType<z.objectOutputType<{
            localized: z.ZodString;
            translations: z.ZodType<z.objectOutputType<{
                textContent: z.ZodType<z.objectOutputType<{
                    format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                    id: z.ZodString;
                    primaryLocale: z.ZodString;
                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                    aiGenerated: z.ZodBoolean;
                    content: z.ZodString;
                    contentId: z.ZodString;
                    id: z.ZodString;
                    locale: z.ZodString;
                    requiresValidation: z.ZodBoolean;
                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        max_value: z.ZodNumber;
        min_label: z.ZodType<z.objectOutputType<{
            localized: z.ZodString;
            translations: z.ZodType<z.objectOutputType<{
                textContent: z.ZodType<z.objectOutputType<{
                    format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                    id: z.ZodString;
                    primaryLocale: z.ZodString;
                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                    aiGenerated: z.ZodBoolean;
                    content: z.ZodString;
                    contentId: z.ZodString;
                    id: z.ZodString;
                    locale: z.ZodString;
                    requiresValidation: z.ZodBoolean;
                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        min_value: z.ZodNumber;
        sub_steps: z.ZodNumber;
    }, z.ZodTypeAny, "passthrough">, z.objectInputType<{
        max_label: z.ZodType<z.objectOutputType<{
            localized: z.ZodString;
            translations: z.ZodType<z.objectOutputType<{
                textContent: z.ZodType<z.objectOutputType<{
                    format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                    id: z.ZodString;
                    primaryLocale: z.ZodString;
                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                    aiGenerated: z.ZodBoolean;
                    content: z.ZodString;
                    contentId: z.ZodString;
                    id: z.ZodString;
                    locale: z.ZodString;
                    requiresValidation: z.ZodBoolean;
                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        max_value: z.ZodNumber;
        min_label: z.ZodType<z.objectOutputType<{
            localized: z.ZodString;
            translations: z.ZodType<z.objectOutputType<{
                textContent: z.ZodType<z.objectOutputType<{
                    format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                    id: z.ZodString;
                    primaryLocale: z.ZodString;
                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                    aiGenerated: z.ZodBoolean;
                    content: z.ZodString;
                    contentId: z.ZodString;
                    id: z.ZodString;
                    locale: z.ZodString;
                    requiresValidation: z.ZodBoolean;
                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        min_value: z.ZodNumber;
        sub_steps: z.ZodNumber;
    }, z.ZodTypeAny, "passthrough">>;
}, "strip", z.ZodTypeAny, {
    continuous: {
        max_label: {
            translations: {
                textContent: {
                    id: string;
                    primaryLocale: string;
                    format: "plain" | "markdown" | "rich";
                } & {
                    [k: string]: unknown;
                };
                textTranslations: z.objectOutputType<{
                    aiGenerated: z.ZodBoolean;
                    content: z.ZodString;
                    contentId: z.ZodString;
                    id: z.ZodString;
                    locale: z.ZodString;
                    requiresValidation: z.ZodBoolean;
                }, z.ZodTypeAny, "passthrough">[];
            } & {
                [k: string]: unknown;
            };
            localized: string;
        } & {
            [k: string]: unknown;
        };
        max_value: number;
        min_label: {
            translations: {
                textContent: {
                    id: string;
                    primaryLocale: string;
                    format: "plain" | "markdown" | "rich";
                } & {
                    [k: string]: unknown;
                };
                textTranslations: z.objectOutputType<{
                    aiGenerated: z.ZodBoolean;
                    content: z.ZodString;
                    contentId: z.ZodString;
                    id: z.ZodString;
                    locale: z.ZodString;
                    requiresValidation: z.ZodBoolean;
                }, z.ZodTypeAny, "passthrough">[];
            } & {
                [k: string]: unknown;
            };
            localized: string;
        } & {
            [k: string]: unknown;
        };
        min_value: number;
        sub_steps: number;
    } & {
        [k: string]: unknown;
    };
}, {
    continuous: {
        max_value: number;
        min_value: number;
        sub_steps: number;
        max_label?: any;
        min_label?: any;
    } & {
        [k: string]: unknown;
    };
}>]>;
export type QuestionTypeWithTranslations = z.infer<typeof _QuestionTypeWithTranslations>;
export declare const QuestionTypeWithTranslations: z.ZodType<QuestionTypeWithTranslations, z.ZodTypeDef, any>;
declare const _QuestionWithTranslations: z.ZodObject<{
    id: z.ZodString;
    text: z.ZodType<z.objectOutputType<{
        localized: z.ZodString;
        translations: z.ZodType<z.objectOutputType<{
            textContent: z.ZodType<z.objectOutputType<{
                format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                id: z.ZodString;
                primaryLocale: z.ZodString;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                aiGenerated: z.ZodBoolean;
                content: z.ZodString;
                contentId: z.ZodString;
                id: z.ZodString;
                locale: z.ZodString;
                requiresValidation: z.ZodBoolean;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    type: z.ZodType<"text" | {
        likert_scale: {
            categories: z.objectOutputType<{
                label: z.ZodType<z.objectOutputType<{
                    localized: z.ZodString;
                    translations: z.ZodType<z.objectOutputType<{
                        textContent: z.ZodType<z.objectOutputType<{
                            format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                            id: z.ZodString;
                            primaryLocale: z.ZodString;
                        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                        textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                            aiGenerated: z.ZodBoolean;
                            content: z.ZodString;
                            contentId: z.ZodString;
                            id: z.ZodString;
                            locale: z.ZodString;
                            requiresValidation: z.ZodBoolean;
                        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
                    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                value: z.ZodNumber;
            }, z.ZodTypeAny, "passthrough">[];
        } & {
            [k: string]: unknown;
        };
    } | {
        continuous: {
            max_label: {
                translations: {
                    textContent: {
                        id: string;
                        primaryLocale: string;
                        format: "plain" | "markdown" | "rich";
                    } & {
                        [k: string]: unknown;
                    };
                    textTranslations: z.objectOutputType<{
                        aiGenerated: z.ZodBoolean;
                        content: z.ZodString;
                        contentId: z.ZodString;
                        id: z.ZodString;
                        locale: z.ZodString;
                        requiresValidation: z.ZodBoolean;
                    }, z.ZodTypeAny, "passthrough">[];
                } & {
                    [k: string]: unknown;
                };
                localized: string;
            } & {
                [k: string]: unknown;
            };
            max_value: number;
            min_label: {
                translations: {
                    textContent: {
                        id: string;
                        primaryLocale: string;
                        format: "plain" | "markdown" | "rich";
                    } & {
                        [k: string]: unknown;
                    };
                    textTranslations: z.objectOutputType<{
                        aiGenerated: z.ZodBoolean;
                        content: z.ZodString;
                        contentId: z.ZodString;
                        id: z.ZodString;
                        locale: z.ZodString;
                        requiresValidation: z.ZodBoolean;
                    }, z.ZodTypeAny, "passthrough">[];
                } & {
                    [k: string]: unknown;
                };
                localized: string;
            } & {
                [k: string]: unknown;
            };
            min_value: number;
            sub_steps: number;
        } & {
            [k: string]: unknown;
        };
    }, z.ZodTypeDef, any>;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    id: z.ZodString;
    text: z.ZodType<z.objectOutputType<{
        localized: z.ZodString;
        translations: z.ZodType<z.objectOutputType<{
            textContent: z.ZodType<z.objectOutputType<{
                format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                id: z.ZodString;
                primaryLocale: z.ZodString;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                aiGenerated: z.ZodBoolean;
                content: z.ZodString;
                contentId: z.ZodString;
                id: z.ZodString;
                locale: z.ZodString;
                requiresValidation: z.ZodBoolean;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    type: z.ZodType<"text" | {
        likert_scale: {
            categories: z.objectOutputType<{
                label: z.ZodType<z.objectOutputType<{
                    localized: z.ZodString;
                    translations: z.ZodType<z.objectOutputType<{
                        textContent: z.ZodType<z.objectOutputType<{
                            format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                            id: z.ZodString;
                            primaryLocale: z.ZodString;
                        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                        textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                            aiGenerated: z.ZodBoolean;
                            content: z.ZodString;
                            contentId: z.ZodString;
                            id: z.ZodString;
                            locale: z.ZodString;
                            requiresValidation: z.ZodBoolean;
                        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
                    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                value: z.ZodNumber;
            }, z.ZodTypeAny, "passthrough">[];
        } & {
            [k: string]: unknown;
        };
    } | {
        continuous: {
            max_label: {
                translations: {
                    textContent: {
                        id: string;
                        primaryLocale: string;
                        format: "plain" | "markdown" | "rich";
                    } & {
                        [k: string]: unknown;
                    };
                    textTranslations: z.objectOutputType<{
                        aiGenerated: z.ZodBoolean;
                        content: z.ZodString;
                        contentId: z.ZodString;
                        id: z.ZodString;
                        locale: z.ZodString;
                        requiresValidation: z.ZodBoolean;
                    }, z.ZodTypeAny, "passthrough">[];
                } & {
                    [k: string]: unknown;
                };
                localized: string;
            } & {
                [k: string]: unknown;
            };
            max_value: number;
            min_label: {
                translations: {
                    textContent: {
                        id: string;
                        primaryLocale: string;
                        format: "plain" | "markdown" | "rich";
                    } & {
                        [k: string]: unknown;
                    };
                    textTranslations: z.objectOutputType<{
                        aiGenerated: z.ZodBoolean;
                        content: z.ZodString;
                        contentId: z.ZodString;
                        id: z.ZodString;
                        locale: z.ZodString;
                        requiresValidation: z.ZodBoolean;
                    }, z.ZodTypeAny, "passthrough">[];
                } & {
                    [k: string]: unknown;
                };
                localized: string;
            } & {
                [k: string]: unknown;
            };
            min_value: number;
            sub_steps: number;
        } & {
            [k: string]: unknown;
        };
    }, z.ZodTypeDef, any>;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    id: z.ZodString;
    text: z.ZodType<z.objectOutputType<{
        localized: z.ZodString;
        translations: z.ZodType<z.objectOutputType<{
            textContent: z.ZodType<z.objectOutputType<{
                format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                id: z.ZodString;
                primaryLocale: z.ZodString;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                aiGenerated: z.ZodBoolean;
                content: z.ZodString;
                contentId: z.ZodString;
                id: z.ZodString;
                locale: z.ZodString;
                requiresValidation: z.ZodBoolean;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    type: z.ZodType<"text" | {
        likert_scale: {
            categories: z.objectOutputType<{
                label: z.ZodType<z.objectOutputType<{
                    localized: z.ZodString;
                    translations: z.ZodType<z.objectOutputType<{
                        textContent: z.ZodType<z.objectOutputType<{
                            format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                            id: z.ZodString;
                            primaryLocale: z.ZodString;
                        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                        textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                            aiGenerated: z.ZodBoolean;
                            content: z.ZodString;
                            contentId: z.ZodString;
                            id: z.ZodString;
                            locale: z.ZodString;
                            requiresValidation: z.ZodBoolean;
                        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
                    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                value: z.ZodNumber;
            }, z.ZodTypeAny, "passthrough">[];
        } & {
            [k: string]: unknown;
        };
    } | {
        continuous: {
            max_label: {
                translations: {
                    textContent: {
                        id: string;
                        primaryLocale: string;
                        format: "plain" | "markdown" | "rich";
                    } & {
                        [k: string]: unknown;
                    };
                    textTranslations: z.objectOutputType<{
                        aiGenerated: z.ZodBoolean;
                        content: z.ZodString;
                        contentId: z.ZodString;
                        id: z.ZodString;
                        locale: z.ZodString;
                        requiresValidation: z.ZodBoolean;
                    }, z.ZodTypeAny, "passthrough">[];
                } & {
                    [k: string]: unknown;
                };
                localized: string;
            } & {
                [k: string]: unknown;
            };
            max_value: number;
            min_label: {
                translations: {
                    textContent: {
                        id: string;
                        primaryLocale: string;
                        format: "plain" | "markdown" | "rich";
                    } & {
                        [k: string]: unknown;
                    };
                    textTranslations: z.objectOutputType<{
                        aiGenerated: z.ZodBoolean;
                        content: z.ZodString;
                        contentId: z.ZodString;
                        id: z.ZodString;
                        locale: z.ZodString;
                        requiresValidation: z.ZodBoolean;
                    }, z.ZodTypeAny, "passthrough">[];
                } & {
                    [k: string]: unknown;
                };
                localized: string;
            } & {
                [k: string]: unknown;
            };
            min_value: number;
            sub_steps: number;
        } & {
            [k: string]: unknown;
        };
    }, z.ZodTypeDef, any>;
}, z.ZodTypeAny, "passthrough">>;
export type QuestionWithTranslations = z.infer<typeof _QuestionWithTranslations>;
export declare const QuestionWithTranslations: z.ZodType<QuestionWithTranslations, z.ZodTypeDef, any>;
declare const _ThinkingSpaceQuestionWithTranslations: z.ZodObject<{
    id: z.ZodString;
    intent: z.ZodType<z.objectOutputType<{
        localized: z.ZodString;
        translations: z.ZodType<z.objectOutputType<{
            textContent: z.ZodType<z.objectOutputType<{
                format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                id: z.ZodString;
                primaryLocale: z.ZodString;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                aiGenerated: z.ZodBoolean;
                content: z.ZodString;
                contentId: z.ZodString;
                id: z.ZodString;
                locale: z.ZodString;
                requiresValidation: z.ZodBoolean;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    text: z.ZodType<z.objectOutputType<{
        localized: z.ZodString;
        translations: z.ZodType<z.objectOutputType<{
            textContent: z.ZodType<z.objectOutputType<{
                format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                id: z.ZodString;
                primaryLocale: z.ZodString;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                aiGenerated: z.ZodBoolean;
                content: z.ZodString;
                contentId: z.ZodString;
                id: z.ZodString;
                locale: z.ZodString;
                requiresValidation: z.ZodBoolean;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    id: z.ZodString;
    intent: z.ZodType<z.objectOutputType<{
        localized: z.ZodString;
        translations: z.ZodType<z.objectOutputType<{
            textContent: z.ZodType<z.objectOutputType<{
                format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                id: z.ZodString;
                primaryLocale: z.ZodString;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                aiGenerated: z.ZodBoolean;
                content: z.ZodString;
                contentId: z.ZodString;
                id: z.ZodString;
                locale: z.ZodString;
                requiresValidation: z.ZodBoolean;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    text: z.ZodType<z.objectOutputType<{
        localized: z.ZodString;
        translations: z.ZodType<z.objectOutputType<{
            textContent: z.ZodType<z.objectOutputType<{
                format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                id: z.ZodString;
                primaryLocale: z.ZodString;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                aiGenerated: z.ZodBoolean;
                content: z.ZodString;
                contentId: z.ZodString;
                id: z.ZodString;
                locale: z.ZodString;
                requiresValidation: z.ZodBoolean;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    id: z.ZodString;
    intent: z.ZodType<z.objectOutputType<{
        localized: z.ZodString;
        translations: z.ZodType<z.objectOutputType<{
            textContent: z.ZodType<z.objectOutputType<{
                format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                id: z.ZodString;
                primaryLocale: z.ZodString;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                aiGenerated: z.ZodBoolean;
                content: z.ZodString;
                contentId: z.ZodString;
                id: z.ZodString;
                locale: z.ZodString;
                requiresValidation: z.ZodBoolean;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    text: z.ZodType<z.objectOutputType<{
        localized: z.ZodString;
        translations: z.ZodType<z.objectOutputType<{
            textContent: z.ZodType<z.objectOutputType<{
                format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                id: z.ZodString;
                primaryLocale: z.ZodString;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                aiGenerated: z.ZodBoolean;
                content: z.ZodString;
                contentId: z.ZodString;
                id: z.ZodString;
                locale: z.ZodString;
                requiresValidation: z.ZodBoolean;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
}, z.ZodTypeAny, "passthrough">>;
export type ThinkingSpaceQuestionWithTranslations = z.infer<typeof _ThinkingSpaceQuestionWithTranslations>;
export declare const ThinkingSpaceQuestionWithTranslations: z.ZodType<ThinkingSpaceQuestionWithTranslations, z.ZodTypeDef, any>;
declare const _ToolConfigWithTranslations: z.ZodUnion<[z.ZodObject<{
    admin_password: z.ZodString;
    admin_user: z.ZodString;
    description: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
    is_active: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
    label_seeds_as_conversation_starter: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
    poll_id: z.ZodString;
    required_votes: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
    server_url: z.ZodString;
    show_remaining_statements: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
    strict_moderation: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
    topic: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
    type: z.ZodLiteral<"polis">;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    admin_password: z.ZodString;
    admin_user: z.ZodString;
    description: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
    is_active: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
    label_seeds_as_conversation_starter: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
    poll_id: z.ZodString;
    required_votes: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
    server_url: z.ZodString;
    show_remaining_statements: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
    strict_moderation: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
    topic: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
    type: z.ZodLiteral<"polis">;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    admin_password: z.ZodString;
    admin_user: z.ZodString;
    description: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
    is_active: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
    label_seeds_as_conversation_starter: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
    poll_id: z.ZodString;
    required_votes: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
    server_url: z.ZodString;
    show_remaining_statements: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
    strict_moderation: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
    topic: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
    type: z.ZodLiteral<"polis">;
}, z.ZodTypeAny, "passthrough">>, z.ZodObject<{
    pages: z.ZodArray<z.ZodType<z.objectOutputType<{
        text_content_id: z.ZodString;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        content: z.ZodString;
        type: z.ZodLiteral<"markdown">;
    }, z.ZodTypeAny, "passthrough">[], z.ZodTypeDef, any>, "many">;
    type: z.ZodLiteral<"learn">;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    pages: z.ZodArray<z.ZodType<z.objectOutputType<{
        text_content_id: z.ZodString;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        content: z.ZodString;
        type: z.ZodLiteral<"markdown">;
    }, z.ZodTypeAny, "passthrough">[], z.ZodTypeDef, any>, "many">;
    type: z.ZodLiteral<"learn">;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    pages: z.ZodArray<z.ZodType<z.objectOutputType<{
        text_content_id: z.ZodString;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        content: z.ZodString;
        type: z.ZodLiteral<"markdown">;
    }, z.ZodTypeAny, "passthrough">[], z.ZodTypeDef, any>, "many">;
    type: z.ZodLiteral<"learn">;
}, z.ZodTypeAny, "passthrough">>, z.ZodObject<{
    admin_password: z.ZodString;
    admin_user: z.ZodString;
    project_id: z.ZodString;
    server_url: z.ZodDefault<z.ZodOptional<z.ZodString>>;
    survey_id: z.ZodString;
    survey_url: z.ZodString;
    type: z.ZodLiteral<"heyform">;
    workspace_id: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    admin_password: z.ZodString;
    admin_user: z.ZodString;
    project_id: z.ZodString;
    server_url: z.ZodDefault<z.ZodOptional<z.ZodString>>;
    survey_id: z.ZodString;
    survey_url: z.ZodString;
    type: z.ZodLiteral<"heyform">;
    workspace_id: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    admin_password: z.ZodString;
    admin_user: z.ZodString;
    project_id: z.ZodString;
    server_url: z.ZodDefault<z.ZodOptional<z.ZodString>>;
    survey_id: z.ZodString;
    survey_url: z.ZodString;
    type: z.ZodLiteral<"heyform">;
    workspace_id: z.ZodString;
}, z.ZodTypeAny, "passthrough">>, z.ZodObject<{
    max_time: z.ZodNumber;
    to_see: z.ZodNumber;
    type: z.ZodLiteral<"stories">;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    max_time: z.ZodNumber;
    to_see: z.ZodNumber;
    type: z.ZodLiteral<"stories">;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    max_time: z.ZodNumber;
    to_see: z.ZodNumber;
    type: z.ZodLiteral<"stories">;
}, z.ZodTypeAny, "passthrough">>, z.ZodObject<{
    topic: z.ZodString;
    type: z.ZodLiteral<"elicitationbot">;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    topic: z.ZodString;
    type: z.ZodLiteral<"elicitationbot">;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    topic: z.ZodString;
    type: z.ZodLiteral<"elicitationbot">;
}, z.ZodTypeAny, "passthrough">>, z.ZodObject<{
    alignment_question_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    questions: z.ZodArray<z.ZodType<z.objectOutputType<{
        id: z.ZodString;
        text: z.ZodType<z.objectOutputType<{
            localized: z.ZodString;
            translations: z.ZodType<z.objectOutputType<{
                textContent: z.ZodType<z.objectOutputType<{
                    format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                    id: z.ZodString;
                    primaryLocale: z.ZodString;
                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                    aiGenerated: z.ZodBoolean;
                    content: z.ZodString;
                    contentId: z.ZodString;
                    id: z.ZodString;
                    locale: z.ZodString;
                    requiresValidation: z.ZodBoolean;
                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        type: z.ZodType<"text" | {
            likert_scale: {
                categories: z.objectOutputType<{
                    label: z.ZodType<z.objectOutputType<{
                        localized: z.ZodString;
                        translations: z.ZodType<z.objectOutputType<{
                            textContent: z.ZodType<z.objectOutputType<{
                                format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                                id: z.ZodString;
                                primaryLocale: z.ZodString;
                            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                            textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                                aiGenerated: z.ZodBoolean;
                                content: z.ZodString;
                                contentId: z.ZodString;
                                id: z.ZodString;
                                locale: z.ZodString;
                                requiresValidation: z.ZodBoolean;
                            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
                        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                    value: z.ZodNumber;
                }, z.ZodTypeAny, "passthrough">[];
            } & {
                [k: string]: unknown;
            };
        } | {
            continuous: {
                max_label: {
                    translations: {
                        textContent: {
                            id: string;
                            primaryLocale: string;
                            format: "plain" | "markdown" | "rich";
                        } & {
                            [k: string]: unknown;
                        };
                        textTranslations: z.objectOutputType<{
                            aiGenerated: z.ZodBoolean;
                            content: z.ZodString;
                            contentId: z.ZodString;
                            id: z.ZodString;
                            locale: z.ZodString;
                            requiresValidation: z.ZodBoolean;
                        }, z.ZodTypeAny, "passthrough">[];
                    } & {
                        [k: string]: unknown;
                    };
                    localized: string;
                } & {
                    [k: string]: unknown;
                };
                max_value: number;
                min_label: {
                    translations: {
                        textContent: {
                            id: string;
                            primaryLocale: string;
                            format: "plain" | "markdown" | "rich";
                        } & {
                            [k: string]: unknown;
                        };
                        textTranslations: z.objectOutputType<{
                            aiGenerated: z.ZodBoolean;
                            content: z.ZodString;
                            contentId: z.ZodString;
                            id: z.ZodString;
                            locale: z.ZodString;
                            requiresValidation: z.ZodBoolean;
                        }, z.ZodTypeAny, "passthrough">[];
                    } & {
                        [k: string]: unknown;
                    };
                    localized: string;
                } & {
                    [k: string]: unknown;
                };
                min_value: number;
                sub_steps: number;
            } & {
                [k: string]: unknown;
            };
        }, z.ZodTypeDef, any>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    randomize_order: z.ZodBoolean;
    required_reviews: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
    section_questions: z.ZodArray<z.ZodType<z.objectOutputType<{
        id: z.ZodString;
        text: z.ZodType<z.objectOutputType<{
            localized: z.ZodString;
            translations: z.ZodType<z.objectOutputType<{
                textContent: z.ZodType<z.objectOutputType<{
                    format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                    id: z.ZodString;
                    primaryLocale: z.ZodString;
                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                    aiGenerated: z.ZodBoolean;
                    content: z.ZodString;
                    contentId: z.ZodString;
                    id: z.ZodString;
                    locale: z.ZodString;
                    requiresValidation: z.ZodBoolean;
                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        type: z.ZodType<"text" | {
            likert_scale: {
                categories: z.objectOutputType<{
                    label: z.ZodType<z.objectOutputType<{
                        localized: z.ZodString;
                        translations: z.ZodType<z.objectOutputType<{
                            textContent: z.ZodType<z.objectOutputType<{
                                format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                                id: z.ZodString;
                                primaryLocale: z.ZodString;
                            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                            textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                                aiGenerated: z.ZodBoolean;
                                content: z.ZodString;
                                contentId: z.ZodString;
                                id: z.ZodString;
                                locale: z.ZodString;
                                requiresValidation: z.ZodBoolean;
                            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
                        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                    value: z.ZodNumber;
                }, z.ZodTypeAny, "passthrough">[];
            } & {
                [k: string]: unknown;
            };
        } | {
            continuous: {
                max_label: {
                    translations: {
                        textContent: {
                            id: string;
                            primaryLocale: string;
                            format: "plain" | "markdown" | "rich";
                        } & {
                            [k: string]: unknown;
                        };
                        textTranslations: z.objectOutputType<{
                            aiGenerated: z.ZodBoolean;
                            content: z.ZodString;
                            contentId: z.ZodString;
                            id: z.ZodString;
                            locale: z.ZodString;
                            requiresValidation: z.ZodBoolean;
                        }, z.ZodTypeAny, "passthrough">[];
                    } & {
                        [k: string]: unknown;
                    };
                    localized: string;
                } & {
                    [k: string]: unknown;
                };
                max_value: number;
                min_label: {
                    translations: {
                        textContent: {
                            id: string;
                            primaryLocale: string;
                            format: "plain" | "markdown" | "rich";
                        } & {
                            [k: string]: unknown;
                        };
                        textTranslations: z.objectOutputType<{
                            aiGenerated: z.ZodBoolean;
                            content: z.ZodString;
                            contentId: z.ZodString;
                            id: z.ZodString;
                            locale: z.ZodString;
                            requiresValidation: z.ZodBoolean;
                        }, z.ZodTypeAny, "passthrough">[];
                    } & {
                        [k: string]: unknown;
                    };
                    localized: string;
                } & {
                    [k: string]: unknown;
                };
                min_value: number;
                sub_steps: number;
            } & {
                [k: string]: unknown;
            };
        }, z.ZodTypeDef, any>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    type: z.ZodLiteral<"prioritization">;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    alignment_question_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    questions: z.ZodArray<z.ZodType<z.objectOutputType<{
        id: z.ZodString;
        text: z.ZodType<z.objectOutputType<{
            localized: z.ZodString;
            translations: z.ZodType<z.objectOutputType<{
                textContent: z.ZodType<z.objectOutputType<{
                    format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                    id: z.ZodString;
                    primaryLocale: z.ZodString;
                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                    aiGenerated: z.ZodBoolean;
                    content: z.ZodString;
                    contentId: z.ZodString;
                    id: z.ZodString;
                    locale: z.ZodString;
                    requiresValidation: z.ZodBoolean;
                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        type: z.ZodType<"text" | {
            likert_scale: {
                categories: z.objectOutputType<{
                    label: z.ZodType<z.objectOutputType<{
                        localized: z.ZodString;
                        translations: z.ZodType<z.objectOutputType<{
                            textContent: z.ZodType<z.objectOutputType<{
                                format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                                id: z.ZodString;
                                primaryLocale: z.ZodString;
                            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                            textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                                aiGenerated: z.ZodBoolean;
                                content: z.ZodString;
                                contentId: z.ZodString;
                                id: z.ZodString;
                                locale: z.ZodString;
                                requiresValidation: z.ZodBoolean;
                            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
                        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                    value: z.ZodNumber;
                }, z.ZodTypeAny, "passthrough">[];
            } & {
                [k: string]: unknown;
            };
        } | {
            continuous: {
                max_label: {
                    translations: {
                        textContent: {
                            id: string;
                            primaryLocale: string;
                            format: "plain" | "markdown" | "rich";
                        } & {
                            [k: string]: unknown;
                        };
                        textTranslations: z.objectOutputType<{
                            aiGenerated: z.ZodBoolean;
                            content: z.ZodString;
                            contentId: z.ZodString;
                            id: z.ZodString;
                            locale: z.ZodString;
                            requiresValidation: z.ZodBoolean;
                        }, z.ZodTypeAny, "passthrough">[];
                    } & {
                        [k: string]: unknown;
                    };
                    localized: string;
                } & {
                    [k: string]: unknown;
                };
                max_value: number;
                min_label: {
                    translations: {
                        textContent: {
                            id: string;
                            primaryLocale: string;
                            format: "plain" | "markdown" | "rich";
                        } & {
                            [k: string]: unknown;
                        };
                        textTranslations: z.objectOutputType<{
                            aiGenerated: z.ZodBoolean;
                            content: z.ZodString;
                            contentId: z.ZodString;
                            id: z.ZodString;
                            locale: z.ZodString;
                            requiresValidation: z.ZodBoolean;
                        }, z.ZodTypeAny, "passthrough">[];
                    } & {
                        [k: string]: unknown;
                    };
                    localized: string;
                } & {
                    [k: string]: unknown;
                };
                min_value: number;
                sub_steps: number;
            } & {
                [k: string]: unknown;
            };
        }, z.ZodTypeDef, any>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    randomize_order: z.ZodBoolean;
    required_reviews: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
    section_questions: z.ZodArray<z.ZodType<z.objectOutputType<{
        id: z.ZodString;
        text: z.ZodType<z.objectOutputType<{
            localized: z.ZodString;
            translations: z.ZodType<z.objectOutputType<{
                textContent: z.ZodType<z.objectOutputType<{
                    format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                    id: z.ZodString;
                    primaryLocale: z.ZodString;
                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                    aiGenerated: z.ZodBoolean;
                    content: z.ZodString;
                    contentId: z.ZodString;
                    id: z.ZodString;
                    locale: z.ZodString;
                    requiresValidation: z.ZodBoolean;
                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        type: z.ZodType<"text" | {
            likert_scale: {
                categories: z.objectOutputType<{
                    label: z.ZodType<z.objectOutputType<{
                        localized: z.ZodString;
                        translations: z.ZodType<z.objectOutputType<{
                            textContent: z.ZodType<z.objectOutputType<{
                                format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                                id: z.ZodString;
                                primaryLocale: z.ZodString;
                            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                            textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                                aiGenerated: z.ZodBoolean;
                                content: z.ZodString;
                                contentId: z.ZodString;
                                id: z.ZodString;
                                locale: z.ZodString;
                                requiresValidation: z.ZodBoolean;
                            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
                        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                    value: z.ZodNumber;
                }, z.ZodTypeAny, "passthrough">[];
            } & {
                [k: string]: unknown;
            };
        } | {
            continuous: {
                max_label: {
                    translations: {
                        textContent: {
                            id: string;
                            primaryLocale: string;
                            format: "plain" | "markdown" | "rich";
                        } & {
                            [k: string]: unknown;
                        };
                        textTranslations: z.objectOutputType<{
                            aiGenerated: z.ZodBoolean;
                            content: z.ZodString;
                            contentId: z.ZodString;
                            id: z.ZodString;
                            locale: z.ZodString;
                            requiresValidation: z.ZodBoolean;
                        }, z.ZodTypeAny, "passthrough">[];
                    } & {
                        [k: string]: unknown;
                    };
                    localized: string;
                } & {
                    [k: string]: unknown;
                };
                max_value: number;
                min_label: {
                    translations: {
                        textContent: {
                            id: string;
                            primaryLocale: string;
                            format: "plain" | "markdown" | "rich";
                        } & {
                            [k: string]: unknown;
                        };
                        textTranslations: z.objectOutputType<{
                            aiGenerated: z.ZodBoolean;
                            content: z.ZodString;
                            contentId: z.ZodString;
                            id: z.ZodString;
                            locale: z.ZodString;
                            requiresValidation: z.ZodBoolean;
                        }, z.ZodTypeAny, "passthrough">[];
                    } & {
                        [k: string]: unknown;
                    };
                    localized: string;
                } & {
                    [k: string]: unknown;
                };
                min_value: number;
                sub_steps: number;
            } & {
                [k: string]: unknown;
            };
        }, z.ZodTypeDef, any>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    type: z.ZodLiteral<"prioritization">;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    alignment_question_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    questions: z.ZodArray<z.ZodType<z.objectOutputType<{
        id: z.ZodString;
        text: z.ZodType<z.objectOutputType<{
            localized: z.ZodString;
            translations: z.ZodType<z.objectOutputType<{
                textContent: z.ZodType<z.objectOutputType<{
                    format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                    id: z.ZodString;
                    primaryLocale: z.ZodString;
                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                    aiGenerated: z.ZodBoolean;
                    content: z.ZodString;
                    contentId: z.ZodString;
                    id: z.ZodString;
                    locale: z.ZodString;
                    requiresValidation: z.ZodBoolean;
                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        type: z.ZodType<"text" | {
            likert_scale: {
                categories: z.objectOutputType<{
                    label: z.ZodType<z.objectOutputType<{
                        localized: z.ZodString;
                        translations: z.ZodType<z.objectOutputType<{
                            textContent: z.ZodType<z.objectOutputType<{
                                format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                                id: z.ZodString;
                                primaryLocale: z.ZodString;
                            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                            textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                                aiGenerated: z.ZodBoolean;
                                content: z.ZodString;
                                contentId: z.ZodString;
                                id: z.ZodString;
                                locale: z.ZodString;
                                requiresValidation: z.ZodBoolean;
                            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
                        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                    value: z.ZodNumber;
                }, z.ZodTypeAny, "passthrough">[];
            } & {
                [k: string]: unknown;
            };
        } | {
            continuous: {
                max_label: {
                    translations: {
                        textContent: {
                            id: string;
                            primaryLocale: string;
                            format: "plain" | "markdown" | "rich";
                        } & {
                            [k: string]: unknown;
                        };
                        textTranslations: z.objectOutputType<{
                            aiGenerated: z.ZodBoolean;
                            content: z.ZodString;
                            contentId: z.ZodString;
                            id: z.ZodString;
                            locale: z.ZodString;
                            requiresValidation: z.ZodBoolean;
                        }, z.ZodTypeAny, "passthrough">[];
                    } & {
                        [k: string]: unknown;
                    };
                    localized: string;
                } & {
                    [k: string]: unknown;
                };
                max_value: number;
                min_label: {
                    translations: {
                        textContent: {
                            id: string;
                            primaryLocale: string;
                            format: "plain" | "markdown" | "rich";
                        } & {
                            [k: string]: unknown;
                        };
                        textTranslations: z.objectOutputType<{
                            aiGenerated: z.ZodBoolean;
                            content: z.ZodString;
                            contentId: z.ZodString;
                            id: z.ZodString;
                            locale: z.ZodString;
                            requiresValidation: z.ZodBoolean;
                        }, z.ZodTypeAny, "passthrough">[];
                    } & {
                        [k: string]: unknown;
                    };
                    localized: string;
                } & {
                    [k: string]: unknown;
                };
                min_value: number;
                sub_steps: number;
            } & {
                [k: string]: unknown;
            };
        }, z.ZodTypeDef, any>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    randomize_order: z.ZodBoolean;
    required_reviews: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
    section_questions: z.ZodArray<z.ZodType<z.objectOutputType<{
        id: z.ZodString;
        text: z.ZodType<z.objectOutputType<{
            localized: z.ZodString;
            translations: z.ZodType<z.objectOutputType<{
                textContent: z.ZodType<z.objectOutputType<{
                    format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                    id: z.ZodString;
                    primaryLocale: z.ZodString;
                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                    aiGenerated: z.ZodBoolean;
                    content: z.ZodString;
                    contentId: z.ZodString;
                    id: z.ZodString;
                    locale: z.ZodString;
                    requiresValidation: z.ZodBoolean;
                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        type: z.ZodType<"text" | {
            likert_scale: {
                categories: z.objectOutputType<{
                    label: z.ZodType<z.objectOutputType<{
                        localized: z.ZodString;
                        translations: z.ZodType<z.objectOutputType<{
                            textContent: z.ZodType<z.objectOutputType<{
                                format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                                id: z.ZodString;
                                primaryLocale: z.ZodString;
                            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                            textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                                aiGenerated: z.ZodBoolean;
                                content: z.ZodString;
                                contentId: z.ZodString;
                                id: z.ZodString;
                                locale: z.ZodString;
                                requiresValidation: z.ZodBoolean;
                            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
                        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                    value: z.ZodNumber;
                }, z.ZodTypeAny, "passthrough">[];
            } & {
                [k: string]: unknown;
            };
        } | {
            continuous: {
                max_label: {
                    translations: {
                        textContent: {
                            id: string;
                            primaryLocale: string;
                            format: "plain" | "markdown" | "rich";
                        } & {
                            [k: string]: unknown;
                        };
                        textTranslations: z.objectOutputType<{
                            aiGenerated: z.ZodBoolean;
                            content: z.ZodString;
                            contentId: z.ZodString;
                            id: z.ZodString;
                            locale: z.ZodString;
                            requiresValidation: z.ZodBoolean;
                        }, z.ZodTypeAny, "passthrough">[];
                    } & {
                        [k: string]: unknown;
                    };
                    localized: string;
                } & {
                    [k: string]: unknown;
                };
                max_value: number;
                min_label: {
                    translations: {
                        textContent: {
                            id: string;
                            primaryLocale: string;
                            format: "plain" | "markdown" | "rich";
                        } & {
                            [k: string]: unknown;
                        };
                        textTranslations: z.objectOutputType<{
                            aiGenerated: z.ZodBoolean;
                            content: z.ZodString;
                            contentId: z.ZodString;
                            id: z.ZodString;
                            locale: z.ZodString;
                            requiresValidation: z.ZodBoolean;
                        }, z.ZodTypeAny, "passthrough">[];
                    } & {
                        [k: string]: unknown;
                    };
                    localized: string;
                } & {
                    [k: string]: unknown;
                };
                min_value: number;
                sub_steps: number;
            } & {
                [k: string]: unknown;
            };
        }, z.ZodTypeDef, any>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    type: z.ZodLiteral<"prioritization">;
}, z.ZodTypeAny, "passthrough">>, z.ZodObject<{
    follow_up_rounds_count: z.ZodNumber;
    root_questions: z.ZodArray<z.ZodType<z.objectOutputType<{
        id: z.ZodString;
        intent: z.ZodType<z.objectOutputType<{
            localized: z.ZodString;
            translations: z.ZodType<z.objectOutputType<{
                textContent: z.ZodType<z.objectOutputType<{
                    format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                    id: z.ZodString;
                    primaryLocale: z.ZodString;
                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                    aiGenerated: z.ZodBoolean;
                    content: z.ZodString;
                    contentId: z.ZodString;
                    id: z.ZodString;
                    locale: z.ZodString;
                    requiresValidation: z.ZodBoolean;
                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        text: z.ZodType<z.objectOutputType<{
            localized: z.ZodString;
            translations: z.ZodType<z.objectOutputType<{
                textContent: z.ZodType<z.objectOutputType<{
                    format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                    id: z.ZodString;
                    primaryLocale: z.ZodString;
                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                    aiGenerated: z.ZodBoolean;
                    content: z.ZodString;
                    contentId: z.ZodString;
                    id: z.ZodString;
                    locale: z.ZodString;
                    requiresValidation: z.ZodBoolean;
                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    topic: z.ZodType<z.objectOutputType<{
        localized: z.ZodString;
        translations: z.ZodType<z.objectOutputType<{
            textContent: z.ZodType<z.objectOutputType<{
                format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                id: z.ZodString;
                primaryLocale: z.ZodString;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                aiGenerated: z.ZodBoolean;
                content: z.ZodString;
                contentId: z.ZodString;
                id: z.ZodString;
                locale: z.ZodString;
                requiresValidation: z.ZodBoolean;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    type: z.ZodLiteral<"thinkingspace">;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    follow_up_rounds_count: z.ZodNumber;
    root_questions: z.ZodArray<z.ZodType<z.objectOutputType<{
        id: z.ZodString;
        intent: z.ZodType<z.objectOutputType<{
            localized: z.ZodString;
            translations: z.ZodType<z.objectOutputType<{
                textContent: z.ZodType<z.objectOutputType<{
                    format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                    id: z.ZodString;
                    primaryLocale: z.ZodString;
                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                    aiGenerated: z.ZodBoolean;
                    content: z.ZodString;
                    contentId: z.ZodString;
                    id: z.ZodString;
                    locale: z.ZodString;
                    requiresValidation: z.ZodBoolean;
                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        text: z.ZodType<z.objectOutputType<{
            localized: z.ZodString;
            translations: z.ZodType<z.objectOutputType<{
                textContent: z.ZodType<z.objectOutputType<{
                    format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                    id: z.ZodString;
                    primaryLocale: z.ZodString;
                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                    aiGenerated: z.ZodBoolean;
                    content: z.ZodString;
                    contentId: z.ZodString;
                    id: z.ZodString;
                    locale: z.ZodString;
                    requiresValidation: z.ZodBoolean;
                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    topic: z.ZodType<z.objectOutputType<{
        localized: z.ZodString;
        translations: z.ZodType<z.objectOutputType<{
            textContent: z.ZodType<z.objectOutputType<{
                format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                id: z.ZodString;
                primaryLocale: z.ZodString;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                aiGenerated: z.ZodBoolean;
                content: z.ZodString;
                contentId: z.ZodString;
                id: z.ZodString;
                locale: z.ZodString;
                requiresValidation: z.ZodBoolean;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    type: z.ZodLiteral<"thinkingspace">;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    follow_up_rounds_count: z.ZodNumber;
    root_questions: z.ZodArray<z.ZodType<z.objectOutputType<{
        id: z.ZodString;
        intent: z.ZodType<z.objectOutputType<{
            localized: z.ZodString;
            translations: z.ZodType<z.objectOutputType<{
                textContent: z.ZodType<z.objectOutputType<{
                    format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                    id: z.ZodString;
                    primaryLocale: z.ZodString;
                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                    aiGenerated: z.ZodBoolean;
                    content: z.ZodString;
                    contentId: z.ZodString;
                    id: z.ZodString;
                    locale: z.ZodString;
                    requiresValidation: z.ZodBoolean;
                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        text: z.ZodType<z.objectOutputType<{
            localized: z.ZodString;
            translations: z.ZodType<z.objectOutputType<{
                textContent: z.ZodType<z.objectOutputType<{
                    format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                    id: z.ZodString;
                    primaryLocale: z.ZodString;
                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                    aiGenerated: z.ZodBoolean;
                    content: z.ZodString;
                    contentId: z.ZodString;
                    id: z.ZodString;
                    locale: z.ZodString;
                    requiresValidation: z.ZodBoolean;
                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    topic: z.ZodType<z.objectOutputType<{
        localized: z.ZodString;
        translations: z.ZodType<z.objectOutputType<{
            textContent: z.ZodType<z.objectOutputType<{
                format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                id: z.ZodString;
                primaryLocale: z.ZodString;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                aiGenerated: z.ZodBoolean;
                content: z.ZodString;
                contentId: z.ZodString;
                id: z.ZodString;
                locale: z.ZodString;
                requiresValidation: z.ZodBoolean;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    type: z.ZodLiteral<"thinkingspace">;
}, z.ZodTypeAny, "passthrough">>]>;
export type ToolConfigWithTranslations = z.infer<typeof _ToolConfigWithTranslations>;
export declare const ToolConfigWithTranslations: z.ZodType<ToolConfigWithTranslations, z.ZodTypeDef, any>;
declare const _Translation4: z.ZodObject<{
    textContent: z.ZodType<z.objectOutputType<{
        format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
        id: z.ZodString;
        primaryLocale: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
        aiGenerated: z.ZodBoolean;
        content: z.ZodString;
        contentId: z.ZodString;
        id: z.ZodString;
        locale: z.ZodString;
        requiresValidation: z.ZodBoolean;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    textContent: z.ZodType<z.objectOutputType<{
        format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
        id: z.ZodString;
        primaryLocale: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
        aiGenerated: z.ZodBoolean;
        content: z.ZodString;
        contentId: z.ZodString;
        id: z.ZodString;
        locale: z.ZodString;
        requiresValidation: z.ZodBoolean;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    textContent: z.ZodType<z.objectOutputType<{
        format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
        id: z.ZodString;
        primaryLocale: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
        aiGenerated: z.ZodBoolean;
        content: z.ZodString;
        contentId: z.ZodString;
        id: z.ZodString;
        locale: z.ZodString;
        requiresValidation: z.ZodBoolean;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
}, z.ZodTypeAny, "passthrough">>;
export type Translation4 = z.infer<typeof _Translation4>;
export declare const Translation4: z.ZodType<Translation4, z.ZodTypeDef, any>;
declare const _WorkflowStepTranslations: z.ZodObject<{
    description: z.ZodType<z.objectOutputType<{
        textContent: z.ZodType<z.objectOutputType<{
            format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
            id: z.ZodString;
            primaryLocale: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
            aiGenerated: z.ZodBoolean;
            content: z.ZodString;
            contentId: z.ZodString;
            id: z.ZodString;
            locale: z.ZodString;
            requiresValidation: z.ZodBoolean;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    name: z.ZodType<z.objectOutputType<{
        textContent: z.ZodType<z.objectOutputType<{
            format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
            id: z.ZodString;
            primaryLocale: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
            aiGenerated: z.ZodBoolean;
            content: z.ZodString;
            contentId: z.ZodString;
            id: z.ZodString;
            locale: z.ZodString;
            requiresValidation: z.ZodBoolean;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    description: z.ZodType<z.objectOutputType<{
        textContent: z.ZodType<z.objectOutputType<{
            format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
            id: z.ZodString;
            primaryLocale: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
            aiGenerated: z.ZodBoolean;
            content: z.ZodString;
            contentId: z.ZodString;
            id: z.ZodString;
            locale: z.ZodString;
            requiresValidation: z.ZodBoolean;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    name: z.ZodType<z.objectOutputType<{
        textContent: z.ZodType<z.objectOutputType<{
            format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
            id: z.ZodString;
            primaryLocale: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
            aiGenerated: z.ZodBoolean;
            content: z.ZodString;
            contentId: z.ZodString;
            id: z.ZodString;
            locale: z.ZodString;
            requiresValidation: z.ZodBoolean;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    description: z.ZodType<z.objectOutputType<{
        textContent: z.ZodType<z.objectOutputType<{
            format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
            id: z.ZodString;
            primaryLocale: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
            aiGenerated: z.ZodBoolean;
            content: z.ZodString;
            contentId: z.ZodString;
            id: z.ZodString;
            locale: z.ZodString;
            requiresValidation: z.ZodBoolean;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    name: z.ZodType<z.objectOutputType<{
        textContent: z.ZodType<z.objectOutputType<{
            format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
            id: z.ZodString;
            primaryLocale: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
            aiGenerated: z.ZodBoolean;
            content: z.ZodString;
            contentId: z.ZodString;
            id: z.ZodString;
            locale: z.ZodString;
            requiresValidation: z.ZodBoolean;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
}, z.ZodTypeAny, "passthrough">>;
export type WorkflowStepTranslations = z.infer<typeof _WorkflowStepTranslations>;
export declare const WorkflowStepTranslations: z.ZodType<WorkflowStepTranslations, z.ZodTypeDef, any>;
declare const _WorkflowStepWithTranslationsDto: z.ZodObject<{
    activationRule: z.ZodType<"manual", z.ZodTypeDef, any>;
    canRevisit: z.ZodBoolean;
    description: z.ZodString;
    id: z.ZodString;
    isOffline: z.ZodBoolean;
    name: z.ZodString;
    previewToolConfig: z.ZodType<z.objectOutputType<{
        admin_password: z.ZodString;
        admin_user: z.ZodString;
        description: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
        is_active: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
        label_seeds_as_conversation_starter: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
        poll_id: z.ZodString;
        required_votes: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        server_url: z.ZodString;
        show_remaining_statements: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
        strict_moderation: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
        topic: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
        type: z.ZodLiteral<"polis">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        pages: z.ZodArray<z.ZodType<z.objectOutputType<{
            text_content_id: z.ZodString;
        }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
            content: z.ZodString;
            type: z.ZodLiteral<"markdown">;
        }, z.ZodTypeAny, "passthrough">[], z.ZodTypeDef, any>, "many">;
        type: z.ZodLiteral<"learn">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        admin_password: z.ZodString;
        admin_user: z.ZodString;
        project_id: z.ZodString;
        server_url: z.ZodDefault<z.ZodOptional<z.ZodString>>;
        survey_id: z.ZodString;
        survey_url: z.ZodString;
        type: z.ZodLiteral<"heyform">;
        workspace_id: z.ZodString;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        max_time: z.ZodNumber;
        to_see: z.ZodNumber;
        type: z.ZodLiteral<"stories">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        topic: z.ZodString;
        type: z.ZodLiteral<"elicitationbot">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        alignment_question_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            text: z.ZodType<z.objectOutputType<{
                localized: z.ZodString;
                translations: z.ZodType<z.objectOutputType<{
                    textContent: z.ZodType<z.objectOutputType<{
                        format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                        id: z.ZodString;
                        primaryLocale: z.ZodString;
                    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                    textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                        aiGenerated: z.ZodBoolean;
                        content: z.ZodString;
                        contentId: z.ZodString;
                        id: z.ZodString;
                        locale: z.ZodString;
                        requiresValidation: z.ZodBoolean;
                    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            type: z.ZodType<"text" | {
                likert_scale: {
                    categories: z.objectOutputType<{
                        label: z.ZodType<z.objectOutputType<{
                            localized: z.ZodString;
                            translations: z.ZodType<z.objectOutputType<{
                                textContent: z.ZodType<z.objectOutputType<{
                                    format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                                    id: z.ZodString;
                                    primaryLocale: z.ZodString;
                                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                                textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                                    aiGenerated: z.ZodBoolean;
                                    content: z.ZodString;
                                    contentId: z.ZodString;
                                    id: z.ZodString;
                                    locale: z.ZodString;
                                    requiresValidation: z.ZodBoolean;
                                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
                            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                        value: z.ZodNumber;
                    }, z.ZodTypeAny, "passthrough">[];
                } & {
                    [k: string]: unknown;
                };
            } | {
                continuous: {
                    max_label: {
                        translations: {
                            textContent: {
                                id: string;
                                primaryLocale: string;
                                format: "plain" | "markdown" | "rich";
                            } & {
                                [k: string]: unknown;
                            };
                            textTranslations: z.objectOutputType<{
                                aiGenerated: z.ZodBoolean;
                                content: z.ZodString;
                                contentId: z.ZodString;
                                id: z.ZodString;
                                locale: z.ZodString;
                                requiresValidation: z.ZodBoolean;
                            }, z.ZodTypeAny, "passthrough">[];
                        } & {
                            [k: string]: unknown;
                        };
                        localized: string;
                    } & {
                        [k: string]: unknown;
                    };
                    max_value: number;
                    min_label: {
                        translations: {
                            textContent: {
                                id: string;
                                primaryLocale: string;
                                format: "plain" | "markdown" | "rich";
                            } & {
                                [k: string]: unknown;
                            };
                            textTranslations: z.objectOutputType<{
                                aiGenerated: z.ZodBoolean;
                                content: z.ZodString;
                                contentId: z.ZodString;
                                id: z.ZodString;
                                locale: z.ZodString;
                                requiresValidation: z.ZodBoolean;
                            }, z.ZodTypeAny, "passthrough">[];
                        } & {
                            [k: string]: unknown;
                        };
                        localized: string;
                    } & {
                        [k: string]: unknown;
                    };
                    min_value: number;
                    sub_steps: number;
                } & {
                    [k: string]: unknown;
                };
            }, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        randomize_order: z.ZodBoolean;
        required_reviews: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        section_questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            text: z.ZodType<z.objectOutputType<{
                localized: z.ZodString;
                translations: z.ZodType<z.objectOutputType<{
                    textContent: z.ZodType<z.objectOutputType<{
                        format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                        id: z.ZodString;
                        primaryLocale: z.ZodString;
                    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                    textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                        aiGenerated: z.ZodBoolean;
                        content: z.ZodString;
                        contentId: z.ZodString;
                        id: z.ZodString;
                        locale: z.ZodString;
                        requiresValidation: z.ZodBoolean;
                    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            type: z.ZodType<"text" | {
                likert_scale: {
                    categories: z.objectOutputType<{
                        label: z.ZodType<z.objectOutputType<{
                            localized: z.ZodString;
                            translations: z.ZodType<z.objectOutputType<{
                                textContent: z.ZodType<z.objectOutputType<{
                                    format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                                    id: z.ZodString;
                                    primaryLocale: z.ZodString;
                                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                                textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                                    aiGenerated: z.ZodBoolean;
                                    content: z.ZodString;
                                    contentId: z.ZodString;
                                    id: z.ZodString;
                                    locale: z.ZodString;
                                    requiresValidation: z.ZodBoolean;
                                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
                            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                        value: z.ZodNumber;
                    }, z.ZodTypeAny, "passthrough">[];
                } & {
                    [k: string]: unknown;
                };
            } | {
                continuous: {
                    max_label: {
                        translations: {
                            textContent: {
                                id: string;
                                primaryLocale: string;
                                format: "plain" | "markdown" | "rich";
                            } & {
                                [k: string]: unknown;
                            };
                            textTranslations: z.objectOutputType<{
                                aiGenerated: z.ZodBoolean;
                                content: z.ZodString;
                                contentId: z.ZodString;
                                id: z.ZodString;
                                locale: z.ZodString;
                                requiresValidation: z.ZodBoolean;
                            }, z.ZodTypeAny, "passthrough">[];
                        } & {
                            [k: string]: unknown;
                        };
                        localized: string;
                    } & {
                        [k: string]: unknown;
                    };
                    max_value: number;
                    min_label: {
                        translations: {
                            textContent: {
                                id: string;
                                primaryLocale: string;
                                format: "plain" | "markdown" | "rich";
                            } & {
                                [k: string]: unknown;
                            };
                            textTranslations: z.objectOutputType<{
                                aiGenerated: z.ZodBoolean;
                                content: z.ZodString;
                                contentId: z.ZodString;
                                id: z.ZodString;
                                locale: z.ZodString;
                                requiresValidation: z.ZodBoolean;
                            }, z.ZodTypeAny, "passthrough">[];
                        } & {
                            [k: string]: unknown;
                        };
                        localized: string;
                    } & {
                        [k: string]: unknown;
                    };
                    min_value: number;
                    sub_steps: number;
                } & {
                    [k: string]: unknown;
                };
            }, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        type: z.ZodLiteral<"prioritization">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        follow_up_rounds_count: z.ZodNumber;
        root_questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            intent: z.ZodType<z.objectOutputType<{
                localized: z.ZodString;
                translations: z.ZodType<z.objectOutputType<{
                    textContent: z.ZodType<z.objectOutputType<{
                        format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                        id: z.ZodString;
                        primaryLocale: z.ZodString;
                    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                    textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                        aiGenerated: z.ZodBoolean;
                        content: z.ZodString;
                        contentId: z.ZodString;
                        id: z.ZodString;
                        locale: z.ZodString;
                        requiresValidation: z.ZodBoolean;
                    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            text: z.ZodType<z.objectOutputType<{
                localized: z.ZodString;
                translations: z.ZodType<z.objectOutputType<{
                    textContent: z.ZodType<z.objectOutputType<{
                        format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                        id: z.ZodString;
                        primaryLocale: z.ZodString;
                    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                    textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                        aiGenerated: z.ZodBoolean;
                        content: z.ZodString;
                        contentId: z.ZodString;
                        id: z.ZodString;
                        locale: z.ZodString;
                        requiresValidation: z.ZodBoolean;
                    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        topic: z.ZodType<z.objectOutputType<{
            localized: z.ZodString;
            translations: z.ZodType<z.objectOutputType<{
                textContent: z.ZodType<z.objectOutputType<{
                    format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                    id: z.ZodString;
                    primaryLocale: z.ZodString;
                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                    aiGenerated: z.ZodBoolean;
                    content: z.ZodString;
                    contentId: z.ZodString;
                    id: z.ZodString;
                    locale: z.ZodString;
                    requiresValidation: z.ZodBoolean;
                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        type: z.ZodLiteral<"thinkingspace">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    requestUserSharePermission: z.ZodBoolean;
    required: z.ZodBoolean;
    stepOrder: z.ZodNumber;
    toolConfig: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
        admin_password: z.ZodString;
        admin_user: z.ZodString;
        description: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
        is_active: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
        label_seeds_as_conversation_starter: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
        poll_id: z.ZodString;
        required_votes: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        server_url: z.ZodString;
        show_remaining_statements: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
        strict_moderation: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
        topic: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
        type: z.ZodLiteral<"polis">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        pages: z.ZodArray<z.ZodType<z.objectOutputType<{
            text_content_id: z.ZodString;
        }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
            content: z.ZodString;
            type: z.ZodLiteral<"markdown">;
        }, z.ZodTypeAny, "passthrough">[], z.ZodTypeDef, any>, "many">;
        type: z.ZodLiteral<"learn">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        admin_password: z.ZodString;
        admin_user: z.ZodString;
        project_id: z.ZodString;
        server_url: z.ZodDefault<z.ZodOptional<z.ZodString>>;
        survey_id: z.ZodString;
        survey_url: z.ZodString;
        type: z.ZodLiteral<"heyform">;
        workspace_id: z.ZodString;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        max_time: z.ZodNumber;
        to_see: z.ZodNumber;
        type: z.ZodLiteral<"stories">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        topic: z.ZodString;
        type: z.ZodLiteral<"elicitationbot">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        alignment_question_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            text: z.ZodType<z.objectOutputType<{
                localized: z.ZodString;
                translations: z.ZodType<z.objectOutputType<{
                    textContent: z.ZodType<z.objectOutputType<{
                        format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                        id: z.ZodString;
                        primaryLocale: z.ZodString;
                    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                    textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                        aiGenerated: z.ZodBoolean;
                        content: z.ZodString;
                        contentId: z.ZodString;
                        id: z.ZodString;
                        locale: z.ZodString;
                        requiresValidation: z.ZodBoolean;
                    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            type: z.ZodType<"text" | {
                likert_scale: {
                    categories: z.objectOutputType<{
                        label: z.ZodType<z.objectOutputType<{
                            localized: z.ZodString;
                            translations: z.ZodType<z.objectOutputType<{
                                textContent: z.ZodType<z.objectOutputType<{
                                    format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                                    id: z.ZodString;
                                    primaryLocale: z.ZodString;
                                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                                textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                                    aiGenerated: z.ZodBoolean;
                                    content: z.ZodString;
                                    contentId: z.ZodString;
                                    id: z.ZodString;
                                    locale: z.ZodString;
                                    requiresValidation: z.ZodBoolean;
                                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
                            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                        value: z.ZodNumber;
                    }, z.ZodTypeAny, "passthrough">[];
                } & {
                    [k: string]: unknown;
                };
            } | {
                continuous: {
                    max_label: {
                        translations: {
                            textContent: {
                                id: string;
                                primaryLocale: string;
                                format: "plain" | "markdown" | "rich";
                            } & {
                                [k: string]: unknown;
                            };
                            textTranslations: z.objectOutputType<{
                                aiGenerated: z.ZodBoolean;
                                content: z.ZodString;
                                contentId: z.ZodString;
                                id: z.ZodString;
                                locale: z.ZodString;
                                requiresValidation: z.ZodBoolean;
                            }, z.ZodTypeAny, "passthrough">[];
                        } & {
                            [k: string]: unknown;
                        };
                        localized: string;
                    } & {
                        [k: string]: unknown;
                    };
                    max_value: number;
                    min_label: {
                        translations: {
                            textContent: {
                                id: string;
                                primaryLocale: string;
                                format: "plain" | "markdown" | "rich";
                            } & {
                                [k: string]: unknown;
                            };
                            textTranslations: z.objectOutputType<{
                                aiGenerated: z.ZodBoolean;
                                content: z.ZodString;
                                contentId: z.ZodString;
                                id: z.ZodString;
                                locale: z.ZodString;
                                requiresValidation: z.ZodBoolean;
                            }, z.ZodTypeAny, "passthrough">[];
                        } & {
                            [k: string]: unknown;
                        };
                        localized: string;
                    } & {
                        [k: string]: unknown;
                    };
                    min_value: number;
                    sub_steps: number;
                } & {
                    [k: string]: unknown;
                };
            }, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        randomize_order: z.ZodBoolean;
        required_reviews: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        section_questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            text: z.ZodType<z.objectOutputType<{
                localized: z.ZodString;
                translations: z.ZodType<z.objectOutputType<{
                    textContent: z.ZodType<z.objectOutputType<{
                        format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                        id: z.ZodString;
                        primaryLocale: z.ZodString;
                    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                    textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                        aiGenerated: z.ZodBoolean;
                        content: z.ZodString;
                        contentId: z.ZodString;
                        id: z.ZodString;
                        locale: z.ZodString;
                        requiresValidation: z.ZodBoolean;
                    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            type: z.ZodType<"text" | {
                likert_scale: {
                    categories: z.objectOutputType<{
                        label: z.ZodType<z.objectOutputType<{
                            localized: z.ZodString;
                            translations: z.ZodType<z.objectOutputType<{
                                textContent: z.ZodType<z.objectOutputType<{
                                    format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                                    id: z.ZodString;
                                    primaryLocale: z.ZodString;
                                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                                textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                                    aiGenerated: z.ZodBoolean;
                                    content: z.ZodString;
                                    contentId: z.ZodString;
                                    id: z.ZodString;
                                    locale: z.ZodString;
                                    requiresValidation: z.ZodBoolean;
                                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
                            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                        value: z.ZodNumber;
                    }, z.ZodTypeAny, "passthrough">[];
                } & {
                    [k: string]: unknown;
                };
            } | {
                continuous: {
                    max_label: {
                        translations: {
                            textContent: {
                                id: string;
                                primaryLocale: string;
                                format: "plain" | "markdown" | "rich";
                            } & {
                                [k: string]: unknown;
                            };
                            textTranslations: z.objectOutputType<{
                                aiGenerated: z.ZodBoolean;
                                content: z.ZodString;
                                contentId: z.ZodString;
                                id: z.ZodString;
                                locale: z.ZodString;
                                requiresValidation: z.ZodBoolean;
                            }, z.ZodTypeAny, "passthrough">[];
                        } & {
                            [k: string]: unknown;
                        };
                        localized: string;
                    } & {
                        [k: string]: unknown;
                    };
                    max_value: number;
                    min_label: {
                        translations: {
                            textContent: {
                                id: string;
                                primaryLocale: string;
                                format: "plain" | "markdown" | "rich";
                            } & {
                                [k: string]: unknown;
                            };
                            textTranslations: z.objectOutputType<{
                                aiGenerated: z.ZodBoolean;
                                content: z.ZodString;
                                contentId: z.ZodString;
                                id: z.ZodString;
                                locale: z.ZodString;
                                requiresValidation: z.ZodBoolean;
                            }, z.ZodTypeAny, "passthrough">[];
                        } & {
                            [k: string]: unknown;
                        };
                        localized: string;
                    } & {
                        [k: string]: unknown;
                    };
                    min_value: number;
                    sub_steps: number;
                } & {
                    [k: string]: unknown;
                };
            }, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        type: z.ZodLiteral<"prioritization">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        follow_up_rounds_count: z.ZodNumber;
        root_questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            intent: z.ZodType<z.objectOutputType<{
                localized: z.ZodString;
                translations: z.ZodType<z.objectOutputType<{
                    textContent: z.ZodType<z.objectOutputType<{
                        format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                        id: z.ZodString;
                        primaryLocale: z.ZodString;
                    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                    textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                        aiGenerated: z.ZodBoolean;
                        content: z.ZodString;
                        contentId: z.ZodString;
                        id: z.ZodString;
                        locale: z.ZodString;
                        requiresValidation: z.ZodBoolean;
                    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            text: z.ZodType<z.objectOutputType<{
                localized: z.ZodString;
                translations: z.ZodType<z.objectOutputType<{
                    textContent: z.ZodType<z.objectOutputType<{
                        format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                        id: z.ZodString;
                        primaryLocale: z.ZodString;
                    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                    textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                        aiGenerated: z.ZodBoolean;
                        content: z.ZodString;
                        contentId: z.ZodString;
                        id: z.ZodString;
                        locale: z.ZodString;
                        requiresValidation: z.ZodBoolean;
                    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        topic: z.ZodType<z.objectOutputType<{
            localized: z.ZodString;
            translations: z.ZodType<z.objectOutputType<{
                textContent: z.ZodType<z.objectOutputType<{
                    format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                    id: z.ZodString;
                    primaryLocale: z.ZodString;
                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                    aiGenerated: z.ZodBoolean;
                    content: z.ZodString;
                    contentId: z.ZodString;
                    id: z.ZodString;
                    locale: z.ZodString;
                    requiresValidation: z.ZodBoolean;
                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        type: z.ZodLiteral<"thinkingspace">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
    translations: z.ZodType<z.objectOutputType<{
        description: z.ZodType<z.objectOutputType<{
            textContent: z.ZodType<z.objectOutputType<{
                format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                id: z.ZodString;
                primaryLocale: z.ZodString;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                aiGenerated: z.ZodBoolean;
                content: z.ZodString;
                contentId: z.ZodString;
                id: z.ZodString;
                locale: z.ZodString;
                requiresValidation: z.ZodBoolean;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        name: z.ZodType<z.objectOutputType<{
            textContent: z.ZodType<z.objectOutputType<{
                format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                id: z.ZodString;
                primaryLocale: z.ZodString;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                aiGenerated: z.ZodBoolean;
                content: z.ZodString;
                contentId: z.ZodString;
                id: z.ZodString;
                locale: z.ZodString;
                requiresValidation: z.ZodBoolean;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    workflowId: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    activationRule: z.ZodType<"manual", z.ZodTypeDef, any>;
    canRevisit: z.ZodBoolean;
    description: z.ZodString;
    id: z.ZodString;
    isOffline: z.ZodBoolean;
    name: z.ZodString;
    previewToolConfig: z.ZodType<z.objectOutputType<{
        admin_password: z.ZodString;
        admin_user: z.ZodString;
        description: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
        is_active: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
        label_seeds_as_conversation_starter: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
        poll_id: z.ZodString;
        required_votes: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        server_url: z.ZodString;
        show_remaining_statements: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
        strict_moderation: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
        topic: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
        type: z.ZodLiteral<"polis">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        pages: z.ZodArray<z.ZodType<z.objectOutputType<{
            text_content_id: z.ZodString;
        }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
            content: z.ZodString;
            type: z.ZodLiteral<"markdown">;
        }, z.ZodTypeAny, "passthrough">[], z.ZodTypeDef, any>, "many">;
        type: z.ZodLiteral<"learn">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        admin_password: z.ZodString;
        admin_user: z.ZodString;
        project_id: z.ZodString;
        server_url: z.ZodDefault<z.ZodOptional<z.ZodString>>;
        survey_id: z.ZodString;
        survey_url: z.ZodString;
        type: z.ZodLiteral<"heyform">;
        workspace_id: z.ZodString;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        max_time: z.ZodNumber;
        to_see: z.ZodNumber;
        type: z.ZodLiteral<"stories">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        topic: z.ZodString;
        type: z.ZodLiteral<"elicitationbot">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        alignment_question_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            text: z.ZodType<z.objectOutputType<{
                localized: z.ZodString;
                translations: z.ZodType<z.objectOutputType<{
                    textContent: z.ZodType<z.objectOutputType<{
                        format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                        id: z.ZodString;
                        primaryLocale: z.ZodString;
                    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                    textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                        aiGenerated: z.ZodBoolean;
                        content: z.ZodString;
                        contentId: z.ZodString;
                        id: z.ZodString;
                        locale: z.ZodString;
                        requiresValidation: z.ZodBoolean;
                    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            type: z.ZodType<"text" | {
                likert_scale: {
                    categories: z.objectOutputType<{
                        label: z.ZodType<z.objectOutputType<{
                            localized: z.ZodString;
                            translations: z.ZodType<z.objectOutputType<{
                                textContent: z.ZodType<z.objectOutputType<{
                                    format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                                    id: z.ZodString;
                                    primaryLocale: z.ZodString;
                                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                                textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                                    aiGenerated: z.ZodBoolean;
                                    content: z.ZodString;
                                    contentId: z.ZodString;
                                    id: z.ZodString;
                                    locale: z.ZodString;
                                    requiresValidation: z.ZodBoolean;
                                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
                            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                        value: z.ZodNumber;
                    }, z.ZodTypeAny, "passthrough">[];
                } & {
                    [k: string]: unknown;
                };
            } | {
                continuous: {
                    max_label: {
                        translations: {
                            textContent: {
                                id: string;
                                primaryLocale: string;
                                format: "plain" | "markdown" | "rich";
                            } & {
                                [k: string]: unknown;
                            };
                            textTranslations: z.objectOutputType<{
                                aiGenerated: z.ZodBoolean;
                                content: z.ZodString;
                                contentId: z.ZodString;
                                id: z.ZodString;
                                locale: z.ZodString;
                                requiresValidation: z.ZodBoolean;
                            }, z.ZodTypeAny, "passthrough">[];
                        } & {
                            [k: string]: unknown;
                        };
                        localized: string;
                    } & {
                        [k: string]: unknown;
                    };
                    max_value: number;
                    min_label: {
                        translations: {
                            textContent: {
                                id: string;
                                primaryLocale: string;
                                format: "plain" | "markdown" | "rich";
                            } & {
                                [k: string]: unknown;
                            };
                            textTranslations: z.objectOutputType<{
                                aiGenerated: z.ZodBoolean;
                                content: z.ZodString;
                                contentId: z.ZodString;
                                id: z.ZodString;
                                locale: z.ZodString;
                                requiresValidation: z.ZodBoolean;
                            }, z.ZodTypeAny, "passthrough">[];
                        } & {
                            [k: string]: unknown;
                        };
                        localized: string;
                    } & {
                        [k: string]: unknown;
                    };
                    min_value: number;
                    sub_steps: number;
                } & {
                    [k: string]: unknown;
                };
            }, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        randomize_order: z.ZodBoolean;
        required_reviews: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        section_questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            text: z.ZodType<z.objectOutputType<{
                localized: z.ZodString;
                translations: z.ZodType<z.objectOutputType<{
                    textContent: z.ZodType<z.objectOutputType<{
                        format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                        id: z.ZodString;
                        primaryLocale: z.ZodString;
                    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                    textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                        aiGenerated: z.ZodBoolean;
                        content: z.ZodString;
                        contentId: z.ZodString;
                        id: z.ZodString;
                        locale: z.ZodString;
                        requiresValidation: z.ZodBoolean;
                    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            type: z.ZodType<"text" | {
                likert_scale: {
                    categories: z.objectOutputType<{
                        label: z.ZodType<z.objectOutputType<{
                            localized: z.ZodString;
                            translations: z.ZodType<z.objectOutputType<{
                                textContent: z.ZodType<z.objectOutputType<{
                                    format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                                    id: z.ZodString;
                                    primaryLocale: z.ZodString;
                                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                                textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                                    aiGenerated: z.ZodBoolean;
                                    content: z.ZodString;
                                    contentId: z.ZodString;
                                    id: z.ZodString;
                                    locale: z.ZodString;
                                    requiresValidation: z.ZodBoolean;
                                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
                            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                        value: z.ZodNumber;
                    }, z.ZodTypeAny, "passthrough">[];
                } & {
                    [k: string]: unknown;
                };
            } | {
                continuous: {
                    max_label: {
                        translations: {
                            textContent: {
                                id: string;
                                primaryLocale: string;
                                format: "plain" | "markdown" | "rich";
                            } & {
                                [k: string]: unknown;
                            };
                            textTranslations: z.objectOutputType<{
                                aiGenerated: z.ZodBoolean;
                                content: z.ZodString;
                                contentId: z.ZodString;
                                id: z.ZodString;
                                locale: z.ZodString;
                                requiresValidation: z.ZodBoolean;
                            }, z.ZodTypeAny, "passthrough">[];
                        } & {
                            [k: string]: unknown;
                        };
                        localized: string;
                    } & {
                        [k: string]: unknown;
                    };
                    max_value: number;
                    min_label: {
                        translations: {
                            textContent: {
                                id: string;
                                primaryLocale: string;
                                format: "plain" | "markdown" | "rich";
                            } & {
                                [k: string]: unknown;
                            };
                            textTranslations: z.objectOutputType<{
                                aiGenerated: z.ZodBoolean;
                                content: z.ZodString;
                                contentId: z.ZodString;
                                id: z.ZodString;
                                locale: z.ZodString;
                                requiresValidation: z.ZodBoolean;
                            }, z.ZodTypeAny, "passthrough">[];
                        } & {
                            [k: string]: unknown;
                        };
                        localized: string;
                    } & {
                        [k: string]: unknown;
                    };
                    min_value: number;
                    sub_steps: number;
                } & {
                    [k: string]: unknown;
                };
            }, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        type: z.ZodLiteral<"prioritization">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        follow_up_rounds_count: z.ZodNumber;
        root_questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            intent: z.ZodType<z.objectOutputType<{
                localized: z.ZodString;
                translations: z.ZodType<z.objectOutputType<{
                    textContent: z.ZodType<z.objectOutputType<{
                        format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                        id: z.ZodString;
                        primaryLocale: z.ZodString;
                    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                    textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                        aiGenerated: z.ZodBoolean;
                        content: z.ZodString;
                        contentId: z.ZodString;
                        id: z.ZodString;
                        locale: z.ZodString;
                        requiresValidation: z.ZodBoolean;
                    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            text: z.ZodType<z.objectOutputType<{
                localized: z.ZodString;
                translations: z.ZodType<z.objectOutputType<{
                    textContent: z.ZodType<z.objectOutputType<{
                        format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                        id: z.ZodString;
                        primaryLocale: z.ZodString;
                    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                    textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                        aiGenerated: z.ZodBoolean;
                        content: z.ZodString;
                        contentId: z.ZodString;
                        id: z.ZodString;
                        locale: z.ZodString;
                        requiresValidation: z.ZodBoolean;
                    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        topic: z.ZodType<z.objectOutputType<{
            localized: z.ZodString;
            translations: z.ZodType<z.objectOutputType<{
                textContent: z.ZodType<z.objectOutputType<{
                    format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                    id: z.ZodString;
                    primaryLocale: z.ZodString;
                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                    aiGenerated: z.ZodBoolean;
                    content: z.ZodString;
                    contentId: z.ZodString;
                    id: z.ZodString;
                    locale: z.ZodString;
                    requiresValidation: z.ZodBoolean;
                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        type: z.ZodLiteral<"thinkingspace">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    requestUserSharePermission: z.ZodBoolean;
    required: z.ZodBoolean;
    stepOrder: z.ZodNumber;
    toolConfig: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
        admin_password: z.ZodString;
        admin_user: z.ZodString;
        description: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
        is_active: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
        label_seeds_as_conversation_starter: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
        poll_id: z.ZodString;
        required_votes: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        server_url: z.ZodString;
        show_remaining_statements: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
        strict_moderation: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
        topic: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
        type: z.ZodLiteral<"polis">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        pages: z.ZodArray<z.ZodType<z.objectOutputType<{
            text_content_id: z.ZodString;
        }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
            content: z.ZodString;
            type: z.ZodLiteral<"markdown">;
        }, z.ZodTypeAny, "passthrough">[], z.ZodTypeDef, any>, "many">;
        type: z.ZodLiteral<"learn">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        admin_password: z.ZodString;
        admin_user: z.ZodString;
        project_id: z.ZodString;
        server_url: z.ZodDefault<z.ZodOptional<z.ZodString>>;
        survey_id: z.ZodString;
        survey_url: z.ZodString;
        type: z.ZodLiteral<"heyform">;
        workspace_id: z.ZodString;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        max_time: z.ZodNumber;
        to_see: z.ZodNumber;
        type: z.ZodLiteral<"stories">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        topic: z.ZodString;
        type: z.ZodLiteral<"elicitationbot">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        alignment_question_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            text: z.ZodType<z.objectOutputType<{
                localized: z.ZodString;
                translations: z.ZodType<z.objectOutputType<{
                    textContent: z.ZodType<z.objectOutputType<{
                        format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                        id: z.ZodString;
                        primaryLocale: z.ZodString;
                    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                    textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                        aiGenerated: z.ZodBoolean;
                        content: z.ZodString;
                        contentId: z.ZodString;
                        id: z.ZodString;
                        locale: z.ZodString;
                        requiresValidation: z.ZodBoolean;
                    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            type: z.ZodType<"text" | {
                likert_scale: {
                    categories: z.objectOutputType<{
                        label: z.ZodType<z.objectOutputType<{
                            localized: z.ZodString;
                            translations: z.ZodType<z.objectOutputType<{
                                textContent: z.ZodType<z.objectOutputType<{
                                    format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                                    id: z.ZodString;
                                    primaryLocale: z.ZodString;
                                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                                textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                                    aiGenerated: z.ZodBoolean;
                                    content: z.ZodString;
                                    contentId: z.ZodString;
                                    id: z.ZodString;
                                    locale: z.ZodString;
                                    requiresValidation: z.ZodBoolean;
                                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
                            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                        value: z.ZodNumber;
                    }, z.ZodTypeAny, "passthrough">[];
                } & {
                    [k: string]: unknown;
                };
            } | {
                continuous: {
                    max_label: {
                        translations: {
                            textContent: {
                                id: string;
                                primaryLocale: string;
                                format: "plain" | "markdown" | "rich";
                            } & {
                                [k: string]: unknown;
                            };
                            textTranslations: z.objectOutputType<{
                                aiGenerated: z.ZodBoolean;
                                content: z.ZodString;
                                contentId: z.ZodString;
                                id: z.ZodString;
                                locale: z.ZodString;
                                requiresValidation: z.ZodBoolean;
                            }, z.ZodTypeAny, "passthrough">[];
                        } & {
                            [k: string]: unknown;
                        };
                        localized: string;
                    } & {
                        [k: string]: unknown;
                    };
                    max_value: number;
                    min_label: {
                        translations: {
                            textContent: {
                                id: string;
                                primaryLocale: string;
                                format: "plain" | "markdown" | "rich";
                            } & {
                                [k: string]: unknown;
                            };
                            textTranslations: z.objectOutputType<{
                                aiGenerated: z.ZodBoolean;
                                content: z.ZodString;
                                contentId: z.ZodString;
                                id: z.ZodString;
                                locale: z.ZodString;
                                requiresValidation: z.ZodBoolean;
                            }, z.ZodTypeAny, "passthrough">[];
                        } & {
                            [k: string]: unknown;
                        };
                        localized: string;
                    } & {
                        [k: string]: unknown;
                    };
                    min_value: number;
                    sub_steps: number;
                } & {
                    [k: string]: unknown;
                };
            }, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        randomize_order: z.ZodBoolean;
        required_reviews: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        section_questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            text: z.ZodType<z.objectOutputType<{
                localized: z.ZodString;
                translations: z.ZodType<z.objectOutputType<{
                    textContent: z.ZodType<z.objectOutputType<{
                        format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                        id: z.ZodString;
                        primaryLocale: z.ZodString;
                    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                    textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                        aiGenerated: z.ZodBoolean;
                        content: z.ZodString;
                        contentId: z.ZodString;
                        id: z.ZodString;
                        locale: z.ZodString;
                        requiresValidation: z.ZodBoolean;
                    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            type: z.ZodType<"text" | {
                likert_scale: {
                    categories: z.objectOutputType<{
                        label: z.ZodType<z.objectOutputType<{
                            localized: z.ZodString;
                            translations: z.ZodType<z.objectOutputType<{
                                textContent: z.ZodType<z.objectOutputType<{
                                    format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                                    id: z.ZodString;
                                    primaryLocale: z.ZodString;
                                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                                textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                                    aiGenerated: z.ZodBoolean;
                                    content: z.ZodString;
                                    contentId: z.ZodString;
                                    id: z.ZodString;
                                    locale: z.ZodString;
                                    requiresValidation: z.ZodBoolean;
                                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
                            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                        value: z.ZodNumber;
                    }, z.ZodTypeAny, "passthrough">[];
                } & {
                    [k: string]: unknown;
                };
            } | {
                continuous: {
                    max_label: {
                        translations: {
                            textContent: {
                                id: string;
                                primaryLocale: string;
                                format: "plain" | "markdown" | "rich";
                            } & {
                                [k: string]: unknown;
                            };
                            textTranslations: z.objectOutputType<{
                                aiGenerated: z.ZodBoolean;
                                content: z.ZodString;
                                contentId: z.ZodString;
                                id: z.ZodString;
                                locale: z.ZodString;
                                requiresValidation: z.ZodBoolean;
                            }, z.ZodTypeAny, "passthrough">[];
                        } & {
                            [k: string]: unknown;
                        };
                        localized: string;
                    } & {
                        [k: string]: unknown;
                    };
                    max_value: number;
                    min_label: {
                        translations: {
                            textContent: {
                                id: string;
                                primaryLocale: string;
                                format: "plain" | "markdown" | "rich";
                            } & {
                                [k: string]: unknown;
                            };
                            textTranslations: z.objectOutputType<{
                                aiGenerated: z.ZodBoolean;
                                content: z.ZodString;
                                contentId: z.ZodString;
                                id: z.ZodString;
                                locale: z.ZodString;
                                requiresValidation: z.ZodBoolean;
                            }, z.ZodTypeAny, "passthrough">[];
                        } & {
                            [k: string]: unknown;
                        };
                        localized: string;
                    } & {
                        [k: string]: unknown;
                    };
                    min_value: number;
                    sub_steps: number;
                } & {
                    [k: string]: unknown;
                };
            }, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        type: z.ZodLiteral<"prioritization">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        follow_up_rounds_count: z.ZodNumber;
        root_questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            intent: z.ZodType<z.objectOutputType<{
                localized: z.ZodString;
                translations: z.ZodType<z.objectOutputType<{
                    textContent: z.ZodType<z.objectOutputType<{
                        format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                        id: z.ZodString;
                        primaryLocale: z.ZodString;
                    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                    textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                        aiGenerated: z.ZodBoolean;
                        content: z.ZodString;
                        contentId: z.ZodString;
                        id: z.ZodString;
                        locale: z.ZodString;
                        requiresValidation: z.ZodBoolean;
                    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            text: z.ZodType<z.objectOutputType<{
                localized: z.ZodString;
                translations: z.ZodType<z.objectOutputType<{
                    textContent: z.ZodType<z.objectOutputType<{
                        format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                        id: z.ZodString;
                        primaryLocale: z.ZodString;
                    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                    textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                        aiGenerated: z.ZodBoolean;
                        content: z.ZodString;
                        contentId: z.ZodString;
                        id: z.ZodString;
                        locale: z.ZodString;
                        requiresValidation: z.ZodBoolean;
                    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        topic: z.ZodType<z.objectOutputType<{
            localized: z.ZodString;
            translations: z.ZodType<z.objectOutputType<{
                textContent: z.ZodType<z.objectOutputType<{
                    format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                    id: z.ZodString;
                    primaryLocale: z.ZodString;
                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                    aiGenerated: z.ZodBoolean;
                    content: z.ZodString;
                    contentId: z.ZodString;
                    id: z.ZodString;
                    locale: z.ZodString;
                    requiresValidation: z.ZodBoolean;
                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        type: z.ZodLiteral<"thinkingspace">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
    translations: z.ZodType<z.objectOutputType<{
        description: z.ZodType<z.objectOutputType<{
            textContent: z.ZodType<z.objectOutputType<{
                format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                id: z.ZodString;
                primaryLocale: z.ZodString;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                aiGenerated: z.ZodBoolean;
                content: z.ZodString;
                contentId: z.ZodString;
                id: z.ZodString;
                locale: z.ZodString;
                requiresValidation: z.ZodBoolean;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        name: z.ZodType<z.objectOutputType<{
            textContent: z.ZodType<z.objectOutputType<{
                format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                id: z.ZodString;
                primaryLocale: z.ZodString;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                aiGenerated: z.ZodBoolean;
                content: z.ZodString;
                contentId: z.ZodString;
                id: z.ZodString;
                locale: z.ZodString;
                requiresValidation: z.ZodBoolean;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    workflowId: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    activationRule: z.ZodType<"manual", z.ZodTypeDef, any>;
    canRevisit: z.ZodBoolean;
    description: z.ZodString;
    id: z.ZodString;
    isOffline: z.ZodBoolean;
    name: z.ZodString;
    previewToolConfig: z.ZodType<z.objectOutputType<{
        admin_password: z.ZodString;
        admin_user: z.ZodString;
        description: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
        is_active: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
        label_seeds_as_conversation_starter: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
        poll_id: z.ZodString;
        required_votes: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        server_url: z.ZodString;
        show_remaining_statements: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
        strict_moderation: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
        topic: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
        type: z.ZodLiteral<"polis">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        pages: z.ZodArray<z.ZodType<z.objectOutputType<{
            text_content_id: z.ZodString;
        }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
            content: z.ZodString;
            type: z.ZodLiteral<"markdown">;
        }, z.ZodTypeAny, "passthrough">[], z.ZodTypeDef, any>, "many">;
        type: z.ZodLiteral<"learn">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        admin_password: z.ZodString;
        admin_user: z.ZodString;
        project_id: z.ZodString;
        server_url: z.ZodDefault<z.ZodOptional<z.ZodString>>;
        survey_id: z.ZodString;
        survey_url: z.ZodString;
        type: z.ZodLiteral<"heyform">;
        workspace_id: z.ZodString;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        max_time: z.ZodNumber;
        to_see: z.ZodNumber;
        type: z.ZodLiteral<"stories">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        topic: z.ZodString;
        type: z.ZodLiteral<"elicitationbot">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        alignment_question_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            text: z.ZodType<z.objectOutputType<{
                localized: z.ZodString;
                translations: z.ZodType<z.objectOutputType<{
                    textContent: z.ZodType<z.objectOutputType<{
                        format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                        id: z.ZodString;
                        primaryLocale: z.ZodString;
                    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                    textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                        aiGenerated: z.ZodBoolean;
                        content: z.ZodString;
                        contentId: z.ZodString;
                        id: z.ZodString;
                        locale: z.ZodString;
                        requiresValidation: z.ZodBoolean;
                    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            type: z.ZodType<"text" | {
                likert_scale: {
                    categories: z.objectOutputType<{
                        label: z.ZodType<z.objectOutputType<{
                            localized: z.ZodString;
                            translations: z.ZodType<z.objectOutputType<{
                                textContent: z.ZodType<z.objectOutputType<{
                                    format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                                    id: z.ZodString;
                                    primaryLocale: z.ZodString;
                                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                                textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                                    aiGenerated: z.ZodBoolean;
                                    content: z.ZodString;
                                    contentId: z.ZodString;
                                    id: z.ZodString;
                                    locale: z.ZodString;
                                    requiresValidation: z.ZodBoolean;
                                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
                            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                        value: z.ZodNumber;
                    }, z.ZodTypeAny, "passthrough">[];
                } & {
                    [k: string]: unknown;
                };
            } | {
                continuous: {
                    max_label: {
                        translations: {
                            textContent: {
                                id: string;
                                primaryLocale: string;
                                format: "plain" | "markdown" | "rich";
                            } & {
                                [k: string]: unknown;
                            };
                            textTranslations: z.objectOutputType<{
                                aiGenerated: z.ZodBoolean;
                                content: z.ZodString;
                                contentId: z.ZodString;
                                id: z.ZodString;
                                locale: z.ZodString;
                                requiresValidation: z.ZodBoolean;
                            }, z.ZodTypeAny, "passthrough">[];
                        } & {
                            [k: string]: unknown;
                        };
                        localized: string;
                    } & {
                        [k: string]: unknown;
                    };
                    max_value: number;
                    min_label: {
                        translations: {
                            textContent: {
                                id: string;
                                primaryLocale: string;
                                format: "plain" | "markdown" | "rich";
                            } & {
                                [k: string]: unknown;
                            };
                            textTranslations: z.objectOutputType<{
                                aiGenerated: z.ZodBoolean;
                                content: z.ZodString;
                                contentId: z.ZodString;
                                id: z.ZodString;
                                locale: z.ZodString;
                                requiresValidation: z.ZodBoolean;
                            }, z.ZodTypeAny, "passthrough">[];
                        } & {
                            [k: string]: unknown;
                        };
                        localized: string;
                    } & {
                        [k: string]: unknown;
                    };
                    min_value: number;
                    sub_steps: number;
                } & {
                    [k: string]: unknown;
                };
            }, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        randomize_order: z.ZodBoolean;
        required_reviews: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        section_questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            text: z.ZodType<z.objectOutputType<{
                localized: z.ZodString;
                translations: z.ZodType<z.objectOutputType<{
                    textContent: z.ZodType<z.objectOutputType<{
                        format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                        id: z.ZodString;
                        primaryLocale: z.ZodString;
                    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                    textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                        aiGenerated: z.ZodBoolean;
                        content: z.ZodString;
                        contentId: z.ZodString;
                        id: z.ZodString;
                        locale: z.ZodString;
                        requiresValidation: z.ZodBoolean;
                    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            type: z.ZodType<"text" | {
                likert_scale: {
                    categories: z.objectOutputType<{
                        label: z.ZodType<z.objectOutputType<{
                            localized: z.ZodString;
                            translations: z.ZodType<z.objectOutputType<{
                                textContent: z.ZodType<z.objectOutputType<{
                                    format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                                    id: z.ZodString;
                                    primaryLocale: z.ZodString;
                                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                                textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                                    aiGenerated: z.ZodBoolean;
                                    content: z.ZodString;
                                    contentId: z.ZodString;
                                    id: z.ZodString;
                                    locale: z.ZodString;
                                    requiresValidation: z.ZodBoolean;
                                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
                            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                        value: z.ZodNumber;
                    }, z.ZodTypeAny, "passthrough">[];
                } & {
                    [k: string]: unknown;
                };
            } | {
                continuous: {
                    max_label: {
                        translations: {
                            textContent: {
                                id: string;
                                primaryLocale: string;
                                format: "plain" | "markdown" | "rich";
                            } & {
                                [k: string]: unknown;
                            };
                            textTranslations: z.objectOutputType<{
                                aiGenerated: z.ZodBoolean;
                                content: z.ZodString;
                                contentId: z.ZodString;
                                id: z.ZodString;
                                locale: z.ZodString;
                                requiresValidation: z.ZodBoolean;
                            }, z.ZodTypeAny, "passthrough">[];
                        } & {
                            [k: string]: unknown;
                        };
                        localized: string;
                    } & {
                        [k: string]: unknown;
                    };
                    max_value: number;
                    min_label: {
                        translations: {
                            textContent: {
                                id: string;
                                primaryLocale: string;
                                format: "plain" | "markdown" | "rich";
                            } & {
                                [k: string]: unknown;
                            };
                            textTranslations: z.objectOutputType<{
                                aiGenerated: z.ZodBoolean;
                                content: z.ZodString;
                                contentId: z.ZodString;
                                id: z.ZodString;
                                locale: z.ZodString;
                                requiresValidation: z.ZodBoolean;
                            }, z.ZodTypeAny, "passthrough">[];
                        } & {
                            [k: string]: unknown;
                        };
                        localized: string;
                    } & {
                        [k: string]: unknown;
                    };
                    min_value: number;
                    sub_steps: number;
                } & {
                    [k: string]: unknown;
                };
            }, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        type: z.ZodLiteral<"prioritization">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        follow_up_rounds_count: z.ZodNumber;
        root_questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            intent: z.ZodType<z.objectOutputType<{
                localized: z.ZodString;
                translations: z.ZodType<z.objectOutputType<{
                    textContent: z.ZodType<z.objectOutputType<{
                        format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                        id: z.ZodString;
                        primaryLocale: z.ZodString;
                    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                    textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                        aiGenerated: z.ZodBoolean;
                        content: z.ZodString;
                        contentId: z.ZodString;
                        id: z.ZodString;
                        locale: z.ZodString;
                        requiresValidation: z.ZodBoolean;
                    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            text: z.ZodType<z.objectOutputType<{
                localized: z.ZodString;
                translations: z.ZodType<z.objectOutputType<{
                    textContent: z.ZodType<z.objectOutputType<{
                        format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                        id: z.ZodString;
                        primaryLocale: z.ZodString;
                    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                    textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                        aiGenerated: z.ZodBoolean;
                        content: z.ZodString;
                        contentId: z.ZodString;
                        id: z.ZodString;
                        locale: z.ZodString;
                        requiresValidation: z.ZodBoolean;
                    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        topic: z.ZodType<z.objectOutputType<{
            localized: z.ZodString;
            translations: z.ZodType<z.objectOutputType<{
                textContent: z.ZodType<z.objectOutputType<{
                    format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                    id: z.ZodString;
                    primaryLocale: z.ZodString;
                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                    aiGenerated: z.ZodBoolean;
                    content: z.ZodString;
                    contentId: z.ZodString;
                    id: z.ZodString;
                    locale: z.ZodString;
                    requiresValidation: z.ZodBoolean;
                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        type: z.ZodLiteral<"thinkingspace">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    requestUserSharePermission: z.ZodBoolean;
    required: z.ZodBoolean;
    stepOrder: z.ZodNumber;
    toolConfig: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
        admin_password: z.ZodString;
        admin_user: z.ZodString;
        description: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
        is_active: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
        label_seeds_as_conversation_starter: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
        poll_id: z.ZodString;
        required_votes: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        server_url: z.ZodString;
        show_remaining_statements: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
        strict_moderation: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
        topic: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
        type: z.ZodLiteral<"polis">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        pages: z.ZodArray<z.ZodType<z.objectOutputType<{
            text_content_id: z.ZodString;
        }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
            content: z.ZodString;
            type: z.ZodLiteral<"markdown">;
        }, z.ZodTypeAny, "passthrough">[], z.ZodTypeDef, any>, "many">;
        type: z.ZodLiteral<"learn">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        admin_password: z.ZodString;
        admin_user: z.ZodString;
        project_id: z.ZodString;
        server_url: z.ZodDefault<z.ZodOptional<z.ZodString>>;
        survey_id: z.ZodString;
        survey_url: z.ZodString;
        type: z.ZodLiteral<"heyform">;
        workspace_id: z.ZodString;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        max_time: z.ZodNumber;
        to_see: z.ZodNumber;
        type: z.ZodLiteral<"stories">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        topic: z.ZodString;
        type: z.ZodLiteral<"elicitationbot">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        alignment_question_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            text: z.ZodType<z.objectOutputType<{
                localized: z.ZodString;
                translations: z.ZodType<z.objectOutputType<{
                    textContent: z.ZodType<z.objectOutputType<{
                        format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                        id: z.ZodString;
                        primaryLocale: z.ZodString;
                    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                    textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                        aiGenerated: z.ZodBoolean;
                        content: z.ZodString;
                        contentId: z.ZodString;
                        id: z.ZodString;
                        locale: z.ZodString;
                        requiresValidation: z.ZodBoolean;
                    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            type: z.ZodType<"text" | {
                likert_scale: {
                    categories: z.objectOutputType<{
                        label: z.ZodType<z.objectOutputType<{
                            localized: z.ZodString;
                            translations: z.ZodType<z.objectOutputType<{
                                textContent: z.ZodType<z.objectOutputType<{
                                    format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                                    id: z.ZodString;
                                    primaryLocale: z.ZodString;
                                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                                textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                                    aiGenerated: z.ZodBoolean;
                                    content: z.ZodString;
                                    contentId: z.ZodString;
                                    id: z.ZodString;
                                    locale: z.ZodString;
                                    requiresValidation: z.ZodBoolean;
                                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
                            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                        value: z.ZodNumber;
                    }, z.ZodTypeAny, "passthrough">[];
                } & {
                    [k: string]: unknown;
                };
            } | {
                continuous: {
                    max_label: {
                        translations: {
                            textContent: {
                                id: string;
                                primaryLocale: string;
                                format: "plain" | "markdown" | "rich";
                            } & {
                                [k: string]: unknown;
                            };
                            textTranslations: z.objectOutputType<{
                                aiGenerated: z.ZodBoolean;
                                content: z.ZodString;
                                contentId: z.ZodString;
                                id: z.ZodString;
                                locale: z.ZodString;
                                requiresValidation: z.ZodBoolean;
                            }, z.ZodTypeAny, "passthrough">[];
                        } & {
                            [k: string]: unknown;
                        };
                        localized: string;
                    } & {
                        [k: string]: unknown;
                    };
                    max_value: number;
                    min_label: {
                        translations: {
                            textContent: {
                                id: string;
                                primaryLocale: string;
                                format: "plain" | "markdown" | "rich";
                            } & {
                                [k: string]: unknown;
                            };
                            textTranslations: z.objectOutputType<{
                                aiGenerated: z.ZodBoolean;
                                content: z.ZodString;
                                contentId: z.ZodString;
                                id: z.ZodString;
                                locale: z.ZodString;
                                requiresValidation: z.ZodBoolean;
                            }, z.ZodTypeAny, "passthrough">[];
                        } & {
                            [k: string]: unknown;
                        };
                        localized: string;
                    } & {
                        [k: string]: unknown;
                    };
                    min_value: number;
                    sub_steps: number;
                } & {
                    [k: string]: unknown;
                };
            }, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        randomize_order: z.ZodBoolean;
        required_reviews: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        section_questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            text: z.ZodType<z.objectOutputType<{
                localized: z.ZodString;
                translations: z.ZodType<z.objectOutputType<{
                    textContent: z.ZodType<z.objectOutputType<{
                        format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                        id: z.ZodString;
                        primaryLocale: z.ZodString;
                    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                    textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                        aiGenerated: z.ZodBoolean;
                        content: z.ZodString;
                        contentId: z.ZodString;
                        id: z.ZodString;
                        locale: z.ZodString;
                        requiresValidation: z.ZodBoolean;
                    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            type: z.ZodType<"text" | {
                likert_scale: {
                    categories: z.objectOutputType<{
                        label: z.ZodType<z.objectOutputType<{
                            localized: z.ZodString;
                            translations: z.ZodType<z.objectOutputType<{
                                textContent: z.ZodType<z.objectOutputType<{
                                    format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                                    id: z.ZodString;
                                    primaryLocale: z.ZodString;
                                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                                textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                                    aiGenerated: z.ZodBoolean;
                                    content: z.ZodString;
                                    contentId: z.ZodString;
                                    id: z.ZodString;
                                    locale: z.ZodString;
                                    requiresValidation: z.ZodBoolean;
                                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
                            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                        value: z.ZodNumber;
                    }, z.ZodTypeAny, "passthrough">[];
                } & {
                    [k: string]: unknown;
                };
            } | {
                continuous: {
                    max_label: {
                        translations: {
                            textContent: {
                                id: string;
                                primaryLocale: string;
                                format: "plain" | "markdown" | "rich";
                            } & {
                                [k: string]: unknown;
                            };
                            textTranslations: z.objectOutputType<{
                                aiGenerated: z.ZodBoolean;
                                content: z.ZodString;
                                contentId: z.ZodString;
                                id: z.ZodString;
                                locale: z.ZodString;
                                requiresValidation: z.ZodBoolean;
                            }, z.ZodTypeAny, "passthrough">[];
                        } & {
                            [k: string]: unknown;
                        };
                        localized: string;
                    } & {
                        [k: string]: unknown;
                    };
                    max_value: number;
                    min_label: {
                        translations: {
                            textContent: {
                                id: string;
                                primaryLocale: string;
                                format: "plain" | "markdown" | "rich";
                            } & {
                                [k: string]: unknown;
                            };
                            textTranslations: z.objectOutputType<{
                                aiGenerated: z.ZodBoolean;
                                content: z.ZodString;
                                contentId: z.ZodString;
                                id: z.ZodString;
                                locale: z.ZodString;
                                requiresValidation: z.ZodBoolean;
                            }, z.ZodTypeAny, "passthrough">[];
                        } & {
                            [k: string]: unknown;
                        };
                        localized: string;
                    } & {
                        [k: string]: unknown;
                    };
                    min_value: number;
                    sub_steps: number;
                } & {
                    [k: string]: unknown;
                };
            }, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        type: z.ZodLiteral<"prioritization">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        follow_up_rounds_count: z.ZodNumber;
        root_questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            intent: z.ZodType<z.objectOutputType<{
                localized: z.ZodString;
                translations: z.ZodType<z.objectOutputType<{
                    textContent: z.ZodType<z.objectOutputType<{
                        format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                        id: z.ZodString;
                        primaryLocale: z.ZodString;
                    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                    textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                        aiGenerated: z.ZodBoolean;
                        content: z.ZodString;
                        contentId: z.ZodString;
                        id: z.ZodString;
                        locale: z.ZodString;
                        requiresValidation: z.ZodBoolean;
                    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            text: z.ZodType<z.objectOutputType<{
                localized: z.ZodString;
                translations: z.ZodType<z.objectOutputType<{
                    textContent: z.ZodType<z.objectOutputType<{
                        format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                        id: z.ZodString;
                        primaryLocale: z.ZodString;
                    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                    textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                        aiGenerated: z.ZodBoolean;
                        content: z.ZodString;
                        contentId: z.ZodString;
                        id: z.ZodString;
                        locale: z.ZodString;
                        requiresValidation: z.ZodBoolean;
                    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        topic: z.ZodType<z.objectOutputType<{
            localized: z.ZodString;
            translations: z.ZodType<z.objectOutputType<{
                textContent: z.ZodType<z.objectOutputType<{
                    format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                    id: z.ZodString;
                    primaryLocale: z.ZodString;
                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                    aiGenerated: z.ZodBoolean;
                    content: z.ZodString;
                    contentId: z.ZodString;
                    id: z.ZodString;
                    locale: z.ZodString;
                    requiresValidation: z.ZodBoolean;
                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        type: z.ZodLiteral<"thinkingspace">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
    translations: z.ZodType<z.objectOutputType<{
        description: z.ZodType<z.objectOutputType<{
            textContent: z.ZodType<z.objectOutputType<{
                format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                id: z.ZodString;
                primaryLocale: z.ZodString;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                aiGenerated: z.ZodBoolean;
                content: z.ZodString;
                contentId: z.ZodString;
                id: z.ZodString;
                locale: z.ZodString;
                requiresValidation: z.ZodBoolean;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        name: z.ZodType<z.objectOutputType<{
            textContent: z.ZodType<z.objectOutputType<{
                format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                id: z.ZodString;
                primaryLocale: z.ZodString;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                aiGenerated: z.ZodBoolean;
                content: z.ZodString;
                contentId: z.ZodString;
                id: z.ZodString;
                locale: z.ZodString;
                requiresValidation: z.ZodBoolean;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    workflowId: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type WorkflowStepWithTranslationsDto = z.infer<typeof _WorkflowStepWithTranslationsDto>;
export declare const WorkflowStepWithTranslationsDto: z.ZodType<WorkflowStepWithTranslationsDto, z.ZodTypeDef, any>;
declare const _LocalizedCategory: z.ZodObject<{
    label: z.ZodString;
    value: z.ZodNumber;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    label: z.ZodString;
    value: z.ZodNumber;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    label: z.ZodString;
    value: z.ZodNumber;
}, z.ZodTypeAny, "passthrough">>;
export type LocalizedCategory = z.infer<typeof _LocalizedCategory>;
export declare const LocalizedCategory: z.ZodType<LocalizedCategory, z.ZodTypeDef, any>;
declare const _LocalizedQuestionType: z.ZodUnion<[z.ZodLiteral<"text">, z.ZodObject<{
    likert_scale: z.ZodObject<{
        categories: z.ZodArray<z.ZodType<z.objectOutputType<{
            label: z.ZodString;
            value: z.ZodNumber;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    }, "passthrough", z.ZodTypeAny, z.objectOutputType<{
        categories: z.ZodArray<z.ZodType<z.objectOutputType<{
            label: z.ZodString;
            value: z.ZodNumber;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    }, z.ZodTypeAny, "passthrough">, z.objectInputType<{
        categories: z.ZodArray<z.ZodType<z.objectOutputType<{
            label: z.ZodString;
            value: z.ZodNumber;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    }, z.ZodTypeAny, "passthrough">>;
}, "strip", z.ZodTypeAny, {
    likert_scale: {
        categories: z.objectOutputType<{
            label: z.ZodString;
            value: z.ZodNumber;
        }, z.ZodTypeAny, "passthrough">[];
    } & {
        [k: string]: unknown;
    };
}, {
    likert_scale: {
        categories: any[];
    } & {
        [k: string]: unknown;
    };
}>, z.ZodObject<{
    continuous: z.ZodObject<{
        max_label: z.ZodString;
        max_value: z.ZodNumber;
        min_label: z.ZodString;
        min_value: z.ZodNumber;
        sub_steps: z.ZodNumber;
    }, "passthrough", z.ZodTypeAny, z.objectOutputType<{
        max_label: z.ZodString;
        max_value: z.ZodNumber;
        min_label: z.ZodString;
        min_value: z.ZodNumber;
        sub_steps: z.ZodNumber;
    }, z.ZodTypeAny, "passthrough">, z.objectInputType<{
        max_label: z.ZodString;
        max_value: z.ZodNumber;
        min_label: z.ZodString;
        min_value: z.ZodNumber;
        sub_steps: z.ZodNumber;
    }, z.ZodTypeAny, "passthrough">>;
}, "strip", z.ZodTypeAny, {
    continuous: {
        max_label: string;
        max_value: number;
        min_label: string;
        min_value: number;
        sub_steps: number;
    } & {
        [k: string]: unknown;
    };
}, {
    continuous: {
        max_label: string;
        max_value: number;
        min_label: string;
        min_value: number;
        sub_steps: number;
    } & {
        [k: string]: unknown;
    };
}>]>;
export type LocalizedQuestionType = z.infer<typeof _LocalizedQuestionType>;
export declare const LocalizedQuestionType: z.ZodType<LocalizedQuestionType, z.ZodTypeDef, any>;
declare const _LocalizedQuestion: z.ZodObject<{
    id: z.ZodString;
    text: z.ZodString;
    type: z.ZodType<"text" | {
        likert_scale: {
            categories: z.objectOutputType<{
                label: z.ZodString;
                value: z.ZodNumber;
            }, z.ZodTypeAny, "passthrough">[];
        } & {
            [k: string]: unknown;
        };
    } | {
        continuous: {
            max_label: string;
            max_value: number;
            min_label: string;
            min_value: number;
            sub_steps: number;
        } & {
            [k: string]: unknown;
        };
    }, z.ZodTypeDef, any>;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    id: z.ZodString;
    text: z.ZodString;
    type: z.ZodType<"text" | {
        likert_scale: {
            categories: z.objectOutputType<{
                label: z.ZodString;
                value: z.ZodNumber;
            }, z.ZodTypeAny, "passthrough">[];
        } & {
            [k: string]: unknown;
        };
    } | {
        continuous: {
            max_label: string;
            max_value: number;
            min_label: string;
            min_value: number;
            sub_steps: number;
        } & {
            [k: string]: unknown;
        };
    }, z.ZodTypeDef, any>;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    id: z.ZodString;
    text: z.ZodString;
    type: z.ZodType<"text" | {
        likert_scale: {
            categories: z.objectOutputType<{
                label: z.ZodString;
                value: z.ZodNumber;
            }, z.ZodTypeAny, "passthrough">[];
        } & {
            [k: string]: unknown;
        };
    } | {
        continuous: {
            max_label: string;
            max_value: number;
            min_label: string;
            min_value: number;
            sub_steps: number;
        } & {
            [k: string]: unknown;
        };
    }, z.ZodTypeDef, any>;
}, z.ZodTypeAny, "passthrough">>;
export type LocalizedQuestion = z.infer<typeof _LocalizedQuestion>;
export declare const LocalizedQuestion: z.ZodType<LocalizedQuestion, z.ZodTypeDef, any>;
declare const _LocalizedThinkingSpaceQuestion: z.ZodObject<{
    id: z.ZodString;
    intent: z.ZodString;
    text: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    id: z.ZodString;
    intent: z.ZodString;
    text: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    id: z.ZodString;
    intent: z.ZodString;
    text: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type LocalizedThinkingSpaceQuestion = z.infer<typeof _LocalizedThinkingSpaceQuestion>;
export declare const LocalizedThinkingSpaceQuestion: z.ZodType<LocalizedThinkingSpaceQuestion, z.ZodTypeDef, any>;
declare const _LocalizedToolConfig: z.ZodUnion<[z.ZodObject<{
    admin_password: z.ZodString;
    admin_user: z.ZodString;
    description: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
    is_active: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
    label_seeds_as_conversation_starter: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
    poll_id: z.ZodString;
    required_votes: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
    server_url: z.ZodString;
    show_remaining_statements: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
    strict_moderation: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
    topic: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
    type: z.ZodLiteral<"polis">;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    admin_password: z.ZodString;
    admin_user: z.ZodString;
    description: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
    is_active: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
    label_seeds_as_conversation_starter: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
    poll_id: z.ZodString;
    required_votes: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
    server_url: z.ZodString;
    show_remaining_statements: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
    strict_moderation: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
    topic: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
    type: z.ZodLiteral<"polis">;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    admin_password: z.ZodString;
    admin_user: z.ZodString;
    description: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
    is_active: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
    label_seeds_as_conversation_starter: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
    poll_id: z.ZodString;
    required_votes: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
    server_url: z.ZodString;
    show_remaining_statements: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
    strict_moderation: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
    topic: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
    type: z.ZodLiteral<"polis">;
}, z.ZodTypeAny, "passthrough">>, z.ZodObject<{
    pages: z.ZodArray<z.ZodType<z.objectOutputType<{
        text_content_id: z.ZodString;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        content: z.ZodString;
        type: z.ZodLiteral<"markdown">;
    }, z.ZodTypeAny, "passthrough">[], z.ZodTypeDef, any>, "many">;
    type: z.ZodLiteral<"learn">;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    pages: z.ZodArray<z.ZodType<z.objectOutputType<{
        text_content_id: z.ZodString;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        content: z.ZodString;
        type: z.ZodLiteral<"markdown">;
    }, z.ZodTypeAny, "passthrough">[], z.ZodTypeDef, any>, "many">;
    type: z.ZodLiteral<"learn">;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    pages: z.ZodArray<z.ZodType<z.objectOutputType<{
        text_content_id: z.ZodString;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        content: z.ZodString;
        type: z.ZodLiteral<"markdown">;
    }, z.ZodTypeAny, "passthrough">[], z.ZodTypeDef, any>, "many">;
    type: z.ZodLiteral<"learn">;
}, z.ZodTypeAny, "passthrough">>, z.ZodObject<{
    admin_password: z.ZodString;
    admin_user: z.ZodString;
    project_id: z.ZodString;
    server_url: z.ZodDefault<z.ZodOptional<z.ZodString>>;
    survey_id: z.ZodString;
    survey_url: z.ZodString;
    type: z.ZodLiteral<"heyform">;
    workspace_id: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    admin_password: z.ZodString;
    admin_user: z.ZodString;
    project_id: z.ZodString;
    server_url: z.ZodDefault<z.ZodOptional<z.ZodString>>;
    survey_id: z.ZodString;
    survey_url: z.ZodString;
    type: z.ZodLiteral<"heyform">;
    workspace_id: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    admin_password: z.ZodString;
    admin_user: z.ZodString;
    project_id: z.ZodString;
    server_url: z.ZodDefault<z.ZodOptional<z.ZodString>>;
    survey_id: z.ZodString;
    survey_url: z.ZodString;
    type: z.ZodLiteral<"heyform">;
    workspace_id: z.ZodString;
}, z.ZodTypeAny, "passthrough">>, z.ZodObject<{
    max_time: z.ZodNumber;
    to_see: z.ZodNumber;
    type: z.ZodLiteral<"stories">;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    max_time: z.ZodNumber;
    to_see: z.ZodNumber;
    type: z.ZodLiteral<"stories">;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    max_time: z.ZodNumber;
    to_see: z.ZodNumber;
    type: z.ZodLiteral<"stories">;
}, z.ZodTypeAny, "passthrough">>, z.ZodObject<{
    topic: z.ZodString;
    type: z.ZodLiteral<"elicitationbot">;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    topic: z.ZodString;
    type: z.ZodLiteral<"elicitationbot">;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    topic: z.ZodString;
    type: z.ZodLiteral<"elicitationbot">;
}, z.ZodTypeAny, "passthrough">>, z.ZodObject<{
    alignment_question_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    questions: z.ZodArray<z.ZodType<z.objectOutputType<{
        id: z.ZodString;
        text: z.ZodString;
        type: z.ZodType<"text" | {
            likert_scale: {
                categories: z.objectOutputType<{
                    label: z.ZodString;
                    value: z.ZodNumber;
                }, z.ZodTypeAny, "passthrough">[];
            } & {
                [k: string]: unknown;
            };
        } | {
            continuous: {
                max_label: string;
                max_value: number;
                min_label: string;
                min_value: number;
                sub_steps: number;
            } & {
                [k: string]: unknown;
            };
        }, z.ZodTypeDef, any>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    randomize_order: z.ZodBoolean;
    required_reviews: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
    section_questions: z.ZodArray<z.ZodType<z.objectOutputType<{
        id: z.ZodString;
        text: z.ZodString;
        type: z.ZodType<"text" | {
            likert_scale: {
                categories: z.objectOutputType<{
                    label: z.ZodString;
                    value: z.ZodNumber;
                }, z.ZodTypeAny, "passthrough">[];
            } & {
                [k: string]: unknown;
            };
        } | {
            continuous: {
                max_label: string;
                max_value: number;
                min_label: string;
                min_value: number;
                sub_steps: number;
            } & {
                [k: string]: unknown;
            };
        }, z.ZodTypeDef, any>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    type: z.ZodLiteral<"prioritization">;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    alignment_question_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    questions: z.ZodArray<z.ZodType<z.objectOutputType<{
        id: z.ZodString;
        text: z.ZodString;
        type: z.ZodType<"text" | {
            likert_scale: {
                categories: z.objectOutputType<{
                    label: z.ZodString;
                    value: z.ZodNumber;
                }, z.ZodTypeAny, "passthrough">[];
            } & {
                [k: string]: unknown;
            };
        } | {
            continuous: {
                max_label: string;
                max_value: number;
                min_label: string;
                min_value: number;
                sub_steps: number;
            } & {
                [k: string]: unknown;
            };
        }, z.ZodTypeDef, any>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    randomize_order: z.ZodBoolean;
    required_reviews: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
    section_questions: z.ZodArray<z.ZodType<z.objectOutputType<{
        id: z.ZodString;
        text: z.ZodString;
        type: z.ZodType<"text" | {
            likert_scale: {
                categories: z.objectOutputType<{
                    label: z.ZodString;
                    value: z.ZodNumber;
                }, z.ZodTypeAny, "passthrough">[];
            } & {
                [k: string]: unknown;
            };
        } | {
            continuous: {
                max_label: string;
                max_value: number;
                min_label: string;
                min_value: number;
                sub_steps: number;
            } & {
                [k: string]: unknown;
            };
        }, z.ZodTypeDef, any>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    type: z.ZodLiteral<"prioritization">;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    alignment_question_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    questions: z.ZodArray<z.ZodType<z.objectOutputType<{
        id: z.ZodString;
        text: z.ZodString;
        type: z.ZodType<"text" | {
            likert_scale: {
                categories: z.objectOutputType<{
                    label: z.ZodString;
                    value: z.ZodNumber;
                }, z.ZodTypeAny, "passthrough">[];
            } & {
                [k: string]: unknown;
            };
        } | {
            continuous: {
                max_label: string;
                max_value: number;
                min_label: string;
                min_value: number;
                sub_steps: number;
            } & {
                [k: string]: unknown;
            };
        }, z.ZodTypeDef, any>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    randomize_order: z.ZodBoolean;
    required_reviews: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
    section_questions: z.ZodArray<z.ZodType<z.objectOutputType<{
        id: z.ZodString;
        text: z.ZodString;
        type: z.ZodType<"text" | {
            likert_scale: {
                categories: z.objectOutputType<{
                    label: z.ZodString;
                    value: z.ZodNumber;
                }, z.ZodTypeAny, "passthrough">[];
            } & {
                [k: string]: unknown;
            };
        } | {
            continuous: {
                max_label: string;
                max_value: number;
                min_label: string;
                min_value: number;
                sub_steps: number;
            } & {
                [k: string]: unknown;
            };
        }, z.ZodTypeDef, any>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    type: z.ZodLiteral<"prioritization">;
}, z.ZodTypeAny, "passthrough">>, z.ZodObject<{
    follow_up_rounds_count: z.ZodNumber;
    root_questions: z.ZodArray<z.ZodType<z.objectOutputType<{
        id: z.ZodString;
        intent: z.ZodString;
        text: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    topic: z.ZodString;
    type: z.ZodLiteral<"thinkingspace">;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    follow_up_rounds_count: z.ZodNumber;
    root_questions: z.ZodArray<z.ZodType<z.objectOutputType<{
        id: z.ZodString;
        intent: z.ZodString;
        text: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    topic: z.ZodString;
    type: z.ZodLiteral<"thinkingspace">;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    follow_up_rounds_count: z.ZodNumber;
    root_questions: z.ZodArray<z.ZodType<z.objectOutputType<{
        id: z.ZodString;
        intent: z.ZodString;
        text: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    topic: z.ZodString;
    type: z.ZodLiteral<"thinkingspace">;
}, z.ZodTypeAny, "passthrough">>]>;
export type LocalizedToolConfig = z.infer<typeof _LocalizedToolConfig>;
export declare const LocalizedToolConfig: z.ZodType<LocalizedToolConfig, z.ZodTypeDef, any>;
declare const _ProgressStatus: z.ZodEnum<["not_started", "in_progress", "done"]>;
export type ProgressStatus = z.infer<typeof _ProgressStatus>;
export declare const ProgressStatus: z.ZodType<ProgressStatus, z.ZodTypeDef, any>;
declare const _LocalizedWorkflowStepWithProgressDto: z.ZodObject<{
    activationRule: z.ZodType<"manual", z.ZodTypeDef, any>;
    canRevisit: z.ZodBoolean;
    description: z.ZodString;
    id: z.ZodString;
    isOffline: z.ZodBoolean;
    name: z.ZodString;
    previewToolConfig: z.ZodType<z.objectOutputType<{
        admin_password: z.ZodString;
        admin_user: z.ZodString;
        description: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
        is_active: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
        label_seeds_as_conversation_starter: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
        poll_id: z.ZodString;
        required_votes: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        server_url: z.ZodString;
        show_remaining_statements: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
        strict_moderation: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
        topic: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
        type: z.ZodLiteral<"polis">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        pages: z.ZodArray<z.ZodType<z.objectOutputType<{
            text_content_id: z.ZodString;
        }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
            content: z.ZodString;
            type: z.ZodLiteral<"markdown">;
        }, z.ZodTypeAny, "passthrough">[], z.ZodTypeDef, any>, "many">;
        type: z.ZodLiteral<"learn">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        admin_password: z.ZodString;
        admin_user: z.ZodString;
        project_id: z.ZodString;
        server_url: z.ZodDefault<z.ZodOptional<z.ZodString>>;
        survey_id: z.ZodString;
        survey_url: z.ZodString;
        type: z.ZodLiteral<"heyform">;
        workspace_id: z.ZodString;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        max_time: z.ZodNumber;
        to_see: z.ZodNumber;
        type: z.ZodLiteral<"stories">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        topic: z.ZodString;
        type: z.ZodLiteral<"elicitationbot">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        alignment_question_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            text: z.ZodString;
            type: z.ZodType<"text" | {
                likert_scale: {
                    categories: z.objectOutputType<{
                        label: z.ZodString;
                        value: z.ZodNumber;
                    }, z.ZodTypeAny, "passthrough">[];
                } & {
                    [k: string]: unknown;
                };
            } | {
                continuous: {
                    max_label: string;
                    max_value: number;
                    min_label: string;
                    min_value: number;
                    sub_steps: number;
                } & {
                    [k: string]: unknown;
                };
            }, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        randomize_order: z.ZodBoolean;
        required_reviews: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        section_questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            text: z.ZodString;
            type: z.ZodType<"text" | {
                likert_scale: {
                    categories: z.objectOutputType<{
                        label: z.ZodString;
                        value: z.ZodNumber;
                    }, z.ZodTypeAny, "passthrough">[];
                } & {
                    [k: string]: unknown;
                };
            } | {
                continuous: {
                    max_label: string;
                    max_value: number;
                    min_label: string;
                    min_value: number;
                    sub_steps: number;
                } & {
                    [k: string]: unknown;
                };
            }, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        type: z.ZodLiteral<"prioritization">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        follow_up_rounds_count: z.ZodNumber;
        root_questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            intent: z.ZodString;
            text: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        topic: z.ZodString;
        type: z.ZodLiteral<"thinkingspace">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    progressStatus: z.ZodType<"not_started" | "in_progress" | "done", z.ZodTypeDef, any>;
    requestUserSharePermission: z.ZodBoolean;
    required: z.ZodBoolean;
    stepOrder: z.ZodNumber;
    toolConfig: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
        admin_password: z.ZodString;
        admin_user: z.ZodString;
        description: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
        is_active: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
        label_seeds_as_conversation_starter: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
        poll_id: z.ZodString;
        required_votes: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        server_url: z.ZodString;
        show_remaining_statements: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
        strict_moderation: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
        topic: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
        type: z.ZodLiteral<"polis">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        pages: z.ZodArray<z.ZodType<z.objectOutputType<{
            text_content_id: z.ZodString;
        }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
            content: z.ZodString;
            type: z.ZodLiteral<"markdown">;
        }, z.ZodTypeAny, "passthrough">[], z.ZodTypeDef, any>, "many">;
        type: z.ZodLiteral<"learn">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        admin_password: z.ZodString;
        admin_user: z.ZodString;
        project_id: z.ZodString;
        server_url: z.ZodDefault<z.ZodOptional<z.ZodString>>;
        survey_id: z.ZodString;
        survey_url: z.ZodString;
        type: z.ZodLiteral<"heyform">;
        workspace_id: z.ZodString;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        max_time: z.ZodNumber;
        to_see: z.ZodNumber;
        type: z.ZodLiteral<"stories">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        topic: z.ZodString;
        type: z.ZodLiteral<"elicitationbot">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        alignment_question_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            text: z.ZodString;
            type: z.ZodType<"text" | {
                likert_scale: {
                    categories: z.objectOutputType<{
                        label: z.ZodString;
                        value: z.ZodNumber;
                    }, z.ZodTypeAny, "passthrough">[];
                } & {
                    [k: string]: unknown;
                };
            } | {
                continuous: {
                    max_label: string;
                    max_value: number;
                    min_label: string;
                    min_value: number;
                    sub_steps: number;
                } & {
                    [k: string]: unknown;
                };
            }, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        randomize_order: z.ZodBoolean;
        required_reviews: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        section_questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            text: z.ZodString;
            type: z.ZodType<"text" | {
                likert_scale: {
                    categories: z.objectOutputType<{
                        label: z.ZodString;
                        value: z.ZodNumber;
                    }, z.ZodTypeAny, "passthrough">[];
                } & {
                    [k: string]: unknown;
                };
            } | {
                continuous: {
                    max_label: string;
                    max_value: number;
                    min_label: string;
                    min_value: number;
                    sub_steps: number;
                } & {
                    [k: string]: unknown;
                };
            }, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        type: z.ZodLiteral<"prioritization">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        follow_up_rounds_count: z.ZodNumber;
        root_questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            intent: z.ZodString;
            text: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        topic: z.ZodString;
        type: z.ZodLiteral<"thinkingspace">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
    workflowId: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    activationRule: z.ZodType<"manual", z.ZodTypeDef, any>;
    canRevisit: z.ZodBoolean;
    description: z.ZodString;
    id: z.ZodString;
    isOffline: z.ZodBoolean;
    name: z.ZodString;
    previewToolConfig: z.ZodType<z.objectOutputType<{
        admin_password: z.ZodString;
        admin_user: z.ZodString;
        description: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
        is_active: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
        label_seeds_as_conversation_starter: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
        poll_id: z.ZodString;
        required_votes: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        server_url: z.ZodString;
        show_remaining_statements: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
        strict_moderation: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
        topic: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
        type: z.ZodLiteral<"polis">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        pages: z.ZodArray<z.ZodType<z.objectOutputType<{
            text_content_id: z.ZodString;
        }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
            content: z.ZodString;
            type: z.ZodLiteral<"markdown">;
        }, z.ZodTypeAny, "passthrough">[], z.ZodTypeDef, any>, "many">;
        type: z.ZodLiteral<"learn">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        admin_password: z.ZodString;
        admin_user: z.ZodString;
        project_id: z.ZodString;
        server_url: z.ZodDefault<z.ZodOptional<z.ZodString>>;
        survey_id: z.ZodString;
        survey_url: z.ZodString;
        type: z.ZodLiteral<"heyform">;
        workspace_id: z.ZodString;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        max_time: z.ZodNumber;
        to_see: z.ZodNumber;
        type: z.ZodLiteral<"stories">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        topic: z.ZodString;
        type: z.ZodLiteral<"elicitationbot">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        alignment_question_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            text: z.ZodString;
            type: z.ZodType<"text" | {
                likert_scale: {
                    categories: z.objectOutputType<{
                        label: z.ZodString;
                        value: z.ZodNumber;
                    }, z.ZodTypeAny, "passthrough">[];
                } & {
                    [k: string]: unknown;
                };
            } | {
                continuous: {
                    max_label: string;
                    max_value: number;
                    min_label: string;
                    min_value: number;
                    sub_steps: number;
                } & {
                    [k: string]: unknown;
                };
            }, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        randomize_order: z.ZodBoolean;
        required_reviews: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        section_questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            text: z.ZodString;
            type: z.ZodType<"text" | {
                likert_scale: {
                    categories: z.objectOutputType<{
                        label: z.ZodString;
                        value: z.ZodNumber;
                    }, z.ZodTypeAny, "passthrough">[];
                } & {
                    [k: string]: unknown;
                };
            } | {
                continuous: {
                    max_label: string;
                    max_value: number;
                    min_label: string;
                    min_value: number;
                    sub_steps: number;
                } & {
                    [k: string]: unknown;
                };
            }, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        type: z.ZodLiteral<"prioritization">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        follow_up_rounds_count: z.ZodNumber;
        root_questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            intent: z.ZodString;
            text: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        topic: z.ZodString;
        type: z.ZodLiteral<"thinkingspace">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    progressStatus: z.ZodType<"not_started" | "in_progress" | "done", z.ZodTypeDef, any>;
    requestUserSharePermission: z.ZodBoolean;
    required: z.ZodBoolean;
    stepOrder: z.ZodNumber;
    toolConfig: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
        admin_password: z.ZodString;
        admin_user: z.ZodString;
        description: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
        is_active: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
        label_seeds_as_conversation_starter: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
        poll_id: z.ZodString;
        required_votes: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        server_url: z.ZodString;
        show_remaining_statements: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
        strict_moderation: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
        topic: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
        type: z.ZodLiteral<"polis">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        pages: z.ZodArray<z.ZodType<z.objectOutputType<{
            text_content_id: z.ZodString;
        }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
            content: z.ZodString;
            type: z.ZodLiteral<"markdown">;
        }, z.ZodTypeAny, "passthrough">[], z.ZodTypeDef, any>, "many">;
        type: z.ZodLiteral<"learn">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        admin_password: z.ZodString;
        admin_user: z.ZodString;
        project_id: z.ZodString;
        server_url: z.ZodDefault<z.ZodOptional<z.ZodString>>;
        survey_id: z.ZodString;
        survey_url: z.ZodString;
        type: z.ZodLiteral<"heyform">;
        workspace_id: z.ZodString;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        max_time: z.ZodNumber;
        to_see: z.ZodNumber;
        type: z.ZodLiteral<"stories">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        topic: z.ZodString;
        type: z.ZodLiteral<"elicitationbot">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        alignment_question_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            text: z.ZodString;
            type: z.ZodType<"text" | {
                likert_scale: {
                    categories: z.objectOutputType<{
                        label: z.ZodString;
                        value: z.ZodNumber;
                    }, z.ZodTypeAny, "passthrough">[];
                } & {
                    [k: string]: unknown;
                };
            } | {
                continuous: {
                    max_label: string;
                    max_value: number;
                    min_label: string;
                    min_value: number;
                    sub_steps: number;
                } & {
                    [k: string]: unknown;
                };
            }, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        randomize_order: z.ZodBoolean;
        required_reviews: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        section_questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            text: z.ZodString;
            type: z.ZodType<"text" | {
                likert_scale: {
                    categories: z.objectOutputType<{
                        label: z.ZodString;
                        value: z.ZodNumber;
                    }, z.ZodTypeAny, "passthrough">[];
                } & {
                    [k: string]: unknown;
                };
            } | {
                continuous: {
                    max_label: string;
                    max_value: number;
                    min_label: string;
                    min_value: number;
                    sub_steps: number;
                } & {
                    [k: string]: unknown;
                };
            }, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        type: z.ZodLiteral<"prioritization">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        follow_up_rounds_count: z.ZodNumber;
        root_questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            intent: z.ZodString;
            text: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        topic: z.ZodString;
        type: z.ZodLiteral<"thinkingspace">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
    workflowId: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    activationRule: z.ZodType<"manual", z.ZodTypeDef, any>;
    canRevisit: z.ZodBoolean;
    description: z.ZodString;
    id: z.ZodString;
    isOffline: z.ZodBoolean;
    name: z.ZodString;
    previewToolConfig: z.ZodType<z.objectOutputType<{
        admin_password: z.ZodString;
        admin_user: z.ZodString;
        description: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
        is_active: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
        label_seeds_as_conversation_starter: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
        poll_id: z.ZodString;
        required_votes: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        server_url: z.ZodString;
        show_remaining_statements: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
        strict_moderation: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
        topic: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
        type: z.ZodLiteral<"polis">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        pages: z.ZodArray<z.ZodType<z.objectOutputType<{
            text_content_id: z.ZodString;
        }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
            content: z.ZodString;
            type: z.ZodLiteral<"markdown">;
        }, z.ZodTypeAny, "passthrough">[], z.ZodTypeDef, any>, "many">;
        type: z.ZodLiteral<"learn">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        admin_password: z.ZodString;
        admin_user: z.ZodString;
        project_id: z.ZodString;
        server_url: z.ZodDefault<z.ZodOptional<z.ZodString>>;
        survey_id: z.ZodString;
        survey_url: z.ZodString;
        type: z.ZodLiteral<"heyform">;
        workspace_id: z.ZodString;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        max_time: z.ZodNumber;
        to_see: z.ZodNumber;
        type: z.ZodLiteral<"stories">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        topic: z.ZodString;
        type: z.ZodLiteral<"elicitationbot">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        alignment_question_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            text: z.ZodString;
            type: z.ZodType<"text" | {
                likert_scale: {
                    categories: z.objectOutputType<{
                        label: z.ZodString;
                        value: z.ZodNumber;
                    }, z.ZodTypeAny, "passthrough">[];
                } & {
                    [k: string]: unknown;
                };
            } | {
                continuous: {
                    max_label: string;
                    max_value: number;
                    min_label: string;
                    min_value: number;
                    sub_steps: number;
                } & {
                    [k: string]: unknown;
                };
            }, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        randomize_order: z.ZodBoolean;
        required_reviews: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        section_questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            text: z.ZodString;
            type: z.ZodType<"text" | {
                likert_scale: {
                    categories: z.objectOutputType<{
                        label: z.ZodString;
                        value: z.ZodNumber;
                    }, z.ZodTypeAny, "passthrough">[];
                } & {
                    [k: string]: unknown;
                };
            } | {
                continuous: {
                    max_label: string;
                    max_value: number;
                    min_label: string;
                    min_value: number;
                    sub_steps: number;
                } & {
                    [k: string]: unknown;
                };
            }, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        type: z.ZodLiteral<"prioritization">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        follow_up_rounds_count: z.ZodNumber;
        root_questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            intent: z.ZodString;
            text: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        topic: z.ZodString;
        type: z.ZodLiteral<"thinkingspace">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    progressStatus: z.ZodType<"not_started" | "in_progress" | "done", z.ZodTypeDef, any>;
    requestUserSharePermission: z.ZodBoolean;
    required: z.ZodBoolean;
    stepOrder: z.ZodNumber;
    toolConfig: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
        admin_password: z.ZodString;
        admin_user: z.ZodString;
        description: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
        is_active: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
        label_seeds_as_conversation_starter: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
        poll_id: z.ZodString;
        required_votes: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        server_url: z.ZodString;
        show_remaining_statements: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
        strict_moderation: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
        topic: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
        type: z.ZodLiteral<"polis">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        pages: z.ZodArray<z.ZodType<z.objectOutputType<{
            text_content_id: z.ZodString;
        }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
            content: z.ZodString;
            type: z.ZodLiteral<"markdown">;
        }, z.ZodTypeAny, "passthrough">[], z.ZodTypeDef, any>, "many">;
        type: z.ZodLiteral<"learn">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        admin_password: z.ZodString;
        admin_user: z.ZodString;
        project_id: z.ZodString;
        server_url: z.ZodDefault<z.ZodOptional<z.ZodString>>;
        survey_id: z.ZodString;
        survey_url: z.ZodString;
        type: z.ZodLiteral<"heyform">;
        workspace_id: z.ZodString;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        max_time: z.ZodNumber;
        to_see: z.ZodNumber;
        type: z.ZodLiteral<"stories">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        topic: z.ZodString;
        type: z.ZodLiteral<"elicitationbot">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        alignment_question_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            text: z.ZodString;
            type: z.ZodType<"text" | {
                likert_scale: {
                    categories: z.objectOutputType<{
                        label: z.ZodString;
                        value: z.ZodNumber;
                    }, z.ZodTypeAny, "passthrough">[];
                } & {
                    [k: string]: unknown;
                };
            } | {
                continuous: {
                    max_label: string;
                    max_value: number;
                    min_label: string;
                    min_value: number;
                    sub_steps: number;
                } & {
                    [k: string]: unknown;
                };
            }, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        randomize_order: z.ZodBoolean;
        required_reviews: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        section_questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            text: z.ZodString;
            type: z.ZodType<"text" | {
                likert_scale: {
                    categories: z.objectOutputType<{
                        label: z.ZodString;
                        value: z.ZodNumber;
                    }, z.ZodTypeAny, "passthrough">[];
                } & {
                    [k: string]: unknown;
                };
            } | {
                continuous: {
                    max_label: string;
                    max_value: number;
                    min_label: string;
                    min_value: number;
                    sub_steps: number;
                } & {
                    [k: string]: unknown;
                };
            }, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        type: z.ZodLiteral<"prioritization">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        follow_up_rounds_count: z.ZodNumber;
        root_questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            intent: z.ZodString;
            text: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        topic: z.ZodString;
        type: z.ZodLiteral<"thinkingspace">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
    workflowId: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type LocalizedWorkflowStepWithProgressDto = z.infer<typeof _LocalizedWorkflowStepWithProgressDto>;
export declare const LocalizedWorkflowStepWithProgressDto: z.ZodType<LocalizedWorkflowStepWithProgressDto, z.ZodTypeDef, any>;
declare const _LocalizedWorkflowStepDto: z.ZodObject<{
    activationRule: z.ZodType<"manual", z.ZodTypeDef, any>;
    canRevisit: z.ZodBoolean;
    description: z.ZodString;
    id: z.ZodString;
    isOffline: z.ZodBoolean;
    name: z.ZodString;
    previewToolConfig: z.ZodType<z.objectOutputType<{
        admin_password: z.ZodString;
        admin_user: z.ZodString;
        description: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
        is_active: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
        label_seeds_as_conversation_starter: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
        poll_id: z.ZodString;
        required_votes: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        server_url: z.ZodString;
        show_remaining_statements: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
        strict_moderation: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
        topic: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
        type: z.ZodLiteral<"polis">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        pages: z.ZodArray<z.ZodType<z.objectOutputType<{
            text_content_id: z.ZodString;
        }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
            content: z.ZodString;
            type: z.ZodLiteral<"markdown">;
        }, z.ZodTypeAny, "passthrough">[], z.ZodTypeDef, any>, "many">;
        type: z.ZodLiteral<"learn">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        admin_password: z.ZodString;
        admin_user: z.ZodString;
        project_id: z.ZodString;
        server_url: z.ZodDefault<z.ZodOptional<z.ZodString>>;
        survey_id: z.ZodString;
        survey_url: z.ZodString;
        type: z.ZodLiteral<"heyform">;
        workspace_id: z.ZodString;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        max_time: z.ZodNumber;
        to_see: z.ZodNumber;
        type: z.ZodLiteral<"stories">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        topic: z.ZodString;
        type: z.ZodLiteral<"elicitationbot">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        alignment_question_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            text: z.ZodString;
            type: z.ZodType<"text" | {
                likert_scale: {
                    categories: z.objectOutputType<{
                        label: z.ZodString;
                        value: z.ZodNumber;
                    }, z.ZodTypeAny, "passthrough">[];
                } & {
                    [k: string]: unknown;
                };
            } | {
                continuous: {
                    max_label: string;
                    max_value: number;
                    min_label: string;
                    min_value: number;
                    sub_steps: number;
                } & {
                    [k: string]: unknown;
                };
            }, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        randomize_order: z.ZodBoolean;
        required_reviews: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        section_questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            text: z.ZodString;
            type: z.ZodType<"text" | {
                likert_scale: {
                    categories: z.objectOutputType<{
                        label: z.ZodString;
                        value: z.ZodNumber;
                    }, z.ZodTypeAny, "passthrough">[];
                } & {
                    [k: string]: unknown;
                };
            } | {
                continuous: {
                    max_label: string;
                    max_value: number;
                    min_label: string;
                    min_value: number;
                    sub_steps: number;
                } & {
                    [k: string]: unknown;
                };
            }, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        type: z.ZodLiteral<"prioritization">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        follow_up_rounds_count: z.ZodNumber;
        root_questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            intent: z.ZodString;
            text: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        topic: z.ZodString;
        type: z.ZodLiteral<"thinkingspace">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    requestUserSharePermission: z.ZodBoolean;
    required: z.ZodBoolean;
    stepOrder: z.ZodNumber;
    toolConfig: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
        admin_password: z.ZodString;
        admin_user: z.ZodString;
        description: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
        is_active: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
        label_seeds_as_conversation_starter: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
        poll_id: z.ZodString;
        required_votes: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        server_url: z.ZodString;
        show_remaining_statements: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
        strict_moderation: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
        topic: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
        type: z.ZodLiteral<"polis">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        pages: z.ZodArray<z.ZodType<z.objectOutputType<{
            text_content_id: z.ZodString;
        }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
            content: z.ZodString;
            type: z.ZodLiteral<"markdown">;
        }, z.ZodTypeAny, "passthrough">[], z.ZodTypeDef, any>, "many">;
        type: z.ZodLiteral<"learn">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        admin_password: z.ZodString;
        admin_user: z.ZodString;
        project_id: z.ZodString;
        server_url: z.ZodDefault<z.ZodOptional<z.ZodString>>;
        survey_id: z.ZodString;
        survey_url: z.ZodString;
        type: z.ZodLiteral<"heyform">;
        workspace_id: z.ZodString;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        max_time: z.ZodNumber;
        to_see: z.ZodNumber;
        type: z.ZodLiteral<"stories">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        topic: z.ZodString;
        type: z.ZodLiteral<"elicitationbot">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        alignment_question_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            text: z.ZodString;
            type: z.ZodType<"text" | {
                likert_scale: {
                    categories: z.objectOutputType<{
                        label: z.ZodString;
                        value: z.ZodNumber;
                    }, z.ZodTypeAny, "passthrough">[];
                } & {
                    [k: string]: unknown;
                };
            } | {
                continuous: {
                    max_label: string;
                    max_value: number;
                    min_label: string;
                    min_value: number;
                    sub_steps: number;
                } & {
                    [k: string]: unknown;
                };
            }, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        randomize_order: z.ZodBoolean;
        required_reviews: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        section_questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            text: z.ZodString;
            type: z.ZodType<"text" | {
                likert_scale: {
                    categories: z.objectOutputType<{
                        label: z.ZodString;
                        value: z.ZodNumber;
                    }, z.ZodTypeAny, "passthrough">[];
                } & {
                    [k: string]: unknown;
                };
            } | {
                continuous: {
                    max_label: string;
                    max_value: number;
                    min_label: string;
                    min_value: number;
                    sub_steps: number;
                } & {
                    [k: string]: unknown;
                };
            }, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        type: z.ZodLiteral<"prioritization">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        follow_up_rounds_count: z.ZodNumber;
        root_questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            intent: z.ZodString;
            text: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        topic: z.ZodString;
        type: z.ZodLiteral<"thinkingspace">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
    workflowId: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    activationRule: z.ZodType<"manual", z.ZodTypeDef, any>;
    canRevisit: z.ZodBoolean;
    description: z.ZodString;
    id: z.ZodString;
    isOffline: z.ZodBoolean;
    name: z.ZodString;
    previewToolConfig: z.ZodType<z.objectOutputType<{
        admin_password: z.ZodString;
        admin_user: z.ZodString;
        description: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
        is_active: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
        label_seeds_as_conversation_starter: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
        poll_id: z.ZodString;
        required_votes: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        server_url: z.ZodString;
        show_remaining_statements: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
        strict_moderation: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
        topic: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
        type: z.ZodLiteral<"polis">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        pages: z.ZodArray<z.ZodType<z.objectOutputType<{
            text_content_id: z.ZodString;
        }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
            content: z.ZodString;
            type: z.ZodLiteral<"markdown">;
        }, z.ZodTypeAny, "passthrough">[], z.ZodTypeDef, any>, "many">;
        type: z.ZodLiteral<"learn">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        admin_password: z.ZodString;
        admin_user: z.ZodString;
        project_id: z.ZodString;
        server_url: z.ZodDefault<z.ZodOptional<z.ZodString>>;
        survey_id: z.ZodString;
        survey_url: z.ZodString;
        type: z.ZodLiteral<"heyform">;
        workspace_id: z.ZodString;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        max_time: z.ZodNumber;
        to_see: z.ZodNumber;
        type: z.ZodLiteral<"stories">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        topic: z.ZodString;
        type: z.ZodLiteral<"elicitationbot">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        alignment_question_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            text: z.ZodString;
            type: z.ZodType<"text" | {
                likert_scale: {
                    categories: z.objectOutputType<{
                        label: z.ZodString;
                        value: z.ZodNumber;
                    }, z.ZodTypeAny, "passthrough">[];
                } & {
                    [k: string]: unknown;
                };
            } | {
                continuous: {
                    max_label: string;
                    max_value: number;
                    min_label: string;
                    min_value: number;
                    sub_steps: number;
                } & {
                    [k: string]: unknown;
                };
            }, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        randomize_order: z.ZodBoolean;
        required_reviews: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        section_questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            text: z.ZodString;
            type: z.ZodType<"text" | {
                likert_scale: {
                    categories: z.objectOutputType<{
                        label: z.ZodString;
                        value: z.ZodNumber;
                    }, z.ZodTypeAny, "passthrough">[];
                } & {
                    [k: string]: unknown;
                };
            } | {
                continuous: {
                    max_label: string;
                    max_value: number;
                    min_label: string;
                    min_value: number;
                    sub_steps: number;
                } & {
                    [k: string]: unknown;
                };
            }, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        type: z.ZodLiteral<"prioritization">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        follow_up_rounds_count: z.ZodNumber;
        root_questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            intent: z.ZodString;
            text: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        topic: z.ZodString;
        type: z.ZodLiteral<"thinkingspace">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    requestUserSharePermission: z.ZodBoolean;
    required: z.ZodBoolean;
    stepOrder: z.ZodNumber;
    toolConfig: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
        admin_password: z.ZodString;
        admin_user: z.ZodString;
        description: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
        is_active: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
        label_seeds_as_conversation_starter: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
        poll_id: z.ZodString;
        required_votes: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        server_url: z.ZodString;
        show_remaining_statements: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
        strict_moderation: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
        topic: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
        type: z.ZodLiteral<"polis">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        pages: z.ZodArray<z.ZodType<z.objectOutputType<{
            text_content_id: z.ZodString;
        }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
            content: z.ZodString;
            type: z.ZodLiteral<"markdown">;
        }, z.ZodTypeAny, "passthrough">[], z.ZodTypeDef, any>, "many">;
        type: z.ZodLiteral<"learn">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        admin_password: z.ZodString;
        admin_user: z.ZodString;
        project_id: z.ZodString;
        server_url: z.ZodDefault<z.ZodOptional<z.ZodString>>;
        survey_id: z.ZodString;
        survey_url: z.ZodString;
        type: z.ZodLiteral<"heyform">;
        workspace_id: z.ZodString;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        max_time: z.ZodNumber;
        to_see: z.ZodNumber;
        type: z.ZodLiteral<"stories">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        topic: z.ZodString;
        type: z.ZodLiteral<"elicitationbot">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        alignment_question_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            text: z.ZodString;
            type: z.ZodType<"text" | {
                likert_scale: {
                    categories: z.objectOutputType<{
                        label: z.ZodString;
                        value: z.ZodNumber;
                    }, z.ZodTypeAny, "passthrough">[];
                } & {
                    [k: string]: unknown;
                };
            } | {
                continuous: {
                    max_label: string;
                    max_value: number;
                    min_label: string;
                    min_value: number;
                    sub_steps: number;
                } & {
                    [k: string]: unknown;
                };
            }, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        randomize_order: z.ZodBoolean;
        required_reviews: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        section_questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            text: z.ZodString;
            type: z.ZodType<"text" | {
                likert_scale: {
                    categories: z.objectOutputType<{
                        label: z.ZodString;
                        value: z.ZodNumber;
                    }, z.ZodTypeAny, "passthrough">[];
                } & {
                    [k: string]: unknown;
                };
            } | {
                continuous: {
                    max_label: string;
                    max_value: number;
                    min_label: string;
                    min_value: number;
                    sub_steps: number;
                } & {
                    [k: string]: unknown;
                };
            }, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        type: z.ZodLiteral<"prioritization">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        follow_up_rounds_count: z.ZodNumber;
        root_questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            intent: z.ZodString;
            text: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        topic: z.ZodString;
        type: z.ZodLiteral<"thinkingspace">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
    workflowId: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    activationRule: z.ZodType<"manual", z.ZodTypeDef, any>;
    canRevisit: z.ZodBoolean;
    description: z.ZodString;
    id: z.ZodString;
    isOffline: z.ZodBoolean;
    name: z.ZodString;
    previewToolConfig: z.ZodType<z.objectOutputType<{
        admin_password: z.ZodString;
        admin_user: z.ZodString;
        description: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
        is_active: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
        label_seeds_as_conversation_starter: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
        poll_id: z.ZodString;
        required_votes: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        server_url: z.ZodString;
        show_remaining_statements: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
        strict_moderation: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
        topic: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
        type: z.ZodLiteral<"polis">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        pages: z.ZodArray<z.ZodType<z.objectOutputType<{
            text_content_id: z.ZodString;
        }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
            content: z.ZodString;
            type: z.ZodLiteral<"markdown">;
        }, z.ZodTypeAny, "passthrough">[], z.ZodTypeDef, any>, "many">;
        type: z.ZodLiteral<"learn">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        admin_password: z.ZodString;
        admin_user: z.ZodString;
        project_id: z.ZodString;
        server_url: z.ZodDefault<z.ZodOptional<z.ZodString>>;
        survey_id: z.ZodString;
        survey_url: z.ZodString;
        type: z.ZodLiteral<"heyform">;
        workspace_id: z.ZodString;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        max_time: z.ZodNumber;
        to_see: z.ZodNumber;
        type: z.ZodLiteral<"stories">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        topic: z.ZodString;
        type: z.ZodLiteral<"elicitationbot">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        alignment_question_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            text: z.ZodString;
            type: z.ZodType<"text" | {
                likert_scale: {
                    categories: z.objectOutputType<{
                        label: z.ZodString;
                        value: z.ZodNumber;
                    }, z.ZodTypeAny, "passthrough">[];
                } & {
                    [k: string]: unknown;
                };
            } | {
                continuous: {
                    max_label: string;
                    max_value: number;
                    min_label: string;
                    min_value: number;
                    sub_steps: number;
                } & {
                    [k: string]: unknown;
                };
            }, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        randomize_order: z.ZodBoolean;
        required_reviews: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        section_questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            text: z.ZodString;
            type: z.ZodType<"text" | {
                likert_scale: {
                    categories: z.objectOutputType<{
                        label: z.ZodString;
                        value: z.ZodNumber;
                    }, z.ZodTypeAny, "passthrough">[];
                } & {
                    [k: string]: unknown;
                };
            } | {
                continuous: {
                    max_label: string;
                    max_value: number;
                    min_label: string;
                    min_value: number;
                    sub_steps: number;
                } & {
                    [k: string]: unknown;
                };
            }, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        type: z.ZodLiteral<"prioritization">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        follow_up_rounds_count: z.ZodNumber;
        root_questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            intent: z.ZodString;
            text: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        topic: z.ZodString;
        type: z.ZodLiteral<"thinkingspace">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    requestUserSharePermission: z.ZodBoolean;
    required: z.ZodBoolean;
    stepOrder: z.ZodNumber;
    toolConfig: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
        admin_password: z.ZodString;
        admin_user: z.ZodString;
        description: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
        is_active: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
        label_seeds_as_conversation_starter: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
        poll_id: z.ZodString;
        required_votes: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        server_url: z.ZodString;
        show_remaining_statements: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
        strict_moderation: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
        topic: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
        type: z.ZodLiteral<"polis">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        pages: z.ZodArray<z.ZodType<z.objectOutputType<{
            text_content_id: z.ZodString;
        }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
            content: z.ZodString;
            type: z.ZodLiteral<"markdown">;
        }, z.ZodTypeAny, "passthrough">[], z.ZodTypeDef, any>, "many">;
        type: z.ZodLiteral<"learn">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        admin_password: z.ZodString;
        admin_user: z.ZodString;
        project_id: z.ZodString;
        server_url: z.ZodDefault<z.ZodOptional<z.ZodString>>;
        survey_id: z.ZodString;
        survey_url: z.ZodString;
        type: z.ZodLiteral<"heyform">;
        workspace_id: z.ZodString;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        max_time: z.ZodNumber;
        to_see: z.ZodNumber;
        type: z.ZodLiteral<"stories">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        topic: z.ZodString;
        type: z.ZodLiteral<"elicitationbot">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        alignment_question_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            text: z.ZodString;
            type: z.ZodType<"text" | {
                likert_scale: {
                    categories: z.objectOutputType<{
                        label: z.ZodString;
                        value: z.ZodNumber;
                    }, z.ZodTypeAny, "passthrough">[];
                } & {
                    [k: string]: unknown;
                };
            } | {
                continuous: {
                    max_label: string;
                    max_value: number;
                    min_label: string;
                    min_value: number;
                    sub_steps: number;
                } & {
                    [k: string]: unknown;
                };
            }, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        randomize_order: z.ZodBoolean;
        required_reviews: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        section_questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            text: z.ZodString;
            type: z.ZodType<"text" | {
                likert_scale: {
                    categories: z.objectOutputType<{
                        label: z.ZodString;
                        value: z.ZodNumber;
                    }, z.ZodTypeAny, "passthrough">[];
                } & {
                    [k: string]: unknown;
                };
            } | {
                continuous: {
                    max_label: string;
                    max_value: number;
                    min_label: string;
                    min_value: number;
                    sub_steps: number;
                } & {
                    [k: string]: unknown;
                };
            }, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        type: z.ZodLiteral<"prioritization">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        follow_up_rounds_count: z.ZodNumber;
        root_questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            intent: z.ZodString;
            text: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        topic: z.ZodString;
        type: z.ZodLiteral<"thinkingspace">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
    workflowId: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type LocalizedWorkflowStepDto = z.infer<typeof _LocalizedWorkflowStepDto>;
export declare const LocalizedWorkflowStepDto: z.ZodType<LocalizedWorkflowStepDto, z.ZodTypeDef, any>;
declare const _WorkflowStepsListResponse: z.ZodUnion<[z.ZodArray<z.ZodType<z.objectOutputType<{
    activationRule: z.ZodType<"manual", z.ZodTypeDef, any>;
    canRevisit: z.ZodBoolean;
    description: z.ZodString;
    id: z.ZodString;
    isOffline: z.ZodBoolean;
    name: z.ZodString;
    previewToolConfig: z.ZodType<z.objectOutputType<{
        admin_password: z.ZodString;
        admin_user: z.ZodString;
        description: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
        is_active: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
        label_seeds_as_conversation_starter: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
        poll_id: z.ZodString;
        required_votes: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        server_url: z.ZodString;
        show_remaining_statements: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
        strict_moderation: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
        topic: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
        type: z.ZodLiteral<"polis">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        pages: z.ZodArray<z.ZodType<z.objectOutputType<{
            text_content_id: z.ZodString;
        }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
            content: z.ZodString;
            type: z.ZodLiteral<"markdown">;
        }, z.ZodTypeAny, "passthrough">[], z.ZodTypeDef, any>, "many">;
        type: z.ZodLiteral<"learn">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        admin_password: z.ZodString;
        admin_user: z.ZodString;
        project_id: z.ZodString;
        server_url: z.ZodDefault<z.ZodOptional<z.ZodString>>;
        survey_id: z.ZodString;
        survey_url: z.ZodString;
        type: z.ZodLiteral<"heyform">;
        workspace_id: z.ZodString;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        max_time: z.ZodNumber;
        to_see: z.ZodNumber;
        type: z.ZodLiteral<"stories">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        topic: z.ZodString;
        type: z.ZodLiteral<"elicitationbot">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        alignment_question_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            text: z.ZodType<z.objectOutputType<{
                localized: z.ZodString;
                translations: z.ZodType<z.objectOutputType<{
                    textContent: z.ZodType<z.objectOutputType<{
                        format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                        id: z.ZodString;
                        primaryLocale: z.ZodString;
                    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                    textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                        aiGenerated: z.ZodBoolean;
                        content: z.ZodString;
                        contentId: z.ZodString;
                        id: z.ZodString;
                        locale: z.ZodString;
                        requiresValidation: z.ZodBoolean;
                    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            type: z.ZodType<"text" | {
                likert_scale: {
                    categories: z.objectOutputType<{
                        label: z.ZodType<z.objectOutputType<{
                            localized: z.ZodString;
                            translations: z.ZodType<z.objectOutputType<{
                                textContent: z.ZodType<z.objectOutputType<{
                                    format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                                    id: z.ZodString;
                                    primaryLocale: z.ZodString;
                                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                                textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                                    aiGenerated: z.ZodBoolean;
                                    content: z.ZodString;
                                    contentId: z.ZodString;
                                    id: z.ZodString;
                                    locale: z.ZodString;
                                    requiresValidation: z.ZodBoolean;
                                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
                            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                        value: z.ZodNumber;
                    }, z.ZodTypeAny, "passthrough">[];
                } & {
                    [k: string]: unknown;
                };
            } | {
                continuous: {
                    max_label: {
                        translations: {
                            textContent: {
                                id: string;
                                primaryLocale: string;
                                format: "plain" | "markdown" | "rich";
                            } & {
                                [k: string]: unknown;
                            };
                            textTranslations: z.objectOutputType<{
                                aiGenerated: z.ZodBoolean;
                                content: z.ZodString;
                                contentId: z.ZodString;
                                id: z.ZodString;
                                locale: z.ZodString;
                                requiresValidation: z.ZodBoolean;
                            }, z.ZodTypeAny, "passthrough">[];
                        } & {
                            [k: string]: unknown;
                        };
                        localized: string;
                    } & {
                        [k: string]: unknown;
                    };
                    max_value: number;
                    min_label: {
                        translations: {
                            textContent: {
                                id: string;
                                primaryLocale: string;
                                format: "plain" | "markdown" | "rich";
                            } & {
                                [k: string]: unknown;
                            };
                            textTranslations: z.objectOutputType<{
                                aiGenerated: z.ZodBoolean;
                                content: z.ZodString;
                                contentId: z.ZodString;
                                id: z.ZodString;
                                locale: z.ZodString;
                                requiresValidation: z.ZodBoolean;
                            }, z.ZodTypeAny, "passthrough">[];
                        } & {
                            [k: string]: unknown;
                        };
                        localized: string;
                    } & {
                        [k: string]: unknown;
                    };
                    min_value: number;
                    sub_steps: number;
                } & {
                    [k: string]: unknown;
                };
            }, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        randomize_order: z.ZodBoolean;
        required_reviews: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        section_questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            text: z.ZodType<z.objectOutputType<{
                localized: z.ZodString;
                translations: z.ZodType<z.objectOutputType<{
                    textContent: z.ZodType<z.objectOutputType<{
                        format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                        id: z.ZodString;
                        primaryLocale: z.ZodString;
                    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                    textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                        aiGenerated: z.ZodBoolean;
                        content: z.ZodString;
                        contentId: z.ZodString;
                        id: z.ZodString;
                        locale: z.ZodString;
                        requiresValidation: z.ZodBoolean;
                    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            type: z.ZodType<"text" | {
                likert_scale: {
                    categories: z.objectOutputType<{
                        label: z.ZodType<z.objectOutputType<{
                            localized: z.ZodString;
                            translations: z.ZodType<z.objectOutputType<{
                                textContent: z.ZodType<z.objectOutputType<{
                                    format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                                    id: z.ZodString;
                                    primaryLocale: z.ZodString;
                                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                                textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                                    aiGenerated: z.ZodBoolean;
                                    content: z.ZodString;
                                    contentId: z.ZodString;
                                    id: z.ZodString;
                                    locale: z.ZodString;
                                    requiresValidation: z.ZodBoolean;
                                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
                            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                        value: z.ZodNumber;
                    }, z.ZodTypeAny, "passthrough">[];
                } & {
                    [k: string]: unknown;
                };
            } | {
                continuous: {
                    max_label: {
                        translations: {
                            textContent: {
                                id: string;
                                primaryLocale: string;
                                format: "plain" | "markdown" | "rich";
                            } & {
                                [k: string]: unknown;
                            };
                            textTranslations: z.objectOutputType<{
                                aiGenerated: z.ZodBoolean;
                                content: z.ZodString;
                                contentId: z.ZodString;
                                id: z.ZodString;
                                locale: z.ZodString;
                                requiresValidation: z.ZodBoolean;
                            }, z.ZodTypeAny, "passthrough">[];
                        } & {
                            [k: string]: unknown;
                        };
                        localized: string;
                    } & {
                        [k: string]: unknown;
                    };
                    max_value: number;
                    min_label: {
                        translations: {
                            textContent: {
                                id: string;
                                primaryLocale: string;
                                format: "plain" | "markdown" | "rich";
                            } & {
                                [k: string]: unknown;
                            };
                            textTranslations: z.objectOutputType<{
                                aiGenerated: z.ZodBoolean;
                                content: z.ZodString;
                                contentId: z.ZodString;
                                id: z.ZodString;
                                locale: z.ZodString;
                                requiresValidation: z.ZodBoolean;
                            }, z.ZodTypeAny, "passthrough">[];
                        } & {
                            [k: string]: unknown;
                        };
                        localized: string;
                    } & {
                        [k: string]: unknown;
                    };
                    min_value: number;
                    sub_steps: number;
                } & {
                    [k: string]: unknown;
                };
            }, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        type: z.ZodLiteral<"prioritization">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        follow_up_rounds_count: z.ZodNumber;
        root_questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            intent: z.ZodType<z.objectOutputType<{
                localized: z.ZodString;
                translations: z.ZodType<z.objectOutputType<{
                    textContent: z.ZodType<z.objectOutputType<{
                        format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                        id: z.ZodString;
                        primaryLocale: z.ZodString;
                    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                    textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                        aiGenerated: z.ZodBoolean;
                        content: z.ZodString;
                        contentId: z.ZodString;
                        id: z.ZodString;
                        locale: z.ZodString;
                        requiresValidation: z.ZodBoolean;
                    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            text: z.ZodType<z.objectOutputType<{
                localized: z.ZodString;
                translations: z.ZodType<z.objectOutputType<{
                    textContent: z.ZodType<z.objectOutputType<{
                        format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                        id: z.ZodString;
                        primaryLocale: z.ZodString;
                    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                    textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                        aiGenerated: z.ZodBoolean;
                        content: z.ZodString;
                        contentId: z.ZodString;
                        id: z.ZodString;
                        locale: z.ZodString;
                        requiresValidation: z.ZodBoolean;
                    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        topic: z.ZodType<z.objectOutputType<{
            localized: z.ZodString;
            translations: z.ZodType<z.objectOutputType<{
                textContent: z.ZodType<z.objectOutputType<{
                    format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                    id: z.ZodString;
                    primaryLocale: z.ZodString;
                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                    aiGenerated: z.ZodBoolean;
                    content: z.ZodString;
                    contentId: z.ZodString;
                    id: z.ZodString;
                    locale: z.ZodString;
                    requiresValidation: z.ZodBoolean;
                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        type: z.ZodLiteral<"thinkingspace">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    requestUserSharePermission: z.ZodBoolean;
    required: z.ZodBoolean;
    stepOrder: z.ZodNumber;
    toolConfig: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
        admin_password: z.ZodString;
        admin_user: z.ZodString;
        description: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
        is_active: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
        label_seeds_as_conversation_starter: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
        poll_id: z.ZodString;
        required_votes: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        server_url: z.ZodString;
        show_remaining_statements: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
        strict_moderation: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
        topic: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
        type: z.ZodLiteral<"polis">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        pages: z.ZodArray<z.ZodType<z.objectOutputType<{
            text_content_id: z.ZodString;
        }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
            content: z.ZodString;
            type: z.ZodLiteral<"markdown">;
        }, z.ZodTypeAny, "passthrough">[], z.ZodTypeDef, any>, "many">;
        type: z.ZodLiteral<"learn">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        admin_password: z.ZodString;
        admin_user: z.ZodString;
        project_id: z.ZodString;
        server_url: z.ZodDefault<z.ZodOptional<z.ZodString>>;
        survey_id: z.ZodString;
        survey_url: z.ZodString;
        type: z.ZodLiteral<"heyform">;
        workspace_id: z.ZodString;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        max_time: z.ZodNumber;
        to_see: z.ZodNumber;
        type: z.ZodLiteral<"stories">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        topic: z.ZodString;
        type: z.ZodLiteral<"elicitationbot">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        alignment_question_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            text: z.ZodType<z.objectOutputType<{
                localized: z.ZodString;
                translations: z.ZodType<z.objectOutputType<{
                    textContent: z.ZodType<z.objectOutputType<{
                        format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                        id: z.ZodString;
                        primaryLocale: z.ZodString;
                    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                    textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                        aiGenerated: z.ZodBoolean;
                        content: z.ZodString;
                        contentId: z.ZodString;
                        id: z.ZodString;
                        locale: z.ZodString;
                        requiresValidation: z.ZodBoolean;
                    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            type: z.ZodType<"text" | {
                likert_scale: {
                    categories: z.objectOutputType<{
                        label: z.ZodType<z.objectOutputType<{
                            localized: z.ZodString;
                            translations: z.ZodType<z.objectOutputType<{
                                textContent: z.ZodType<z.objectOutputType<{
                                    format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                                    id: z.ZodString;
                                    primaryLocale: z.ZodString;
                                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                                textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                                    aiGenerated: z.ZodBoolean;
                                    content: z.ZodString;
                                    contentId: z.ZodString;
                                    id: z.ZodString;
                                    locale: z.ZodString;
                                    requiresValidation: z.ZodBoolean;
                                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
                            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                        value: z.ZodNumber;
                    }, z.ZodTypeAny, "passthrough">[];
                } & {
                    [k: string]: unknown;
                };
            } | {
                continuous: {
                    max_label: {
                        translations: {
                            textContent: {
                                id: string;
                                primaryLocale: string;
                                format: "plain" | "markdown" | "rich";
                            } & {
                                [k: string]: unknown;
                            };
                            textTranslations: z.objectOutputType<{
                                aiGenerated: z.ZodBoolean;
                                content: z.ZodString;
                                contentId: z.ZodString;
                                id: z.ZodString;
                                locale: z.ZodString;
                                requiresValidation: z.ZodBoolean;
                            }, z.ZodTypeAny, "passthrough">[];
                        } & {
                            [k: string]: unknown;
                        };
                        localized: string;
                    } & {
                        [k: string]: unknown;
                    };
                    max_value: number;
                    min_label: {
                        translations: {
                            textContent: {
                                id: string;
                                primaryLocale: string;
                                format: "plain" | "markdown" | "rich";
                            } & {
                                [k: string]: unknown;
                            };
                            textTranslations: z.objectOutputType<{
                                aiGenerated: z.ZodBoolean;
                                content: z.ZodString;
                                contentId: z.ZodString;
                                id: z.ZodString;
                                locale: z.ZodString;
                                requiresValidation: z.ZodBoolean;
                            }, z.ZodTypeAny, "passthrough">[];
                        } & {
                            [k: string]: unknown;
                        };
                        localized: string;
                    } & {
                        [k: string]: unknown;
                    };
                    min_value: number;
                    sub_steps: number;
                } & {
                    [k: string]: unknown;
                };
            }, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        randomize_order: z.ZodBoolean;
        required_reviews: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        section_questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            text: z.ZodType<z.objectOutputType<{
                localized: z.ZodString;
                translations: z.ZodType<z.objectOutputType<{
                    textContent: z.ZodType<z.objectOutputType<{
                        format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                        id: z.ZodString;
                        primaryLocale: z.ZodString;
                    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                    textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                        aiGenerated: z.ZodBoolean;
                        content: z.ZodString;
                        contentId: z.ZodString;
                        id: z.ZodString;
                        locale: z.ZodString;
                        requiresValidation: z.ZodBoolean;
                    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            type: z.ZodType<"text" | {
                likert_scale: {
                    categories: z.objectOutputType<{
                        label: z.ZodType<z.objectOutputType<{
                            localized: z.ZodString;
                            translations: z.ZodType<z.objectOutputType<{
                                textContent: z.ZodType<z.objectOutputType<{
                                    format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                                    id: z.ZodString;
                                    primaryLocale: z.ZodString;
                                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                                textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                                    aiGenerated: z.ZodBoolean;
                                    content: z.ZodString;
                                    contentId: z.ZodString;
                                    id: z.ZodString;
                                    locale: z.ZodString;
                                    requiresValidation: z.ZodBoolean;
                                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
                            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                        value: z.ZodNumber;
                    }, z.ZodTypeAny, "passthrough">[];
                } & {
                    [k: string]: unknown;
                };
            } | {
                continuous: {
                    max_label: {
                        translations: {
                            textContent: {
                                id: string;
                                primaryLocale: string;
                                format: "plain" | "markdown" | "rich";
                            } & {
                                [k: string]: unknown;
                            };
                            textTranslations: z.objectOutputType<{
                                aiGenerated: z.ZodBoolean;
                                content: z.ZodString;
                                contentId: z.ZodString;
                                id: z.ZodString;
                                locale: z.ZodString;
                                requiresValidation: z.ZodBoolean;
                            }, z.ZodTypeAny, "passthrough">[];
                        } & {
                            [k: string]: unknown;
                        };
                        localized: string;
                    } & {
                        [k: string]: unknown;
                    };
                    max_value: number;
                    min_label: {
                        translations: {
                            textContent: {
                                id: string;
                                primaryLocale: string;
                                format: "plain" | "markdown" | "rich";
                            } & {
                                [k: string]: unknown;
                            };
                            textTranslations: z.objectOutputType<{
                                aiGenerated: z.ZodBoolean;
                                content: z.ZodString;
                                contentId: z.ZodString;
                                id: z.ZodString;
                                locale: z.ZodString;
                                requiresValidation: z.ZodBoolean;
                            }, z.ZodTypeAny, "passthrough">[];
                        } & {
                            [k: string]: unknown;
                        };
                        localized: string;
                    } & {
                        [k: string]: unknown;
                    };
                    min_value: number;
                    sub_steps: number;
                } & {
                    [k: string]: unknown;
                };
            }, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        type: z.ZodLiteral<"prioritization">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        follow_up_rounds_count: z.ZodNumber;
        root_questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            intent: z.ZodType<z.objectOutputType<{
                localized: z.ZodString;
                translations: z.ZodType<z.objectOutputType<{
                    textContent: z.ZodType<z.objectOutputType<{
                        format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                        id: z.ZodString;
                        primaryLocale: z.ZodString;
                    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                    textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                        aiGenerated: z.ZodBoolean;
                        content: z.ZodString;
                        contentId: z.ZodString;
                        id: z.ZodString;
                        locale: z.ZodString;
                        requiresValidation: z.ZodBoolean;
                    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            text: z.ZodType<z.objectOutputType<{
                localized: z.ZodString;
                translations: z.ZodType<z.objectOutputType<{
                    textContent: z.ZodType<z.objectOutputType<{
                        format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                        id: z.ZodString;
                        primaryLocale: z.ZodString;
                    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                    textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                        aiGenerated: z.ZodBoolean;
                        content: z.ZodString;
                        contentId: z.ZodString;
                        id: z.ZodString;
                        locale: z.ZodString;
                        requiresValidation: z.ZodBoolean;
                    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        topic: z.ZodType<z.objectOutputType<{
            localized: z.ZodString;
            translations: z.ZodType<z.objectOutputType<{
                textContent: z.ZodType<z.objectOutputType<{
                    format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                    id: z.ZodString;
                    primaryLocale: z.ZodString;
                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
                textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                    aiGenerated: z.ZodBoolean;
                    content: z.ZodString;
                    contentId: z.ZodString;
                    id: z.ZodString;
                    locale: z.ZodString;
                    requiresValidation: z.ZodBoolean;
                }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        type: z.ZodLiteral<"thinkingspace">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
    translations: z.ZodType<z.objectOutputType<{
        description: z.ZodType<z.objectOutputType<{
            textContent: z.ZodType<z.objectOutputType<{
                format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                id: z.ZodString;
                primaryLocale: z.ZodString;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                aiGenerated: z.ZodBoolean;
                content: z.ZodString;
                contentId: z.ZodString;
                id: z.ZodString;
                locale: z.ZodString;
                requiresValidation: z.ZodBoolean;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        name: z.ZodType<z.objectOutputType<{
            textContent: z.ZodType<z.objectOutputType<{
                format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                id: z.ZodString;
                primaryLocale: z.ZodString;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                aiGenerated: z.ZodBoolean;
                content: z.ZodString;
                contentId: z.ZodString;
                id: z.ZodString;
                locale: z.ZodString;
                requiresValidation: z.ZodBoolean;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    workflowId: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">, z.ZodArray<z.ZodType<z.objectOutputType<{
    activationRule: z.ZodType<"manual", z.ZodTypeDef, any>;
    canRevisit: z.ZodBoolean;
    description: z.ZodString;
    id: z.ZodString;
    isOffline: z.ZodBoolean;
    name: z.ZodString;
    previewToolConfig: z.ZodType<z.objectOutputType<{
        admin_password: z.ZodString;
        admin_user: z.ZodString;
        description: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
        is_active: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
        label_seeds_as_conversation_starter: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
        poll_id: z.ZodString;
        required_votes: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        server_url: z.ZodString;
        show_remaining_statements: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
        strict_moderation: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
        topic: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
        type: z.ZodLiteral<"polis">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        pages: z.ZodArray<z.ZodType<z.objectOutputType<{
            text_content_id: z.ZodString;
        }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
            content: z.ZodString;
            type: z.ZodLiteral<"markdown">;
        }, z.ZodTypeAny, "passthrough">[], z.ZodTypeDef, any>, "many">;
        type: z.ZodLiteral<"learn">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        admin_password: z.ZodString;
        admin_user: z.ZodString;
        project_id: z.ZodString;
        server_url: z.ZodDefault<z.ZodOptional<z.ZodString>>;
        survey_id: z.ZodString;
        survey_url: z.ZodString;
        type: z.ZodLiteral<"heyform">;
        workspace_id: z.ZodString;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        max_time: z.ZodNumber;
        to_see: z.ZodNumber;
        type: z.ZodLiteral<"stories">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        topic: z.ZodString;
        type: z.ZodLiteral<"elicitationbot">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        alignment_question_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            text: z.ZodString;
            type: z.ZodType<"text" | {
                likert_scale: {
                    categories: z.objectOutputType<{
                        label: z.ZodString;
                        value: z.ZodNumber;
                    }, z.ZodTypeAny, "passthrough">[];
                } & {
                    [k: string]: unknown;
                };
            } | {
                continuous: {
                    max_label: string;
                    max_value: number;
                    min_label: string;
                    min_value: number;
                    sub_steps: number;
                } & {
                    [k: string]: unknown;
                };
            }, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        randomize_order: z.ZodBoolean;
        required_reviews: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        section_questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            text: z.ZodString;
            type: z.ZodType<"text" | {
                likert_scale: {
                    categories: z.objectOutputType<{
                        label: z.ZodString;
                        value: z.ZodNumber;
                    }, z.ZodTypeAny, "passthrough">[];
                } & {
                    [k: string]: unknown;
                };
            } | {
                continuous: {
                    max_label: string;
                    max_value: number;
                    min_label: string;
                    min_value: number;
                    sub_steps: number;
                } & {
                    [k: string]: unknown;
                };
            }, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        type: z.ZodLiteral<"prioritization">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        follow_up_rounds_count: z.ZodNumber;
        root_questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            intent: z.ZodString;
            text: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        topic: z.ZodString;
        type: z.ZodLiteral<"thinkingspace">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    progressStatus: z.ZodType<"not_started" | "in_progress" | "done", z.ZodTypeDef, any>;
    requestUserSharePermission: z.ZodBoolean;
    required: z.ZodBoolean;
    stepOrder: z.ZodNumber;
    toolConfig: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
        admin_password: z.ZodString;
        admin_user: z.ZodString;
        description: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
        is_active: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
        label_seeds_as_conversation_starter: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
        poll_id: z.ZodString;
        required_votes: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        server_url: z.ZodString;
        show_remaining_statements: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
        strict_moderation: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
        topic: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
        type: z.ZodLiteral<"polis">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        pages: z.ZodArray<z.ZodType<z.objectOutputType<{
            text_content_id: z.ZodString;
        }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
            content: z.ZodString;
            type: z.ZodLiteral<"markdown">;
        }, z.ZodTypeAny, "passthrough">[], z.ZodTypeDef, any>, "many">;
        type: z.ZodLiteral<"learn">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        admin_password: z.ZodString;
        admin_user: z.ZodString;
        project_id: z.ZodString;
        server_url: z.ZodDefault<z.ZodOptional<z.ZodString>>;
        survey_id: z.ZodString;
        survey_url: z.ZodString;
        type: z.ZodLiteral<"heyform">;
        workspace_id: z.ZodString;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        max_time: z.ZodNumber;
        to_see: z.ZodNumber;
        type: z.ZodLiteral<"stories">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        topic: z.ZodString;
        type: z.ZodLiteral<"elicitationbot">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        alignment_question_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            text: z.ZodString;
            type: z.ZodType<"text" | {
                likert_scale: {
                    categories: z.objectOutputType<{
                        label: z.ZodString;
                        value: z.ZodNumber;
                    }, z.ZodTypeAny, "passthrough">[];
                } & {
                    [k: string]: unknown;
                };
            } | {
                continuous: {
                    max_label: string;
                    max_value: number;
                    min_label: string;
                    min_value: number;
                    sub_steps: number;
                } & {
                    [k: string]: unknown;
                };
            }, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        randomize_order: z.ZodBoolean;
        required_reviews: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        section_questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            text: z.ZodString;
            type: z.ZodType<"text" | {
                likert_scale: {
                    categories: z.objectOutputType<{
                        label: z.ZodString;
                        value: z.ZodNumber;
                    }, z.ZodTypeAny, "passthrough">[];
                } & {
                    [k: string]: unknown;
                };
            } | {
                continuous: {
                    max_label: string;
                    max_value: number;
                    min_label: string;
                    min_value: number;
                    sub_steps: number;
                } & {
                    [k: string]: unknown;
                };
            }, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        type: z.ZodLiteral<"prioritization">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        follow_up_rounds_count: z.ZodNumber;
        root_questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            intent: z.ZodString;
            text: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        topic: z.ZodString;
        type: z.ZodLiteral<"thinkingspace">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
    workflowId: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">, z.ZodArray<z.ZodType<z.objectOutputType<{
    activationRule: z.ZodType<"manual", z.ZodTypeDef, any>;
    canRevisit: z.ZodBoolean;
    description: z.ZodString;
    id: z.ZodString;
    isOffline: z.ZodBoolean;
    name: z.ZodString;
    previewToolConfig: z.ZodType<z.objectOutputType<{
        admin_password: z.ZodString;
        admin_user: z.ZodString;
        description: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
        is_active: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
        label_seeds_as_conversation_starter: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
        poll_id: z.ZodString;
        required_votes: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        server_url: z.ZodString;
        show_remaining_statements: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
        strict_moderation: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
        topic: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
        type: z.ZodLiteral<"polis">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        pages: z.ZodArray<z.ZodType<z.objectOutputType<{
            text_content_id: z.ZodString;
        }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
            content: z.ZodString;
            type: z.ZodLiteral<"markdown">;
        }, z.ZodTypeAny, "passthrough">[], z.ZodTypeDef, any>, "many">;
        type: z.ZodLiteral<"learn">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        admin_password: z.ZodString;
        admin_user: z.ZodString;
        project_id: z.ZodString;
        server_url: z.ZodDefault<z.ZodOptional<z.ZodString>>;
        survey_id: z.ZodString;
        survey_url: z.ZodString;
        type: z.ZodLiteral<"heyform">;
        workspace_id: z.ZodString;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        max_time: z.ZodNumber;
        to_see: z.ZodNumber;
        type: z.ZodLiteral<"stories">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        topic: z.ZodString;
        type: z.ZodLiteral<"elicitationbot">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        alignment_question_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            text: z.ZodString;
            type: z.ZodType<"text" | {
                likert_scale: {
                    categories: z.objectOutputType<{
                        label: z.ZodString;
                        value: z.ZodNumber;
                    }, z.ZodTypeAny, "passthrough">[];
                } & {
                    [k: string]: unknown;
                };
            } | {
                continuous: {
                    max_label: string;
                    max_value: number;
                    min_label: string;
                    min_value: number;
                    sub_steps: number;
                } & {
                    [k: string]: unknown;
                };
            }, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        randomize_order: z.ZodBoolean;
        required_reviews: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        section_questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            text: z.ZodString;
            type: z.ZodType<"text" | {
                likert_scale: {
                    categories: z.objectOutputType<{
                        label: z.ZodString;
                        value: z.ZodNumber;
                    }, z.ZodTypeAny, "passthrough">[];
                } & {
                    [k: string]: unknown;
                };
            } | {
                continuous: {
                    max_label: string;
                    max_value: number;
                    min_label: string;
                    min_value: number;
                    sub_steps: number;
                } & {
                    [k: string]: unknown;
                };
            }, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        type: z.ZodLiteral<"prioritization">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        follow_up_rounds_count: z.ZodNumber;
        root_questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            intent: z.ZodString;
            text: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        topic: z.ZodString;
        type: z.ZodLiteral<"thinkingspace">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    requestUserSharePermission: z.ZodBoolean;
    required: z.ZodBoolean;
    stepOrder: z.ZodNumber;
    toolConfig: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
        admin_password: z.ZodString;
        admin_user: z.ZodString;
        description: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
        is_active: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
        label_seeds_as_conversation_starter: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
        poll_id: z.ZodString;
        required_votes: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        server_url: z.ZodString;
        show_remaining_statements: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
        strict_moderation: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
        topic: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
        type: z.ZodLiteral<"polis">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        pages: z.ZodArray<z.ZodType<z.objectOutputType<{
            text_content_id: z.ZodString;
        }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
            content: z.ZodString;
            type: z.ZodLiteral<"markdown">;
        }, z.ZodTypeAny, "passthrough">[], z.ZodTypeDef, any>, "many">;
        type: z.ZodLiteral<"learn">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        admin_password: z.ZodString;
        admin_user: z.ZodString;
        project_id: z.ZodString;
        server_url: z.ZodDefault<z.ZodOptional<z.ZodString>>;
        survey_id: z.ZodString;
        survey_url: z.ZodString;
        type: z.ZodLiteral<"heyform">;
        workspace_id: z.ZodString;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        max_time: z.ZodNumber;
        to_see: z.ZodNumber;
        type: z.ZodLiteral<"stories">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        topic: z.ZodString;
        type: z.ZodLiteral<"elicitationbot">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        alignment_question_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            text: z.ZodString;
            type: z.ZodType<"text" | {
                likert_scale: {
                    categories: z.objectOutputType<{
                        label: z.ZodString;
                        value: z.ZodNumber;
                    }, z.ZodTypeAny, "passthrough">[];
                } & {
                    [k: string]: unknown;
                };
            } | {
                continuous: {
                    max_label: string;
                    max_value: number;
                    min_label: string;
                    min_value: number;
                    sub_steps: number;
                } & {
                    [k: string]: unknown;
                };
            }, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        randomize_order: z.ZodBoolean;
        required_reviews: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        section_questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            text: z.ZodString;
            type: z.ZodType<"text" | {
                likert_scale: {
                    categories: z.objectOutputType<{
                        label: z.ZodString;
                        value: z.ZodNumber;
                    }, z.ZodTypeAny, "passthrough">[];
                } & {
                    [k: string]: unknown;
                };
            } | {
                continuous: {
                    max_label: string;
                    max_value: number;
                    min_label: string;
                    min_value: number;
                    sub_steps: number;
                } & {
                    [k: string]: unknown;
                };
            }, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        type: z.ZodLiteral<"prioritization">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        follow_up_rounds_count: z.ZodNumber;
        root_questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            intent: z.ZodString;
            text: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        topic: z.ZodString;
        type: z.ZodLiteral<"thinkingspace">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
    workflowId: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">]>;
export type WorkflowStepsListResponse = z.infer<typeof _WorkflowStepsListResponse>;
export declare const WorkflowStepsListResponse: z.ZodType<WorkflowStepsListResponse, z.ZodTypeDef, any>;
declare const _SetupCategory: z.ZodObject<{
    label: z.ZodString;
    value: z.ZodNumber;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    label: z.ZodString;
    value: z.ZodNumber;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    label: z.ZodString;
    value: z.ZodNumber;
}, z.ZodTypeAny, "passthrough">>;
export type SetupCategory = z.infer<typeof _SetupCategory>;
export declare const SetupCategory: z.ZodType<SetupCategory, z.ZodTypeDef, any>;
declare const _SetupQuestionType: z.ZodUnion<[z.ZodLiteral<"text">, z.ZodObject<{
    likert_scale: z.ZodObject<{
        categories: z.ZodArray<z.ZodType<z.objectOutputType<{
            label: z.ZodString;
            value: z.ZodNumber;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    }, "passthrough", z.ZodTypeAny, z.objectOutputType<{
        categories: z.ZodArray<z.ZodType<z.objectOutputType<{
            label: z.ZodString;
            value: z.ZodNumber;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    }, z.ZodTypeAny, "passthrough">, z.objectInputType<{
        categories: z.ZodArray<z.ZodType<z.objectOutputType<{
            label: z.ZodString;
            value: z.ZodNumber;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    }, z.ZodTypeAny, "passthrough">>;
}, "strip", z.ZodTypeAny, {
    likert_scale: {
        categories: z.objectOutputType<{
            label: z.ZodString;
            value: z.ZodNumber;
        }, z.ZodTypeAny, "passthrough">[];
    } & {
        [k: string]: unknown;
    };
}, {
    likert_scale: {
        categories: any[];
    } & {
        [k: string]: unknown;
    };
}>, z.ZodObject<{
    continuous: z.ZodObject<{
        max_label: z.ZodString;
        max_value: z.ZodNumber;
        min_label: z.ZodString;
        min_value: z.ZodNumber;
        sub_steps: z.ZodNumber;
    }, "passthrough", z.ZodTypeAny, z.objectOutputType<{
        max_label: z.ZodString;
        max_value: z.ZodNumber;
        min_label: z.ZodString;
        min_value: z.ZodNumber;
        sub_steps: z.ZodNumber;
    }, z.ZodTypeAny, "passthrough">, z.objectInputType<{
        max_label: z.ZodString;
        max_value: z.ZodNumber;
        min_label: z.ZodString;
        min_value: z.ZodNumber;
        sub_steps: z.ZodNumber;
    }, z.ZodTypeAny, "passthrough">>;
}, "strip", z.ZodTypeAny, {
    continuous: {
        max_label: string;
        max_value: number;
        min_label: string;
        min_value: number;
        sub_steps: number;
    } & {
        [k: string]: unknown;
    };
}, {
    continuous: {
        max_label: string;
        max_value: number;
        min_label: string;
        min_value: number;
        sub_steps: number;
    } & {
        [k: string]: unknown;
    };
}>]>;
export type SetupQuestionType = z.infer<typeof _SetupQuestionType>;
export declare const SetupQuestionType: z.ZodType<SetupQuestionType, z.ZodTypeDef, any>;
declare const _SetupQuestion: z.ZodObject<{
    text: z.ZodString;
    type: z.ZodType<"text" | {
        likert_scale: {
            categories: z.objectOutputType<{
                label: z.ZodString;
                value: z.ZodNumber;
            }, z.ZodTypeAny, "passthrough">[];
        } & {
            [k: string]: unknown;
        };
    } | {
        continuous: {
            max_label: string;
            max_value: number;
            min_label: string;
            min_value: number;
            sub_steps: number;
        } & {
            [k: string]: unknown;
        };
    }, z.ZodTypeDef, any>;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    text: z.ZodString;
    type: z.ZodType<"text" | {
        likert_scale: {
            categories: z.objectOutputType<{
                label: z.ZodString;
                value: z.ZodNumber;
            }, z.ZodTypeAny, "passthrough">[];
        } & {
            [k: string]: unknown;
        };
    } | {
        continuous: {
            max_label: string;
            max_value: number;
            min_label: string;
            min_value: number;
            sub_steps: number;
        } & {
            [k: string]: unknown;
        };
    }, z.ZodTypeDef, any>;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    text: z.ZodString;
    type: z.ZodType<"text" | {
        likert_scale: {
            categories: z.objectOutputType<{
                label: z.ZodString;
                value: z.ZodNumber;
            }, z.ZodTypeAny, "passthrough">[];
        } & {
            [k: string]: unknown;
        };
    } | {
        continuous: {
            max_label: string;
            max_value: number;
            min_label: string;
            min_value: number;
            sub_steps: number;
        } & {
            [k: string]: unknown;
        };
    }, z.ZodTypeDef, any>;
}, z.ZodTypeAny, "passthrough">>;
export type SetupQuestion = z.infer<typeof _SetupQuestion>;
export declare const SetupQuestion: z.ZodType<SetupQuestion, z.ZodTypeDef, any>;
declare const _ThinkingSpaceSetupQuestion: z.ZodObject<{
    intent: z.ZodString;
    text: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    intent: z.ZodString;
    text: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    intent: z.ZodString;
    text: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type ThinkingSpaceSetupQuestion = z.infer<typeof _ThinkingSpaceSetupQuestion>;
export declare const ThinkingSpaceSetupQuestion: z.ZodType<ThinkingSpaceSetupQuestion, z.ZodTypeDef, any>;
declare const _ToolSetup: z.ZodUnion<[z.ZodObject<{
    required_votes: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
    show_remaining_statements: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
    topic: z.ZodString;
    type: z.ZodLiteral<"polis">;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    required_votes: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
    show_remaining_statements: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
    topic: z.ZodString;
    type: z.ZodLiteral<"polis">;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    required_votes: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
    show_remaining_statements: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
    topic: z.ZodString;
    type: z.ZodLiteral<"polis">;
}, z.ZodTypeAny, "passthrough">>, z.ZodObject<{
    pages: z.ZodArray<z.ZodType<z.objectOutputType<{
        text_content_id: z.ZodString;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        content: z.ZodString;
        type: z.ZodLiteral<"markdown">;
    }, z.ZodTypeAny, "passthrough">[], z.ZodTypeDef, any>, "many">;
    type: z.ZodLiteral<"learn">;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    pages: z.ZodArray<z.ZodType<z.objectOutputType<{
        text_content_id: z.ZodString;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        content: z.ZodString;
        type: z.ZodLiteral<"markdown">;
    }, z.ZodTypeAny, "passthrough">[], z.ZodTypeDef, any>, "many">;
    type: z.ZodLiteral<"learn">;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    pages: z.ZodArray<z.ZodType<z.objectOutputType<{
        text_content_id: z.ZodString;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        content: z.ZodString;
        type: z.ZodLiteral<"markdown">;
    }, z.ZodTypeAny, "passthrough">[], z.ZodTypeDef, any>, "many">;
    type: z.ZodLiteral<"learn">;
}, z.ZodTypeAny, "passthrough">>, z.ZodObject<{
    server_url: z.ZodDefault<z.ZodOptional<z.ZodString>>;
    type: z.ZodLiteral<"heyform">;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    server_url: z.ZodDefault<z.ZodOptional<z.ZodString>>;
    type: z.ZodLiteral<"heyform">;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    server_url: z.ZodDefault<z.ZodOptional<z.ZodString>>;
    type: z.ZodLiteral<"heyform">;
}, z.ZodTypeAny, "passthrough">>, z.ZodObject<{
    max_time: z.ZodNumber;
    to_see: z.ZodNumber;
    type: z.ZodLiteral<"stories">;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    max_time: z.ZodNumber;
    to_see: z.ZodNumber;
    type: z.ZodLiteral<"stories">;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    max_time: z.ZodNumber;
    to_see: z.ZodNumber;
    type: z.ZodLiteral<"stories">;
}, z.ZodTypeAny, "passthrough">>, z.ZodObject<{
    topic: z.ZodString;
    type: z.ZodLiteral<"elicitationbot">;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    topic: z.ZodString;
    type: z.ZodLiteral<"elicitationbot">;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    topic: z.ZodString;
    type: z.ZodLiteral<"elicitationbot">;
}, z.ZodTypeAny, "passthrough">>, z.ZodObject<{
    questions: z.ZodArray<z.ZodType<z.objectOutputType<{
        text: z.ZodString;
        type: z.ZodType<"text" | {
            likert_scale: {
                categories: z.objectOutputType<{
                    label: z.ZodString;
                    value: z.ZodNumber;
                }, z.ZodTypeAny, "passthrough">[];
            } & {
                [k: string]: unknown;
            };
        } | {
            continuous: {
                max_label: string;
                max_value: number;
                min_label: string;
                min_value: number;
                sub_steps: number;
            } & {
                [k: string]: unknown;
            };
        }, z.ZodTypeDef, any>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    type: z.ZodLiteral<"prioritization">;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    questions: z.ZodArray<z.ZodType<z.objectOutputType<{
        text: z.ZodString;
        type: z.ZodType<"text" | {
            likert_scale: {
                categories: z.objectOutputType<{
                    label: z.ZodString;
                    value: z.ZodNumber;
                }, z.ZodTypeAny, "passthrough">[];
            } & {
                [k: string]: unknown;
            };
        } | {
            continuous: {
                max_label: string;
                max_value: number;
                min_label: string;
                min_value: number;
                sub_steps: number;
            } & {
                [k: string]: unknown;
            };
        }, z.ZodTypeDef, any>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    type: z.ZodLiteral<"prioritization">;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    questions: z.ZodArray<z.ZodType<z.objectOutputType<{
        text: z.ZodString;
        type: z.ZodType<"text" | {
            likert_scale: {
                categories: z.objectOutputType<{
                    label: z.ZodString;
                    value: z.ZodNumber;
                }, z.ZodTypeAny, "passthrough">[];
            } & {
                [k: string]: unknown;
            };
        } | {
            continuous: {
                max_label: string;
                max_value: number;
                min_label: string;
                min_value: number;
                sub_steps: number;
            } & {
                [k: string]: unknown;
            };
        }, z.ZodTypeDef, any>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    type: z.ZodLiteral<"prioritization">;
}, z.ZodTypeAny, "passthrough">>, z.ZodObject<{
    follow_up_rounds_count: z.ZodNumber;
    root_questions: z.ZodArray<z.ZodType<z.objectOutputType<{
        intent: z.ZodString;
        text: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    topic: z.ZodString;
    type: z.ZodLiteral<"thinkingspace">;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    follow_up_rounds_count: z.ZodNumber;
    root_questions: z.ZodArray<z.ZodType<z.objectOutputType<{
        intent: z.ZodString;
        text: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    topic: z.ZodString;
    type: z.ZodLiteral<"thinkingspace">;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    follow_up_rounds_count: z.ZodNumber;
    root_questions: z.ZodArray<z.ZodType<z.objectOutputType<{
        intent: z.ZodString;
        text: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    topic: z.ZodString;
    type: z.ZodLiteral<"thinkingspace">;
}, z.ZodTypeAny, "passthrough">>]>;
export type ToolSetup = z.infer<typeof _ToolSetup>;
export declare const ToolSetup: z.ZodType<ToolSetup, z.ZodTypeDef, any>;
declare const _CreateWorkflowStep: z.ZodObject<{
    activation_rule: z.ZodType<"manual", z.ZodTypeDef, any>;
    description: z.ZodString;
    is_offline: z.ZodBoolean;
    name: z.ZodString;
    required: z.ZodBoolean;
    step_order: z.ZodNumber;
    tool_setup: z.ZodType<z.objectOutputType<{
        required_votes: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        show_remaining_statements: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
        topic: z.ZodString;
        type: z.ZodLiteral<"polis">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        pages: z.ZodArray<z.ZodType<z.objectOutputType<{
            text_content_id: z.ZodString;
        }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
            content: z.ZodString;
            type: z.ZodLiteral<"markdown">;
        }, z.ZodTypeAny, "passthrough">[], z.ZodTypeDef, any>, "many">;
        type: z.ZodLiteral<"learn">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        server_url: z.ZodDefault<z.ZodOptional<z.ZodString>>;
        type: z.ZodLiteral<"heyform">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        max_time: z.ZodNumber;
        to_see: z.ZodNumber;
        type: z.ZodLiteral<"stories">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        topic: z.ZodString;
        type: z.ZodLiteral<"elicitationbot">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            text: z.ZodString;
            type: z.ZodType<"text" | {
                likert_scale: {
                    categories: z.objectOutputType<{
                        label: z.ZodString;
                        value: z.ZodNumber;
                    }, z.ZodTypeAny, "passthrough">[];
                } & {
                    [k: string]: unknown;
                };
            } | {
                continuous: {
                    max_label: string;
                    max_value: number;
                    min_label: string;
                    min_value: number;
                    sub_steps: number;
                } & {
                    [k: string]: unknown;
                };
            }, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        type: z.ZodLiteral<"prioritization">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        follow_up_rounds_count: z.ZodNumber;
        root_questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            intent: z.ZodString;
            text: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        topic: z.ZodString;
        type: z.ZodLiteral<"thinkingspace">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    activation_rule: z.ZodType<"manual", z.ZodTypeDef, any>;
    description: z.ZodString;
    is_offline: z.ZodBoolean;
    name: z.ZodString;
    required: z.ZodBoolean;
    step_order: z.ZodNumber;
    tool_setup: z.ZodType<z.objectOutputType<{
        required_votes: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        show_remaining_statements: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
        topic: z.ZodString;
        type: z.ZodLiteral<"polis">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        pages: z.ZodArray<z.ZodType<z.objectOutputType<{
            text_content_id: z.ZodString;
        }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
            content: z.ZodString;
            type: z.ZodLiteral<"markdown">;
        }, z.ZodTypeAny, "passthrough">[], z.ZodTypeDef, any>, "many">;
        type: z.ZodLiteral<"learn">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        server_url: z.ZodDefault<z.ZodOptional<z.ZodString>>;
        type: z.ZodLiteral<"heyform">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        max_time: z.ZodNumber;
        to_see: z.ZodNumber;
        type: z.ZodLiteral<"stories">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        topic: z.ZodString;
        type: z.ZodLiteral<"elicitationbot">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            text: z.ZodString;
            type: z.ZodType<"text" | {
                likert_scale: {
                    categories: z.objectOutputType<{
                        label: z.ZodString;
                        value: z.ZodNumber;
                    }, z.ZodTypeAny, "passthrough">[];
                } & {
                    [k: string]: unknown;
                };
            } | {
                continuous: {
                    max_label: string;
                    max_value: number;
                    min_label: string;
                    min_value: number;
                    sub_steps: number;
                } & {
                    [k: string]: unknown;
                };
            }, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        type: z.ZodLiteral<"prioritization">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        follow_up_rounds_count: z.ZodNumber;
        root_questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            intent: z.ZodString;
            text: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        topic: z.ZodString;
        type: z.ZodLiteral<"thinkingspace">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    activation_rule: z.ZodType<"manual", z.ZodTypeDef, any>;
    description: z.ZodString;
    is_offline: z.ZodBoolean;
    name: z.ZodString;
    required: z.ZodBoolean;
    step_order: z.ZodNumber;
    tool_setup: z.ZodType<z.objectOutputType<{
        required_votes: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        show_remaining_statements: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
        topic: z.ZodString;
        type: z.ZodLiteral<"polis">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        pages: z.ZodArray<z.ZodType<z.objectOutputType<{
            text_content_id: z.ZodString;
        }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
            content: z.ZodString;
            type: z.ZodLiteral<"markdown">;
        }, z.ZodTypeAny, "passthrough">[], z.ZodTypeDef, any>, "many">;
        type: z.ZodLiteral<"learn">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        server_url: z.ZodDefault<z.ZodOptional<z.ZodString>>;
        type: z.ZodLiteral<"heyform">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        max_time: z.ZodNumber;
        to_see: z.ZodNumber;
        type: z.ZodLiteral<"stories">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        topic: z.ZodString;
        type: z.ZodLiteral<"elicitationbot">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            text: z.ZodString;
            type: z.ZodType<"text" | {
                likert_scale: {
                    categories: z.objectOutputType<{
                        label: z.ZodString;
                        value: z.ZodNumber;
                    }, z.ZodTypeAny, "passthrough">[];
                } & {
                    [k: string]: unknown;
                };
            } | {
                continuous: {
                    max_label: string;
                    max_value: number;
                    min_label: string;
                    min_value: number;
                    sub_steps: number;
                } & {
                    [k: string]: unknown;
                };
            }, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        type: z.ZodLiteral<"prioritization">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        follow_up_rounds_count: z.ZodNumber;
        root_questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            intent: z.ZodString;
            text: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        topic: z.ZodString;
        type: z.ZodLiteral<"thinkingspace">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
}, z.ZodTypeAny, "passthrough">>;
export type CreateWorkflowStep = z.infer<typeof _CreateWorkflowStep>;
export declare const CreateWorkflowStep: z.ZodType<CreateWorkflowStep, z.ZodTypeDef, any>;
declare const _WorkflowStepDto: z.ZodObject<{
    activationRule: z.ZodType<"manual", z.ZodTypeDef, any>;
    canRevisit: z.ZodBoolean;
    description: z.ZodString;
    id: z.ZodString;
    isOffline: z.ZodBoolean;
    name: z.ZodString;
    previewToolConfig: z.ZodType<z.objectOutputType<{
        admin_password: z.ZodString;
        admin_user: z.ZodString;
        description: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
        is_active: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
        label_seeds_as_conversation_starter: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
        poll_id: z.ZodString;
        required_votes: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        server_url: z.ZodString;
        show_remaining_statements: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
        strict_moderation: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
        topic: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
        type: z.ZodLiteral<"polis">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        pages: z.ZodArray<z.ZodType<z.objectOutputType<{
            text_content_id: z.ZodString;
        }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
            content: z.ZodString;
            type: z.ZodLiteral<"markdown">;
        }, z.ZodTypeAny, "passthrough">[], z.ZodTypeDef, any>, "many">;
        type: z.ZodLiteral<"learn">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        admin_password: z.ZodString;
        admin_user: z.ZodString;
        project_id: z.ZodString;
        server_url: z.ZodDefault<z.ZodOptional<z.ZodString>>;
        survey_id: z.ZodString;
        survey_url: z.ZodString;
        type: z.ZodLiteral<"heyform">;
        workspace_id: z.ZodString;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        max_time: z.ZodNumber;
        to_see: z.ZodNumber;
        type: z.ZodLiteral<"stories">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        topic: z.ZodString;
        type: z.ZodLiteral<"elicitationbot">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        alignment_question_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            text: z.ZodString;
            type: z.ZodType<"text" | {
                likert_scale: {
                    categories: z.objectOutputType<{
                        label: z.ZodString;
                        value: z.ZodNumber;
                    }, z.ZodTypeAny, "passthrough">[];
                } & {
                    [k: string]: unknown;
                };
            } | {
                continuous: {
                    max_label: string;
                    max_value: number;
                    min_label: string;
                    min_value: number;
                    sub_steps: number;
                } & {
                    [k: string]: unknown;
                };
            }, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        randomize_order: z.ZodBoolean;
        required_reviews: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>>;
        section_questions: z.ZodDefault<z.ZodOptional<z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            text: z.ZodString;
            type: z.ZodType<"text" | {
                likert_scale: {
                    categories: z.objectOutputType<{
                        label: z.ZodString;
                        value: z.ZodNumber;
                    }, z.ZodTypeAny, "passthrough">[];
                } & {
                    [k: string]: unknown;
                };
            } | {
                continuous: {
                    max_label: string;
                    max_value: number;
                    min_label: string;
                    min_value: number;
                    sub_steps: number;
                } & {
                    [k: string]: unknown;
                };
            }, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">>>;
        type: z.ZodLiteral<"prioritization">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        follow_up_rounds_count: z.ZodNumber;
        root_questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            intent: z.ZodString;
            text: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        topic: z.ZodString;
        type: z.ZodLiteral<"thinkingspace">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    requestUserSharePermission: z.ZodBoolean;
    required: z.ZodBoolean;
    stepOrder: z.ZodNumber;
    toolConfig: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
        admin_password: z.ZodString;
        admin_user: z.ZodString;
        description: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
        is_active: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
        label_seeds_as_conversation_starter: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
        poll_id: z.ZodString;
        required_votes: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        server_url: z.ZodString;
        show_remaining_statements: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
        strict_moderation: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
        topic: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
        type: z.ZodLiteral<"polis">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        pages: z.ZodArray<z.ZodType<z.objectOutputType<{
            text_content_id: z.ZodString;
        }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
            content: z.ZodString;
            type: z.ZodLiteral<"markdown">;
        }, z.ZodTypeAny, "passthrough">[], z.ZodTypeDef, any>, "many">;
        type: z.ZodLiteral<"learn">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        admin_password: z.ZodString;
        admin_user: z.ZodString;
        project_id: z.ZodString;
        server_url: z.ZodDefault<z.ZodOptional<z.ZodString>>;
        survey_id: z.ZodString;
        survey_url: z.ZodString;
        type: z.ZodLiteral<"heyform">;
        workspace_id: z.ZodString;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        max_time: z.ZodNumber;
        to_see: z.ZodNumber;
        type: z.ZodLiteral<"stories">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        topic: z.ZodString;
        type: z.ZodLiteral<"elicitationbot">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        alignment_question_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            text: z.ZodString;
            type: z.ZodType<"text" | {
                likert_scale: {
                    categories: z.objectOutputType<{
                        label: z.ZodString;
                        value: z.ZodNumber;
                    }, z.ZodTypeAny, "passthrough">[];
                } & {
                    [k: string]: unknown;
                };
            } | {
                continuous: {
                    max_label: string;
                    max_value: number;
                    min_label: string;
                    min_value: number;
                    sub_steps: number;
                } & {
                    [k: string]: unknown;
                };
            }, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        randomize_order: z.ZodBoolean;
        required_reviews: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>>;
        section_questions: z.ZodDefault<z.ZodOptional<z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            text: z.ZodString;
            type: z.ZodType<"text" | {
                likert_scale: {
                    categories: z.objectOutputType<{
                        label: z.ZodString;
                        value: z.ZodNumber;
                    }, z.ZodTypeAny, "passthrough">[];
                } & {
                    [k: string]: unknown;
                };
            } | {
                continuous: {
                    max_label: string;
                    max_value: number;
                    min_label: string;
                    min_value: number;
                    sub_steps: number;
                } & {
                    [k: string]: unknown;
                };
            }, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">>>;
        type: z.ZodLiteral<"prioritization">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        follow_up_rounds_count: z.ZodNumber;
        root_questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            intent: z.ZodString;
            text: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        topic: z.ZodString;
        type: z.ZodLiteral<"thinkingspace">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
    workflowId: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    activationRule: z.ZodType<"manual", z.ZodTypeDef, any>;
    canRevisit: z.ZodBoolean;
    description: z.ZodString;
    id: z.ZodString;
    isOffline: z.ZodBoolean;
    name: z.ZodString;
    previewToolConfig: z.ZodType<z.objectOutputType<{
        admin_password: z.ZodString;
        admin_user: z.ZodString;
        description: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
        is_active: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
        label_seeds_as_conversation_starter: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
        poll_id: z.ZodString;
        required_votes: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        server_url: z.ZodString;
        show_remaining_statements: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
        strict_moderation: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
        topic: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
        type: z.ZodLiteral<"polis">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        pages: z.ZodArray<z.ZodType<z.objectOutputType<{
            text_content_id: z.ZodString;
        }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
            content: z.ZodString;
            type: z.ZodLiteral<"markdown">;
        }, z.ZodTypeAny, "passthrough">[], z.ZodTypeDef, any>, "many">;
        type: z.ZodLiteral<"learn">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        admin_password: z.ZodString;
        admin_user: z.ZodString;
        project_id: z.ZodString;
        server_url: z.ZodDefault<z.ZodOptional<z.ZodString>>;
        survey_id: z.ZodString;
        survey_url: z.ZodString;
        type: z.ZodLiteral<"heyform">;
        workspace_id: z.ZodString;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        max_time: z.ZodNumber;
        to_see: z.ZodNumber;
        type: z.ZodLiteral<"stories">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        topic: z.ZodString;
        type: z.ZodLiteral<"elicitationbot">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        alignment_question_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            text: z.ZodString;
            type: z.ZodType<"text" | {
                likert_scale: {
                    categories: z.objectOutputType<{
                        label: z.ZodString;
                        value: z.ZodNumber;
                    }, z.ZodTypeAny, "passthrough">[];
                } & {
                    [k: string]: unknown;
                };
            } | {
                continuous: {
                    max_label: string;
                    max_value: number;
                    min_label: string;
                    min_value: number;
                    sub_steps: number;
                } & {
                    [k: string]: unknown;
                };
            }, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        randomize_order: z.ZodBoolean;
        required_reviews: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>>;
        section_questions: z.ZodDefault<z.ZodOptional<z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            text: z.ZodString;
            type: z.ZodType<"text" | {
                likert_scale: {
                    categories: z.objectOutputType<{
                        label: z.ZodString;
                        value: z.ZodNumber;
                    }, z.ZodTypeAny, "passthrough">[];
                } & {
                    [k: string]: unknown;
                };
            } | {
                continuous: {
                    max_label: string;
                    max_value: number;
                    min_label: string;
                    min_value: number;
                    sub_steps: number;
                } & {
                    [k: string]: unknown;
                };
            }, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">>>;
        type: z.ZodLiteral<"prioritization">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        follow_up_rounds_count: z.ZodNumber;
        root_questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            intent: z.ZodString;
            text: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        topic: z.ZodString;
        type: z.ZodLiteral<"thinkingspace">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    requestUserSharePermission: z.ZodBoolean;
    required: z.ZodBoolean;
    stepOrder: z.ZodNumber;
    toolConfig: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
        admin_password: z.ZodString;
        admin_user: z.ZodString;
        description: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
        is_active: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
        label_seeds_as_conversation_starter: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
        poll_id: z.ZodString;
        required_votes: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        server_url: z.ZodString;
        show_remaining_statements: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
        strict_moderation: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
        topic: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
        type: z.ZodLiteral<"polis">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        pages: z.ZodArray<z.ZodType<z.objectOutputType<{
            text_content_id: z.ZodString;
        }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
            content: z.ZodString;
            type: z.ZodLiteral<"markdown">;
        }, z.ZodTypeAny, "passthrough">[], z.ZodTypeDef, any>, "many">;
        type: z.ZodLiteral<"learn">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        admin_password: z.ZodString;
        admin_user: z.ZodString;
        project_id: z.ZodString;
        server_url: z.ZodDefault<z.ZodOptional<z.ZodString>>;
        survey_id: z.ZodString;
        survey_url: z.ZodString;
        type: z.ZodLiteral<"heyform">;
        workspace_id: z.ZodString;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        max_time: z.ZodNumber;
        to_see: z.ZodNumber;
        type: z.ZodLiteral<"stories">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        topic: z.ZodString;
        type: z.ZodLiteral<"elicitationbot">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        alignment_question_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            text: z.ZodString;
            type: z.ZodType<"text" | {
                likert_scale: {
                    categories: z.objectOutputType<{
                        label: z.ZodString;
                        value: z.ZodNumber;
                    }, z.ZodTypeAny, "passthrough">[];
                } & {
                    [k: string]: unknown;
                };
            } | {
                continuous: {
                    max_label: string;
                    max_value: number;
                    min_label: string;
                    min_value: number;
                    sub_steps: number;
                } & {
                    [k: string]: unknown;
                };
            }, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        randomize_order: z.ZodBoolean;
        required_reviews: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>>;
        section_questions: z.ZodDefault<z.ZodOptional<z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            text: z.ZodString;
            type: z.ZodType<"text" | {
                likert_scale: {
                    categories: z.objectOutputType<{
                        label: z.ZodString;
                        value: z.ZodNumber;
                    }, z.ZodTypeAny, "passthrough">[];
                } & {
                    [k: string]: unknown;
                };
            } | {
                continuous: {
                    max_label: string;
                    max_value: number;
                    min_label: string;
                    min_value: number;
                    sub_steps: number;
                } & {
                    [k: string]: unknown;
                };
            }, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">>>;
        type: z.ZodLiteral<"prioritization">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        follow_up_rounds_count: z.ZodNumber;
        root_questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            intent: z.ZodString;
            text: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        topic: z.ZodString;
        type: z.ZodLiteral<"thinkingspace">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
    workflowId: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    activationRule: z.ZodType<"manual", z.ZodTypeDef, any>;
    canRevisit: z.ZodBoolean;
    description: z.ZodString;
    id: z.ZodString;
    isOffline: z.ZodBoolean;
    name: z.ZodString;
    previewToolConfig: z.ZodType<z.objectOutputType<{
        admin_password: z.ZodString;
        admin_user: z.ZodString;
        description: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
        is_active: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
        label_seeds_as_conversation_starter: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
        poll_id: z.ZodString;
        required_votes: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        server_url: z.ZodString;
        show_remaining_statements: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
        strict_moderation: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
        topic: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
        type: z.ZodLiteral<"polis">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        pages: z.ZodArray<z.ZodType<z.objectOutputType<{
            text_content_id: z.ZodString;
        }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
            content: z.ZodString;
            type: z.ZodLiteral<"markdown">;
        }, z.ZodTypeAny, "passthrough">[], z.ZodTypeDef, any>, "many">;
        type: z.ZodLiteral<"learn">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        admin_password: z.ZodString;
        admin_user: z.ZodString;
        project_id: z.ZodString;
        server_url: z.ZodDefault<z.ZodOptional<z.ZodString>>;
        survey_id: z.ZodString;
        survey_url: z.ZodString;
        type: z.ZodLiteral<"heyform">;
        workspace_id: z.ZodString;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        max_time: z.ZodNumber;
        to_see: z.ZodNumber;
        type: z.ZodLiteral<"stories">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        topic: z.ZodString;
        type: z.ZodLiteral<"elicitationbot">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        alignment_question_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            text: z.ZodString;
            type: z.ZodType<"text" | {
                likert_scale: {
                    categories: z.objectOutputType<{
                        label: z.ZodString;
                        value: z.ZodNumber;
                    }, z.ZodTypeAny, "passthrough">[];
                } & {
                    [k: string]: unknown;
                };
            } | {
                continuous: {
                    max_label: string;
                    max_value: number;
                    min_label: string;
                    min_value: number;
                    sub_steps: number;
                } & {
                    [k: string]: unknown;
                };
            }, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        randomize_order: z.ZodBoolean;
        required_reviews: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>>;
        section_questions: z.ZodDefault<z.ZodOptional<z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            text: z.ZodString;
            type: z.ZodType<"text" | {
                likert_scale: {
                    categories: z.objectOutputType<{
                        label: z.ZodString;
                        value: z.ZodNumber;
                    }, z.ZodTypeAny, "passthrough">[];
                } & {
                    [k: string]: unknown;
                };
            } | {
                continuous: {
                    max_label: string;
                    max_value: number;
                    min_label: string;
                    min_value: number;
                    sub_steps: number;
                } & {
                    [k: string]: unknown;
                };
            }, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">>>;
        type: z.ZodLiteral<"prioritization">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        follow_up_rounds_count: z.ZodNumber;
        root_questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            intent: z.ZodString;
            text: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        topic: z.ZodString;
        type: z.ZodLiteral<"thinkingspace">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    requestUserSharePermission: z.ZodBoolean;
    required: z.ZodBoolean;
    stepOrder: z.ZodNumber;
    toolConfig: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
        admin_password: z.ZodString;
        admin_user: z.ZodString;
        description: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
        is_active: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
        label_seeds_as_conversation_starter: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
        poll_id: z.ZodString;
        required_votes: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        server_url: z.ZodString;
        show_remaining_statements: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
        strict_moderation: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
        topic: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
        type: z.ZodLiteral<"polis">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        pages: z.ZodArray<z.ZodType<z.objectOutputType<{
            text_content_id: z.ZodString;
        }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
            content: z.ZodString;
            type: z.ZodLiteral<"markdown">;
        }, z.ZodTypeAny, "passthrough">[], z.ZodTypeDef, any>, "many">;
        type: z.ZodLiteral<"learn">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        admin_password: z.ZodString;
        admin_user: z.ZodString;
        project_id: z.ZodString;
        server_url: z.ZodDefault<z.ZodOptional<z.ZodString>>;
        survey_id: z.ZodString;
        survey_url: z.ZodString;
        type: z.ZodLiteral<"heyform">;
        workspace_id: z.ZodString;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        max_time: z.ZodNumber;
        to_see: z.ZodNumber;
        type: z.ZodLiteral<"stories">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        topic: z.ZodString;
        type: z.ZodLiteral<"elicitationbot">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        alignment_question_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            text: z.ZodString;
            type: z.ZodType<"text" | {
                likert_scale: {
                    categories: z.objectOutputType<{
                        label: z.ZodString;
                        value: z.ZodNumber;
                    }, z.ZodTypeAny, "passthrough">[];
                } & {
                    [k: string]: unknown;
                };
            } | {
                continuous: {
                    max_label: string;
                    max_value: number;
                    min_label: string;
                    min_value: number;
                    sub_steps: number;
                } & {
                    [k: string]: unknown;
                };
            }, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        randomize_order: z.ZodBoolean;
        required_reviews: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>>;
        section_questions: z.ZodDefault<z.ZodOptional<z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            text: z.ZodString;
            type: z.ZodType<"text" | {
                likert_scale: {
                    categories: z.objectOutputType<{
                        label: z.ZodString;
                        value: z.ZodNumber;
                    }, z.ZodTypeAny, "passthrough">[];
                } & {
                    [k: string]: unknown;
                };
            } | {
                continuous: {
                    max_label: string;
                    max_value: number;
                    min_label: string;
                    min_value: number;
                    sub_steps: number;
                } & {
                    [k: string]: unknown;
                };
            }, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">>>;
        type: z.ZodLiteral<"prioritization">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        follow_up_rounds_count: z.ZodNumber;
        root_questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            intent: z.ZodString;
            text: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        topic: z.ZodString;
        type: z.ZodLiteral<"thinkingspace">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
    workflowId: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type WorkflowStepDto = z.infer<typeof _WorkflowStepDto>;
export declare const WorkflowStepDto: z.ZodType<WorkflowStepDto, z.ZodTypeDef, any>;
declare const _PartialWorkflowStep: z.ZodObject<{
    activation_rule: z.ZodOptional<z.ZodUnion<[z.ZodType<"manual", z.ZodTypeDef, any>, z.ZodNull]>>;
    can_revisit: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    description: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    is_offline: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    name: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    preview_tool_config: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
        admin_password: z.ZodString;
        admin_user: z.ZodString;
        description: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
        is_active: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
        label_seeds_as_conversation_starter: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
        poll_id: z.ZodString;
        required_votes: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        server_url: z.ZodString;
        show_remaining_statements: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
        strict_moderation: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
        topic: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
        type: z.ZodLiteral<"polis">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        pages: z.ZodArray<z.ZodType<z.objectOutputType<{
            text_content_id: z.ZodString;
        }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
            content: z.ZodString;
            type: z.ZodLiteral<"markdown">;
        }, z.ZodTypeAny, "passthrough">[], z.ZodTypeDef, any>, "many">;
        type: z.ZodLiteral<"learn">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        admin_password: z.ZodString;
        admin_user: z.ZodString;
        project_id: z.ZodString;
        server_url: z.ZodDefault<z.ZodOptional<z.ZodString>>;
        survey_id: z.ZodString;
        survey_url: z.ZodString;
        type: z.ZodLiteral<"heyform">;
        workspace_id: z.ZodString;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        max_time: z.ZodNumber;
        to_see: z.ZodNumber;
        type: z.ZodLiteral<"stories">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        topic: z.ZodString;
        type: z.ZodLiteral<"elicitationbot">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        alignment_question_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            text: z.ZodString;
            type: z.ZodType<"text" | {
                likert_scale: {
                    categories: z.objectOutputType<{
                        label: z.ZodString;
                        value: z.ZodNumber;
                    }, z.ZodTypeAny, "passthrough">[];
                } & {
                    [k: string]: unknown;
                };
            } | {
                continuous: {
                    max_label: string;
                    max_value: number;
                    min_label: string;
                    min_value: number;
                    sub_steps: number;
                } & {
                    [k: string]: unknown;
                };
            }, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        randomize_order: z.ZodBoolean;
        required_reviews: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>>;
        section_questions: z.ZodDefault<z.ZodOptional<z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            text: z.ZodString;
            type: z.ZodType<"text" | {
                likert_scale: {
                    categories: z.objectOutputType<{
                        label: z.ZodString;
                        value: z.ZodNumber;
                    }, z.ZodTypeAny, "passthrough">[];
                } & {
                    [k: string]: unknown;
                };
            } | {
                continuous: {
                    max_label: string;
                    max_value: number;
                    min_label: string;
                    min_value: number;
                    sub_steps: number;
                } & {
                    [k: string]: unknown;
                };
            }, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">>>;
        type: z.ZodLiteral<"prioritization">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        follow_up_rounds_count: z.ZodNumber;
        root_questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            intent: z.ZodString;
            text: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        topic: z.ZodString;
        type: z.ZodLiteral<"thinkingspace">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
    request_user_share_permission: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    required: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    step_order: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
    tool_config: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
        admin_password: z.ZodString;
        admin_user: z.ZodString;
        description: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
        is_active: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
        label_seeds_as_conversation_starter: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
        poll_id: z.ZodString;
        required_votes: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        server_url: z.ZodString;
        show_remaining_statements: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
        strict_moderation: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
        topic: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
        type: z.ZodLiteral<"polis">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        pages: z.ZodArray<z.ZodType<z.objectOutputType<{
            text_content_id: z.ZodString;
        }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
            content: z.ZodString;
            type: z.ZodLiteral<"markdown">;
        }, z.ZodTypeAny, "passthrough">[], z.ZodTypeDef, any>, "many">;
        type: z.ZodLiteral<"learn">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        admin_password: z.ZodString;
        admin_user: z.ZodString;
        project_id: z.ZodString;
        server_url: z.ZodDefault<z.ZodOptional<z.ZodString>>;
        survey_id: z.ZodString;
        survey_url: z.ZodString;
        type: z.ZodLiteral<"heyform">;
        workspace_id: z.ZodString;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        max_time: z.ZodNumber;
        to_see: z.ZodNumber;
        type: z.ZodLiteral<"stories">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        topic: z.ZodString;
        type: z.ZodLiteral<"elicitationbot">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        alignment_question_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            text: z.ZodString;
            type: z.ZodType<"text" | {
                likert_scale: {
                    categories: z.objectOutputType<{
                        label: z.ZodString;
                        value: z.ZodNumber;
                    }, z.ZodTypeAny, "passthrough">[];
                } & {
                    [k: string]: unknown;
                };
            } | {
                continuous: {
                    max_label: string;
                    max_value: number;
                    min_label: string;
                    min_value: number;
                    sub_steps: number;
                } & {
                    [k: string]: unknown;
                };
            }, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        randomize_order: z.ZodBoolean;
        required_reviews: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>>;
        section_questions: z.ZodDefault<z.ZodOptional<z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            text: z.ZodString;
            type: z.ZodType<"text" | {
                likert_scale: {
                    categories: z.objectOutputType<{
                        label: z.ZodString;
                        value: z.ZodNumber;
                    }, z.ZodTypeAny, "passthrough">[];
                } & {
                    [k: string]: unknown;
                };
            } | {
                continuous: {
                    max_label: string;
                    max_value: number;
                    min_label: string;
                    min_value: number;
                    sub_steps: number;
                } & {
                    [k: string]: unknown;
                };
            }, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">>>;
        type: z.ZodLiteral<"prioritization">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        follow_up_rounds_count: z.ZodNumber;
        root_questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            intent: z.ZodString;
            text: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        topic: z.ZodString;
        type: z.ZodLiteral<"thinkingspace">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    activation_rule: z.ZodOptional<z.ZodUnion<[z.ZodType<"manual", z.ZodTypeDef, any>, z.ZodNull]>>;
    can_revisit: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    description: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    is_offline: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    name: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    preview_tool_config: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
        admin_password: z.ZodString;
        admin_user: z.ZodString;
        description: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
        is_active: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
        label_seeds_as_conversation_starter: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
        poll_id: z.ZodString;
        required_votes: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        server_url: z.ZodString;
        show_remaining_statements: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
        strict_moderation: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
        topic: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
        type: z.ZodLiteral<"polis">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        pages: z.ZodArray<z.ZodType<z.objectOutputType<{
            text_content_id: z.ZodString;
        }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
            content: z.ZodString;
            type: z.ZodLiteral<"markdown">;
        }, z.ZodTypeAny, "passthrough">[], z.ZodTypeDef, any>, "many">;
        type: z.ZodLiteral<"learn">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        admin_password: z.ZodString;
        admin_user: z.ZodString;
        project_id: z.ZodString;
        server_url: z.ZodDefault<z.ZodOptional<z.ZodString>>;
        survey_id: z.ZodString;
        survey_url: z.ZodString;
        type: z.ZodLiteral<"heyform">;
        workspace_id: z.ZodString;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        max_time: z.ZodNumber;
        to_see: z.ZodNumber;
        type: z.ZodLiteral<"stories">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        topic: z.ZodString;
        type: z.ZodLiteral<"elicitationbot">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        alignment_question_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            text: z.ZodString;
            type: z.ZodType<"text" | {
                likert_scale: {
                    categories: z.objectOutputType<{
                        label: z.ZodString;
                        value: z.ZodNumber;
                    }, z.ZodTypeAny, "passthrough">[];
                } & {
                    [k: string]: unknown;
                };
            } | {
                continuous: {
                    max_label: string;
                    max_value: number;
                    min_label: string;
                    min_value: number;
                    sub_steps: number;
                } & {
                    [k: string]: unknown;
                };
            }, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        randomize_order: z.ZodBoolean;
        required_reviews: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>>;
        section_questions: z.ZodDefault<z.ZodOptional<z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            text: z.ZodString;
            type: z.ZodType<"text" | {
                likert_scale: {
                    categories: z.objectOutputType<{
                        label: z.ZodString;
                        value: z.ZodNumber;
                    }, z.ZodTypeAny, "passthrough">[];
                } & {
                    [k: string]: unknown;
                };
            } | {
                continuous: {
                    max_label: string;
                    max_value: number;
                    min_label: string;
                    min_value: number;
                    sub_steps: number;
                } & {
                    [k: string]: unknown;
                };
            }, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">>>;
        type: z.ZodLiteral<"prioritization">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        follow_up_rounds_count: z.ZodNumber;
        root_questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            intent: z.ZodString;
            text: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        topic: z.ZodString;
        type: z.ZodLiteral<"thinkingspace">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
    request_user_share_permission: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    required: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    step_order: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
    tool_config: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
        admin_password: z.ZodString;
        admin_user: z.ZodString;
        description: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
        is_active: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
        label_seeds_as_conversation_starter: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
        poll_id: z.ZodString;
        required_votes: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        server_url: z.ZodString;
        show_remaining_statements: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
        strict_moderation: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
        topic: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
        type: z.ZodLiteral<"polis">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        pages: z.ZodArray<z.ZodType<z.objectOutputType<{
            text_content_id: z.ZodString;
        }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
            content: z.ZodString;
            type: z.ZodLiteral<"markdown">;
        }, z.ZodTypeAny, "passthrough">[], z.ZodTypeDef, any>, "many">;
        type: z.ZodLiteral<"learn">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        admin_password: z.ZodString;
        admin_user: z.ZodString;
        project_id: z.ZodString;
        server_url: z.ZodDefault<z.ZodOptional<z.ZodString>>;
        survey_id: z.ZodString;
        survey_url: z.ZodString;
        type: z.ZodLiteral<"heyform">;
        workspace_id: z.ZodString;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        max_time: z.ZodNumber;
        to_see: z.ZodNumber;
        type: z.ZodLiteral<"stories">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        topic: z.ZodString;
        type: z.ZodLiteral<"elicitationbot">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        alignment_question_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            text: z.ZodString;
            type: z.ZodType<"text" | {
                likert_scale: {
                    categories: z.objectOutputType<{
                        label: z.ZodString;
                        value: z.ZodNumber;
                    }, z.ZodTypeAny, "passthrough">[];
                } & {
                    [k: string]: unknown;
                };
            } | {
                continuous: {
                    max_label: string;
                    max_value: number;
                    min_label: string;
                    min_value: number;
                    sub_steps: number;
                } & {
                    [k: string]: unknown;
                };
            }, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        randomize_order: z.ZodBoolean;
        required_reviews: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>>;
        section_questions: z.ZodDefault<z.ZodOptional<z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            text: z.ZodString;
            type: z.ZodType<"text" | {
                likert_scale: {
                    categories: z.objectOutputType<{
                        label: z.ZodString;
                        value: z.ZodNumber;
                    }, z.ZodTypeAny, "passthrough">[];
                } & {
                    [k: string]: unknown;
                };
            } | {
                continuous: {
                    max_label: string;
                    max_value: number;
                    min_label: string;
                    min_value: number;
                    sub_steps: number;
                } & {
                    [k: string]: unknown;
                };
            }, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">>>;
        type: z.ZodLiteral<"prioritization">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        follow_up_rounds_count: z.ZodNumber;
        root_questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            intent: z.ZodString;
            text: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        topic: z.ZodString;
        type: z.ZodLiteral<"thinkingspace">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    activation_rule: z.ZodOptional<z.ZodUnion<[z.ZodType<"manual", z.ZodTypeDef, any>, z.ZodNull]>>;
    can_revisit: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    description: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    is_offline: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    name: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    preview_tool_config: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
        admin_password: z.ZodString;
        admin_user: z.ZodString;
        description: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
        is_active: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
        label_seeds_as_conversation_starter: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
        poll_id: z.ZodString;
        required_votes: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        server_url: z.ZodString;
        show_remaining_statements: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
        strict_moderation: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
        topic: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
        type: z.ZodLiteral<"polis">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        pages: z.ZodArray<z.ZodType<z.objectOutputType<{
            text_content_id: z.ZodString;
        }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
            content: z.ZodString;
            type: z.ZodLiteral<"markdown">;
        }, z.ZodTypeAny, "passthrough">[], z.ZodTypeDef, any>, "many">;
        type: z.ZodLiteral<"learn">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        admin_password: z.ZodString;
        admin_user: z.ZodString;
        project_id: z.ZodString;
        server_url: z.ZodDefault<z.ZodOptional<z.ZodString>>;
        survey_id: z.ZodString;
        survey_url: z.ZodString;
        type: z.ZodLiteral<"heyform">;
        workspace_id: z.ZodString;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        max_time: z.ZodNumber;
        to_see: z.ZodNumber;
        type: z.ZodLiteral<"stories">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        topic: z.ZodString;
        type: z.ZodLiteral<"elicitationbot">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        alignment_question_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            text: z.ZodString;
            type: z.ZodType<"text" | {
                likert_scale: {
                    categories: z.objectOutputType<{
                        label: z.ZodString;
                        value: z.ZodNumber;
                    }, z.ZodTypeAny, "passthrough">[];
                } & {
                    [k: string]: unknown;
                };
            } | {
                continuous: {
                    max_label: string;
                    max_value: number;
                    min_label: string;
                    min_value: number;
                    sub_steps: number;
                } & {
                    [k: string]: unknown;
                };
            }, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        randomize_order: z.ZodBoolean;
        required_reviews: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>>;
        section_questions: z.ZodDefault<z.ZodOptional<z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            text: z.ZodString;
            type: z.ZodType<"text" | {
                likert_scale: {
                    categories: z.objectOutputType<{
                        label: z.ZodString;
                        value: z.ZodNumber;
                    }, z.ZodTypeAny, "passthrough">[];
                } & {
                    [k: string]: unknown;
                };
            } | {
                continuous: {
                    max_label: string;
                    max_value: number;
                    min_label: string;
                    min_value: number;
                    sub_steps: number;
                } & {
                    [k: string]: unknown;
                };
            }, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">>>;
        type: z.ZodLiteral<"prioritization">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        follow_up_rounds_count: z.ZodNumber;
        root_questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            intent: z.ZodString;
            text: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        topic: z.ZodString;
        type: z.ZodLiteral<"thinkingspace">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
    request_user_share_permission: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    required: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    step_order: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
    tool_config: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
        admin_password: z.ZodString;
        admin_user: z.ZodString;
        description: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
        is_active: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
        label_seeds_as_conversation_starter: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
        poll_id: z.ZodString;
        required_votes: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        server_url: z.ZodString;
        show_remaining_statements: z.ZodDefault<z.ZodOptional<z.ZodBoolean>>;
        strict_moderation: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>>;
        topic: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>>;
        type: z.ZodLiteral<"polis">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        pages: z.ZodArray<z.ZodType<z.objectOutputType<{
            text_content_id: z.ZodString;
        }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
            content: z.ZodString;
            type: z.ZodLiteral<"markdown">;
        }, z.ZodTypeAny, "passthrough">[], z.ZodTypeDef, any>, "many">;
        type: z.ZodLiteral<"learn">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        admin_password: z.ZodString;
        admin_user: z.ZodString;
        project_id: z.ZodString;
        server_url: z.ZodDefault<z.ZodOptional<z.ZodString>>;
        survey_id: z.ZodString;
        survey_url: z.ZodString;
        type: z.ZodLiteral<"heyform">;
        workspace_id: z.ZodString;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        max_time: z.ZodNumber;
        to_see: z.ZodNumber;
        type: z.ZodLiteral<"stories">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        topic: z.ZodString;
        type: z.ZodLiteral<"elicitationbot">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        alignment_question_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            text: z.ZodString;
            type: z.ZodType<"text" | {
                likert_scale: {
                    categories: z.objectOutputType<{
                        label: z.ZodString;
                        value: z.ZodNumber;
                    }, z.ZodTypeAny, "passthrough">[];
                } & {
                    [k: string]: unknown;
                };
            } | {
                continuous: {
                    max_label: string;
                    max_value: number;
                    min_label: string;
                    min_value: number;
                    sub_steps: number;
                } & {
                    [k: string]: unknown;
                };
            }, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        randomize_order: z.ZodBoolean;
        required_reviews: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>>;
        section_questions: z.ZodDefault<z.ZodOptional<z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            text: z.ZodString;
            type: z.ZodType<"text" | {
                likert_scale: {
                    categories: z.objectOutputType<{
                        label: z.ZodString;
                        value: z.ZodNumber;
                    }, z.ZodTypeAny, "passthrough">[];
                } & {
                    [k: string]: unknown;
                };
            } | {
                continuous: {
                    max_label: string;
                    max_value: number;
                    min_label: string;
                    min_value: number;
                    sub_steps: number;
                } & {
                    [k: string]: unknown;
                };
            }, z.ZodTypeDef, any>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">>>;
        type: z.ZodLiteral<"prioritization">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        follow_up_rounds_count: z.ZodNumber;
        root_questions: z.ZodArray<z.ZodType<z.objectOutputType<{
            id: z.ZodString;
            intent: z.ZodString;
            text: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        topic: z.ZodString;
        type: z.ZodLiteral<"thinkingspace">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">>;
export type PartialWorkflowStep = z.infer<typeof _PartialWorkflowStep>;
export declare const PartialWorkflowStep: z.ZodType<PartialWorkflowStep, z.ZodTypeDef, any>;
declare const _UserProgressDto: z.ZodObject<{
    id: z.ZodString;
    permissionToShareWithOrganizers: z.ZodBoolean;
    status: z.ZodType<"not_started" | "in_progress" | "done", z.ZodTypeDef, any>;
    userId: z.ZodString;
    workflowStepId: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    id: z.ZodString;
    permissionToShareWithOrganizers: z.ZodBoolean;
    status: z.ZodType<"not_started" | "in_progress" | "done", z.ZodTypeDef, any>;
    userId: z.ZodString;
    workflowStepId: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    id: z.ZodString;
    permissionToShareWithOrganizers: z.ZodBoolean;
    status: z.ZodType<"not_started" | "in_progress" | "done", z.ZodTypeDef, any>;
    userId: z.ZodString;
    workflowStepId: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type UserProgressDto = z.infer<typeof _UserProgressDto>;
export declare const UserProgressDto: z.ZodType<UserProgressDto, z.ZodTypeDef, any>;
declare const _UpdateUserProgress: z.ZodObject<{
    permission_to_share_with_organizers: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    permission_to_share_with_other_participants: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    status: z.ZodOptional<z.ZodUnion<[z.ZodType<"not_started" | "in_progress" | "done", z.ZodTypeDef, any>, z.ZodNull]>>;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    permission_to_share_with_organizers: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    permission_to_share_with_other_participants: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    status: z.ZodOptional<z.ZodUnion<[z.ZodType<"not_started" | "in_progress" | "done", z.ZodTypeDef, any>, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    permission_to_share_with_organizers: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    permission_to_share_with_other_participants: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    status: z.ZodOptional<z.ZodUnion<[z.ZodType<"not_started" | "in_progress" | "done", z.ZodTypeDef, any>, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">>;
export type UpdateUserProgress = z.infer<typeof _UpdateUserProgress>;
export declare const UpdateUserProgress: z.ZodType<UpdateUserProgress, z.ZodTypeDef, any>;
declare const _RecruitmentTargetDto: z.ZodObject<{
    bucket: z.ZodString;
    createdAt: z.ZodString;
    id: z.ZodString;
    metric: z.ZodString;
    targetCount: z.ZodNumber;
    updatedAt: z.ZodString;
    workflowId: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    bucket: z.ZodString;
    createdAt: z.ZodString;
    id: z.ZodString;
    metric: z.ZodString;
    targetCount: z.ZodNumber;
    updatedAt: z.ZodString;
    workflowId: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    bucket: z.ZodString;
    createdAt: z.ZodString;
    id: z.ZodString;
    metric: z.ZodString;
    targetCount: z.ZodNumber;
    updatedAt: z.ZodString;
    workflowId: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type RecruitmentTargetDto = z.infer<typeof _RecruitmentTargetDto>;
export declare const RecruitmentTargetDto: z.ZodType<RecruitmentTargetDto, z.ZodTypeDef, any>;
declare const _CreateRecruitmentTarget: z.ZodObject<{
    bucket: z.ZodString;
    metric: z.ZodString;
    target_count: z.ZodNumber;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    bucket: z.ZodString;
    metric: z.ZodString;
    target_count: z.ZodNumber;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    bucket: z.ZodString;
    metric: z.ZodString;
    target_count: z.ZodNumber;
}, z.ZodTypeAny, "passthrough">>;
export type CreateRecruitmentTarget = z.infer<typeof _CreateRecruitmentTarget>;
export declare const CreateRecruitmentTarget: z.ZodType<CreateRecruitmentTarget, z.ZodTypeDef, any>;
declare const _PartialRecruitmentTarget: z.ZodObject<{
    bucket: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    metric: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    target_count: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    bucket: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    metric: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    target_count: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    bucket: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    metric: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    target_count: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">>;
export type PartialRecruitmentTarget = z.infer<typeof _PartialRecruitmentTarget>;
export declare const PartialRecruitmentTarget: z.ZodType<PartialRecruitmentTarget, z.ZodTypeDef, any>;
declare const _InviteType: z.ZodUnion<[z.ZodObject<{
    email: z.ZodString;
}, "strip", z.ZodTypeAny, {
    email: string;
}, {
    email: string;
}>, z.ZodObject<{
    user: z.ZodString;
}, "strip", z.ZodTypeAny, {
    user: string;
}, {
    user: string;
}>, z.ZodLiteral<"singleuse">, z.ZodLiteral<"open">]>;
export type InviteType = z.infer<typeof _InviteType>;
export declare const InviteType: z.ZodType<InviteType, z.ZodTypeDef, any>;
declare const _LoginBehaviour: z.ZodUnion<[z.ZodLiteral<"manual">, z.ZodLiteral<"auto_create_guest">]>;
export type LoginBehaviour = z.infer<typeof _LoginBehaviour>;
export declare const LoginBehaviour: z.ZodType<LoginBehaviour, z.ZodTypeDef, any>;
declare const _InviteStatus: z.ZodUnion<[z.ZodLiteral<"pending">, z.ZodLiteral<"open">, z.ZodLiteral<"accepted">, z.ZodLiteral<"rejected">, z.ZodLiteral<"expired">]>;
export type InviteStatus = z.infer<typeof _InviteStatus>;
export declare const InviteStatus: z.ZodType<InviteStatus, z.ZodTypeDef, any>;
declare const _InviteDto: z.ZodObject<{
    acceptCount: z.ZodNumber;
    conversationId: z.ZodString;
    createdAt: z.ZodString;
    createdBy: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    eventId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    expiresAt: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    id: z.ZodString;
    inviteType: z.ZodType<{
        email: string;
    } | {
        user: string;
    } | "singleuse" | "open", z.ZodTypeDef, any>;
    label: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    loginBehaviour: z.ZodType<"manual" | "auto_create_guest", z.ZodTypeDef, any>;
    status: z.ZodType<"accepted" | "rejected" | "pending" | "open" | "expired", z.ZodTypeDef, any>;
    tags: z.ZodArray<z.ZodString, "many">;
    workflowId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    workflowStepId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    acceptCount: z.ZodNumber;
    conversationId: z.ZodString;
    createdAt: z.ZodString;
    createdBy: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    eventId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    expiresAt: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    id: z.ZodString;
    inviteType: z.ZodType<{
        email: string;
    } | {
        user: string;
    } | "singleuse" | "open", z.ZodTypeDef, any>;
    label: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    loginBehaviour: z.ZodType<"manual" | "auto_create_guest", z.ZodTypeDef, any>;
    status: z.ZodType<"accepted" | "rejected" | "pending" | "open" | "expired", z.ZodTypeDef, any>;
    tags: z.ZodArray<z.ZodString, "many">;
    workflowId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    workflowStepId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    acceptCount: z.ZodNumber;
    conversationId: z.ZodString;
    createdAt: z.ZodString;
    createdBy: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    eventId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    expiresAt: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    id: z.ZodString;
    inviteType: z.ZodType<{
        email: string;
    } | {
        user: string;
    } | "singleuse" | "open", z.ZodTypeDef, any>;
    label: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    loginBehaviour: z.ZodType<"manual" | "auto_create_guest", z.ZodTypeDef, any>;
    status: z.ZodType<"accepted" | "rejected" | "pending" | "open" | "expired", z.ZodTypeDef, any>;
    tags: z.ZodArray<z.ZodString, "many">;
    workflowId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    workflowStepId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">>;
export type InviteDto = z.infer<typeof _InviteDto>;
export declare const InviteDto: z.ZodType<InviteDto, z.ZodTypeDef, any>;
declare const _CreateInviteDTO: z.ZodObject<{
    event_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    expires_at: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    invite_type: z.ZodType<{
        email: string;
    } | {
        user: string;
    } | "singleuse" | "open", z.ZodTypeDef, any>;
    label: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    login_behaviour: z.ZodOptional<z.ZodType<"manual" | "auto_create_guest", z.ZodTypeDef, any>>;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    event_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    expires_at: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    invite_type: z.ZodType<{
        email: string;
    } | {
        user: string;
    } | "singleuse" | "open", z.ZodTypeDef, any>;
    label: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    login_behaviour: z.ZodOptional<z.ZodType<"manual" | "auto_create_guest", z.ZodTypeDef, any>>;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    event_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    expires_at: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    invite_type: z.ZodType<{
        email: string;
    } | {
        user: string;
    } | "singleuse" | "open", z.ZodTypeDef, any>;
    label: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    login_behaviour: z.ZodOptional<z.ZodType<"manual" | "auto_create_guest", z.ZodTypeDef, any>>;
}, z.ZodTypeAny, "passthrough">>;
export type CreateInviteDTO = z.infer<typeof _CreateInviteDTO>;
export declare const CreateInviteDTO: z.ZodType<CreateInviteDTO, z.ZodTypeDef, any>;
declare const _PartialInvite: z.ZodObject<{
    accept_count: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
    conversation_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    event_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    expires_at: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    invite_type: z.ZodOptional<z.ZodUnion<[z.ZodType<{
        email: string;
    } | {
        user: string;
    } | "singleuse" | "open", z.ZodTypeDef, any>, z.ZodNull]>>;
    label: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    login_behaviour: z.ZodOptional<z.ZodUnion<[z.ZodType<"manual" | "auto_create_guest", z.ZodTypeDef, any>, z.ZodNull]>>;
    status: z.ZodOptional<z.ZodUnion<[z.ZodType<"accepted" | "rejected" | "pending" | "open" | "expired", z.ZodTypeDef, any>, z.ZodNull]>>;
    tags: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodString, "many">, z.ZodNull]>>;
    workflow_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    workflow_step_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    accept_count: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
    conversation_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    event_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    expires_at: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    invite_type: z.ZodOptional<z.ZodUnion<[z.ZodType<{
        email: string;
    } | {
        user: string;
    } | "singleuse" | "open", z.ZodTypeDef, any>, z.ZodNull]>>;
    label: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    login_behaviour: z.ZodOptional<z.ZodUnion<[z.ZodType<"manual" | "auto_create_guest", z.ZodTypeDef, any>, z.ZodNull]>>;
    status: z.ZodOptional<z.ZodUnion<[z.ZodType<"accepted" | "rejected" | "pending" | "open" | "expired", z.ZodTypeDef, any>, z.ZodNull]>>;
    tags: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodString, "many">, z.ZodNull]>>;
    workflow_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    workflow_step_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    accept_count: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
    conversation_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    event_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    expires_at: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    invite_type: z.ZodOptional<z.ZodUnion<[z.ZodType<{
        email: string;
    } | {
        user: string;
    } | "singleuse" | "open", z.ZodTypeDef, any>, z.ZodNull]>>;
    label: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    login_behaviour: z.ZodOptional<z.ZodUnion<[z.ZodType<"manual" | "auto_create_guest", z.ZodTypeDef, any>, z.ZodNull]>>;
    status: z.ZodOptional<z.ZodUnion<[z.ZodType<"accepted" | "rejected" | "pending" | "open" | "expired", z.ZodTypeDef, any>, z.ZodNull]>>;
    tags: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodString, "many">, z.ZodNull]>>;
    workflow_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    workflow_step_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">>;
export type PartialInvite = z.infer<typeof _PartialInvite>;
export declare const PartialInvite: z.ZodType<PartialInvite, z.ZodTypeDef, any>;
declare const _DailyResponseStats: z.ZodObject<{
    accept: z.ZodNumber;
    day: z.ZodString;
    reject: z.ZodNumber;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    accept: z.ZodNumber;
    day: z.ZodString;
    reject: z.ZodNumber;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    accept: z.ZodNumber;
    day: z.ZodString;
    reject: z.ZodNumber;
}, z.ZodTypeAny, "passthrough">>;
export type DailyResponseStats = z.infer<typeof _DailyResponseStats>;
export declare const DailyResponseStats: z.ZodType<DailyResponseStats, z.ZodTypeDef, any>;
declare const _PolisReport: z.ZodNull;
export type PolisReport = z.infer<typeof _PolisReport>;
export declare const PolisReport: z.ZodType<PolisReport, z.ZodTypeDef, any>;
declare const _HeyFormReport: z.ZodNull;
export type HeyFormReport = z.infer<typeof _HeyFormReport>;
export declare const HeyFormReport: z.ZodType<HeyFormReport, z.ZodTypeDef, any>;
declare const _LearnReport: z.ZodNull;
export type LearnReport = z.infer<typeof _LearnReport>;
export declare const LearnReport: z.ZodType<LearnReport, z.ZodTypeDef, any>;
declare const _StoriesReport: z.ZodNull;
export type StoriesReport = z.infer<typeof _StoriesReport>;
export declare const StoriesReport: z.ZodType<StoriesReport, z.ZodTypeDef, any>;
declare const _ElicitationBotReport: z.ZodNull;
export type ElicitationBotReport = z.infer<typeof _ElicitationBotReport>;
export declare const ElicitationBotReport: z.ZodType<ElicitationBotReport, z.ZodTypeDef, any>;
declare const _PrioritizationReport: z.ZodNull;
export type PrioritizationReport = z.infer<typeof _PrioritizationReport>;
export declare const PrioritizationReport: z.ZodType<PrioritizationReport, z.ZodTypeDef, any>;
declare const _ThinkingSpaceReport: z.ZodNull;
export type ThinkingSpaceReport = z.infer<typeof _ThinkingSpaceReport>;
export declare const ThinkingSpaceReport: z.ZodType<ThinkingSpaceReport, z.ZodTypeDef, any>;
declare const _ReportConfig: z.ZodUnion<[z.ZodObject<{
    Polis: z.ZodType<null, z.ZodTypeDef, any>;
}, "strip", z.ZodTypeAny, {
    Polis: null;
}, {
    Polis?: any;
}>, z.ZodObject<{
    HeyForm: z.ZodType<null, z.ZodTypeDef, any>;
}, "strip", z.ZodTypeAny, {
    HeyForm: null;
}, {
    HeyForm?: any;
}>, z.ZodObject<{
    Learn: z.ZodType<null, z.ZodTypeDef, any>;
}, "strip", z.ZodTypeAny, {
    Learn: null;
}, {
    Learn?: any;
}>, z.ZodObject<{
    Stories: z.ZodType<null, z.ZodTypeDef, any>;
}, "strip", z.ZodTypeAny, {
    Stories: null;
}, {
    Stories?: any;
}>, z.ZodObject<{
    ElicitationBot: z.ZodType<null, z.ZodTypeDef, any>;
}, "strip", z.ZodTypeAny, {
    ElicitationBot: null;
}, {
    ElicitationBot?: any;
}>, z.ZodObject<{
    Prioritization: z.ZodType<null, z.ZodTypeDef, any>;
}, "strip", z.ZodTypeAny, {
    Prioritization: null;
}, {
    Prioritization?: any;
}>, z.ZodObject<{
    ThinkingSpace: z.ZodType<null, z.ZodTypeDef, any>;
}, "strip", z.ZodTypeAny, {
    ThinkingSpace: null;
}, {
    ThinkingSpace?: any;
}>]>;
export type ReportConfig = z.infer<typeof _ReportConfig>;
export declare const ReportConfig: z.ZodType<ReportConfig, z.ZodTypeDef, any>;
declare const _ReportSectionConfig: z.ZodObject<{
    ai_generated: z.ZodBoolean;
    config: z.ZodType<{
        Polis: null;
    } | {
        HeyForm: null;
    } | {
        Learn: null;
    } | {
        Stories: null;
    } | {
        ElicitationBot: null;
    } | {
        Prioritization: null;
    } | {
        ThinkingSpace: null;
    }, z.ZodTypeDef, any>;
    verified: z.ZodBoolean;
    workflow_step_id: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    ai_generated: z.ZodBoolean;
    config: z.ZodType<{
        Polis: null;
    } | {
        HeyForm: null;
    } | {
        Learn: null;
    } | {
        Stories: null;
    } | {
        ElicitationBot: null;
    } | {
        Prioritization: null;
    } | {
        ThinkingSpace: null;
    }, z.ZodTypeDef, any>;
    verified: z.ZodBoolean;
    workflow_step_id: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    ai_generated: z.ZodBoolean;
    config: z.ZodType<{
        Polis: null;
    } | {
        HeyForm: null;
    } | {
        Learn: null;
    } | {
        Stories: null;
    } | {
        ElicitationBot: null;
    } | {
        Prioritization: null;
    } | {
        ThinkingSpace: null;
    }, z.ZodTypeDef, any>;
    verified: z.ZodBoolean;
    workflow_step_id: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type ReportSectionConfig = z.infer<typeof _ReportSectionConfig>;
export declare const ReportSectionConfig: z.ZodType<ReportSectionConfig, z.ZodTypeDef, any>;
declare const _ReportSectionConfigs: z.ZodArray<z.ZodType<z.objectOutputType<{
    ai_generated: z.ZodBoolean;
    config: z.ZodType<{
        Polis: null;
    } | {
        HeyForm: null;
    } | {
        Learn: null;
    } | {
        Stories: null;
    } | {
        ElicitationBot: null;
    } | {
        Prioritization: null;
    } | {
        ThinkingSpace: null;
    }, z.ZodTypeDef, any>;
    verified: z.ZodBoolean;
    workflow_step_id: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
export type ReportSectionConfigs = z.infer<typeof _ReportSectionConfigs>;
export declare const ReportSectionConfigs: z.ZodType<ReportSectionConfigs, z.ZodTypeDef, any>;
declare const _Translation5: z.ZodObject<{
    textContent: z.ZodType<z.objectOutputType<{
        format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
        id: z.ZodString;
        primaryLocale: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
        aiGenerated: z.ZodBoolean;
        content: z.ZodString;
        contentId: z.ZodString;
        id: z.ZodString;
        locale: z.ZodString;
        requiresValidation: z.ZodBoolean;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    textContent: z.ZodType<z.objectOutputType<{
        format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
        id: z.ZodString;
        primaryLocale: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
        aiGenerated: z.ZodBoolean;
        content: z.ZodString;
        contentId: z.ZodString;
        id: z.ZodString;
        locale: z.ZodString;
        requiresValidation: z.ZodBoolean;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    textContent: z.ZodType<z.objectOutputType<{
        format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
        id: z.ZodString;
        primaryLocale: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
        aiGenerated: z.ZodBoolean;
        content: z.ZodString;
        contentId: z.ZodString;
        id: z.ZodString;
        locale: z.ZodString;
        requiresValidation: z.ZodBoolean;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
}, z.ZodTypeAny, "passthrough">>;
export type Translation5 = z.infer<typeof _Translation5>;
export declare const Translation5: z.ZodType<Translation5, z.ZodTypeDef, any>;
declare const _ReportTranslations: z.ZodObject<{
    summary: z.ZodType<z.objectOutputType<{
        textContent: z.ZodType<z.objectOutputType<{
            format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
            id: z.ZodString;
            primaryLocale: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
            aiGenerated: z.ZodBoolean;
            content: z.ZodString;
            contentId: z.ZodString;
            id: z.ZodString;
            locale: z.ZodString;
            requiresValidation: z.ZodBoolean;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    summary: z.ZodType<z.objectOutputType<{
        textContent: z.ZodType<z.objectOutputType<{
            format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
            id: z.ZodString;
            primaryLocale: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
            aiGenerated: z.ZodBoolean;
            content: z.ZodString;
            contentId: z.ZodString;
            id: z.ZodString;
            locale: z.ZodString;
            requiresValidation: z.ZodBoolean;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    summary: z.ZodType<z.objectOutputType<{
        textContent: z.ZodType<z.objectOutputType<{
            format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
            id: z.ZodString;
            primaryLocale: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
            aiGenerated: z.ZodBoolean;
            content: z.ZodString;
            contentId: z.ZodString;
            id: z.ZodString;
            locale: z.ZodString;
            requiresValidation: z.ZodBoolean;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
}, z.ZodTypeAny, "passthrough">>;
export type ReportTranslations = z.infer<typeof _ReportTranslations>;
export declare const ReportTranslations: z.ZodType<ReportTranslations, z.ZodTypeDef, any>;
declare const _ReportWithTranslations: z.ZodObject<{
    conversationId: z.ZodString;
    createdAt: z.ZodString;
    id: z.ZodString;
    isPublic: z.ZodBoolean;
    sectionConfigs: z.ZodType<z.objectOutputType<{
        ai_generated: z.ZodBoolean;
        config: z.ZodType<{
            Polis: null;
        } | {
            HeyForm: null;
        } | {
            Learn: null;
        } | {
            Stories: null;
        } | {
            ElicitationBot: null;
        } | {
            Prioritization: null;
        } | {
            ThinkingSpace: null;
        }, z.ZodTypeDef, any>;
        verified: z.ZodBoolean;
        workflow_step_id: z.ZodString;
    }, z.ZodTypeAny, "passthrough">[], z.ZodTypeDef, any>;
    summary: z.ZodString;
    translations: z.ZodType<z.objectOutputType<{
        summary: z.ZodType<z.objectOutputType<{
            textContent: z.ZodType<z.objectOutputType<{
                format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                id: z.ZodString;
                primaryLocale: z.ZodString;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                aiGenerated: z.ZodBoolean;
                content: z.ZodString;
                contentId: z.ZodString;
                id: z.ZodString;
                locale: z.ZodString;
                requiresValidation: z.ZodBoolean;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    updatedAt: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    conversationId: z.ZodString;
    createdAt: z.ZodString;
    id: z.ZodString;
    isPublic: z.ZodBoolean;
    sectionConfigs: z.ZodType<z.objectOutputType<{
        ai_generated: z.ZodBoolean;
        config: z.ZodType<{
            Polis: null;
        } | {
            HeyForm: null;
        } | {
            Learn: null;
        } | {
            Stories: null;
        } | {
            ElicitationBot: null;
        } | {
            Prioritization: null;
        } | {
            ThinkingSpace: null;
        }, z.ZodTypeDef, any>;
        verified: z.ZodBoolean;
        workflow_step_id: z.ZodString;
    }, z.ZodTypeAny, "passthrough">[], z.ZodTypeDef, any>;
    summary: z.ZodString;
    translations: z.ZodType<z.objectOutputType<{
        summary: z.ZodType<z.objectOutputType<{
            textContent: z.ZodType<z.objectOutputType<{
                format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                id: z.ZodString;
                primaryLocale: z.ZodString;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                aiGenerated: z.ZodBoolean;
                content: z.ZodString;
                contentId: z.ZodString;
                id: z.ZodString;
                locale: z.ZodString;
                requiresValidation: z.ZodBoolean;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    updatedAt: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    conversationId: z.ZodString;
    createdAt: z.ZodString;
    id: z.ZodString;
    isPublic: z.ZodBoolean;
    sectionConfigs: z.ZodType<z.objectOutputType<{
        ai_generated: z.ZodBoolean;
        config: z.ZodType<{
            Polis: null;
        } | {
            HeyForm: null;
        } | {
            Learn: null;
        } | {
            Stories: null;
        } | {
            ElicitationBot: null;
        } | {
            Prioritization: null;
        } | {
            ThinkingSpace: null;
        }, z.ZodTypeDef, any>;
        verified: z.ZodBoolean;
        workflow_step_id: z.ZodString;
    }, z.ZodTypeAny, "passthrough">[], z.ZodTypeDef, any>;
    summary: z.ZodString;
    translations: z.ZodType<z.objectOutputType<{
        summary: z.ZodType<z.objectOutputType<{
            textContent: z.ZodType<z.objectOutputType<{
                format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                id: z.ZodString;
                primaryLocale: z.ZodString;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                aiGenerated: z.ZodBoolean;
                content: z.ZodString;
                contentId: z.ZodString;
                id: z.ZodString;
                locale: z.ZodString;
                requiresValidation: z.ZodBoolean;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    updatedAt: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type ReportWithTranslations = z.infer<typeof _ReportWithTranslations>;
export declare const ReportWithTranslations: z.ZodType<ReportWithTranslations, z.ZodTypeDef, any>;
declare const _LocalizedReportDto: z.ZodObject<{
    conversationId: z.ZodString;
    createdAt: z.ZodString;
    id: z.ZodString;
    isPublic: z.ZodBoolean;
    sectionConfigs: z.ZodType<z.objectOutputType<{
        ai_generated: z.ZodBoolean;
        config: z.ZodType<{
            Polis: null;
        } | {
            HeyForm: null;
        } | {
            Learn: null;
        } | {
            Stories: null;
        } | {
            ElicitationBot: null;
        } | {
            Prioritization: null;
        } | {
            ThinkingSpace: null;
        }, z.ZodTypeDef, any>;
        verified: z.ZodBoolean;
        workflow_step_id: z.ZodString;
    }, z.ZodTypeAny, "passthrough">[], z.ZodTypeDef, any>;
    summary: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    conversationId: z.ZodString;
    createdAt: z.ZodString;
    id: z.ZodString;
    isPublic: z.ZodBoolean;
    sectionConfigs: z.ZodType<z.objectOutputType<{
        ai_generated: z.ZodBoolean;
        config: z.ZodType<{
            Polis: null;
        } | {
            HeyForm: null;
        } | {
            Learn: null;
        } | {
            Stories: null;
        } | {
            ElicitationBot: null;
        } | {
            Prioritization: null;
        } | {
            ThinkingSpace: null;
        }, z.ZodTypeDef, any>;
        verified: z.ZodBoolean;
        workflow_step_id: z.ZodString;
    }, z.ZodTypeAny, "passthrough">[], z.ZodTypeDef, any>;
    summary: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    conversationId: z.ZodString;
    createdAt: z.ZodString;
    id: z.ZodString;
    isPublic: z.ZodBoolean;
    sectionConfigs: z.ZodType<z.objectOutputType<{
        ai_generated: z.ZodBoolean;
        config: z.ZodType<{
            Polis: null;
        } | {
            HeyForm: null;
        } | {
            Learn: null;
        } | {
            Stories: null;
        } | {
            ElicitationBot: null;
        } | {
            Prioritization: null;
        } | {
            ThinkingSpace: null;
        }, z.ZodTypeDef, any>;
        verified: z.ZodBoolean;
        workflow_step_id: z.ZodString;
    }, z.ZodTypeAny, "passthrough">[], z.ZodTypeDef, any>;
    summary: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type LocalizedReportDto = z.infer<typeof _LocalizedReportDto>;
export declare const LocalizedReportDto: z.ZodType<LocalizedReportDto, z.ZodTypeDef, any>;
declare const _FullReportDto: z.ZodUnion<[z.ZodType<z.objectOutputType<{
    conversationId: z.ZodString;
    createdAt: z.ZodString;
    id: z.ZodString;
    isPublic: z.ZodBoolean;
    sectionConfigs: z.ZodType<z.objectOutputType<{
        ai_generated: z.ZodBoolean;
        config: z.ZodType<{
            Polis: null;
        } | {
            HeyForm: null;
        } | {
            Learn: null;
        } | {
            Stories: null;
        } | {
            ElicitationBot: null;
        } | {
            Prioritization: null;
        } | {
            ThinkingSpace: null;
        }, z.ZodTypeDef, any>;
        verified: z.ZodBoolean;
        workflow_step_id: z.ZodString;
    }, z.ZodTypeAny, "passthrough">[], z.ZodTypeDef, any>;
    summary: z.ZodString;
    translations: z.ZodType<z.objectOutputType<{
        summary: z.ZodType<z.objectOutputType<{
            textContent: z.ZodType<z.objectOutputType<{
                format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                id: z.ZodString;
                primaryLocale: z.ZodString;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                aiGenerated: z.ZodBoolean;
                content: z.ZodString;
                contentId: z.ZodString;
                id: z.ZodString;
                locale: z.ZodString;
                requiresValidation: z.ZodBoolean;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    updatedAt: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodType<z.objectOutputType<{
    conversationId: z.ZodString;
    createdAt: z.ZodString;
    id: z.ZodString;
    isPublic: z.ZodBoolean;
    sectionConfigs: z.ZodType<z.objectOutputType<{
        ai_generated: z.ZodBoolean;
        config: z.ZodType<{
            Polis: null;
        } | {
            HeyForm: null;
        } | {
            Learn: null;
        } | {
            Stories: null;
        } | {
            ElicitationBot: null;
        } | {
            Prioritization: null;
        } | {
            ThinkingSpace: null;
        }, z.ZodTypeDef, any>;
        verified: z.ZodBoolean;
        workflow_step_id: z.ZodString;
    }, z.ZodTypeAny, "passthrough">[], z.ZodTypeDef, any>;
    summary: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>]>;
export type FullReportDto = z.infer<typeof _FullReportDto>;
export declare const FullReportDto: z.ZodType<FullReportDto, z.ZodTypeDef, any>;
declare const _PartialReport: z.ZodObject<{
    conversation_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    is_public: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    section_configs: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
        ai_generated: z.ZodBoolean;
        config: z.ZodType<{
            Polis: null;
        } | {
            HeyForm: null;
        } | {
            Learn: null;
        } | {
            Stories: null;
        } | {
            ElicitationBot: null;
        } | {
            Prioritization: null;
        } | {
            ThinkingSpace: null;
        }, z.ZodTypeDef, any>;
        verified: z.ZodBoolean;
        workflow_step_id: z.ZodString;
    }, z.ZodTypeAny, "passthrough">[], z.ZodTypeDef, any>, z.ZodNull]>>;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    conversation_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    is_public: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    section_configs: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
        ai_generated: z.ZodBoolean;
        config: z.ZodType<{
            Polis: null;
        } | {
            HeyForm: null;
        } | {
            Learn: null;
        } | {
            Stories: null;
        } | {
            ElicitationBot: null;
        } | {
            Prioritization: null;
        } | {
            ThinkingSpace: null;
        }, z.ZodTypeDef, any>;
        verified: z.ZodBoolean;
        workflow_step_id: z.ZodString;
    }, z.ZodTypeAny, "passthrough">[], z.ZodTypeDef, any>, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    conversation_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    is_public: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    section_configs: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
        ai_generated: z.ZodBoolean;
        config: z.ZodType<{
            Polis: null;
        } | {
            HeyForm: null;
        } | {
            Learn: null;
        } | {
            Stories: null;
        } | {
            ElicitationBot: null;
        } | {
            Prioritization: null;
        } | {
            ThinkingSpace: null;
        }, z.ZodTypeDef, any>;
        verified: z.ZodBoolean;
        workflow_step_id: z.ZodString;
    }, z.ZodTypeAny, "passthrough">[], z.ZodTypeDef, any>, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">>;
export type PartialReport = z.infer<typeof _PartialReport>;
export declare const PartialReport: z.ZodType<PartialReport, z.ZodTypeDef, any>;
declare const _ReportDto: z.ZodObject<{
    conversationId: z.ZodString;
    createdAt: z.ZodString;
    id: z.ZodString;
    isPublic: z.ZodBoolean;
    sectionConfigs: z.ZodType<z.objectOutputType<{
        ai_generated: z.ZodBoolean;
        config: z.ZodType<{
            Polis: null;
        } | {
            HeyForm: null;
        } | {
            Learn: null;
        } | {
            Stories: null;
        } | {
            ElicitationBot: null;
        } | {
            Prioritization: null;
        } | {
            ThinkingSpace: null;
        }, z.ZodTypeDef, any>;
        verified: z.ZodBoolean;
        workflow_step_id: z.ZodString;
    }, z.ZodTypeAny, "passthrough">[], z.ZodTypeDef, any>;
    summary: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    conversationId: z.ZodString;
    createdAt: z.ZodString;
    id: z.ZodString;
    isPublic: z.ZodBoolean;
    sectionConfigs: z.ZodType<z.objectOutputType<{
        ai_generated: z.ZodBoolean;
        config: z.ZodType<{
            Polis: null;
        } | {
            HeyForm: null;
        } | {
            Learn: null;
        } | {
            Stories: null;
        } | {
            ElicitationBot: null;
        } | {
            Prioritization: null;
        } | {
            ThinkingSpace: null;
        }, z.ZodTypeDef, any>;
        verified: z.ZodBoolean;
        workflow_step_id: z.ZodString;
    }, z.ZodTypeAny, "passthrough">[], z.ZodTypeDef, any>;
    summary: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    conversationId: z.ZodString;
    createdAt: z.ZodString;
    id: z.ZodString;
    isPublic: z.ZodBoolean;
    sectionConfigs: z.ZodType<z.objectOutputType<{
        ai_generated: z.ZodBoolean;
        config: z.ZodType<{
            Polis: null;
        } | {
            HeyForm: null;
        } | {
            Learn: null;
        } | {
            Stories: null;
        } | {
            ElicitationBot: null;
        } | {
            Prioritization: null;
        } | {
            ThinkingSpace: null;
        }, z.ZodTypeDef, any>;
        verified: z.ZodBoolean;
        workflow_step_id: z.ZodString;
    }, z.ZodTypeAny, "passthrough">[], z.ZodTypeDef, any>;
    summary: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type ReportDto = z.infer<typeof _ReportDto>;
export declare const ReportDto: z.ZodType<ReportDto, z.ZodTypeDef, any>;
declare const _ReportImpactDto: z.ZodObject<{
    createdAt: z.ZodString;
    createdBy: z.ZodString;
    details: z.ZodString;
    id: z.ZodString;
    kind: z.ZodString;
    reportId: z.ZodString;
    title: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    createdAt: z.ZodString;
    createdBy: z.ZodString;
    details: z.ZodString;
    id: z.ZodString;
    kind: z.ZodString;
    reportId: z.ZodString;
    title: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    createdAt: z.ZodString;
    createdBy: z.ZodString;
    details: z.ZodString;
    id: z.ZodString;
    kind: z.ZodString;
    reportId: z.ZodString;
    title: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type ReportImpactDto = z.infer<typeof _ReportImpactDto>;
export declare const ReportImpactDto: z.ZodType<ReportImpactDto, z.ZodTypeDef, any>;
declare const _PartialReportImpact: z.ZodObject<{
    created_at: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    created_by: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    details: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    kind: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    report_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    title: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    updated_at: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    created_at: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    created_by: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    details: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    kind: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    report_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    title: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    updated_at: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    created_at: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    created_by: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    details: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    kind: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    report_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    title: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    updated_at: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">>;
export type PartialReportImpact = z.infer<typeof _PartialReportImpact>;
export declare const PartialReportImpact: z.ZodType<PartialReportImpact, z.ZodTypeDef, any>;
declare const _CreateImpactDTO: z.ZodObject<{
    details: z.ZodString;
    kind: z.ZodString;
    title: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    details: z.ZodString;
    kind: z.ZodString;
    title: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    details: z.ZodString;
    kind: z.ZodString;
    title: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type CreateImpactDTO = z.infer<typeof _CreateImpactDTO>;
export declare const CreateImpactDTO: z.ZodType<CreateImpactDTO, z.ZodTypeDef, any>;
declare const _FeedbackDto: z.ZodObject<{
    content: z.ZodString;
    conversationId: z.ZodString;
    id: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    content: z.ZodString;
    conversationId: z.ZodString;
    id: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    content: z.ZodString;
    conversationId: z.ZodString;
    id: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type FeedbackDto = z.infer<typeof _FeedbackDto>;
export declare const FeedbackDto: z.ZodType<FeedbackDto, z.ZodTypeDef, any>;
declare const _CreateFeedbackDTO: z.ZodObject<{
    content: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    content: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    content: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type CreateFeedbackDTO = z.infer<typeof _CreateFeedbackDTO>;
export declare const CreateFeedbackDTO: z.ZodType<CreateFeedbackDTO, z.ZodTypeDef, any>;
declare const _PartialFeedback: z.ZodObject<{
    content: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    content: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    content: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">>;
export type PartialFeedback = z.infer<typeof _PartialFeedback>;
export declare const PartialFeedback: z.ZodType<PartialFeedback, z.ZodTypeDef, any>;
declare const _ComhairleLlm: z.ZodObject<{
    model_name: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    model_name: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    model_name: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">>;
export type ComhairleLlm = z.infer<typeof _ComhairleLlm>;
export declare const ComhairleLlm: z.ZodType<ComhairleLlm, z.ZodTypeDef, any>;
declare const _ComhairlePrompt: z.ZodObject<{
    cross_languages: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodString, "many">, z.ZodNull]>>;
    empty_response: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    llm_prompt: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    opener: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    cross_languages: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodString, "many">, z.ZodNull]>>;
    empty_response: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    llm_prompt: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    opener: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    cross_languages: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodString, "many">, z.ZodNull]>>;
    empty_response: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    llm_prompt: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    opener: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">>;
export type ComhairlePrompt = z.infer<typeof _ComhairlePrompt>;
export declare const ComhairlePrompt: z.ZodType<ComhairlePrompt, z.ZodTypeDef, any>;
declare const _ComhairleChat: z.ZodObject<{
    id: z.ZodString;
    knowledge_base_ids: z.ZodArray<z.ZodString, "many">;
    llm_model: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
        model_name: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
    name: z.ZodString;
    prompt: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
        cross_languages: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodString, "many">, z.ZodNull]>>;
        empty_response: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        llm_prompt: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        opener: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    id: z.ZodString;
    knowledge_base_ids: z.ZodArray<z.ZodString, "many">;
    llm_model: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
        model_name: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
    name: z.ZodString;
    prompt: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
        cross_languages: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodString, "many">, z.ZodNull]>>;
        empty_response: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        llm_prompt: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        opener: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    id: z.ZodString;
    knowledge_base_ids: z.ZodArray<z.ZodString, "many">;
    llm_model: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
        model_name: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
    name: z.ZodString;
    prompt: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
        cross_languages: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodString, "many">, z.ZodNull]>>;
        empty_response: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        llm_prompt: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        opener: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">>;
export type ComhairleChat = z.infer<typeof _ComhairleChat>;
export declare const ComhairleChat: z.ZodType<ComhairleChat, z.ZodTypeDef, any>;
declare const _UpdateChatRequest: z.ZodObject<{
    knowledge_base_ids: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodString, "many">, z.ZodNull]>>;
    llm_model: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
        model_name: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
    name: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    prompt: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
        cross_languages: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodString, "many">, z.ZodNull]>>;
        empty_response: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        llm_prompt: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        opener: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    knowledge_base_ids: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodString, "many">, z.ZodNull]>>;
    llm_model: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
        model_name: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
    name: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    prompt: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
        cross_languages: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodString, "many">, z.ZodNull]>>;
        empty_response: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        llm_prompt: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        opener: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    knowledge_base_ids: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodString, "many">, z.ZodNull]>>;
    llm_model: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
        model_name: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
    name: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    prompt: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
        cross_languages: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodString, "many">, z.ZodNull]>>;
        empty_response: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        llm_prompt: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        opener: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">>;
export type UpdateChatRequest = z.infer<typeof _UpdateChatRequest>;
export declare const UpdateChatRequest: z.ZodType<UpdateChatRequest, z.ZodTypeDef, any>;
declare const _ComhairleChatSession: z.ZodObject<{
    chat_id: z.ZodString;
    id: z.ZodString;
    messages: z.ZodArray<z.ZodType<z.objectOutputType<{
        content: z.ZodString;
        id: z.ZodString;
        reference: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodType<z.objectOutputType<{
            content: z.ZodString;
            dataset_id: z.ZodString;
            document_id: z.ZodString;
            document_name: z.ZodString;
            id: z.ZodString;
            positions: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodArray<z.ZodNumber, "many">, "many">, z.ZodNull]>>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">, z.ZodNull]>>;
        role: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    name: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    chat_id: z.ZodString;
    id: z.ZodString;
    messages: z.ZodArray<z.ZodType<z.objectOutputType<{
        content: z.ZodString;
        id: z.ZodString;
        reference: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodType<z.objectOutputType<{
            content: z.ZodString;
            dataset_id: z.ZodString;
            document_id: z.ZodString;
            document_name: z.ZodString;
            id: z.ZodString;
            positions: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodArray<z.ZodNumber, "many">, "many">, z.ZodNull]>>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">, z.ZodNull]>>;
        role: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    name: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    chat_id: z.ZodString;
    id: z.ZodString;
    messages: z.ZodArray<z.ZodType<z.objectOutputType<{
        content: z.ZodString;
        id: z.ZodString;
        reference: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodType<z.objectOutputType<{
            content: z.ZodString;
            dataset_id: z.ZodString;
            document_id: z.ZodString;
            document_name: z.ZodString;
            id: z.ZodString;
            positions: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodArray<z.ZodNumber, "many">, "many">, z.ZodNull]>>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">, z.ZodNull]>>;
        role: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    name: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">>;
export type ComhairleChatSession = z.infer<typeof _ComhairleChatSession>;
export declare const ComhairleChatSession: z.ZodType<ComhairleChatSession, z.ZodTypeDef, any>;
declare const _ChatConversationRequest: z.ZodObject<{
    question: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    question: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    question: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type ChatConversationRequest = z.infer<typeof _ChatConversationRequest>;
export declare const ChatConversationRequest: z.ZodType<ChatConversationRequest, z.ZodTypeDef, any>;
declare const _page_size: z.ZodDefault<z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>>;
export type page_size = z.infer<typeof _page_size>;
export declare const page_size: z.ZodType<page_size, z.ZodTypeDef, any>;
declare const _ComhairleDocument: z.ZodObject<{
    id: z.ZodString;
    name: z.ZodString;
    parse_progress: z.ZodNumber;
    parse_status: z.ZodString;
    size: z.ZodNumber;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    id: z.ZodString;
    name: z.ZodString;
    parse_progress: z.ZodNumber;
    parse_status: z.ZodString;
    size: z.ZodNumber;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    id: z.ZodString;
    name: z.ZodString;
    parse_progress: z.ZodNumber;
    parse_status: z.ZodString;
    size: z.ZodNumber;
}, z.ZodTypeAny, "passthrough">>;
export type ComhairleDocument = z.infer<typeof _ComhairleDocument>;
export declare const ComhairleDocument: z.ZodType<ComhairleDocument, z.ZodTypeDef, any>;
declare const _UploadFileResponse: z.ZodObject<{
    document: z.ZodType<z.objectOutputType<{
        id: z.ZodString;
        name: z.ZodString;
        parse_progress: z.ZodNumber;
        parse_status: z.ZodString;
        size: z.ZodNumber;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    job_id: z.ZodString;
    message: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    document: z.ZodType<z.objectOutputType<{
        id: z.ZodString;
        name: z.ZodString;
        parse_progress: z.ZodNumber;
        parse_status: z.ZodString;
        size: z.ZodNumber;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    job_id: z.ZodString;
    message: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    document: z.ZodType<z.objectOutputType<{
        id: z.ZodString;
        name: z.ZodString;
        parse_progress: z.ZodNumber;
        parse_status: z.ZodString;
        size: z.ZodNumber;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    job_id: z.ZodString;
    message: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type UploadFileResponse = z.infer<typeof _UploadFileResponse>;
export declare const UploadFileResponse: z.ZodType<UploadFileResponse, z.ZodTypeDef, any>;
declare const _SyncLearningContentResponse: z.ZodObject<{
    document: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
        id: z.ZodString;
        name: z.ZodString;
        parse_progress: z.ZodNumber;
        parse_status: z.ZodString;
        size: z.ZodNumber;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
    job_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    message: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    document: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
        id: z.ZodString;
        name: z.ZodString;
        parse_progress: z.ZodNumber;
        parse_status: z.ZodString;
        size: z.ZodNumber;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
    job_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    message: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    document: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
        id: z.ZodString;
        name: z.ZodString;
        parse_progress: z.ZodNumber;
        parse_status: z.ZodString;
        size: z.ZodNumber;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
    job_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    message: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type SyncLearningContentResponse = z.infer<typeof _SyncLearningContentResponse>;
export declare const SyncLearningContentResponse: z.ZodType<SyncLearningContentResponse, z.ZodTypeDef, any>;
declare const _LearnContentPage: z.ZodObject<{
    content: z.ZodString;
    is_rich: z.ZodBoolean;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    content: z.ZodString;
    is_rich: z.ZodBoolean;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    content: z.ZodString;
    is_rich: z.ZodBoolean;
}, z.ZodTypeAny, "passthrough">>;
export type LearnContentPage = z.infer<typeof _LearnContentPage>;
export declare const LearnContentPage: z.ZodType<LearnContentPage, z.ZodTypeDef, any>;
declare const _LearnContentSection: z.ZodObject<{
    heading: z.ZodString;
    pages: z.ZodArray<z.ZodType<z.objectOutputType<{
        content: z.ZodString;
        is_rich: z.ZodBoolean;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    heading: z.ZodString;
    pages: z.ZodArray<z.ZodType<z.objectOutputType<{
        content: z.ZodString;
        is_rich: z.ZodBoolean;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    heading: z.ZodString;
    pages: z.ZodArray<z.ZodType<z.objectOutputType<{
        content: z.ZodString;
        is_rich: z.ZodBoolean;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
}, z.ZodTypeAny, "passthrough">>;
export type LearnContentSection = z.infer<typeof _LearnContentSection>;
export declare const LearnContentSection: z.ZodType<LearnContentSection, z.ZodTypeDef, any>;
declare const _LearnContentResponse: z.ZodObject<{
    sections: z.ZodArray<z.ZodType<z.objectOutputType<{
        heading: z.ZodString;
        pages: z.ZodArray<z.ZodType<z.objectOutputType<{
            content: z.ZodString;
            is_rich: z.ZodBoolean;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    sections: z.ZodArray<z.ZodType<z.objectOutputType<{
        heading: z.ZodString;
        pages: z.ZodArray<z.ZodType<z.objectOutputType<{
            content: z.ZodString;
            is_rich: z.ZodBoolean;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    sections: z.ZodArray<z.ZodType<z.objectOutputType<{
        heading: z.ZodString;
        pages: z.ZodArray<z.ZodType<z.objectOutputType<{
            content: z.ZodString;
            is_rich: z.ZodBoolean;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
}, z.ZodTypeAny, "passthrough">>;
export type LearnContentResponse = z.infer<typeof _LearnContentResponse>;
export declare const LearnContentResponse: z.ZodType<LearnContentResponse, z.ZodTypeDef, any>;
declare const _Order: z.ZodEnum<["asc", "desc"]>;
export type Order = z.infer<typeof _Order>;
export declare const Order: z.ZodType<Order, z.ZodTypeDef, any>;
declare const _created_at: z.ZodOptional<z.ZodUnion<[z.ZodType<"asc" | "desc", z.ZodTypeDef, any>, z.ZodNull]>>;
export type created_at = z.infer<typeof _created_at>;
export declare const created_at: z.ZodType<created_at, z.ZodTypeDef, any>;
declare const _CapacityStatus: z.ZodEnum<["full", "available"]>;
export type CapacityStatus = z.infer<typeof _CapacityStatus>;
export declare const CapacityStatus: z.ZodType<CapacityStatus, z.ZodTypeDef, any>;
declare const _capacity_status: z.ZodOptional<z.ZodUnion<[z.ZodType<"full" | "available", z.ZodTypeDef, any>, z.ZodNull]>>;
export type capacity_status = z.infer<typeof _capacity_status>;
export declare const capacity_status: z.ZodType<capacity_status, z.ZodTypeDef, any>;
declare const _TimeStatus: z.ZodEnum<["past", "future"]>;
export type TimeStatus = z.infer<typeof _TimeStatus>;
export declare const TimeStatus: z.ZodType<TimeStatus, z.ZodTypeDef, any>;
declare const _time_status: z.ZodOptional<z.ZodUnion<[z.ZodType<"past" | "future", z.ZodTypeDef, any>, z.ZodNull]>>;
export type time_status = z.infer<typeof _time_status>;
export declare const time_status: z.ZodType<time_status, z.ZodTypeDef, any>;
declare const _BasicEventAgendaItem: z.ZodObject<{
    description: z.ZodString;
    estimated_time: z.ZodNumber;
    title: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    description: z.ZodString;
    estimated_time: z.ZodNumber;
    title: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    description: z.ZodString;
    estimated_time: z.ZodNumber;
    title: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type BasicEventAgendaItem = z.infer<typeof _BasicEventAgendaItem>;
export declare const BasicEventAgendaItem: z.ZodType<BasicEventAgendaItem, z.ZodTypeDef, any>;
declare const _BreakoutRoomAgendaItem: z.ZodObject<{
    estimated_time: z.ZodNumber;
    instructions: z.ZodString;
    max_per_room: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
    prompt: z.ZodString;
    time_limit: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    estimated_time: z.ZodNumber;
    instructions: z.ZodString;
    max_per_room: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
    prompt: z.ZodString;
    time_limit: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    estimated_time: z.ZodNumber;
    instructions: z.ZodString;
    max_per_room: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
    prompt: z.ZodString;
    time_limit: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">>;
export type BreakoutRoomAgendaItem = z.infer<typeof _BreakoutRoomAgendaItem>;
export declare const BreakoutRoomAgendaItem: z.ZodType<BreakoutRoomAgendaItem, z.ZodTypeDef, any>;
declare const _EventAgendaItem: z.ZodUnion<[z.ZodObject<{
    Basic: z.ZodType<z.objectOutputType<{
        description: z.ZodString;
        estimated_time: z.ZodNumber;
        title: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
}, "strip", z.ZodTypeAny, {
    Basic: {
        description: string;
        title: string;
        estimated_time: number;
    } & {
        [k: string]: unknown;
    };
}, {
    Basic?: any;
}>, z.ZodObject<{
    BreakoutRoom: z.ZodType<z.objectOutputType<{
        estimated_time: z.ZodNumber;
        instructions: z.ZodString;
        max_per_room: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        prompt: z.ZodString;
        time_limit: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
}, "strip", z.ZodTypeAny, {
    BreakoutRoom: {
        prompt: string;
        estimated_time: number;
        instructions: string;
        max_per_room?: number | null | undefined;
        time_limit?: number | null | undefined;
    } & {
        [k: string]: unknown;
    };
}, {
    BreakoutRoom?: any;
}>]>;
export type EventAgendaItem = z.infer<typeof _EventAgendaItem>;
export declare const EventAgendaItem: z.ZodType<EventAgendaItem, z.ZodTypeDef, any>;
declare const _EventFormat: z.ZodEnum<["online", "in_person"]>;
export type EventFormat = z.infer<typeof _EventFormat>;
export declare const EventFormat: z.ZodType<EventFormat, z.ZodTypeDef, any>;
declare const _EventLocation: z.ZodObject<{
    address_line_1: z.ZodString;
    address_line_2: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    address_line_3: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    city: z.ZodString;
    country_code: z.ZodString;
    postal_code: z.ZodString;
    state_province: z.ZodString;
    venue_name: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    address_line_1: z.ZodString;
    address_line_2: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    address_line_3: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    city: z.ZodString;
    country_code: z.ZodString;
    postal_code: z.ZodString;
    state_province: z.ZodString;
    venue_name: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    address_line_1: z.ZodString;
    address_line_2: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    address_line_3: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    city: z.ZodString;
    country_code: z.ZodString;
    postal_code: z.ZodString;
    state_province: z.ZodString;
    venue_name: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type EventLocation = z.infer<typeof _EventLocation>;
export declare const EventLocation: z.ZodType<EventLocation, z.ZodTypeDef, any>;
declare const _LocalizedEventDto: z.ZodObject<{
    agenda: z.ZodArray<z.ZodType<{
        Basic: {
            description: string;
            title: string;
            estimated_time: number;
        } & {
            [k: string]: unknown;
        };
    } | {
        BreakoutRoom: {
            prompt: string;
            estimated_time: number;
            instructions: string;
            max_per_room?: number | null | undefined;
            time_limit?: number | null | undefined;
        } & {
            [k: string]: unknown;
        };
    }, z.ZodTypeDef, any>, "many">;
    capacity: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
    conversationId: z.ZodString;
    createdAt: z.ZodString;
    currentAttendance: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
    customEventLink: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    description: z.ZodString;
    endTime: z.ZodString;
    format: z.ZodType<"online" | "in_person", z.ZodTypeDef, any>;
    id: z.ZodString;
    location: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
        address_line_1: z.ZodString;
        address_line_2: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        address_line_3: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        city: z.ZodString;
        country_code: z.ZodString;
        postal_code: z.ZodString;
        state_province: z.ZodString;
        venue_name: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
    metadata: z.ZodOptional<z.ZodUnknown>;
    name: z.ZodString;
    signupMode: z.ZodString;
    startTime: z.ZodString;
    videoMeetingId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    agenda: z.ZodArray<z.ZodType<{
        Basic: {
            description: string;
            title: string;
            estimated_time: number;
        } & {
            [k: string]: unknown;
        };
    } | {
        BreakoutRoom: {
            prompt: string;
            estimated_time: number;
            instructions: string;
            max_per_room?: number | null | undefined;
            time_limit?: number | null | undefined;
        } & {
            [k: string]: unknown;
        };
    }, z.ZodTypeDef, any>, "many">;
    capacity: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
    conversationId: z.ZodString;
    createdAt: z.ZodString;
    currentAttendance: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
    customEventLink: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    description: z.ZodString;
    endTime: z.ZodString;
    format: z.ZodType<"online" | "in_person", z.ZodTypeDef, any>;
    id: z.ZodString;
    location: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
        address_line_1: z.ZodString;
        address_line_2: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        address_line_3: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        city: z.ZodString;
        country_code: z.ZodString;
        postal_code: z.ZodString;
        state_province: z.ZodString;
        venue_name: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
    metadata: z.ZodOptional<z.ZodUnknown>;
    name: z.ZodString;
    signupMode: z.ZodString;
    startTime: z.ZodString;
    videoMeetingId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    agenda: z.ZodArray<z.ZodType<{
        Basic: {
            description: string;
            title: string;
            estimated_time: number;
        } & {
            [k: string]: unknown;
        };
    } | {
        BreakoutRoom: {
            prompt: string;
            estimated_time: number;
            instructions: string;
            max_per_room?: number | null | undefined;
            time_limit?: number | null | undefined;
        } & {
            [k: string]: unknown;
        };
    }, z.ZodTypeDef, any>, "many">;
    capacity: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
    conversationId: z.ZodString;
    createdAt: z.ZodString;
    currentAttendance: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
    customEventLink: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    description: z.ZodString;
    endTime: z.ZodString;
    format: z.ZodType<"online" | "in_person", z.ZodTypeDef, any>;
    id: z.ZodString;
    location: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
        address_line_1: z.ZodString;
        address_line_2: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        address_line_3: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        city: z.ZodString;
        country_code: z.ZodString;
        postal_code: z.ZodString;
        state_province: z.ZodString;
        venue_name: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
    metadata: z.ZodOptional<z.ZodUnknown>;
    name: z.ZodString;
    signupMode: z.ZodString;
    startTime: z.ZodString;
    videoMeetingId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">>;
export type LocalizedEventDto = z.infer<typeof _LocalizedEventDto>;
export declare const LocalizedEventDto: z.ZodType<LocalizedEventDto, z.ZodTypeDef, any>;
declare const _PaginatedResults_for_LocalizedEventDto: z.ZodObject<{
    records: z.ZodArray<z.ZodType<z.objectOutputType<{
        agenda: z.ZodArray<z.ZodType<{
            Basic: {
                description: string;
                title: string;
                estimated_time: number;
            } & {
                [k: string]: unknown;
            };
        } | {
            BreakoutRoom: {
                prompt: string;
                estimated_time: number;
                instructions: string;
                max_per_room?: number | null | undefined;
                time_limit?: number | null | undefined;
            } & {
                [k: string]: unknown;
            };
        }, z.ZodTypeDef, any>, "many">;
        capacity: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        conversationId: z.ZodString;
        createdAt: z.ZodString;
        currentAttendance: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        customEventLink: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        description: z.ZodString;
        endTime: z.ZodString;
        format: z.ZodType<"online" | "in_person", z.ZodTypeDef, any>;
        id: z.ZodString;
        location: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
            address_line_1: z.ZodString;
            address_line_2: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
            address_line_3: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
            city: z.ZodString;
            country_code: z.ZodString;
            postal_code: z.ZodString;
            state_province: z.ZodString;
            venue_name: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
        metadata: z.ZodOptional<z.ZodUnknown>;
        name: z.ZodString;
        signupMode: z.ZodString;
        startTime: z.ZodString;
        videoMeetingId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    total: z.ZodNumber;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    records: z.ZodArray<z.ZodType<z.objectOutputType<{
        agenda: z.ZodArray<z.ZodType<{
            Basic: {
                description: string;
                title: string;
                estimated_time: number;
            } & {
                [k: string]: unknown;
            };
        } | {
            BreakoutRoom: {
                prompt: string;
                estimated_time: number;
                instructions: string;
                max_per_room?: number | null | undefined;
                time_limit?: number | null | undefined;
            } & {
                [k: string]: unknown;
            };
        }, z.ZodTypeDef, any>, "many">;
        capacity: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        conversationId: z.ZodString;
        createdAt: z.ZodString;
        currentAttendance: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        customEventLink: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        description: z.ZodString;
        endTime: z.ZodString;
        format: z.ZodType<"online" | "in_person", z.ZodTypeDef, any>;
        id: z.ZodString;
        location: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
            address_line_1: z.ZodString;
            address_line_2: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
            address_line_3: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
            city: z.ZodString;
            country_code: z.ZodString;
            postal_code: z.ZodString;
            state_province: z.ZodString;
            venue_name: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
        metadata: z.ZodOptional<z.ZodUnknown>;
        name: z.ZodString;
        signupMode: z.ZodString;
        startTime: z.ZodString;
        videoMeetingId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    total: z.ZodNumber;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    records: z.ZodArray<z.ZodType<z.objectOutputType<{
        agenda: z.ZodArray<z.ZodType<{
            Basic: {
                description: string;
                title: string;
                estimated_time: number;
            } & {
                [k: string]: unknown;
            };
        } | {
            BreakoutRoom: {
                prompt: string;
                estimated_time: number;
                instructions: string;
                max_per_room?: number | null | undefined;
                time_limit?: number | null | undefined;
            } & {
                [k: string]: unknown;
            };
        }, z.ZodTypeDef, any>, "many">;
        capacity: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        conversationId: z.ZodString;
        createdAt: z.ZodString;
        currentAttendance: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        customEventLink: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        description: z.ZodString;
        endTime: z.ZodString;
        format: z.ZodType<"online" | "in_person", z.ZodTypeDef, any>;
        id: z.ZodString;
        location: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
            address_line_1: z.ZodString;
            address_line_2: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
            address_line_3: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
            city: z.ZodString;
            country_code: z.ZodString;
            postal_code: z.ZodString;
            state_province: z.ZodString;
            venue_name: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
        metadata: z.ZodOptional<z.ZodUnknown>;
        name: z.ZodString;
        signupMode: z.ZodString;
        startTime: z.ZodString;
        videoMeetingId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    total: z.ZodNumber;
}, z.ZodTypeAny, "passthrough">>;
export type PaginatedResults_for_LocalizedEventDto = z.infer<typeof _PaginatedResults_for_LocalizedEventDto>;
export declare const PaginatedResults_for_LocalizedEventDto: z.ZodType<PaginatedResults_for_LocalizedEventDto, z.ZodTypeDef, any>;
declare const _CreateEvent: z.ZodObject<{
    agenda: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodType<{
        Basic: {
            description: string;
            title: string;
            estimated_time: number;
        } & {
            [k: string]: unknown;
        };
    } | {
        BreakoutRoom: {
            prompt: string;
            estimated_time: number;
            instructions: string;
            max_per_room?: number | null | undefined;
            time_limit?: number | null | undefined;
        } & {
            [k: string]: unknown;
        };
    }, z.ZodTypeDef, any>, "many">, z.ZodNull]>>;
    capacity: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
    custom_event_link: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    default_time_zone: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    description: z.ZodString;
    end_time: z.ZodString;
    location: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
        address_line_1: z.ZodString;
        address_line_2: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        address_line_3: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        city: z.ZodString;
        country_code: z.ZodString;
        postal_code: z.ZodString;
        state_province: z.ZodString;
        venue_name: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
    name: z.ZodString;
    signup_mode: z.ZodString;
    start_time: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    agenda: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodType<{
        Basic: {
            description: string;
            title: string;
            estimated_time: number;
        } & {
            [k: string]: unknown;
        };
    } | {
        BreakoutRoom: {
            prompt: string;
            estimated_time: number;
            instructions: string;
            max_per_room?: number | null | undefined;
            time_limit?: number | null | undefined;
        } & {
            [k: string]: unknown;
        };
    }, z.ZodTypeDef, any>, "many">, z.ZodNull]>>;
    capacity: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
    custom_event_link: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    default_time_zone: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    description: z.ZodString;
    end_time: z.ZodString;
    location: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
        address_line_1: z.ZodString;
        address_line_2: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        address_line_3: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        city: z.ZodString;
        country_code: z.ZodString;
        postal_code: z.ZodString;
        state_province: z.ZodString;
        venue_name: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
    name: z.ZodString;
    signup_mode: z.ZodString;
    start_time: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    agenda: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodType<{
        Basic: {
            description: string;
            title: string;
            estimated_time: number;
        } & {
            [k: string]: unknown;
        };
    } | {
        BreakoutRoom: {
            prompt: string;
            estimated_time: number;
            instructions: string;
            max_per_room?: number | null | undefined;
            time_limit?: number | null | undefined;
        } & {
            [k: string]: unknown;
        };
    }, z.ZodTypeDef, any>, "many">, z.ZodNull]>>;
    capacity: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
    custom_event_link: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    default_time_zone: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    description: z.ZodString;
    end_time: z.ZodString;
    location: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
        address_line_1: z.ZodString;
        address_line_2: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        address_line_3: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        city: z.ZodString;
        country_code: z.ZodString;
        postal_code: z.ZodString;
        state_province: z.ZodString;
        venue_name: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
    name: z.ZodString;
    signup_mode: z.ZodString;
    start_time: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type CreateEvent = z.infer<typeof _CreateEvent>;
export declare const CreateEvent: z.ZodType<CreateEvent, z.ZodTypeDef, any>;
declare const _EventDto: z.ZodObject<{
    agenda: z.ZodArray<z.ZodType<{
        Basic: {
            description: string;
            title: string;
            estimated_time: number;
        } & {
            [k: string]: unknown;
        };
    } | {
        BreakoutRoom: {
            prompt: string;
            estimated_time: number;
            instructions: string;
            max_per_room?: number | null | undefined;
            time_limit?: number | null | undefined;
        } & {
            [k: string]: unknown;
        };
    }, z.ZodTypeDef, any>, "many">;
    capacity: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
    conversationId: z.ZodString;
    createdAt: z.ZodString;
    customEventLink: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    description: z.ZodString;
    endTime: z.ZodString;
    format: z.ZodType<"online" | "in_person", z.ZodTypeDef, any>;
    id: z.ZodString;
    location: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
        address_line_1: z.ZodString;
        address_line_2: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        address_line_3: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        city: z.ZodString;
        country_code: z.ZodString;
        postal_code: z.ZodString;
        state_province: z.ZodString;
        venue_name: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
    metadata: z.ZodOptional<z.ZodUnknown>;
    name: z.ZodString;
    signupMode: z.ZodString;
    startTime: z.ZodString;
    videoMeetingId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    agenda: z.ZodArray<z.ZodType<{
        Basic: {
            description: string;
            title: string;
            estimated_time: number;
        } & {
            [k: string]: unknown;
        };
    } | {
        BreakoutRoom: {
            prompt: string;
            estimated_time: number;
            instructions: string;
            max_per_room?: number | null | undefined;
            time_limit?: number | null | undefined;
        } & {
            [k: string]: unknown;
        };
    }, z.ZodTypeDef, any>, "many">;
    capacity: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
    conversationId: z.ZodString;
    createdAt: z.ZodString;
    customEventLink: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    description: z.ZodString;
    endTime: z.ZodString;
    format: z.ZodType<"online" | "in_person", z.ZodTypeDef, any>;
    id: z.ZodString;
    location: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
        address_line_1: z.ZodString;
        address_line_2: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        address_line_3: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        city: z.ZodString;
        country_code: z.ZodString;
        postal_code: z.ZodString;
        state_province: z.ZodString;
        venue_name: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
    metadata: z.ZodOptional<z.ZodUnknown>;
    name: z.ZodString;
    signupMode: z.ZodString;
    startTime: z.ZodString;
    videoMeetingId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    agenda: z.ZodArray<z.ZodType<{
        Basic: {
            description: string;
            title: string;
            estimated_time: number;
        } & {
            [k: string]: unknown;
        };
    } | {
        BreakoutRoom: {
            prompt: string;
            estimated_time: number;
            instructions: string;
            max_per_room?: number | null | undefined;
            time_limit?: number | null | undefined;
        } & {
            [k: string]: unknown;
        };
    }, z.ZodTypeDef, any>, "many">;
    capacity: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
    conversationId: z.ZodString;
    createdAt: z.ZodString;
    customEventLink: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    description: z.ZodString;
    endTime: z.ZodString;
    format: z.ZodType<"online" | "in_person", z.ZodTypeDef, any>;
    id: z.ZodString;
    location: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
        address_line_1: z.ZodString;
        address_line_2: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        address_line_3: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        city: z.ZodString;
        country_code: z.ZodString;
        postal_code: z.ZodString;
        state_province: z.ZodString;
        venue_name: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
    metadata: z.ZodOptional<z.ZodUnknown>;
    name: z.ZodString;
    signupMode: z.ZodString;
    startTime: z.ZodString;
    videoMeetingId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">>;
export type EventDto = z.infer<typeof _EventDto>;
export declare const EventDto: z.ZodType<EventDto, z.ZodTypeDef, any>;
declare const _BreakoutSeat: z.ZodObject<{
    invite_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    is_moderator: z.ZodOptional<z.ZodDefault<z.ZodBoolean>>;
    user_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    invite_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    is_moderator: z.ZodOptional<z.ZodDefault<z.ZodBoolean>>;
    user_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    invite_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    is_moderator: z.ZodOptional<z.ZodDefault<z.ZodBoolean>>;
    user_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">>;
export type BreakoutSeat = z.infer<typeof _BreakoutSeat>;
export declare const BreakoutSeat: z.ZodType<BreakoutSeat, z.ZodTypeDef, any>;
declare const _BreakoutPlanRoom: z.ZodObject<{
    seats: z.ZodOptional<z.ZodDefault<z.ZodArray<z.ZodType<z.objectOutputType<{
        invite_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        is_moderator: z.ZodOptional<z.ZodDefault<z.ZodBoolean>>;
        user_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">>>;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    seats: z.ZodOptional<z.ZodDefault<z.ZodArray<z.ZodType<z.objectOutputType<{
        invite_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        is_moderator: z.ZodOptional<z.ZodDefault<z.ZodBoolean>>;
        user_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">>>;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    seats: z.ZodOptional<z.ZodDefault<z.ZodArray<z.ZodType<z.objectOutputType<{
        invite_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        is_moderator: z.ZodOptional<z.ZodDefault<z.ZodBoolean>>;
        user_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">>>;
}, z.ZodTypeAny, "passthrough">>;
export type BreakoutPlanRoom = z.infer<typeof _BreakoutPlanRoom>;
export declare const BreakoutPlanRoom: z.ZodType<BreakoutPlanRoom, z.ZodTypeDef, any>;
declare const _Translation6: z.ZodObject<{
    textContent: z.ZodType<z.objectOutputType<{
        format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
        id: z.ZodString;
        primaryLocale: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
        aiGenerated: z.ZodBoolean;
        content: z.ZodString;
        contentId: z.ZodString;
        id: z.ZodString;
        locale: z.ZodString;
        requiresValidation: z.ZodBoolean;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    textContent: z.ZodType<z.objectOutputType<{
        format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
        id: z.ZodString;
        primaryLocale: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
        aiGenerated: z.ZodBoolean;
        content: z.ZodString;
        contentId: z.ZodString;
        id: z.ZodString;
        locale: z.ZodString;
        requiresValidation: z.ZodBoolean;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    textContent: z.ZodType<z.objectOutputType<{
        format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
        id: z.ZodString;
        primaryLocale: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
        aiGenerated: z.ZodBoolean;
        content: z.ZodString;
        contentId: z.ZodString;
        id: z.ZodString;
        locale: z.ZodString;
        requiresValidation: z.ZodBoolean;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
}, z.ZodTypeAny, "passthrough">>;
export type Translation6 = z.infer<typeof _Translation6>;
export declare const Translation6: z.ZodType<Translation6, z.ZodTypeDef, any>;
declare const _EventTranslations: z.ZodObject<{
    description: z.ZodType<z.objectOutputType<{
        textContent: z.ZodType<z.objectOutputType<{
            format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
            id: z.ZodString;
            primaryLocale: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
            aiGenerated: z.ZodBoolean;
            content: z.ZodString;
            contentId: z.ZodString;
            id: z.ZodString;
            locale: z.ZodString;
            requiresValidation: z.ZodBoolean;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    name: z.ZodType<z.objectOutputType<{
        textContent: z.ZodType<z.objectOutputType<{
            format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
            id: z.ZodString;
            primaryLocale: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
            aiGenerated: z.ZodBoolean;
            content: z.ZodString;
            contentId: z.ZodString;
            id: z.ZodString;
            locale: z.ZodString;
            requiresValidation: z.ZodBoolean;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    description: z.ZodType<z.objectOutputType<{
        textContent: z.ZodType<z.objectOutputType<{
            format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
            id: z.ZodString;
            primaryLocale: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
            aiGenerated: z.ZodBoolean;
            content: z.ZodString;
            contentId: z.ZodString;
            id: z.ZodString;
            locale: z.ZodString;
            requiresValidation: z.ZodBoolean;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    name: z.ZodType<z.objectOutputType<{
        textContent: z.ZodType<z.objectOutputType<{
            format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
            id: z.ZodString;
            primaryLocale: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
            aiGenerated: z.ZodBoolean;
            content: z.ZodString;
            contentId: z.ZodString;
            id: z.ZodString;
            locale: z.ZodString;
            requiresValidation: z.ZodBoolean;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    description: z.ZodType<z.objectOutputType<{
        textContent: z.ZodType<z.objectOutputType<{
            format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
            id: z.ZodString;
            primaryLocale: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
            aiGenerated: z.ZodBoolean;
            content: z.ZodString;
            contentId: z.ZodString;
            id: z.ZodString;
            locale: z.ZodString;
            requiresValidation: z.ZodBoolean;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    name: z.ZodType<z.objectOutputType<{
        textContent: z.ZodType<z.objectOutputType<{
            format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
            id: z.ZodString;
            primaryLocale: z.ZodString;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
            aiGenerated: z.ZodBoolean;
            content: z.ZodString;
            contentId: z.ZodString;
            id: z.ZodString;
            locale: z.ZodString;
            requiresValidation: z.ZodBoolean;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
}, z.ZodTypeAny, "passthrough">>;
export type EventTranslations = z.infer<typeof _EventTranslations>;
export declare const EventTranslations: z.ZodType<EventTranslations, z.ZodTypeDef, any>;
declare const _EventWithTranslations: z.ZodObject<{
    agenda: z.ZodArray<z.ZodType<{
        Basic: {
            description: string;
            title: string;
            estimated_time: number;
        } & {
            [k: string]: unknown;
        };
    } | {
        BreakoutRoom: {
            prompt: string;
            estimated_time: number;
            instructions: string;
            max_per_room?: number | null | undefined;
            time_limit?: number | null | undefined;
        } & {
            [k: string]: unknown;
        };
    }, z.ZodTypeDef, any>, "many">;
    breakoutPlan: z.ZodArray<z.ZodType<z.objectOutputType<{
        seats: z.ZodOptional<z.ZodDefault<z.ZodArray<z.ZodType<z.objectOutputType<{
            invite_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
            is_moderator: z.ZodOptional<z.ZodDefault<z.ZodBoolean>>;
            user_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">>>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    capacity: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
    conversationId: z.ZodString;
    createdAt: z.ZodString;
    customEventLink: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    defaultTimeZone: z.ZodString;
    description: z.ZodString;
    endTime: z.ZodString;
    format: z.ZodType<"online" | "in_person", z.ZodTypeDef, any>;
    id: z.ZodString;
    location: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
        address_line_1: z.ZodString;
        address_line_2: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        address_line_3: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        city: z.ZodString;
        country_code: z.ZodString;
        postal_code: z.ZodString;
        state_province: z.ZodString;
        venue_name: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
    metadata: z.ZodOptional<z.ZodUnknown>;
    name: z.ZodString;
    signupMode: z.ZodString;
    startTime: z.ZodString;
    translations: z.ZodType<z.objectOutputType<{
        description: z.ZodType<z.objectOutputType<{
            textContent: z.ZodType<z.objectOutputType<{
                format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                id: z.ZodString;
                primaryLocale: z.ZodString;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                aiGenerated: z.ZodBoolean;
                content: z.ZodString;
                contentId: z.ZodString;
                id: z.ZodString;
                locale: z.ZodString;
                requiresValidation: z.ZodBoolean;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        name: z.ZodType<z.objectOutputType<{
            textContent: z.ZodType<z.objectOutputType<{
                format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                id: z.ZodString;
                primaryLocale: z.ZodString;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                aiGenerated: z.ZodBoolean;
                content: z.ZodString;
                contentId: z.ZodString;
                id: z.ZodString;
                locale: z.ZodString;
                requiresValidation: z.ZodBoolean;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    updatedAt: z.ZodString;
    videoMeetingId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    agenda: z.ZodArray<z.ZodType<{
        Basic: {
            description: string;
            title: string;
            estimated_time: number;
        } & {
            [k: string]: unknown;
        };
    } | {
        BreakoutRoom: {
            prompt: string;
            estimated_time: number;
            instructions: string;
            max_per_room?: number | null | undefined;
            time_limit?: number | null | undefined;
        } & {
            [k: string]: unknown;
        };
    }, z.ZodTypeDef, any>, "many">;
    breakoutPlan: z.ZodArray<z.ZodType<z.objectOutputType<{
        seats: z.ZodOptional<z.ZodDefault<z.ZodArray<z.ZodType<z.objectOutputType<{
            invite_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
            is_moderator: z.ZodOptional<z.ZodDefault<z.ZodBoolean>>;
            user_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">>>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    capacity: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
    conversationId: z.ZodString;
    createdAt: z.ZodString;
    customEventLink: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    defaultTimeZone: z.ZodString;
    description: z.ZodString;
    endTime: z.ZodString;
    format: z.ZodType<"online" | "in_person", z.ZodTypeDef, any>;
    id: z.ZodString;
    location: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
        address_line_1: z.ZodString;
        address_line_2: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        address_line_3: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        city: z.ZodString;
        country_code: z.ZodString;
        postal_code: z.ZodString;
        state_province: z.ZodString;
        venue_name: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
    metadata: z.ZodOptional<z.ZodUnknown>;
    name: z.ZodString;
    signupMode: z.ZodString;
    startTime: z.ZodString;
    translations: z.ZodType<z.objectOutputType<{
        description: z.ZodType<z.objectOutputType<{
            textContent: z.ZodType<z.objectOutputType<{
                format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                id: z.ZodString;
                primaryLocale: z.ZodString;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                aiGenerated: z.ZodBoolean;
                content: z.ZodString;
                contentId: z.ZodString;
                id: z.ZodString;
                locale: z.ZodString;
                requiresValidation: z.ZodBoolean;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        name: z.ZodType<z.objectOutputType<{
            textContent: z.ZodType<z.objectOutputType<{
                format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                id: z.ZodString;
                primaryLocale: z.ZodString;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                aiGenerated: z.ZodBoolean;
                content: z.ZodString;
                contentId: z.ZodString;
                id: z.ZodString;
                locale: z.ZodString;
                requiresValidation: z.ZodBoolean;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    updatedAt: z.ZodString;
    videoMeetingId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    agenda: z.ZodArray<z.ZodType<{
        Basic: {
            description: string;
            title: string;
            estimated_time: number;
        } & {
            [k: string]: unknown;
        };
    } | {
        BreakoutRoom: {
            prompt: string;
            estimated_time: number;
            instructions: string;
            max_per_room?: number | null | undefined;
            time_limit?: number | null | undefined;
        } & {
            [k: string]: unknown;
        };
    }, z.ZodTypeDef, any>, "many">;
    breakoutPlan: z.ZodArray<z.ZodType<z.objectOutputType<{
        seats: z.ZodOptional<z.ZodDefault<z.ZodArray<z.ZodType<z.objectOutputType<{
            invite_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
            is_moderator: z.ZodOptional<z.ZodDefault<z.ZodBoolean>>;
            user_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">>>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    capacity: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
    conversationId: z.ZodString;
    createdAt: z.ZodString;
    customEventLink: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    defaultTimeZone: z.ZodString;
    description: z.ZodString;
    endTime: z.ZodString;
    format: z.ZodType<"online" | "in_person", z.ZodTypeDef, any>;
    id: z.ZodString;
    location: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
        address_line_1: z.ZodString;
        address_line_2: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        address_line_3: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        city: z.ZodString;
        country_code: z.ZodString;
        postal_code: z.ZodString;
        state_province: z.ZodString;
        venue_name: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
    metadata: z.ZodOptional<z.ZodUnknown>;
    name: z.ZodString;
    signupMode: z.ZodString;
    startTime: z.ZodString;
    translations: z.ZodType<z.objectOutputType<{
        description: z.ZodType<z.objectOutputType<{
            textContent: z.ZodType<z.objectOutputType<{
                format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                id: z.ZodString;
                primaryLocale: z.ZodString;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                aiGenerated: z.ZodBoolean;
                content: z.ZodString;
                contentId: z.ZodString;
                id: z.ZodString;
                locale: z.ZodString;
                requiresValidation: z.ZodBoolean;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        name: z.ZodType<z.objectOutputType<{
            textContent: z.ZodType<z.objectOutputType<{
                format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                id: z.ZodString;
                primaryLocale: z.ZodString;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                aiGenerated: z.ZodBoolean;
                content: z.ZodString;
                contentId: z.ZodString;
                id: z.ZodString;
                locale: z.ZodString;
                requiresValidation: z.ZodBoolean;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    updatedAt: z.ZodString;
    videoMeetingId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">>;
export type EventWithTranslations = z.infer<typeof _EventWithTranslations>;
export declare const EventWithTranslations: z.ZodType<EventWithTranslations, z.ZodTypeDef, any>;
declare const _EventResponse: z.ZodUnion<[z.ZodType<z.objectOutputType<{
    agenda: z.ZodArray<z.ZodType<{
        Basic: {
            description: string;
            title: string;
            estimated_time: number;
        } & {
            [k: string]: unknown;
        };
    } | {
        BreakoutRoom: {
            prompt: string;
            estimated_time: number;
            instructions: string;
            max_per_room?: number | null | undefined;
            time_limit?: number | null | undefined;
        } & {
            [k: string]: unknown;
        };
    }, z.ZodTypeDef, any>, "many">;
    capacity: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
    conversationId: z.ZodString;
    createdAt: z.ZodString;
    currentAttendance: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
    customEventLink: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    description: z.ZodString;
    endTime: z.ZodString;
    format: z.ZodType<"online" | "in_person", z.ZodTypeDef, any>;
    id: z.ZodString;
    location: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
        address_line_1: z.ZodString;
        address_line_2: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        address_line_3: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        city: z.ZodString;
        country_code: z.ZodString;
        postal_code: z.ZodString;
        state_province: z.ZodString;
        venue_name: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
    metadata: z.ZodOptional<z.ZodUnknown>;
    name: z.ZodString;
    signupMode: z.ZodString;
    startTime: z.ZodString;
    videoMeetingId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodType<z.objectOutputType<{
    agenda: z.ZodArray<z.ZodType<{
        Basic: {
            description: string;
            title: string;
            estimated_time: number;
        } & {
            [k: string]: unknown;
        };
    } | {
        BreakoutRoom: {
            prompt: string;
            estimated_time: number;
            instructions: string;
            max_per_room?: number | null | undefined;
            time_limit?: number | null | undefined;
        } & {
            [k: string]: unknown;
        };
    }, z.ZodTypeDef, any>, "many">;
    breakoutPlan: z.ZodArray<z.ZodType<z.objectOutputType<{
        seats: z.ZodOptional<z.ZodDefault<z.ZodArray<z.ZodType<z.objectOutputType<{
            invite_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
            is_moderator: z.ZodOptional<z.ZodDefault<z.ZodBoolean>>;
            user_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">>>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    capacity: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
    conversationId: z.ZodString;
    createdAt: z.ZodString;
    customEventLink: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    defaultTimeZone: z.ZodString;
    description: z.ZodString;
    endTime: z.ZodString;
    format: z.ZodType<"online" | "in_person", z.ZodTypeDef, any>;
    id: z.ZodString;
    location: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
        address_line_1: z.ZodString;
        address_line_2: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        address_line_3: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        city: z.ZodString;
        country_code: z.ZodString;
        postal_code: z.ZodString;
        state_province: z.ZodString;
        venue_name: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
    metadata: z.ZodOptional<z.ZodUnknown>;
    name: z.ZodString;
    signupMode: z.ZodString;
    startTime: z.ZodString;
    translations: z.ZodType<z.objectOutputType<{
        description: z.ZodType<z.objectOutputType<{
            textContent: z.ZodType<z.objectOutputType<{
                format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                id: z.ZodString;
                primaryLocale: z.ZodString;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                aiGenerated: z.ZodBoolean;
                content: z.ZodString;
                contentId: z.ZodString;
                id: z.ZodString;
                locale: z.ZodString;
                requiresValidation: z.ZodBoolean;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
        name: z.ZodType<z.objectOutputType<{
            textContent: z.ZodType<z.objectOutputType<{
                format: z.ZodType<"plain" | "markdown" | "rich", z.ZodTypeDef, any>;
                id: z.ZodString;
                primaryLocale: z.ZodString;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
            textTranslations: z.ZodArray<z.ZodType<z.objectOutputType<{
                aiGenerated: z.ZodBoolean;
                content: z.ZodString;
                contentId: z.ZodString;
                id: z.ZodString;
                locale: z.ZodString;
                requiresValidation: z.ZodBoolean;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    updatedAt: z.ZodString;
    videoMeetingId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>]>;
export type EventResponse = z.infer<typeof _EventResponse>;
export declare const EventResponse: z.ZodType<EventResponse, z.ZodTypeDef, any>;
declare const _PartialEvent: z.ZodObject<{
    agenda: z.ZodOptional<z.ZodDefault<z.ZodUnion<[z.ZodArray<z.ZodType<{
        Basic: {
            description: string;
            title: string;
            estimated_time: number;
        } & {
            [k: string]: unknown;
        };
    } | {
        BreakoutRoom: {
            prompt: string;
            estimated_time: number;
            instructions: string;
            max_per_room?: number | null | undefined;
            time_limit?: number | null | undefined;
        } & {
            [k: string]: unknown;
        };
    }, z.ZodTypeDef, any>, "many">, z.ZodNull]>>>;
    capacity: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
    custom_event_link: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    default_time_zone: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    description: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    end_time: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    format: z.ZodOptional<z.ZodUnion<[z.ZodType<"online" | "in_person", z.ZodTypeDef, any>, z.ZodNull]>>;
    location: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
        address_line_1: z.ZodString;
        address_line_2: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        address_line_3: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        city: z.ZodString;
        country_code: z.ZodString;
        postal_code: z.ZodString;
        state_province: z.ZodString;
        venue_name: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
    metadata: z.ZodOptional<z.ZodUnknown>;
    name: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    signup_mode: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    start_time: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    agenda: z.ZodOptional<z.ZodDefault<z.ZodUnion<[z.ZodArray<z.ZodType<{
        Basic: {
            description: string;
            title: string;
            estimated_time: number;
        } & {
            [k: string]: unknown;
        };
    } | {
        BreakoutRoom: {
            prompt: string;
            estimated_time: number;
            instructions: string;
            max_per_room?: number | null | undefined;
            time_limit?: number | null | undefined;
        } & {
            [k: string]: unknown;
        };
    }, z.ZodTypeDef, any>, "many">, z.ZodNull]>>>;
    capacity: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
    custom_event_link: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    default_time_zone: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    description: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    end_time: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    format: z.ZodOptional<z.ZodUnion<[z.ZodType<"online" | "in_person", z.ZodTypeDef, any>, z.ZodNull]>>;
    location: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
        address_line_1: z.ZodString;
        address_line_2: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        address_line_3: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        city: z.ZodString;
        country_code: z.ZodString;
        postal_code: z.ZodString;
        state_province: z.ZodString;
        venue_name: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
    metadata: z.ZodOptional<z.ZodUnknown>;
    name: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    signup_mode: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    start_time: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    agenda: z.ZodOptional<z.ZodDefault<z.ZodUnion<[z.ZodArray<z.ZodType<{
        Basic: {
            description: string;
            title: string;
            estimated_time: number;
        } & {
            [k: string]: unknown;
        };
    } | {
        BreakoutRoom: {
            prompt: string;
            estimated_time: number;
            instructions: string;
            max_per_room?: number | null | undefined;
            time_limit?: number | null | undefined;
        } & {
            [k: string]: unknown;
        };
    }, z.ZodTypeDef, any>, "many">, z.ZodNull]>>>;
    capacity: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
    custom_event_link: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    default_time_zone: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    description: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    end_time: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    format: z.ZodOptional<z.ZodUnion<[z.ZodType<"online" | "in_person", z.ZodTypeDef, any>, z.ZodNull]>>;
    location: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
        address_line_1: z.ZodString;
        address_line_2: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        address_line_3: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        city: z.ZodString;
        country_code: z.ZodString;
        postal_code: z.ZodString;
        state_province: z.ZodString;
        venue_name: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
    metadata: z.ZodOptional<z.ZodUnknown>;
    name: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    signup_mode: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    start_time: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">>;
export type PartialEvent = z.infer<typeof _PartialEvent>;
export declare const PartialEvent: z.ZodType<PartialEvent, z.ZodTypeDef, any>;
declare const _JwtResponse: z.ZodObject<{
    isModerator: z.ZodBoolean;
    jwt: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    isModerator: z.ZodBoolean;
    jwt: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    isModerator: z.ZodBoolean;
    jwt: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type JwtResponse = z.infer<typeof _JwtResponse>;
export declare const JwtResponse: z.ZodType<JwtResponse, z.ZodTypeDef, any>;
declare const _BreakoutSeatDto: z.ZodObject<{
    inviteId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    isModerator: z.ZodBoolean;
    label: z.ZodString;
    pending: z.ZodBoolean;
    userId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    inviteId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    isModerator: z.ZodBoolean;
    label: z.ZodString;
    pending: z.ZodBoolean;
    userId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    inviteId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    isModerator: z.ZodBoolean;
    label: z.ZodString;
    pending: z.ZodBoolean;
    userId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">>;
export type BreakoutSeatDto = z.infer<typeof _BreakoutSeatDto>;
export declare const BreakoutSeatDto: z.ZodType<BreakoutSeatDto, z.ZodTypeDef, any>;
declare const _BreakoutRoomDto: z.ZodObject<{
    seats: z.ZodArray<z.ZodType<z.objectOutputType<{
        inviteId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        isModerator: z.ZodBoolean;
        label: z.ZodString;
        pending: z.ZodBoolean;
        userId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    seats: z.ZodArray<z.ZodType<z.objectOutputType<{
        inviteId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        isModerator: z.ZodBoolean;
        label: z.ZodString;
        pending: z.ZodBoolean;
        userId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    seats: z.ZodArray<z.ZodType<z.objectOutputType<{
        inviteId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        isModerator: z.ZodBoolean;
        label: z.ZodString;
        pending: z.ZodBoolean;
        userId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
}, z.ZodTypeAny, "passthrough">>;
export type BreakoutRoomDto = z.infer<typeof _BreakoutRoomDto>;
export declare const BreakoutRoomDto: z.ZodType<BreakoutRoomDto, z.ZodTypeDef, any>;
declare const _BreakoutPlanDto: z.ZodObject<{
    rooms: z.ZodArray<z.ZodType<z.objectOutputType<{
        seats: z.ZodArray<z.ZodType<z.objectOutputType<{
            inviteId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
            isModerator: z.ZodBoolean;
            label: z.ZodString;
            pending: z.ZodBoolean;
            userId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    rooms: z.ZodArray<z.ZodType<z.objectOutputType<{
        seats: z.ZodArray<z.ZodType<z.objectOutputType<{
            inviteId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
            isModerator: z.ZodBoolean;
            label: z.ZodString;
            pending: z.ZodBoolean;
            userId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    rooms: z.ZodArray<z.ZodType<z.objectOutputType<{
        seats: z.ZodArray<z.ZodType<z.objectOutputType<{
            inviteId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
            isModerator: z.ZodBoolean;
            label: z.ZodString;
            pending: z.ZodBoolean;
            userId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
}, z.ZodTypeAny, "passthrough">>;
export type BreakoutPlanDto = z.infer<typeof _BreakoutPlanDto>;
export declare const BreakoutPlanDto: z.ZodType<BreakoutPlanDto, z.ZodTypeDef, any>;
declare const _SaveBreakoutPlanRequest: z.ZodObject<{
    rooms: z.ZodArray<z.ZodType<z.objectOutputType<{
        seats: z.ZodOptional<z.ZodDefault<z.ZodArray<z.ZodType<z.objectOutputType<{
            invite_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
            is_moderator: z.ZodOptional<z.ZodDefault<z.ZodBoolean>>;
            user_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">>>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    rooms: z.ZodArray<z.ZodType<z.objectOutputType<{
        seats: z.ZodOptional<z.ZodDefault<z.ZodArray<z.ZodType<z.objectOutputType<{
            invite_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
            is_moderator: z.ZodOptional<z.ZodDefault<z.ZodBoolean>>;
            user_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">>>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    rooms: z.ZodArray<z.ZodType<z.objectOutputType<{
        seats: z.ZodOptional<z.ZodDefault<z.ZodArray<z.ZodType<z.objectOutputType<{
            invite_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
            is_moderator: z.ZodOptional<z.ZodDefault<z.ZodBoolean>>;
            user_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">>>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
}, z.ZodTypeAny, "passthrough">>;
export type SaveBreakoutPlanRequest = z.infer<typeof _SaveBreakoutPlanRequest>;
export declare const SaveBreakoutPlanRequest: z.ZodType<SaveBreakoutPlanRequest, z.ZodTypeDef, any>;
declare const _EventAttendanceEtx: z.ZodObject<{
    createdAt: z.ZodString;
    email: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    eventId: z.ZodString;
    id: z.ZodString;
    role: z.ZodString;
    updatedAt: z.ZodString;
    userId: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    createdAt: z.ZodString;
    email: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    eventId: z.ZodString;
    id: z.ZodString;
    role: z.ZodString;
    updatedAt: z.ZodString;
    userId: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    createdAt: z.ZodString;
    email: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    eventId: z.ZodString;
    id: z.ZodString;
    role: z.ZodString;
    updatedAt: z.ZodString;
    userId: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type EventAttendanceEtx = z.infer<typeof _EventAttendanceEtx>;
export declare const EventAttendanceEtx: z.ZodType<EventAttendanceEtx, z.ZodTypeDef, any>;
declare const _PaginatedResults_for_EventAttendanceEtx: z.ZodObject<{
    records: z.ZodArray<z.ZodType<z.objectOutputType<{
        createdAt: z.ZodString;
        email: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        eventId: z.ZodString;
        id: z.ZodString;
        role: z.ZodString;
        updatedAt: z.ZodString;
        userId: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    total: z.ZodNumber;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    records: z.ZodArray<z.ZodType<z.objectOutputType<{
        createdAt: z.ZodString;
        email: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        eventId: z.ZodString;
        id: z.ZodString;
        role: z.ZodString;
        updatedAt: z.ZodString;
        userId: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    total: z.ZodNumber;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    records: z.ZodArray<z.ZodType<z.objectOutputType<{
        createdAt: z.ZodString;
        email: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        eventId: z.ZodString;
        id: z.ZodString;
        role: z.ZodString;
        updatedAt: z.ZodString;
        userId: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    total: z.ZodNumber;
}, z.ZodTypeAny, "passthrough">>;
export type PaginatedResults_for_EventAttendanceEtx = z.infer<typeof _PaginatedResults_for_EventAttendanceEtx>;
export declare const PaginatedResults_for_EventAttendanceEtx: z.ZodType<PaginatedResults_for_EventAttendanceEtx, z.ZodTypeDef, any>;
declare const _CreateEventAttendanceRequest: z.ZodObject<{
    role: z.ZodString;
    user_email: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    role: z.ZodString;
    user_email: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    role: z.ZodString;
    user_email: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">>;
export type CreateEventAttendanceRequest = z.infer<typeof _CreateEventAttendanceRequest>;
export declare const CreateEventAttendanceRequest: z.ZodType<CreateEventAttendanceRequest, z.ZodTypeDef, any>;
declare const _EventAttendanceDto: z.ZodObject<{
    createdAt: z.ZodString;
    eventId: z.ZodString;
    id: z.ZodString;
    role: z.ZodString;
    userId: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    createdAt: z.ZodString;
    eventId: z.ZodString;
    id: z.ZodString;
    role: z.ZodString;
    userId: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    createdAt: z.ZodString;
    eventId: z.ZodString;
    id: z.ZodString;
    role: z.ZodString;
    userId: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type EventAttendanceDto = z.infer<typeof _EventAttendanceDto>;
export declare const EventAttendanceDto: z.ZodType<EventAttendanceDto, z.ZodTypeDef, any>;
declare const _UpdateEventAttendanceRequest: z.ZodObject<{
    role: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    role: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    role: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">>;
export type UpdateEventAttendanceRequest = z.infer<typeof _UpdateEventAttendanceRequest>;
export declare const UpdateEventAttendanceRequest: z.ZodType<UpdateEventAttendanceRequest, z.ZodTypeDef, any>;
declare const _CreateFacilitatorRequest: z.ZodObject<{
    email: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    email: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    email: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type CreateFacilitatorRequest = z.infer<typeof _CreateFacilitatorRequest>;
export declare const CreateFacilitatorRequest: z.ZodType<CreateFacilitatorRequest, z.ZodTypeDef, any>;
declare const _AudioFormat: z.ZodEnum<["wav", "mp3", "m4a", "mp4", "ogg", "flac", "webm"]>;
export type AudioFormat = z.infer<typeof _AudioFormat>;
export declare const AudioFormat: z.ZodType<AudioFormat, z.ZodTypeDef, any>;
declare const _AudioRecordingStatus: z.ZodUnion<[z.ZodLiteral<"awaiting_upload">, z.ZodLiteral<"transcribing">, z.ZodLiteral<"categorizing">, z.ZodLiteral<"complete">, z.ZodLiteral<"transcription_failed">, z.ZodLiteral<"categorization_failed">]>;
export type AudioRecordingStatus = z.infer<typeof _AudioRecordingStatus>;
export declare const AudioRecordingStatus: z.ZodType<AudioRecordingStatus, z.ZodTypeDef, any>;
declare const _AudioRecordingDto: z.ZodObject<{
    createdAt: z.ZodString;
    eventId: z.ZodString;
    fileExtension: z.ZodType<"wav" | "mp3" | "m4a" | "mp4" | "ogg" | "flac" | "webm", z.ZodTypeDef, any>;
    id: z.ZodString;
    name: z.ZodString;
    s3KeyPrefix: z.ZodString;
    status: z.ZodType<"awaiting_upload" | "transcribing" | "categorizing" | "complete" | "transcription_failed" | "categorization_failed", z.ZodTypeDef, any>;
    updatedAt: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    createdAt: z.ZodString;
    eventId: z.ZodString;
    fileExtension: z.ZodType<"wav" | "mp3" | "m4a" | "mp4" | "ogg" | "flac" | "webm", z.ZodTypeDef, any>;
    id: z.ZodString;
    name: z.ZodString;
    s3KeyPrefix: z.ZodString;
    status: z.ZodType<"awaiting_upload" | "transcribing" | "categorizing" | "complete" | "transcription_failed" | "categorization_failed", z.ZodTypeDef, any>;
    updatedAt: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    createdAt: z.ZodString;
    eventId: z.ZodString;
    fileExtension: z.ZodType<"wav" | "mp3" | "m4a" | "mp4" | "ogg" | "flac" | "webm", z.ZodTypeDef, any>;
    id: z.ZodString;
    name: z.ZodString;
    s3KeyPrefix: z.ZodString;
    status: z.ZodType<"awaiting_upload" | "transcribing" | "categorizing" | "complete" | "transcription_failed" | "categorization_failed", z.ZodTypeDef, any>;
    updatedAt: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type AudioRecordingDto = z.infer<typeof _AudioRecordingDto>;
export declare const AudioRecordingDto: z.ZodType<AudioRecordingDto, z.ZodTypeDef, any>;
declare const _CreateRecordingRequest: z.ZodObject<{
    fileExtension: z.ZodType<"wav" | "mp3" | "m4a" | "mp4" | "ogg" | "flac" | "webm", z.ZodTypeDef, any>;
    name: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    fileExtension: z.ZodType<"wav" | "mp3" | "m4a" | "mp4" | "ogg" | "flac" | "webm", z.ZodTypeDef, any>;
    name: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    fileExtension: z.ZodType<"wav" | "mp3" | "m4a" | "mp4" | "ogg" | "flac" | "webm", z.ZodTypeDef, any>;
    name: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type CreateRecordingRequest = z.infer<typeof _CreateRecordingRequest>;
export declare const CreateRecordingRequest: z.ZodType<CreateRecordingRequest, z.ZodTypeDef, any>;
declare const _CreateRecordingResponse: z.ZodObject<{
    recording: z.ZodType<z.objectOutputType<{
        createdAt: z.ZodString;
        eventId: z.ZodString;
        fileExtension: z.ZodType<"wav" | "mp3" | "m4a" | "mp4" | "ogg" | "flac" | "webm", z.ZodTypeDef, any>;
        id: z.ZodString;
        name: z.ZodString;
        s3KeyPrefix: z.ZodString;
        status: z.ZodType<"awaiting_upload" | "transcribing" | "categorizing" | "complete" | "transcription_failed" | "categorization_failed", z.ZodTypeDef, any>;
        updatedAt: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    uploadUrl: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    recording: z.ZodType<z.objectOutputType<{
        createdAt: z.ZodString;
        eventId: z.ZodString;
        fileExtension: z.ZodType<"wav" | "mp3" | "m4a" | "mp4" | "ogg" | "flac" | "webm", z.ZodTypeDef, any>;
        id: z.ZodString;
        name: z.ZodString;
        s3KeyPrefix: z.ZodString;
        status: z.ZodType<"awaiting_upload" | "transcribing" | "categorizing" | "complete" | "transcription_failed" | "categorization_failed", z.ZodTypeDef, any>;
        updatedAt: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    uploadUrl: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    recording: z.ZodType<z.objectOutputType<{
        createdAt: z.ZodString;
        eventId: z.ZodString;
        fileExtension: z.ZodType<"wav" | "mp3" | "m4a" | "mp4" | "ogg" | "flac" | "webm", z.ZodTypeDef, any>;
        id: z.ZodString;
        name: z.ZodString;
        s3KeyPrefix: z.ZodString;
        status: z.ZodType<"awaiting_upload" | "transcribing" | "categorizing" | "complete" | "transcription_failed" | "categorization_failed", z.ZodTypeDef, any>;
        updatedAt: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    uploadUrl: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type CreateRecordingResponse = z.infer<typeof _CreateRecordingResponse>;
export declare const CreateRecordingResponse: z.ZodType<CreateRecordingResponse, z.ZodTypeDef, any>;
declare const _RecordingDownloadUrls: z.ZodObject<{
    recordingUrl: z.ZodString;
    reportUrl: z.ZodString;
    transcriptUrl: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    recordingUrl: z.ZodString;
    reportUrl: z.ZodString;
    transcriptUrl: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    recordingUrl: z.ZodString;
    reportUrl: z.ZodString;
    transcriptUrl: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type RecordingDownloadUrls = z.infer<typeof _RecordingDownloadUrls>;
export declare const RecordingDownloadUrls: z.ZodType<RecordingDownloadUrls, z.ZodTypeDef, any>;
declare const _RecordingDetailResponse: z.ZodObject<{
    downloads: z.ZodType<z.objectOutputType<{
        recordingUrl: z.ZodString;
        reportUrl: z.ZodString;
        transcriptUrl: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    recording: z.ZodType<z.objectOutputType<{
        createdAt: z.ZodString;
        eventId: z.ZodString;
        fileExtension: z.ZodType<"wav" | "mp3" | "m4a" | "mp4" | "ogg" | "flac" | "webm", z.ZodTypeDef, any>;
        id: z.ZodString;
        name: z.ZodString;
        s3KeyPrefix: z.ZodString;
        status: z.ZodType<"awaiting_upload" | "transcribing" | "categorizing" | "complete" | "transcription_failed" | "categorization_failed", z.ZodTypeDef, any>;
        updatedAt: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    downloads: z.ZodType<z.objectOutputType<{
        recordingUrl: z.ZodString;
        reportUrl: z.ZodString;
        transcriptUrl: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    recording: z.ZodType<z.objectOutputType<{
        createdAt: z.ZodString;
        eventId: z.ZodString;
        fileExtension: z.ZodType<"wav" | "mp3" | "m4a" | "mp4" | "ogg" | "flac" | "webm", z.ZodTypeDef, any>;
        id: z.ZodString;
        name: z.ZodString;
        s3KeyPrefix: z.ZodString;
        status: z.ZodType<"awaiting_upload" | "transcribing" | "categorizing" | "complete" | "transcription_failed" | "categorization_failed", z.ZodTypeDef, any>;
        updatedAt: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    downloads: z.ZodType<z.objectOutputType<{
        recordingUrl: z.ZodString;
        reportUrl: z.ZodString;
        transcriptUrl: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    recording: z.ZodType<z.objectOutputType<{
        createdAt: z.ZodString;
        eventId: z.ZodString;
        fileExtension: z.ZodType<"wav" | "mp3" | "m4a" | "mp4" | "ogg" | "flac" | "webm", z.ZodTypeDef, any>;
        id: z.ZodString;
        name: z.ZodString;
        s3KeyPrefix: z.ZodString;
        status: z.ZodType<"awaiting_upload" | "transcribing" | "categorizing" | "complete" | "transcription_failed" | "categorization_failed", z.ZodTypeDef, any>;
        updatedAt: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
}, z.ZodTypeAny, "passthrough">>;
export type RecordingDetailResponse = z.infer<typeof _RecordingDetailResponse>;
export declare const RecordingDetailResponse: z.ZodType<RecordingDetailResponse, z.ZodTypeDef, any>;
declare const _DeleteRecordingResponse: z.ZodObject<{
    recording: z.ZodType<z.objectOutputType<{
        createdAt: z.ZodString;
        eventId: z.ZodString;
        fileExtension: z.ZodType<"wav" | "mp3" | "m4a" | "mp4" | "ogg" | "flac" | "webm", z.ZodTypeDef, any>;
        id: z.ZodString;
        name: z.ZodString;
        s3KeyPrefix: z.ZodString;
        status: z.ZodType<"awaiting_upload" | "transcribing" | "categorizing" | "complete" | "transcription_failed" | "categorization_failed", z.ZodTypeDef, any>;
        updatedAt: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    recording: z.ZodType<z.objectOutputType<{
        createdAt: z.ZodString;
        eventId: z.ZodString;
        fileExtension: z.ZodType<"wav" | "mp3" | "m4a" | "mp4" | "ogg" | "flac" | "webm", z.ZodTypeDef, any>;
        id: z.ZodString;
        name: z.ZodString;
        s3KeyPrefix: z.ZodString;
        status: z.ZodType<"awaiting_upload" | "transcribing" | "categorizing" | "complete" | "transcription_failed" | "categorization_failed", z.ZodTypeDef, any>;
        updatedAt: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    recording: z.ZodType<z.objectOutputType<{
        createdAt: z.ZodString;
        eventId: z.ZodString;
        fileExtension: z.ZodType<"wav" | "mp3" | "m4a" | "mp4" | "ogg" | "flac" | "webm", z.ZodTypeDef, any>;
        id: z.ZodString;
        name: z.ZodString;
        s3KeyPrefix: z.ZodString;
        status: z.ZodType<"awaiting_upload" | "transcribing" | "categorizing" | "complete" | "transcription_failed" | "categorization_failed", z.ZodTypeDef, any>;
        updatedAt: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
}, z.ZodTypeAny, "passthrough">>;
export type DeleteRecordingResponse = z.infer<typeof _DeleteRecordingResponse>;
export declare const DeleteRecordingResponse: z.ZodType<DeleteRecordingResponse, z.ZodTypeDef, any>;
declare const _ProcessRecordingResponse: z.ZodObject<{
    jobId: z.ZodString;
    message: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    jobId: z.ZodString;
    message: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    jobId: z.ZodString;
    message: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type ProcessRecordingResponse = z.infer<typeof _ProcessRecordingResponse>;
export declare const ProcessRecordingResponse: z.ZodType<ProcessRecordingResponse, z.ZodTypeDef, any>;
declare const _SubmitReportResponse: z.ZodObject<{
    success: z.ZodBoolean;
    url: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    success: z.ZodBoolean;
    url: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    success: z.ZodBoolean;
    url: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type SubmitReportResponse = z.infer<typeof _SubmitReportResponse>;
export declare const SubmitReportResponse: z.ZodType<SubmitReportResponse, z.ZodTypeDef, any>;
declare const _WebSocketStats: z.ZodObject<{
    connected_users: z.ZodArray<z.ZodString, "many">;
    total_connections: z.ZodNumber;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    connected_users: z.ZodArray<z.ZodString, "many">;
    total_connections: z.ZodNumber;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    connected_users: z.ZodArray<z.ZodString, "many">;
    total_connections: z.ZodNumber;
}, z.ZodTypeAny, "passthrough">>;
export type WebSocketStats = z.infer<typeof _WebSocketStats>;
export declare const WebSocketStats: z.ZodType<WebSocketStats, z.ZodTypeDef, any>;
declare const _BroadcastMessage: z.ZodObject<{
    authenticated_only: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    message: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    authenticated_only: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    message: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    authenticated_only: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    message: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type BroadcastMessage = z.infer<typeof _BroadcastMessage>;
export declare const BroadcastMessage: z.ZodType<BroadcastMessage, z.ZodTypeDef, any>;
declare const _BroadcastResponse: z.ZodObject<{
    message: z.ZodString;
    sent_to: z.ZodNumber;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    message: z.ZodString;
    sent_to: z.ZodNumber;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    message: z.ZodString;
    sent_to: z.ZodNumber;
}, z.ZodTypeAny, "passthrough">>;
export type BroadcastResponse = z.infer<typeof _BroadcastResponse>;
export declare const BroadcastResponse: z.ZodType<BroadcastResponse, z.ZodTypeDef, any>;
declare const _SendToUserMessage: z.ZodObject<{
    message: z.ZodString;
    user_id: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    message: z.ZodString;
    user_id: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    message: z.ZodString;
    user_id: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type SendToUserMessage = z.infer<typeof _SendToUserMessage>;
export declare const SendToUserMessage: z.ZodType<SendToUserMessage, z.ZodTypeDef, any>;
declare const _PaginatedResults_for_LocalizedOrganizationDto: z.ZodObject<{
    records: z.ZodArray<z.ZodType<z.objectOutputType<{
        contactEmail: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        createdAt: z.ZodString;
        description: z.ZodString;
        externalUrl: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        id: z.ZodString;
        metadata: z.ZodOptional<z.ZodUnknown>;
        mission: z.ZodString;
        name: z.ZodString;
        orgType: z.ZodType<"non_profit" | "governmental" | "other", z.ZodTypeDef, any>;
        regions: z.ZodArray<z.ZodString, "many">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    total: z.ZodNumber;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    records: z.ZodArray<z.ZodType<z.objectOutputType<{
        contactEmail: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        createdAt: z.ZodString;
        description: z.ZodString;
        externalUrl: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        id: z.ZodString;
        metadata: z.ZodOptional<z.ZodUnknown>;
        mission: z.ZodString;
        name: z.ZodString;
        orgType: z.ZodType<"non_profit" | "governmental" | "other", z.ZodTypeDef, any>;
        regions: z.ZodArray<z.ZodString, "many">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    total: z.ZodNumber;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    records: z.ZodArray<z.ZodType<z.objectOutputType<{
        contactEmail: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        createdAt: z.ZodString;
        description: z.ZodString;
        externalUrl: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        id: z.ZodString;
        metadata: z.ZodOptional<z.ZodUnknown>;
        mission: z.ZodString;
        name: z.ZodString;
        orgType: z.ZodType<"non_profit" | "governmental" | "other", z.ZodTypeDef, any>;
        regions: z.ZodArray<z.ZodString, "many">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    total: z.ZodNumber;
}, z.ZodTypeAny, "passthrough">>;
export type PaginatedResults_for_LocalizedOrganizationDto = z.infer<typeof _PaginatedResults_for_LocalizedOrganizationDto>;
export declare const PaginatedResults_for_LocalizedOrganizationDto: z.ZodType<PaginatedResults_for_LocalizedOrganizationDto, z.ZodTypeDef, any>;
declare const _CreateOrganization: z.ZodObject<{
    contact_email: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    description: z.ZodString;
    external_url: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    mission: z.ZodString;
    name: z.ZodString;
    org_type: z.ZodType<"non_profit" | "governmental" | "other", z.ZodTypeDef, any>;
    regions: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodString, "many">, z.ZodNull]>>;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    contact_email: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    description: z.ZodString;
    external_url: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    mission: z.ZodString;
    name: z.ZodString;
    org_type: z.ZodType<"non_profit" | "governmental" | "other", z.ZodTypeDef, any>;
    regions: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodString, "many">, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    contact_email: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    description: z.ZodString;
    external_url: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    mission: z.ZodString;
    name: z.ZodString;
    org_type: z.ZodType<"non_profit" | "governmental" | "other", z.ZodTypeDef, any>;
    regions: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodString, "many">, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">>;
export type CreateOrganization = z.infer<typeof _CreateOrganization>;
export declare const CreateOrganization: z.ZodType<CreateOrganization, z.ZodTypeDef, any>;
declare const _OrganizationDto: z.ZodObject<{
    contactEmail: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    createdAt: z.ZodString;
    description: z.ZodString;
    externalUrl: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    id: z.ZodString;
    metadata: z.ZodOptional<z.ZodUnknown>;
    mission: z.ZodString;
    name: z.ZodString;
    orgType: z.ZodType<"non_profit" | "governmental" | "other", z.ZodTypeDef, any>;
    regions: z.ZodArray<z.ZodString, "many">;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    contactEmail: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    createdAt: z.ZodString;
    description: z.ZodString;
    externalUrl: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    id: z.ZodString;
    metadata: z.ZodOptional<z.ZodUnknown>;
    mission: z.ZodString;
    name: z.ZodString;
    orgType: z.ZodType<"non_profit" | "governmental" | "other", z.ZodTypeDef, any>;
    regions: z.ZodArray<z.ZodString, "many">;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    contactEmail: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    createdAt: z.ZodString;
    description: z.ZodString;
    externalUrl: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    id: z.ZodString;
    metadata: z.ZodOptional<z.ZodUnknown>;
    mission: z.ZodString;
    name: z.ZodString;
    orgType: z.ZodType<"non_profit" | "governmental" | "other", z.ZodTypeDef, any>;
    regions: z.ZodArray<z.ZodString, "many">;
}, z.ZodTypeAny, "passthrough">>;
export type OrganizationDto = z.infer<typeof _OrganizationDto>;
export declare const OrganizationDto: z.ZodType<OrganizationDto, z.ZodTypeDef, any>;
declare const _UpdateOrganizationBody: z.ZodObject<{
    contact_email: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    description: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    external_url: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    metadata: z.ZodOptional<z.ZodUnknown>;
    mission: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    name: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    org_type: z.ZodOptional<z.ZodUnion<[z.ZodType<"non_profit" | "governmental" | "other", z.ZodTypeDef, any>, z.ZodNull]>>;
    regions: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodString, "many">, z.ZodNull]>>;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    contact_email: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    description: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    external_url: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    metadata: z.ZodOptional<z.ZodUnknown>;
    mission: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    name: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    org_type: z.ZodOptional<z.ZodUnion<[z.ZodType<"non_profit" | "governmental" | "other", z.ZodTypeDef, any>, z.ZodNull]>>;
    regions: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodString, "many">, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    contact_email: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    description: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    external_url: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    metadata: z.ZodOptional<z.ZodUnknown>;
    mission: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    name: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    org_type: z.ZodOptional<z.ZodUnion<[z.ZodType<"non_profit" | "governmental" | "other", z.ZodTypeDef, any>, z.ZodNull]>>;
    regions: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodString, "many">, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">>;
export type UpdateOrganizationBody = z.infer<typeof _UpdateOrganizationBody>;
export declare const UpdateOrganizationBody: z.ZodType<UpdateOrganizationBody, z.ZodTypeDef, any>;
declare const _OrganizationTeamRole: z.ZodEnum<["member", "admin"]>;
export type OrganizationTeamRole = z.infer<typeof _OrganizationTeamRole>;
export declare const OrganizationTeamRole: z.ZodType<OrganizationTeamRole, z.ZodTypeDef, any>;
declare const _OrganizationTeamUserDto: z.ZodObject<{
    email: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    id: z.ZodString;
    role: z.ZodType<"member" | "admin", z.ZodTypeDef, any>;
    username: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    email: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    id: z.ZodString;
    role: z.ZodType<"member" | "admin", z.ZodTypeDef, any>;
    username: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    email: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    id: z.ZodString;
    role: z.ZodType<"member" | "admin", z.ZodTypeDef, any>;
    username: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">>;
export type OrganizationTeamUserDto = z.infer<typeof _OrganizationTeamUserDto>;
export declare const OrganizationTeamUserDto: z.ZodType<OrganizationTeamUserDto, z.ZodTypeDef, any>;
declare const _OrganizationTeamResponseDto: z.ZodObject<{
    members: z.ZodArray<z.ZodType<z.objectOutputType<{
        email: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        id: z.ZodString;
        role: z.ZodType<"member" | "admin", z.ZodTypeDef, any>;
        username: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    members: z.ZodArray<z.ZodType<z.objectOutputType<{
        email: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        id: z.ZodString;
        role: z.ZodType<"member" | "admin", z.ZodTypeDef, any>;
        username: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    members: z.ZodArray<z.ZodType<z.objectOutputType<{
        email: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        id: z.ZodString;
        role: z.ZodType<"member" | "admin", z.ZodTypeDef, any>;
        username: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
}, z.ZodTypeAny, "passthrough">>;
export type OrganizationTeamResponseDto = z.infer<typeof _OrganizationTeamResponseDto>;
export declare const OrganizationTeamResponseDto: z.ZodType<OrganizationTeamResponseDto, z.ZodTypeDef, any>;
declare const _UpsertOrganizationUserBody: z.ZodObject<{
    allow_create_user: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    email: z.ZodString;
    role: z.ZodOptional<z.ZodUnion<[z.ZodType<"member" | "admin", z.ZodTypeDef, any>, z.ZodNull]>>;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    allow_create_user: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    email: z.ZodString;
    role: z.ZodOptional<z.ZodUnion<[z.ZodType<"member" | "admin", z.ZodTypeDef, any>, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    allow_create_user: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodNull]>>;
    email: z.ZodString;
    role: z.ZodOptional<z.ZodUnion<[z.ZodType<"member" | "admin", z.ZodTypeDef, any>, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">>;
export type UpsertOrganizationUserBody = z.infer<typeof _UpsertOrganizationUserBody>;
export declare const UpsertOrganizationUserBody: z.ZodType<UpsertOrganizationUserBody, z.ZodTypeDef, any>;
declare const _UpsertOrganizationUserResponseDto: z.ZodObject<{
    createdAccount: z.ZodBoolean;
    emailed: z.ZodBoolean;
    user: z.ZodType<z.objectOutputType<{
        email: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        id: z.ZodString;
        role: z.ZodType<"member" | "admin", z.ZodTypeDef, any>;
        username: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    createdAccount: z.ZodBoolean;
    emailed: z.ZodBoolean;
    user: z.ZodType<z.objectOutputType<{
        email: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        id: z.ZodString;
        role: z.ZodType<"member" | "admin", z.ZodTypeDef, any>;
        username: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    createdAccount: z.ZodBoolean;
    emailed: z.ZodBoolean;
    user: z.ZodType<z.objectOutputType<{
        email: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        id: z.ZodString;
        role: z.ZodType<"member" | "admin", z.ZodTypeDef, any>;
        username: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
}, z.ZodTypeAny, "passthrough">>;
export type UpsertOrganizationUserResponseDto = z.infer<typeof _UpsertOrganizationUserResponseDto>;
export declare const UpsertOrganizationUserResponseDto: z.ZodType<UpsertOrganizationUserResponseDto, z.ZodTypeDef, any>;
declare const _UpdateOrganizationMemberRoleBody: z.ZodObject<{
    role: z.ZodType<"member" | "admin", z.ZodTypeDef, any>;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    role: z.ZodType<"member" | "admin", z.ZodTypeDef, any>;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    role: z.ZodType<"member" | "admin", z.ZodTypeDef, any>;
}, z.ZodTypeAny, "passthrough">>;
export type UpdateOrganizationMemberRoleBody = z.infer<typeof _UpdateOrganizationMemberRoleBody>;
export declare const UpdateOrganizationMemberRoleBody: z.ZodType<UpdateOrganizationMemberRoleBody, z.ZodTypeDef, any>;
declare const _RegionType: z.ZodEnum<["custom", "official"]>;
export type RegionType = z.infer<typeof _RegionType>;
export declare const RegionType: z.ZodType<RegionType, z.ZodTypeDef, any>;
declare const _LocalizedRegionDto: z.ZodObject<{
    created_at: z.ZodString;
    description: z.ZodString;
    id: z.ZodString;
    metadata: z.ZodOptional<z.ZodUnknown>;
    name: z.ZodString;
    official_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    region_type: z.ZodType<"custom" | "official", z.ZodTypeDef, any>;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    created_at: z.ZodString;
    description: z.ZodString;
    id: z.ZodString;
    metadata: z.ZodOptional<z.ZodUnknown>;
    name: z.ZodString;
    official_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    region_type: z.ZodType<"custom" | "official", z.ZodTypeDef, any>;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    created_at: z.ZodString;
    description: z.ZodString;
    id: z.ZodString;
    metadata: z.ZodOptional<z.ZodUnknown>;
    name: z.ZodString;
    official_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    region_type: z.ZodType<"custom" | "official", z.ZodTypeDef, any>;
}, z.ZodTypeAny, "passthrough">>;
export type LocalizedRegionDto = z.infer<typeof _LocalizedRegionDto>;
export declare const LocalizedRegionDto: z.ZodType<LocalizedRegionDto, z.ZodTypeDef, any>;
declare const _PaginatedResults_for_LocalizedRegionDto: z.ZodObject<{
    records: z.ZodArray<z.ZodType<z.objectOutputType<{
        created_at: z.ZodString;
        description: z.ZodString;
        id: z.ZodString;
        metadata: z.ZodOptional<z.ZodUnknown>;
        name: z.ZodString;
        official_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        region_type: z.ZodType<"custom" | "official", z.ZodTypeDef, any>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    total: z.ZodNumber;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    records: z.ZodArray<z.ZodType<z.objectOutputType<{
        created_at: z.ZodString;
        description: z.ZodString;
        id: z.ZodString;
        metadata: z.ZodOptional<z.ZodUnknown>;
        name: z.ZodString;
        official_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        region_type: z.ZodType<"custom" | "official", z.ZodTypeDef, any>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    total: z.ZodNumber;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    records: z.ZodArray<z.ZodType<z.objectOutputType<{
        created_at: z.ZodString;
        description: z.ZodString;
        id: z.ZodString;
        metadata: z.ZodOptional<z.ZodUnknown>;
        name: z.ZodString;
        official_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        region_type: z.ZodType<"custom" | "official", z.ZodTypeDef, any>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    total: z.ZodNumber;
}, z.ZodTypeAny, "passthrough">>;
export type PaginatedResults_for_LocalizedRegionDto = z.infer<typeof _PaginatedResults_for_LocalizedRegionDto>;
export declare const PaginatedResults_for_LocalizedRegionDto: z.ZodType<PaginatedResults_for_LocalizedRegionDto, z.ZodTypeDef, any>;
declare const _CreateRegion: z.ZodObject<{
    description: z.ZodString;
    name: z.ZodString;
    official_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    region_type: z.ZodType<"custom" | "official", z.ZodTypeDef, any>;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    description: z.ZodString;
    name: z.ZodString;
    official_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    region_type: z.ZodType<"custom" | "official", z.ZodTypeDef, any>;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    description: z.ZodString;
    name: z.ZodString;
    official_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    region_type: z.ZodType<"custom" | "official", z.ZodTypeDef, any>;
}, z.ZodTypeAny, "passthrough">>;
export type CreateRegion = z.infer<typeof _CreateRegion>;
export declare const CreateRegion: z.ZodType<CreateRegion, z.ZodTypeDef, any>;
declare const _RegionDto: z.ZodObject<{
    created_at: z.ZodString;
    description: z.ZodString;
    id: z.ZodString;
    metadata: z.ZodOptional<z.ZodUnknown>;
    name: z.ZodString;
    official_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    region_type: z.ZodType<"custom" | "official", z.ZodTypeDef, any>;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    created_at: z.ZodString;
    description: z.ZodString;
    id: z.ZodString;
    metadata: z.ZodOptional<z.ZodUnknown>;
    name: z.ZodString;
    official_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    region_type: z.ZodType<"custom" | "official", z.ZodTypeDef, any>;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    created_at: z.ZodString;
    description: z.ZodString;
    id: z.ZodString;
    metadata: z.ZodOptional<z.ZodUnknown>;
    name: z.ZodString;
    official_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    region_type: z.ZodType<"custom" | "official", z.ZodTypeDef, any>;
}, z.ZodTypeAny, "passthrough">>;
export type RegionDto = z.infer<typeof _RegionDto>;
export declare const RegionDto: z.ZodType<RegionDto, z.ZodTypeDef, any>;
declare const _PartialRegion: z.ZodObject<{
    metadata: z.ZodOptional<z.ZodUnknown>;
    official_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    region_type: z.ZodOptional<z.ZodUnion<[z.ZodType<"custom" | "official", z.ZodTypeDef, any>, z.ZodNull]>>;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    metadata: z.ZodOptional<z.ZodUnknown>;
    official_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    region_type: z.ZodOptional<z.ZodUnion<[z.ZodType<"custom" | "official", z.ZodTypeDef, any>, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    metadata: z.ZodOptional<z.ZodUnknown>;
    official_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    region_type: z.ZodOptional<z.ZodUnion<[z.ZodType<"custom" | "official", z.ZodTypeDef, any>, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">>;
export type PartialRegion = z.infer<typeof _PartialRegion>;
export declare const PartialRegion: z.ZodType<PartialRegion, z.ZodTypeDef, any>;
declare const _RegionAreaLinksDto: z.ZodObject<{
    area_ids: z.ZodArray<z.ZodString, "many">;
    region_id: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    area_ids: z.ZodArray<z.ZodString, "many">;
    region_id: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    area_ids: z.ZodArray<z.ZodString, "many">;
    region_id: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type RegionAreaLinksDto = z.infer<typeof _RegionAreaLinksDto>;
export declare const RegionAreaLinksDto: z.ZodType<RegionAreaLinksDto, z.ZodTypeDef, any>;
declare const _RegionAreaLinksRequestDto: z.ZodObject<{
    area_ids: z.ZodArray<z.ZodString, "many">;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    area_ids: z.ZodArray<z.ZodString, "many">;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    area_ids: z.ZodArray<z.ZodString, "many">;
}, z.ZodTypeAny, "passthrough">>;
export type RegionAreaLinksRequestDto = z.infer<typeof _RegionAreaLinksRequestDto>;
export declare const RegionAreaLinksRequestDto: z.ZodType<RegionAreaLinksRequestDto, z.ZodTypeDef, any>;
declare const _RegionAreaDto: z.ZodObject<{
    createdAt: z.ZodString;
    id: z.ZodString;
    zipPrefix: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    createdAt: z.ZodString;
    id: z.ZodString;
    zipPrefix: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    createdAt: z.ZodString;
    id: z.ZodString;
    zipPrefix: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type RegionAreaDto = z.infer<typeof _RegionAreaDto>;
export declare const RegionAreaDto: z.ZodType<RegionAreaDto, z.ZodTypeDef, any>;
declare const _CreateRegionArea: z.ZodObject<{
    zip_prefix: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    zip_prefix: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    zip_prefix: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type CreateRegionArea = z.infer<typeof _CreateRegionArea>;
export declare const CreateRegionArea: z.ZodType<CreateRegionArea, z.ZodTypeDef, any>;
declare const _PartialRegionArea: z.ZodObject<{
    zip_prefix: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    zip_prefix: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    zip_prefix: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">>;
export type PartialRegionArea = z.infer<typeof _PartialRegionArea>;
export declare const PartialRegionArea: z.ZodType<PartialRegionArea, z.ZodTypeDef, any>;
declare const _MediaContentType: z.ZodEnum<["image/jpeg", "image/png", "image/gif", "image/webp", "video/mp4", "video/mpeg", "video/webm", "audio/mpeg", "audio/mp4", "audio/webm", "audio/wav", "audio/ogg"]>;
export type MediaContentType = z.infer<typeof _MediaContentType>;
export declare const MediaContentType: z.ZodType<MediaContentType, z.ZodTypeDef, any>;
declare const _content_type: z.ZodOptional<z.ZodUnion<[z.ZodType<"image/jpeg" | "image/png" | "image/gif" | "image/webp" | "video/mp4" | "video/mpeg" | "video/webm" | "audio/mpeg" | "audio/mp4" | "audio/webm" | "audio/wav" | "audio/ogg", z.ZodTypeDef, any>, z.ZodNull]>>;
export type content_type = z.infer<typeof _content_type>;
export declare const content_type: z.ZodType<content_type, z.ZodTypeDef, any>;
declare const _MediaDto: z.ZodObject<{
    alt: z.ZodString;
    contentType: z.ZodType<"image/jpeg" | "image/png" | "image/gif" | "image/webp" | "video/mp4" | "video/mpeg" | "video/webm" | "audio/mpeg" | "audio/mp4" | "audio/webm" | "audio/wav" | "audio/ogg", z.ZodTypeDef, any>;
    createdAt: z.ZodString;
    filename: z.ZodString;
    id: z.ZodString;
    name: z.ZodString;
    ownerId: z.ZodString;
    storageKey: z.ZodString;
    storeName: z.ZodString;
    url: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    alt: z.ZodString;
    contentType: z.ZodType<"image/jpeg" | "image/png" | "image/gif" | "image/webp" | "video/mp4" | "video/mpeg" | "video/webm" | "audio/mpeg" | "audio/mp4" | "audio/webm" | "audio/wav" | "audio/ogg", z.ZodTypeDef, any>;
    createdAt: z.ZodString;
    filename: z.ZodString;
    id: z.ZodString;
    name: z.ZodString;
    ownerId: z.ZodString;
    storageKey: z.ZodString;
    storeName: z.ZodString;
    url: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    alt: z.ZodString;
    contentType: z.ZodType<"image/jpeg" | "image/png" | "image/gif" | "image/webp" | "video/mp4" | "video/mpeg" | "video/webm" | "audio/mpeg" | "audio/mp4" | "audio/webm" | "audio/wav" | "audio/ogg", z.ZodTypeDef, any>;
    createdAt: z.ZodString;
    filename: z.ZodString;
    id: z.ZodString;
    name: z.ZodString;
    ownerId: z.ZodString;
    storageKey: z.ZodString;
    storeName: z.ZodString;
    url: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type MediaDto = z.infer<typeof _MediaDto>;
export declare const MediaDto: z.ZodType<MediaDto, z.ZodTypeDef, any>;
declare const _PaginatedResults_for_MediaDto: z.ZodObject<{
    records: z.ZodArray<z.ZodType<z.objectOutputType<{
        alt: z.ZodString;
        contentType: z.ZodType<"image/jpeg" | "image/png" | "image/gif" | "image/webp" | "video/mp4" | "video/mpeg" | "video/webm" | "audio/mpeg" | "audio/mp4" | "audio/webm" | "audio/wav" | "audio/ogg", z.ZodTypeDef, any>;
        createdAt: z.ZodString;
        filename: z.ZodString;
        id: z.ZodString;
        name: z.ZodString;
        ownerId: z.ZodString;
        storageKey: z.ZodString;
        storeName: z.ZodString;
        url: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    total: z.ZodNumber;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    records: z.ZodArray<z.ZodType<z.objectOutputType<{
        alt: z.ZodString;
        contentType: z.ZodType<"image/jpeg" | "image/png" | "image/gif" | "image/webp" | "video/mp4" | "video/mpeg" | "video/webm" | "audio/mpeg" | "audio/mp4" | "audio/webm" | "audio/wav" | "audio/ogg", z.ZodTypeDef, any>;
        createdAt: z.ZodString;
        filename: z.ZodString;
        id: z.ZodString;
        name: z.ZodString;
        ownerId: z.ZodString;
        storageKey: z.ZodString;
        storeName: z.ZodString;
        url: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    total: z.ZodNumber;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    records: z.ZodArray<z.ZodType<z.objectOutputType<{
        alt: z.ZodString;
        contentType: z.ZodType<"image/jpeg" | "image/png" | "image/gif" | "image/webp" | "video/mp4" | "video/mpeg" | "video/webm" | "audio/mpeg" | "audio/mp4" | "audio/webm" | "audio/wav" | "audio/ogg", z.ZodTypeDef, any>;
        createdAt: z.ZodString;
        filename: z.ZodString;
        id: z.ZodString;
        name: z.ZodString;
        ownerId: z.ZodString;
        storageKey: z.ZodString;
        storeName: z.ZodString;
        url: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    total: z.ZodNumber;
}, z.ZodTypeAny, "passthrough">>;
export type PaginatedResults_for_MediaDto = z.infer<typeof _PaginatedResults_for_MediaDto>;
export declare const PaginatedResults_for_MediaDto: z.ZodType<PaginatedResults_for_MediaDto, z.ZodTypeDef, any>;
declare const _MediaEditableFields: z.ZodObject<{
    alt: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    name: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    alt: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    name: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    alt: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    name: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">>;
export type MediaEditableFields = z.infer<typeof _MediaEditableFields>;
export declare const MediaEditableFields: z.ZodType<MediaEditableFields, z.ZodTypeDef, any>;
declare const _Job: z.ZodObject<{
    completion_message: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    created_at: z.ZodString;
    error: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    finished_at: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    id: z.ZodString;
    progress: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
    status: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    step: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    completion_message: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    created_at: z.ZodString;
    error: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    finished_at: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    id: z.ZodString;
    progress: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
    status: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    step: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    completion_message: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    created_at: z.ZodString;
    error: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    finished_at: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    id: z.ZodString;
    progress: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
    status: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    step: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">>;
export type Job = z.infer<typeof _Job>;
export declare const Job: z.ZodType<Job, z.ZodTypeDef, any>;
declare const _PaginatedResults_for_Job: z.ZodObject<{
    records: z.ZodArray<z.ZodType<z.objectOutputType<{
        completion_message: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        created_at: z.ZodString;
        error: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        finished_at: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        id: z.ZodString;
        progress: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        status: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        step: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    total: z.ZodNumber;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    records: z.ZodArray<z.ZodType<z.objectOutputType<{
        completion_message: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        created_at: z.ZodString;
        error: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        finished_at: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        id: z.ZodString;
        progress: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        status: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        step: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    total: z.ZodNumber;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    records: z.ZodArray<z.ZodType<z.objectOutputType<{
        completion_message: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        created_at: z.ZodString;
        error: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        finished_at: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        id: z.ZodString;
        progress: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        status: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        step: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    total: z.ZodNumber;
}, z.ZodTypeAny, "passthrough">>;
export type PaginatedResults_for_Job = z.infer<typeof _PaginatedResults_for_Job>;
export declare const PaginatedResults_for_Job: z.ZodType<PaginatedResults_for_Job, z.ZodTypeDef, any>;
declare const _CreateJob: z.ZodObject<{
    progress: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
    step: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    progress: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
    step: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    progress: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
    step: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">>;
export type CreateJob = z.infer<typeof _CreateJob>;
export declare const CreateJob: z.ZodType<CreateJob, z.ZodTypeDef, any>;
declare const _ComhairleServices: z.ZodObject<{
    botService: z.ZodBoolean;
    translationService: z.ZodBoolean;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    botService: z.ZodBoolean;
    translationService: z.ZodBoolean;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    botService: z.ZodBoolean;
    translationService: z.ZodBoolean;
}, z.ZodTypeAny, "passthrough">>;
export type ComhairleServices = z.infer<typeof _ComhairleServices>;
export declare const ComhairleServices: z.ZodType<ComhairleServices, z.ZodTypeDef, any>;
declare const _CreateApiKeyRequest: z.ZodObject<{
    name: z.ZodString;
    prefix: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    name: z.ZodString;
    prefix: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    name: z.ZodString;
    prefix: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type CreateApiKeyRequest = z.infer<typeof _CreateApiKeyRequest>;
export declare const CreateApiKeyRequest: z.ZodType<CreateApiKeyRequest, z.ZodTypeDef, any>;
declare const _CreateResponse2: z.ZodObject<{
    key: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    key: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    key: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type CreateResponse2 = z.infer<typeof _CreateResponse2>;
export declare const CreateResponse2: z.ZodType<CreateResponse2, z.ZodTypeDef, any>;
declare const _EmailType: z.ZodEnum<["conversation_invite", "event_registration_invite", "event_registration_confirmation", "event_reminder"]>;
export type EmailType = z.infer<typeof _EmailType>;
export declare const EmailType: z.ZodType<EmailType, z.ZodTypeDef, any>;
declare const _email_type: z.ZodOptional<z.ZodUnion<[z.ZodType<"conversation_invite" | "event_registration_invite" | "event_registration_confirmation" | "event_reminder", z.ZodTypeDef, any>, z.ZodNull]>>;
export type email_type = z.infer<typeof _email_type>;
export declare const email_type: z.ZodType<email_type, z.ZodTypeDef, any>;
declare const _EmailTemplateSlots: z.ZodUnion<[z.ZodObject<{
    body: z.ZodString;
    footer: z.ZodString;
    heading: z.ZodString;
    intro: z.ZodString;
    type: z.ZodLiteral<"conversation_invite">;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    body: z.ZodString;
    footer: z.ZodString;
    heading: z.ZodString;
    intro: z.ZodString;
    type: z.ZodLiteral<"conversation_invite">;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    body: z.ZodString;
    footer: z.ZodString;
    heading: z.ZodString;
    intro: z.ZodString;
    type: z.ZodLiteral<"conversation_invite">;
}, z.ZodTypeAny, "passthrough">>, z.ZodObject<{
    body: z.ZodString;
    footer: z.ZodString;
    heading: z.ZodString;
    intro: z.ZodString;
    type: z.ZodLiteral<"event_registration_invite">;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    body: z.ZodString;
    footer: z.ZodString;
    heading: z.ZodString;
    intro: z.ZodString;
    type: z.ZodLiteral<"event_registration_invite">;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    body: z.ZodString;
    footer: z.ZodString;
    heading: z.ZodString;
    intro: z.ZodString;
    type: z.ZodLiteral<"event_registration_invite">;
}, z.ZodTypeAny, "passthrough">>, z.ZodObject<{
    body: z.ZodString;
    footer: z.ZodString;
    heading: z.ZodString;
    intro: z.ZodString;
    type: z.ZodLiteral<"event_registration_confirmation">;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    body: z.ZodString;
    footer: z.ZodString;
    heading: z.ZodString;
    intro: z.ZodString;
    type: z.ZodLiteral<"event_registration_confirmation">;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    body: z.ZodString;
    footer: z.ZodString;
    heading: z.ZodString;
    intro: z.ZodString;
    type: z.ZodLiteral<"event_registration_confirmation">;
}, z.ZodTypeAny, "passthrough">>, z.ZodObject<{
    body: z.ZodString;
    footer: z.ZodString;
    heading: z.ZodString;
    intro: z.ZodString;
    type: z.ZodLiteral<"event_reminder">;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    body: z.ZodString;
    footer: z.ZodString;
    heading: z.ZodString;
    intro: z.ZodString;
    type: z.ZodLiteral<"event_reminder">;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    body: z.ZodString;
    footer: z.ZodString;
    heading: z.ZodString;
    intro: z.ZodString;
    type: z.ZodLiteral<"event_reminder">;
}, z.ZodTypeAny, "passthrough">>]>;
export type EmailTemplateSlots = z.infer<typeof _EmailTemplateSlots>;
export declare const EmailTemplateSlots: z.ZodType<EmailTemplateSlots, z.ZodTypeDef, any>;
declare const _EmailTemplateConfigDto: z.ZodObject<{
    createdAt: z.ZodString;
    emailType: z.ZodType<"conversation_invite" | "event_registration_invite" | "event_registration_confirmation" | "event_reminder", z.ZodTypeDef, any>;
    id: z.ZodString;
    organizationId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    ownerId: z.ZodString;
    slots: z.ZodType<z.objectOutputType<{
        body: z.ZodString;
        footer: z.ZodString;
        heading: z.ZodString;
        intro: z.ZodString;
        type: z.ZodLiteral<"conversation_invite">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        body: z.ZodString;
        footer: z.ZodString;
        heading: z.ZodString;
        intro: z.ZodString;
        type: z.ZodLiteral<"event_registration_invite">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        body: z.ZodString;
        footer: z.ZodString;
        heading: z.ZodString;
        intro: z.ZodString;
        type: z.ZodLiteral<"event_registration_confirmation">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        body: z.ZodString;
        footer: z.ZodString;
        heading: z.ZodString;
        intro: z.ZodString;
        type: z.ZodLiteral<"event_reminder">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    subject: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    createdAt: z.ZodString;
    emailType: z.ZodType<"conversation_invite" | "event_registration_invite" | "event_registration_confirmation" | "event_reminder", z.ZodTypeDef, any>;
    id: z.ZodString;
    organizationId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    ownerId: z.ZodString;
    slots: z.ZodType<z.objectOutputType<{
        body: z.ZodString;
        footer: z.ZodString;
        heading: z.ZodString;
        intro: z.ZodString;
        type: z.ZodLiteral<"conversation_invite">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        body: z.ZodString;
        footer: z.ZodString;
        heading: z.ZodString;
        intro: z.ZodString;
        type: z.ZodLiteral<"event_registration_invite">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        body: z.ZodString;
        footer: z.ZodString;
        heading: z.ZodString;
        intro: z.ZodString;
        type: z.ZodLiteral<"event_registration_confirmation">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        body: z.ZodString;
        footer: z.ZodString;
        heading: z.ZodString;
        intro: z.ZodString;
        type: z.ZodLiteral<"event_reminder">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    subject: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    createdAt: z.ZodString;
    emailType: z.ZodType<"conversation_invite" | "event_registration_invite" | "event_registration_confirmation" | "event_reminder", z.ZodTypeDef, any>;
    id: z.ZodString;
    organizationId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    ownerId: z.ZodString;
    slots: z.ZodType<z.objectOutputType<{
        body: z.ZodString;
        footer: z.ZodString;
        heading: z.ZodString;
        intro: z.ZodString;
        type: z.ZodLiteral<"conversation_invite">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        body: z.ZodString;
        footer: z.ZodString;
        heading: z.ZodString;
        intro: z.ZodString;
        type: z.ZodLiteral<"event_registration_invite">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        body: z.ZodString;
        footer: z.ZodString;
        heading: z.ZodString;
        intro: z.ZodString;
        type: z.ZodLiteral<"event_registration_confirmation">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        body: z.ZodString;
        footer: z.ZodString;
        heading: z.ZodString;
        intro: z.ZodString;
        type: z.ZodLiteral<"event_reminder">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    subject: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">>;
export type EmailTemplateConfigDto = z.infer<typeof _EmailTemplateConfigDto>;
export declare const EmailTemplateConfigDto: z.ZodType<EmailTemplateConfigDto, z.ZodTypeDef, any>;
declare const _CreateEmailTemplateConfig: z.ZodObject<{
    slots: z.ZodType<z.objectOutputType<{
        body: z.ZodString;
        footer: z.ZodString;
        heading: z.ZodString;
        intro: z.ZodString;
        type: z.ZodLiteral<"conversation_invite">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        body: z.ZodString;
        footer: z.ZodString;
        heading: z.ZodString;
        intro: z.ZodString;
        type: z.ZodLiteral<"event_registration_invite">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        body: z.ZodString;
        footer: z.ZodString;
        heading: z.ZodString;
        intro: z.ZodString;
        type: z.ZodLiteral<"event_registration_confirmation">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        body: z.ZodString;
        footer: z.ZodString;
        heading: z.ZodString;
        intro: z.ZodString;
        type: z.ZodLiteral<"event_reminder">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    subject: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    slots: z.ZodType<z.objectOutputType<{
        body: z.ZodString;
        footer: z.ZodString;
        heading: z.ZodString;
        intro: z.ZodString;
        type: z.ZodLiteral<"conversation_invite">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        body: z.ZodString;
        footer: z.ZodString;
        heading: z.ZodString;
        intro: z.ZodString;
        type: z.ZodLiteral<"event_registration_invite">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        body: z.ZodString;
        footer: z.ZodString;
        heading: z.ZodString;
        intro: z.ZodString;
        type: z.ZodLiteral<"event_registration_confirmation">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        body: z.ZodString;
        footer: z.ZodString;
        heading: z.ZodString;
        intro: z.ZodString;
        type: z.ZodLiteral<"event_reminder">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    subject: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    slots: z.ZodType<z.objectOutputType<{
        body: z.ZodString;
        footer: z.ZodString;
        heading: z.ZodString;
        intro: z.ZodString;
        type: z.ZodLiteral<"conversation_invite">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        body: z.ZodString;
        footer: z.ZodString;
        heading: z.ZodString;
        intro: z.ZodString;
        type: z.ZodLiteral<"event_registration_invite">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        body: z.ZodString;
        footer: z.ZodString;
        heading: z.ZodString;
        intro: z.ZodString;
        type: z.ZodLiteral<"event_registration_confirmation">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        body: z.ZodString;
        footer: z.ZodString;
        heading: z.ZodString;
        intro: z.ZodString;
        type: z.ZodLiteral<"event_reminder">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
    subject: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">>;
export type CreateEmailTemplateConfig = z.infer<typeof _CreateEmailTemplateConfig>;
export declare const CreateEmailTemplateConfig: z.ZodType<CreateEmailTemplateConfig, z.ZodTypeDef, any>;
declare const _UpdateEmailTemplateConfig: z.ZodObject<{
    slots: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
        body: z.ZodString;
        footer: z.ZodString;
        heading: z.ZodString;
        intro: z.ZodString;
        type: z.ZodLiteral<"conversation_invite">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        body: z.ZodString;
        footer: z.ZodString;
        heading: z.ZodString;
        intro: z.ZodString;
        type: z.ZodLiteral<"event_registration_invite">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        body: z.ZodString;
        footer: z.ZodString;
        heading: z.ZodString;
        intro: z.ZodString;
        type: z.ZodLiteral<"event_registration_confirmation">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        body: z.ZodString;
        footer: z.ZodString;
        heading: z.ZodString;
        intro: z.ZodString;
        type: z.ZodLiteral<"event_reminder">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
    subject: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    slots: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
        body: z.ZodString;
        footer: z.ZodString;
        heading: z.ZodString;
        intro: z.ZodString;
        type: z.ZodLiteral<"conversation_invite">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        body: z.ZodString;
        footer: z.ZodString;
        heading: z.ZodString;
        intro: z.ZodString;
        type: z.ZodLiteral<"event_registration_invite">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        body: z.ZodString;
        footer: z.ZodString;
        heading: z.ZodString;
        intro: z.ZodString;
        type: z.ZodLiteral<"event_registration_confirmation">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        body: z.ZodString;
        footer: z.ZodString;
        heading: z.ZodString;
        intro: z.ZodString;
        type: z.ZodLiteral<"event_reminder">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
    subject: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    slots: z.ZodOptional<z.ZodUnion<[z.ZodType<z.objectOutputType<{
        body: z.ZodString;
        footer: z.ZodString;
        heading: z.ZodString;
        intro: z.ZodString;
        type: z.ZodLiteral<"conversation_invite">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        body: z.ZodString;
        footer: z.ZodString;
        heading: z.ZodString;
        intro: z.ZodString;
        type: z.ZodLiteral<"event_registration_invite">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        body: z.ZodString;
        footer: z.ZodString;
        heading: z.ZodString;
        intro: z.ZodString;
        type: z.ZodLiteral<"event_registration_confirmation">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        body: z.ZodString;
        footer: z.ZodString;
        heading: z.ZodString;
        intro: z.ZodString;
        type: z.ZodLiteral<"event_reminder">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, z.ZodNull]>>;
    subject: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">>;
export type UpdateEmailTemplateConfig = z.infer<typeof _UpdateEmailTemplateConfig>;
export declare const UpdateEmailTemplateConfig: z.ZodType<UpdateEmailTemplateConfig, z.ZodTypeDef, any>;
declare const _ContentType: z.ZodEnum<["plain_text", "rich_text"]>;
export type ContentType = z.infer<typeof _ContentType>;
export declare const ContentType: z.ZodType<ContentType, z.ZodTypeDef, any>;
declare const _SlotSchemaDefinition: z.ZodObject<{
    content_type: z.ZodType<"plain_text" | "rich_text", z.ZodTypeDef, any>;
    default_content: z.ZodString;
    hint: z.ZodString;
    key: z.ZodString;
    label: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    content_type: z.ZodType<"plain_text" | "rich_text", z.ZodTypeDef, any>;
    default_content: z.ZodString;
    hint: z.ZodString;
    key: z.ZodString;
    label: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    content_type: z.ZodType<"plain_text" | "rich_text", z.ZodTypeDef, any>;
    default_content: z.ZodString;
    hint: z.ZodString;
    key: z.ZodString;
    label: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type SlotSchemaDefinition = z.infer<typeof _SlotSchemaDefinition>;
export declare const SlotSchemaDefinition: z.ZodType<SlotSchemaDefinition, z.ZodTypeDef, any>;
declare const _EmailTypeSchema: z.ZodObject<{
    default_subject: z.ZodString;
    email_type: z.ZodType<"conversation_invite" | "event_registration_invite" | "event_registration_confirmation" | "event_reminder", z.ZodTypeDef, any>;
    slots: z.ZodArray<z.ZodType<z.objectOutputType<{
        content_type: z.ZodType<"plain_text" | "rich_text", z.ZodTypeDef, any>;
        default_content: z.ZodString;
        hint: z.ZodString;
        key: z.ZodString;
        label: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    template: z.ZodString;
    variables: z.ZodArray<z.ZodString, "many">;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    default_subject: z.ZodString;
    email_type: z.ZodType<"conversation_invite" | "event_registration_invite" | "event_registration_confirmation" | "event_reminder", z.ZodTypeDef, any>;
    slots: z.ZodArray<z.ZodType<z.objectOutputType<{
        content_type: z.ZodType<"plain_text" | "rich_text", z.ZodTypeDef, any>;
        default_content: z.ZodString;
        hint: z.ZodString;
        key: z.ZodString;
        label: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    template: z.ZodString;
    variables: z.ZodArray<z.ZodString, "many">;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    default_subject: z.ZodString;
    email_type: z.ZodType<"conversation_invite" | "event_registration_invite" | "event_registration_confirmation" | "event_reminder", z.ZodTypeDef, any>;
    slots: z.ZodArray<z.ZodType<z.objectOutputType<{
        content_type: z.ZodType<"plain_text" | "rich_text", z.ZodTypeDef, any>;
        default_content: z.ZodString;
        hint: z.ZodString;
        key: z.ZodString;
        label: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    template: z.ZodString;
    variables: z.ZodArray<z.ZodString, "many">;
}, z.ZodTypeAny, "passthrough">>;
export type EmailTypeSchema = z.infer<typeof _EmailTypeSchema>;
export declare const EmailTypeSchema: z.ZodType<EmailTypeSchema, z.ZodTypeDef, any>;
declare const _PreviewEmailTemplateConfigRequest: z.ZodObject<{
    slots: z.ZodType<z.objectOutputType<{
        body: z.ZodString;
        footer: z.ZodString;
        heading: z.ZodString;
        intro: z.ZodString;
        type: z.ZodLiteral<"conversation_invite">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        body: z.ZodString;
        footer: z.ZodString;
        heading: z.ZodString;
        intro: z.ZodString;
        type: z.ZodLiteral<"event_registration_invite">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        body: z.ZodString;
        footer: z.ZodString;
        heading: z.ZodString;
        intro: z.ZodString;
        type: z.ZodLiteral<"event_registration_confirmation">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        body: z.ZodString;
        footer: z.ZodString;
        heading: z.ZodString;
        intro: z.ZodString;
        type: z.ZodLiteral<"event_reminder">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    slots: z.ZodType<z.objectOutputType<{
        body: z.ZodString;
        footer: z.ZodString;
        heading: z.ZodString;
        intro: z.ZodString;
        type: z.ZodLiteral<"conversation_invite">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        body: z.ZodString;
        footer: z.ZodString;
        heading: z.ZodString;
        intro: z.ZodString;
        type: z.ZodLiteral<"event_registration_invite">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        body: z.ZodString;
        footer: z.ZodString;
        heading: z.ZodString;
        intro: z.ZodString;
        type: z.ZodLiteral<"event_registration_confirmation">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        body: z.ZodString;
        footer: z.ZodString;
        heading: z.ZodString;
        intro: z.ZodString;
        type: z.ZodLiteral<"event_reminder">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    slots: z.ZodType<z.objectOutputType<{
        body: z.ZodString;
        footer: z.ZodString;
        heading: z.ZodString;
        intro: z.ZodString;
        type: z.ZodLiteral<"conversation_invite">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        body: z.ZodString;
        footer: z.ZodString;
        heading: z.ZodString;
        intro: z.ZodString;
        type: z.ZodLiteral<"event_registration_invite">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        body: z.ZodString;
        footer: z.ZodString;
        heading: z.ZodString;
        intro: z.ZodString;
        type: z.ZodLiteral<"event_registration_confirmation">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        body: z.ZodString;
        footer: z.ZodString;
        heading: z.ZodString;
        intro: z.ZodString;
        type: z.ZodLiteral<"event_reminder">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>;
}, z.ZodTypeAny, "passthrough">>;
export type PreviewEmailTemplateConfigRequest = z.infer<typeof _PreviewEmailTemplateConfigRequest>;
export declare const PreviewEmailTemplateConfigRequest: z.ZodType<PreviewEmailTemplateConfigRequest, z.ZodTypeDef, any>;
declare const _PreviewEmailTemplateConfigResponse: z.ZodObject<{
    html: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    html: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    html: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type PreviewEmailTemplateConfigResponse = z.infer<typeof _PreviewEmailTemplateConfigResponse>;
export declare const PreviewEmailTemplateConfigResponse: z.ZodType<PreviewEmailTemplateConfigResponse, z.ZodTypeDef, any>;
declare const _ResourcePermission: z.ZodObject<{
    grant_reason: z.ZodString;
    granted_at: z.ZodString;
    granted_by: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    id: z.ZodString;
    organization_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    resource_id: z.ZodString;
    resource_type: z.ZodString;
    role_name: z.ZodString;
    user_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    grant_reason: z.ZodString;
    granted_at: z.ZodString;
    granted_by: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    id: z.ZodString;
    organization_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    resource_id: z.ZodString;
    resource_type: z.ZodString;
    role_name: z.ZodString;
    user_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    grant_reason: z.ZodString;
    granted_at: z.ZodString;
    granted_by: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    id: z.ZodString;
    organization_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    resource_id: z.ZodString;
    resource_type: z.ZodString;
    role_name: z.ZodString;
    user_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">>;
export type ResourcePermission = z.infer<typeof _ResourcePermission>;
export declare const ResourcePermission: z.ZodType<ResourcePermission, z.ZodTypeDef, any>;
declare const _PaginatedResults_for_ResourcePermission: z.ZodObject<{
    records: z.ZodArray<z.ZodType<z.objectOutputType<{
        grant_reason: z.ZodString;
        granted_at: z.ZodString;
        granted_by: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        id: z.ZodString;
        organization_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        resource_id: z.ZodString;
        resource_type: z.ZodString;
        role_name: z.ZodString;
        user_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    total: z.ZodNumber;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    records: z.ZodArray<z.ZodType<z.objectOutputType<{
        grant_reason: z.ZodString;
        granted_at: z.ZodString;
        granted_by: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        id: z.ZodString;
        organization_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        resource_id: z.ZodString;
        resource_type: z.ZodString;
        role_name: z.ZodString;
        user_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    total: z.ZodNumber;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    records: z.ZodArray<z.ZodType<z.objectOutputType<{
        grant_reason: z.ZodString;
        granted_at: z.ZodString;
        granted_by: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        id: z.ZodString;
        organization_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        resource_id: z.ZodString;
        resource_type: z.ZodString;
        role_name: z.ZodString;
        user_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    total: z.ZodNumber;
}, z.ZodTypeAny, "passthrough">>;
export type PaginatedResults_for_ResourcePermission = z.infer<typeof _PaginatedResults_for_ResourcePermission>;
export declare const PaginatedResults_for_ResourcePermission: z.ZodType<PaginatedResults_for_ResourcePermission, z.ZodTypeDef, any>;
declare const _GrantPermissionBody: z.ZodObject<{
    grant_reason: z.ZodString;
    organization_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    role_name: z.ZodString;
    user_email: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    user_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    grant_reason: z.ZodString;
    organization_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    role_name: z.ZodString;
    user_email: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    user_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    grant_reason: z.ZodString;
    organization_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    role_name: z.ZodString;
    user_email: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    user_id: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">>;
export type GrantPermissionBody = z.infer<typeof _GrantPermissionBody>;
export declare const GrantPermissionBody: z.ZodType<GrantPermissionBody, z.ZodTypeDef, any>;
declare const _UserWithPermissionDto: z.ZodObject<{
    email: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    id: z.ZodString;
    roleName: z.ZodString;
    username: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    email: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    id: z.ZodString;
    roleName: z.ZodString;
    username: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    email: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    id: z.ZodString;
    roleName: z.ZodString;
    username: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">>;
export type UserWithPermissionDto = z.infer<typeof _UserWithPermissionDto>;
export declare const UserWithPermissionDto: z.ZodType<UserWithPermissionDto, z.ZodTypeDef, any>;
declare const _ConversationDemographics: z.ZodObject<{
    conversationId: z.ZodString;
    questionSlug: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    conversationId: z.ZodString;
    questionSlug: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    conversationId: z.ZodString;
    questionSlug: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type ConversationDemographics = z.infer<typeof _ConversationDemographics>;
export declare const ConversationDemographics: z.ZodType<ConversationDemographics, z.ZodTypeDef, any>;
declare const _PaginatedResults_for_ConversationDemographics: z.ZodObject<{
    records: z.ZodArray<z.ZodType<z.objectOutputType<{
        conversationId: z.ZodString;
        questionSlug: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    total: z.ZodNumber;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    records: z.ZodArray<z.ZodType<z.objectOutputType<{
        conversationId: z.ZodString;
        questionSlug: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    total: z.ZodNumber;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    records: z.ZodArray<z.ZodType<z.objectOutputType<{
        conversationId: z.ZodString;
        questionSlug: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    total: z.ZodNumber;
}, z.ZodTypeAny, "passthrough">>;
export type PaginatedResults_for_ConversationDemographics = z.infer<typeof _PaginatedResults_for_ConversationDemographics>;
export declare const PaginatedResults_for_ConversationDemographics: z.ZodType<PaginatedResults_for_ConversationDemographics, z.ZodTypeDef, any>;
declare const _CreateConversationDemographics: z.ZodObject<{
    conversationId: z.ZodString;
    questionSlug: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    conversationId: z.ZodString;
    questionSlug: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    conversationId: z.ZodString;
    questionSlug: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type CreateConversationDemographics = z.infer<typeof _CreateConversationDemographics>;
export declare const CreateConversationDemographics: z.ZodType<CreateConversationDemographics, z.ZodTypeDef, any>;
declare const _NumericBucket: z.ZodObject<{
    label: z.ZodString;
    max: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
    min: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    label: z.ZodString;
    max: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
    min: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    label: z.ZodString;
    max: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
    min: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">>;
export type NumericBucket = z.infer<typeof _NumericBucket>;
export declare const NumericBucket: z.ZodType<NumericBucket, z.ZodTypeDef, any>;
declare const _TextBucket: z.ZodObject<{
    label: z.ZodString;
    values: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodString, "many">, z.ZodNull]>>;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    label: z.ZodString;
    values: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodString, "many">, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    label: z.ZodString;
    values: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodString, "many">, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">>;
export type TextBucket = z.infer<typeof _TextBucket>;
export declare const TextBucket: z.ZodType<TextBucket, z.ZodTypeDef, any>;
declare const _ValueBuckets: z.ZodUnion<[z.ZodObject<{
    buckets: z.ZodArray<z.ZodType<z.objectOutputType<{
        label: z.ZodString;
        max: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        min: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    type: z.ZodLiteral<"numeric">;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    buckets: z.ZodArray<z.ZodType<z.objectOutputType<{
        label: z.ZodString;
        max: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        min: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    type: z.ZodLiteral<"numeric">;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    buckets: z.ZodArray<z.ZodType<z.objectOutputType<{
        label: z.ZodString;
        max: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        min: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    type: z.ZodLiteral<"numeric">;
}, z.ZodTypeAny, "passthrough">>, z.ZodObject<{
    buckets: z.ZodArray<z.ZodType<z.objectOutputType<{
        label: z.ZodString;
        values: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodString, "many">, z.ZodNull]>>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    type: z.ZodLiteral<"text">;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    buckets: z.ZodArray<z.ZodType<z.objectOutputType<{
        label: z.ZodString;
        values: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodString, "many">, z.ZodNull]>>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    type: z.ZodLiteral<"text">;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    buckets: z.ZodArray<z.ZodType<z.objectOutputType<{
        label: z.ZodString;
        values: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodString, "many">, z.ZodNull]>>;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    type: z.ZodLiteral<"text">;
}, z.ZodTypeAny, "passthrough">>]>;
export type ValueBuckets = z.infer<typeof _ValueBuckets>;
export declare const ValueBuckets: z.ZodType<ValueBuckets, z.ZodTypeDef, any>;
declare const _DemographicsQuestionResponseType: z.ZodEnum<["string", "number"]>;
export type DemographicsQuestionResponseType = z.infer<typeof _DemographicsQuestionResponseType>;
export declare const DemographicsQuestionResponseType: z.ZodType<DemographicsQuestionResponseType, z.ZodTypeDef, any>;
declare const _DemographicsQuestion: z.ZodObject<{
    bucketConfig: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodType<z.objectOutputType<{
        buckets: z.ZodArray<z.ZodType<z.objectOutputType<{
            label: z.ZodString;
            max: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
            min: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        type: z.ZodLiteral<"numeric">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        buckets: z.ZodArray<z.ZodType<z.objectOutputType<{
            label: z.ZodString;
            values: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodString, "many">, z.ZodNull]>>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        type: z.ZodLiteral<"text">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">, z.ZodNull]>>;
    displayName: z.ZodString;
    responseType: z.ZodType<"string" | "number", z.ZodTypeDef, any>;
    slug: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    bucketConfig: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodType<z.objectOutputType<{
        buckets: z.ZodArray<z.ZodType<z.objectOutputType<{
            label: z.ZodString;
            max: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
            min: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        type: z.ZodLiteral<"numeric">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        buckets: z.ZodArray<z.ZodType<z.objectOutputType<{
            label: z.ZodString;
            values: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodString, "many">, z.ZodNull]>>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        type: z.ZodLiteral<"text">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">, z.ZodNull]>>;
    displayName: z.ZodString;
    responseType: z.ZodType<"string" | "number", z.ZodTypeDef, any>;
    slug: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    bucketConfig: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodType<z.objectOutputType<{
        buckets: z.ZodArray<z.ZodType<z.objectOutputType<{
            label: z.ZodString;
            max: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
            min: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        type: z.ZodLiteral<"numeric">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        buckets: z.ZodArray<z.ZodType<z.objectOutputType<{
            label: z.ZodString;
            values: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodString, "many">, z.ZodNull]>>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        type: z.ZodLiteral<"text">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">, z.ZodNull]>>;
    displayName: z.ZodString;
    responseType: z.ZodType<"string" | "number", z.ZodTypeDef, any>;
    slug: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type DemographicsQuestion = z.infer<typeof _DemographicsQuestion>;
export declare const DemographicsQuestion: z.ZodType<DemographicsQuestion, z.ZodTypeDef, any>;
declare const _PaginatedResults_for_DemographicsQuestion: z.ZodObject<{
    records: z.ZodArray<z.ZodType<z.objectOutputType<{
        bucketConfig: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodType<z.objectOutputType<{
            buckets: z.ZodArray<z.ZodType<z.objectOutputType<{
                label: z.ZodString;
                max: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
                min: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
            type: z.ZodLiteral<"numeric">;
        }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
            buckets: z.ZodArray<z.ZodType<z.objectOutputType<{
                label: z.ZodString;
                values: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodString, "many">, z.ZodNull]>>;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
            type: z.ZodLiteral<"text">;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">, z.ZodNull]>>;
        displayName: z.ZodString;
        responseType: z.ZodType<"string" | "number", z.ZodTypeDef, any>;
        slug: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    total: z.ZodNumber;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    records: z.ZodArray<z.ZodType<z.objectOutputType<{
        bucketConfig: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodType<z.objectOutputType<{
            buckets: z.ZodArray<z.ZodType<z.objectOutputType<{
                label: z.ZodString;
                max: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
                min: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
            type: z.ZodLiteral<"numeric">;
        }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
            buckets: z.ZodArray<z.ZodType<z.objectOutputType<{
                label: z.ZodString;
                values: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodString, "many">, z.ZodNull]>>;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
            type: z.ZodLiteral<"text">;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">, z.ZodNull]>>;
        displayName: z.ZodString;
        responseType: z.ZodType<"string" | "number", z.ZodTypeDef, any>;
        slug: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    total: z.ZodNumber;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    records: z.ZodArray<z.ZodType<z.objectOutputType<{
        bucketConfig: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodType<z.objectOutputType<{
            buckets: z.ZodArray<z.ZodType<z.objectOutputType<{
                label: z.ZodString;
                max: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
                min: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
            type: z.ZodLiteral<"numeric">;
        }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
            buckets: z.ZodArray<z.ZodType<z.objectOutputType<{
                label: z.ZodString;
                values: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodString, "many">, z.ZodNull]>>;
            }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
            type: z.ZodLiteral<"text">;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">, z.ZodNull]>>;
        displayName: z.ZodString;
        responseType: z.ZodType<"string" | "number", z.ZodTypeDef, any>;
        slug: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    total: z.ZodNumber;
}, z.ZodTypeAny, "passthrough">>;
export type PaginatedResults_for_DemographicsQuestion = z.infer<typeof _PaginatedResults_for_DemographicsQuestion>;
export declare const PaginatedResults_for_DemographicsQuestion: z.ZodType<PaginatedResults_for_DemographicsQuestion, z.ZodTypeDef, any>;
declare const _CreateDemographicsQuestion: z.ZodObject<{
    bucketConfig: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodType<z.objectOutputType<{
        buckets: z.ZodArray<z.ZodType<z.objectOutputType<{
            label: z.ZodString;
            max: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
            min: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        type: z.ZodLiteral<"numeric">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        buckets: z.ZodArray<z.ZodType<z.objectOutputType<{
            label: z.ZodString;
            values: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodString, "many">, z.ZodNull]>>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        type: z.ZodLiteral<"text">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">, z.ZodNull]>>;
    displayName: z.ZodString;
    responseType: z.ZodType<"string" | "number", z.ZodTypeDef, any>;
    slug: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    bucketConfig: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodType<z.objectOutputType<{
        buckets: z.ZodArray<z.ZodType<z.objectOutputType<{
            label: z.ZodString;
            max: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
            min: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        type: z.ZodLiteral<"numeric">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        buckets: z.ZodArray<z.ZodType<z.objectOutputType<{
            label: z.ZodString;
            values: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodString, "many">, z.ZodNull]>>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        type: z.ZodLiteral<"text">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">, z.ZodNull]>>;
    displayName: z.ZodString;
    responseType: z.ZodType<"string" | "number", z.ZodTypeDef, any>;
    slug: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    bucketConfig: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodType<z.objectOutputType<{
        buckets: z.ZodArray<z.ZodType<z.objectOutputType<{
            label: z.ZodString;
            max: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
            min: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        type: z.ZodLiteral<"numeric">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        buckets: z.ZodArray<z.ZodType<z.objectOutputType<{
            label: z.ZodString;
            values: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodString, "many">, z.ZodNull]>>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        type: z.ZodLiteral<"text">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">, z.ZodNull]>>;
    displayName: z.ZodString;
    responseType: z.ZodType<"string" | "number", z.ZodTypeDef, any>;
    slug: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type CreateDemographicsQuestion = z.infer<typeof _CreateDemographicsQuestion>;
export declare const CreateDemographicsQuestion: z.ZodType<CreateDemographicsQuestion, z.ZodTypeDef, any>;
declare const _PartialDemographicsQuestion: z.ZodObject<{
    bucketConfig: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodType<z.objectOutputType<{
        buckets: z.ZodArray<z.ZodType<z.objectOutputType<{
            label: z.ZodString;
            max: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
            min: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        type: z.ZodLiteral<"numeric">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        buckets: z.ZodArray<z.ZodType<z.objectOutputType<{
            label: z.ZodString;
            values: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodString, "many">, z.ZodNull]>>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        type: z.ZodLiteral<"text">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">, z.ZodNull]>>;
    displayName: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    responseType: z.ZodOptional<z.ZodUnion<[z.ZodType<"string" | "number", z.ZodTypeDef, any>, z.ZodNull]>>;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    bucketConfig: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodType<z.objectOutputType<{
        buckets: z.ZodArray<z.ZodType<z.objectOutputType<{
            label: z.ZodString;
            max: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
            min: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        type: z.ZodLiteral<"numeric">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        buckets: z.ZodArray<z.ZodType<z.objectOutputType<{
            label: z.ZodString;
            values: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodString, "many">, z.ZodNull]>>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        type: z.ZodLiteral<"text">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">, z.ZodNull]>>;
    displayName: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    responseType: z.ZodOptional<z.ZodUnion<[z.ZodType<"string" | "number", z.ZodTypeDef, any>, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    bucketConfig: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodType<z.objectOutputType<{
        buckets: z.ZodArray<z.ZodType<z.objectOutputType<{
            label: z.ZodString;
            max: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
            min: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodNull]>>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        type: z.ZodLiteral<"numeric">;
    }, z.ZodTypeAny, "passthrough"> | z.objectOutputType<{
        buckets: z.ZodArray<z.ZodType<z.objectOutputType<{
            label: z.ZodString;
            values: z.ZodOptional<z.ZodUnion<[z.ZodArray<z.ZodString, "many">, z.ZodNull]>>;
        }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
        type: z.ZodLiteral<"text">;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">, z.ZodNull]>>;
    displayName: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    responseType: z.ZodOptional<z.ZodUnion<[z.ZodType<"string" | "number", z.ZodTypeDef, any>, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">>;
export type PartialDemographicsQuestion = z.infer<typeof _PartialDemographicsQuestion>;
export declare const PartialDemographicsQuestion: z.ZodType<PartialDemographicsQuestion, z.ZodTypeDef, any>;
declare const _DemographicsResponse: z.ZodObject<{
    id: z.ZodString;
    questionSlug: z.ZodString;
    userId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    value: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    id: z.ZodString;
    questionSlug: z.ZodString;
    userId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    value: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    id: z.ZodString;
    questionSlug: z.ZodString;
    userId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
    value: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type DemographicsResponse = z.infer<typeof _DemographicsResponse>;
export declare const DemographicsResponse: z.ZodType<DemographicsResponse, z.ZodTypeDef, any>;
declare const _PaginatedResults_for_DemographicsResponse: z.ZodObject<{
    records: z.ZodArray<z.ZodType<z.objectOutputType<{
        id: z.ZodString;
        questionSlug: z.ZodString;
        userId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        value: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    total: z.ZodNumber;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    records: z.ZodArray<z.ZodType<z.objectOutputType<{
        id: z.ZodString;
        questionSlug: z.ZodString;
        userId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        value: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    total: z.ZodNumber;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    records: z.ZodArray<z.ZodType<z.objectOutputType<{
        id: z.ZodString;
        questionSlug: z.ZodString;
        userId: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
        value: z.ZodString;
    }, z.ZodTypeAny, "passthrough">, z.ZodTypeDef, any>, "many">;
    total: z.ZodNumber;
}, z.ZodTypeAny, "passthrough">>;
export type PaginatedResults_for_DemographicsResponse = z.infer<typeof _PaginatedResults_for_DemographicsResponse>;
export declare const PaginatedResults_for_DemographicsResponse: z.ZodType<PaginatedResults_for_DemographicsResponse, z.ZodTypeDef, any>;
declare const _CreateDemographicsResponse: z.ZodObject<{
    questionSlug: z.ZodString;
    userId: z.ZodString;
    value: z.ZodString;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    questionSlug: z.ZodString;
    userId: z.ZodString;
    value: z.ZodString;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    questionSlug: z.ZodString;
    userId: z.ZodString;
    value: z.ZodString;
}, z.ZodTypeAny, "passthrough">>;
export type CreateDemographicsResponse = z.infer<typeof _CreateDemographicsResponse>;
export declare const CreateDemographicsResponse: z.ZodType<CreateDemographicsResponse, z.ZodTypeDef, any>;
declare const _PartialDemographicsResponse: z.ZodObject<{
    value: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, "passthrough", z.ZodTypeAny, z.objectOutputType<{
    value: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">, z.objectInputType<{
    value: z.ZodOptional<z.ZodUnion<[z.ZodString, z.ZodNull]>>;
}, z.ZodTypeAny, "passthrough">>;
export type PartialDemographicsResponse = z.infer<typeof _PartialDemographicsResponse>;
export declare const PartialDemographicsResponse: z.ZodType<PartialDemographicsResponse, z.ZodTypeDef, any>;
export declare const schemas: Record<string, z.ZodType<any>>;
declare const endpoints: ZodiosEndpointDefinitions;
export interface ApiError {
    message: string;
    name: string;
    description: string;
    number: number;
    fileName: string;
    lineNumber: number;
    columnNumber: number;
    stack: string;
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
export declare const api: ZodiosInstance<typeof endpoints>;
export type ApiClient = typeof api;
export declare function createApiClient(baseUrl: string, options?: ZodiosOptions): ZodiosInstance<typeof endpoints>;
export {};

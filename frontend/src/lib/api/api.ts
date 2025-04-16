import { makeApi, Zodios, type ZodiosOptions } from "@zodios/core";
import { z } from "zod";


export const AnnonLoginRequest = z.object({ username: z.string() }).passthrough();
export type AnnonLoginRequest = z.infer<typeof AnnonLoginRequest>;
export const UserAuthType = z.enum(["annon", "email_password", "scot_account"]);
export type UserAuthType = z.infer<typeof UserAuthType>;
export const User = z.object({ auth_type: UserAuthType, avatar_url: z.union([z.string(), z.null()]).optional(), email: z.union([z.string(), z.null()]).optional(), id: z.string().uuid(), password: z.union([z.string(), z.null()]).optional(), username: z.union([z.string(), z.null()]).optional() }).passthrough();
export type User = z.infer<typeof User>;
export const LoginRequest = z.object({ email: z.string(), password: z.string() }).passthrough();
export type LoginRequest = z.infer<typeof LoginRequest>;
export const SignupRequest = z.object({ avatar_url: z.union([z.string(), z.null()]).optional(), email: z.string(), password: z.string(), username: z.string() }).passthrough();
export type SignupRequest = z.infer<typeof SignupRequest>;
export const created_after = z.union([z.string(), z.null()]).optional();
export type created_after = z.infer<typeof created_after>;
export const is_complete = z.union([z.boolean(), z.null()]).optional();
export type is_complete = z.infer<typeof is_complete>;
export const limit = z.union([z.number(), z.null()]).optional();
export type limit = z.infer<typeof limit>;
export const Conversation = z.object({ created_at: z.string().datetime({ offset: true }), description: z.string(), id: z.string().uuid(), image_url: z.string(), is_complete: z.boolean(), is_invite_only: z.boolean(), is_public: z.boolean(), owner_id: z.string().uuid(), short_description: z.string(), slug: z.union([z.string(), z.null()]).optional(), tags: z.array(z.string()), title: z.string(), updated_at: z.string().datetime({ offset: true }), video_url: z.union([z.string(), z.null()]).optional() }).passthrough();
export type Conversation = z.infer<typeof Conversation>;
export const PaginatedResults_for_Conversation = z.object({ records: z.array(Conversation), total: z.number().int() }).passthrough();
export type PaginatedResults_for_Conversation = z.infer<typeof PaginatedResults_for_Conversation>;
export const CreateConversation = z.object({ description: z.string(), image_url: z.string(), is_invite_only: z.boolean(), is_public: z.boolean(), short_description: z.string(), slug: z.union([z.string(), z.null()]).optional(), tags: z.union([z.array(z.string()), z.null()]).optional(), title: z.string(), video_url: z.union([z.string(), z.null()]).optional() }).passthrough();
export type CreateConversation = z.infer<typeof CreateConversation>;
export const PartialConversation = z.object({ description: z.union([z.string(), z.null()]), image_url: z.union([z.string(), z.null()]), is_complete: z.union([z.boolean(), z.null()]), is_invite_only: z.union([z.boolean(), z.null()]), is_public: z.union([z.boolean(), z.null()]), short_description: z.union([z.string(), z.null()]), slug: z.union([z.string(), z.null()]), tags: z.union([z.array(z.string()), z.null()]), title: z.union([z.string(), z.null()]), video_url: z.union([z.string(), z.null()]) }).partial().passthrough();
export type PartialConversation = z.infer<typeof PartialConversation>;
export const Workflow = z.object({ conversation_id: z.string().uuid(), created_at: z.string().datetime({ offset: true }), description: z.string(), id: z.string().uuid(), is_active: z.boolean(), is_public: z.boolean(), name: z.string(), owner_id: z.string().uuid(), updated_at: z.string().datetime({ offset: true }) }).passthrough();
export type Workflow = z.infer<typeof Workflow>;
export const CreateWorkflow = z.object({ description: z.string(), is_active: z.boolean(), is_public: z.boolean(), name: z.string() }).passthrough();
export type CreateWorkflow = z.infer<typeof CreateWorkflow>;
export const WorkflowStats = z.object({ total_users: z.number().int(), users_completed_step: z.record(z.number().int()) }).passthrough();
export type WorkflowStats = z.infer<typeof WorkflowStats>;
export const PartialWorkflow = z.object({ description: z.union([z.string(), z.null()]), is_active: z.union([z.boolean(), z.null()]), is_public: z.union([z.boolean(), z.null()]), name: z.union([z.string(), z.null()]) }).partial().passthrough();
export type PartialWorkflow = z.infer<typeof PartialWorkflow>;
export const ActivationRule = z.literal("manual");
export type ActivationRule = z.infer<typeof ActivationRule>;
export const LocalisedPage = z.object({ content: z.string(), type: z.literal("markdown") }).passthrough();
export type LocalisedPage = z.infer<typeof LocalisedPage>;
export const Page = z.array(LocalisedPage);
export type Page = z.infer<typeof Page>;
export const ToolConfig = z.union([z.object({ admin_password: z.string(), admin_user: z.string(), poll_id: z.string(), server_url: z.string(), type: z.literal("polis") }).passthrough(), z.object({ pages: z.array(Page), type: z.literal("learn") }).passthrough(), z.object({ survey_id: z.string(), survey_url: z.string(), type: z.literal("heyform") }).passthrough(), z.object({ max_time: z.number().int(), to_see: z.number().int(), type: z.literal("stories") }).passthrough()]);
export type ToolConfig = z.infer<typeof ToolConfig>;
export const WorkflowStep = z.object({ activation_rule: ActivationRule, created_at: z.string().datetime({ offset: true }), description: z.string(), id: z.string().uuid(), is_offline: z.boolean(), name: z.string(), step_order: z.number().int(), tool_config: ToolConfig, updated_at: z.string().datetime({ offset: true }), workflow_id: z.string().uuid() }).passthrough();
export type WorkflowStep = z.infer<typeof WorkflowStep>;
export const ToolSetup = z.union([z.object({ topic: z.string(), type: z.literal("polis") }).passthrough(), z.object({ pages: z.array(Page), type: z.literal("learn") }).passthrough(), z.object({ type: z.literal("heyform") }).passthrough(), z.object({ max_time: z.number().int(), to_see: z.number().int(), type: z.literal("stoies") }).passthrough()]);
export type ToolSetup = z.infer<typeof ToolSetup>;
export const CreateWorkflowStep = z.object({ activation_rule: ActivationRule, description: z.string(), is_offline: z.boolean(), name: z.string(), step_order: z.number().int(), tool_setup: ToolSetup }).passthrough();
export type CreateWorkflowStep = z.infer<typeof CreateWorkflowStep>;
export const PartialWorkflowStep = z.object({ activation_rule: z.union([ActivationRule, z.null()]), description: z.union([z.string(), z.null()]), is_offline: z.union([z.boolean(), z.null()]), name: z.union([z.string(), z.null()]), step_order: z.union([z.number(), z.null()]), tool_config: z.union([ToolConfig, z.null()]) }).partial().passthrough();
export type PartialWorkflowStep = z.infer<typeof PartialWorkflowStep>;
export const UserParticipation = z.object({ created_at: z.string().datetime({ offset: true }), id: z.string().uuid(), updated_at: z.string().datetime({ offset: true }), user_id: z.string().uuid(), workflow_id: z.string().uuid() }).passthrough();
export type UserParticipation = z.infer<typeof UserParticipation>;
export const ProgressStatus = z.enum(["not_started", "in_progress", "done"]);
export type ProgressStatus = z.infer<typeof ProgressStatus>;
export const UserProgress = z.object({ created_at: z.string().datetime({ offset: true }), id: z.string().uuid(), status: ProgressStatus, updated_at: z.string().datetime({ offset: true }), user_id: z.string().uuid(), workflow_step_id: z.string().uuid() }).passthrough();
export type UserProgress = z.infer<typeof UserProgress>;
export const MediaType = z.enum(["Video", "Image", "Text"]);
export type MediaType = z.infer<typeof MediaType>;
export const ResourceSource = z.enum(["S3", "Url"]);
export type ResourceSource = z.infer<typeof ResourceSource>;
export const CreateResource = z.object({ description: z.string(), media_type: MediaType, name: z.string(), storage_type: ResourceSource, url: z.string() }).passthrough();
export type CreateResource = z.infer<typeof CreateResource>;


export const schemas = {
	AnnonLoginRequest,
	UserAuthType,
	User,
	LoginRequest,
	SignupRequest,
	created_after,
	is_complete,
	limit,
	Conversation,
	PaginatedResults_for_Conversation,
	CreateConversation,
	PartialConversation,
	Workflow,
	CreateWorkflow,
	WorkflowStats,
	PartialWorkflow,
	ActivationRule,
	LocalisedPage,
	Page,
	ToolConfig,
	WorkflowStep,
	ToolSetup,
	CreateWorkflowStep,
	PartialWorkflowStep,
	UserParticipation,
	ProgressStatus,
	UserProgress,
	MediaType,
	ResourceSource,
	CreateResource,
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
		response: PaginatedResults_for_Conversation,
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
		response: Conversation,
	},
	{
		method: "get",
		path: "/conversation/:conversation_id",
		alias: "GetConversation",
		description: `Get a converation by id or slug`,
		requestFormat: "json",
		response: Conversation,
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
		response: Conversation,
	},
	{
		method: "delete",
		path: "/conversation/:conversation_id",
		alias: "DeleteConversation",
		description: `Delete the conversation and all related content`,
		requestFormat: "json",
		response: Conversation,
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
		method: "get",
		path: "/conversation/:conversation_id/workflow/:workflow_id/participation",
		alias: "GetUserParticipation",
		requestFormat: "json",
		response: z.union([UserParticipation, z.null()]),
	},
	{
		method: "post",
		path: "/conversation/:conversation_id/workflow/:workflow_id/participation",
		alias: "RegisterUserForWorkflow",
		requestFormat: "json",
		response: UserParticipation,
	},
	{
		method: "delete",
		path: "/conversation/:conversation_id/workflow/:workflow_id/participation",
		alias: "UnregisterUserForWorkflow",
		requestFormat: "json",
		response: UserParticipation,
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
		response: z.array(WorkflowStep),
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
		response: WorkflowStep,
	},
	{
		method: "get",
		path: "/conversation/:conversation_id/workflow/:workflow_id/workflow_step/:workflow_step_id",
		alias: "GetWorkflowStep",
		requestFormat: "json",
		response: WorkflowStep,
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
		method: "post",
		path: "/conversation/:conversation_id/workflow/resource/resource",
		alias: "CreateResource",
		description: `Get resource by id`,
		requestFormat: "json",
		parameters: [
			{
				name: "body",
				type: "Body",
				schema: CreateResource
			},
		],
		response: z.void(),
	},
	{
		method: "get",
		path: "/conversation/:conversation_id/workflow/resource/resource/:id",
		alias: "GetResource",
		description: `Get resource by id`,
		requestFormat: "json",
		response: z.void(),
	},
	{
		method: "post",
		path: "/conversation/:conversation_id/workflow/resource/upload_request",
		alias: "UploadRequst",
		description: `Request an upload url for a resource`,
		requestFormat: "json",
		response: z.void(),
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
		response: PaginatedResults_for_Conversation,
	},
]);

export const api = new Zodios(endpoints);

export function createApiClient(baseUrl: string, options?: ZodiosOptions) {
    return new Zodios(baseUrl, endpoints, options);
}

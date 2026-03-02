import { makeApi, Zodios, type ZodiosOptions } from "@zodios/core";
import { z } from "zod";



type CreateWorkflowStep = {
    activation_rule: ActivationRule;
    description: string;
    is_offline: boolean;
    name: string;
    step_order: number;
    tool_config: ToolConfig;
};;
type ActivationRule = "manual";;
type ToolConfig = {
    admin_password: string;
    admin_user: string;
    poll_id: string;
    server_url: string;
    type: "polis";
} | {
    pages: Array<Page>;
    type: "learn";
} | {
    survey_id: string;
    survey_url: string;
    type: "heyform";
};;
type Page = Array<LocalizedPage>;;
type LocalizedPage = {
    content: string;
    type: "markdown";
};;
type PaginatedResults_for_Conversation = {
    records: Array<Conversation>;
    total: number;
};;
type Conversation = {
    created_at: string;
    description: string;
    id: string;
    image_url: string;
    is_complete: boolean;
    is_invite_only: boolean;
    is_public: boolean;
    owner_id: string;
    short_description: string;
    slug?: (string | null) | undefined;
    tags: Array<string>;
    title: string;
    updated_at: string;
    video_url?: (string | null) | undefined;
};;
type PartialWorkflowStep = Partial<{
    activation_rule: (ActivationRule | null) | Array<ActivationRule | null>;
    description: string | null;
    is_offline: boolean | null;
    name: string | null;
    step_order: number | null;
    tool_config: (ToolConfig | null) | Array<ToolConfig | null>;
}>;;
type UserProgress = {
    created_at: string;
    id: string;
    status: ProgressStatus;
    updated_at: string;
    user_id: string;
    workflow_step_id: string;
};;
type ProgressStatus = "not_started" | "in_progress" | "done";;
type WorkflowStep = {
    activation_rule: ActivationRule;
    created_at: string;
    description: string;
    id: string;
    is_offline: boolean;
    name: string;
    step_order: number;
    tool_config: ToolConfig;
    updated_at: string;
    workflow_id: string;
};;

const LoginRequest = z.object({ email: z.string(), password: z.string() }).passthrough();
const SignupRequest = z.object({ avatar_url: z.union([z.string(), z.null()]).optional(), email: z.string(), password: z.string(), username: z.string() }).passthrough();
const created_after = z.union([z.string(), z.null()]).optional();
const is_complete = z.union([z.boolean(), z.null()]).optional();
const limit = z.union([z.number(), z.null()]).optional();
const Conversation: z.ZodType<Conversation> = z.object({ created_at: z.string().datetime({ offset: true }), description: z.string(), id: z.string().uuid(), image_url: z.string(), is_complete: z.boolean(), is_invite_only: z.boolean(), is_public: z.boolean(), owner_id: z.string().uuid(), short_description: z.string(), slug: z.union([z.string(), z.null()]).optional(), tags: z.array(z.string()), title: z.string(), updated_at: z.string().datetime({ offset: true }), video_url: z.union([z.string(), z.null()]).optional() }).passthrough();
const PaginatedResults_for_Conversation: z.ZodType<PaginatedResults_for_Conversation> = z.object({ records: z.array(Conversation), total: z.number().int() }).passthrough();
const CreateConversation = z.object({ description: z.string(), image_url: z.string(), is_invite_only: z.boolean(), is_public: z.boolean(), short_description: z.string(), slug: z.union([z.string(), z.null()]).optional(), tags: z.union([z.array(z.string()), z.null()]).optional(), title: z.string(), video_url: z.union([z.string(), z.null()]).optional() }).passthrough();
const PartialConversation = z.object({ description: z.union([z.string(), z.null()]), image_url: z.union([z.string(), z.null()]), is_complete: z.union([z.boolean(), z.null()]), is_invite_only: z.union([z.boolean(), z.null()]), is_public: z.union([z.boolean(), z.null()]), short_description: z.union([z.string(), z.null()]), slug: z.union([z.string(), z.null()]), tags: z.union([z.array(z.string()), z.null()]), title: z.union([z.string(), z.null()]), video_url: z.union([z.string(), z.null()]) }).partial().passthrough();
const Workflow = z.object({ conversation_id: z.string().uuid(), created_at: z.string().datetime({ offset: true }), description: z.string(), id: z.string().uuid(), is_active: z.boolean(), is_public: z.boolean(), name: z.string(), owner_id: z.string().uuid(), updated_at: z.string().datetime({ offset: true }) }).passthrough();
const CreateWorkflow = z.object({ description: z.string(), is_active: z.boolean(), is_public: z.boolean(), name: z.string() }).passthrough();
const PartialWorkflow = z.object({ description: z.union([z.string(), z.null()]), is_active: z.union([z.boolean(), z.null()]), is_public: z.union([z.boolean(), z.null()]), name: z.union([z.string(), z.null()]) }).partial().passthrough();
const ActivationRule = z.literal("manual");
const LocalizedPage: z.ZodType<LocalizedPage> = z.object({ content: z.string(), type: z.literal("markdown") }).passthrough();
const Page = z.array(LocalizedPage);
const ToolConfig = z.union([z.object({ admin_password: z.string(), admin_user: z.string(), poll_id: z.string(), server_url: z.string(), type: z.literal("polis") }).passthrough(), z.object({ pages: z.array(Page), type: z.literal("learn") }).passthrough(), z.object({ survey_id: z.string(), survey_url: z.string(), type: z.literal("heyform") }).passthrough()]);
const WorkflowStep: z.ZodType<WorkflowStep> = z.object({ activation_rule: ActivationRule, created_at: z.string().datetime({ offset: true }), description: z.string(), id: z.string().uuid(), is_offline: z.boolean(), name: z.string(), step_order: z.number().int(), tool_config: ToolConfig, updated_at: z.string().datetime({ offset: true }), workflow_id: z.string().uuid() }).passthrough();
const CreateWorkflowStep: z.ZodType<CreateWorkflowStep> = z.object({ activation_rule: ActivationRule, description: z.string(), is_offline: z.boolean(), name: z.string(), step_order: z.number().int(), tool_config: ToolConfig }).passthrough();
const PartialWorkflowStep: z.ZodType<PartialWorkflowStep> = z.object({ activation_rule: z.union([ActivationRule, z.null()]), description: z.union([z.string(), z.null()]), is_offline: z.union([z.boolean(), z.null()]), name: z.union([z.string(), z.null()]), step_order: z.union([z.number(), z.null()]), tool_config: z.union([ToolConfig, z.null()]) }).partial().passthrough();
const UserParticipation = z.object({ created_at: z.string().datetime({ offset: true }), id: z.string().uuid(), updated_at: z.string().datetime({ offset: true }), user_id: z.string().uuid(), workflow_id: z.string().uuid() }).passthrough();
const ProgressStatus = z.enum(["not_started", "in_progress", "done"]);
const UserProgress: z.ZodType<UserProgress> = z.object({ created_at: z.string().datetime({ offset: true }), id: z.string().uuid(), status: ProgressStatus, updated_at: z.string().datetime({ offset: true }), user_id: z.string().uuid(), workflow_step_id: z.string().uuid() }).passthrough();

export const schemas = {
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
	PartialWorkflow,
	ActivationRule,
	LocalizedPage,
	Page,
	ToolConfig,
	WorkflowStep,
	CreateWorkflowStep,
	PartialWorkflowStep,
	UserParticipation,
	ProgressStatus,
	UserProgress,
};

const endpoints = makeApi([
	{
		method: "get",
		path: "/auth/current_user",
		alias: "CurrentUser",
		requestFormat: "json",
		response: z.void(),
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
		response: z.void(),
	},
	{
		method: "post",
		path: "/auth/logout",
		alias: "LogoutUser",
		requestFormat: "json",
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
		response: z.void(),
	},
	{
		method: "post",
		path: "/auth/signup_annon",
		alias: "SignupAnnonUser",
		requestFormat: "json",
		response: z.void(),
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
		response: UserParticipation,
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
]);

export const api = new Zodios(endpoints);

export function createApiClient(baseUrl: string, options?: ZodiosOptions) {
    return new Zodios(baseUrl, endpoints, options);
}

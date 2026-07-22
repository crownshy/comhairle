import type { ToolConfig, WorkflowStepWithTranslations } from '@crownshy/api-client/api';

export type InstancedToolConfig<T> = Extract<ToolConfig, { type: T }>;

export type WorkflowStepWithTranslationsAndTool<T> = Exclude<
	WorkflowStepWithTranslations,
	'toolConfig' | 'previewToolConfig'
> & {
	toolConfig: InstancedToolConfig<T>;
	previewToolConfig: InstancedToolConfig<T>;
};

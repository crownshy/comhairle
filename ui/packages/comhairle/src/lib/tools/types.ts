import type { ToolConfig, WorkflowStepWithTranslationsDto } from '@crownshy/api-client/api';

export type InstancedToolConfig<T> = Extract<ToolConfig, { type: T }>;

export type WorkflowStepWithTranslationsAndTool<T> = Exclude<
	WorkflowStepWithTranslationsDto,
	'toolConfig' | 'previewToolConfig'
> & {
	toolConfig: InstancedToolConfig<T>;
	previewToolConfig: InstancedToolConfig<T>;
};

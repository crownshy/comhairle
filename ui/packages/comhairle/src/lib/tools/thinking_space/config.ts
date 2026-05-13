import type { ThinkingSpaceConfig, QuestionConfig } from './types';

/**
 * LocalStorage-backed config store for Thinking Space.
 *
 * Backend currently only exposes `topic` on the workflow step `toolConfig`.
 * Questions and follow-up count live client-side until backend schema is
 * extended (see THINKING_SPACE_TODO.md).
 */

const CONFIG_KEY_PREFIX = 'thinking_space_config_';

const DEFAULT_QUESTIONS: QuestionConfig[] = [{ id: 'q-default-1', text: '' }];

const DEFAULT_CONFIG: ThinkingSpaceConfig = {
	questions: DEFAULT_QUESTIONS,
	followUpCount: 2
};

function key(workflowStepId: string): string {
	return `${CONFIG_KEY_PREFIX}${workflowStepId}`;
}

export function loadConfig(workflowStepId: string): ThinkingSpaceConfig {
	if (typeof window === 'undefined') return structuredClone(DEFAULT_CONFIG);
	try {
		const raw = localStorage.getItem(key(workflowStepId));
		if (!raw) return structuredClone(DEFAULT_CONFIG);
		const parsed = JSON.parse(raw) as Partial<ThinkingSpaceConfig>;
		return {
			questions:
				Array.isArray(parsed.questions) && parsed.questions.length > 0
					? parsed.questions
					: structuredClone(DEFAULT_QUESTIONS),
			followUpCount:
				typeof parsed.followUpCount === 'number'
					? Math.max(0, Math.min(5, parsed.followUpCount))
					: DEFAULT_CONFIG.followUpCount
		};
	} catch (e) {
		console.error('thinking_space: failed to load config', e);
		return structuredClone(DEFAULT_CONFIG);
	}
}

export function saveConfig(workflowStepId: string, config: ThinkingSpaceConfig): void {
	if (typeof window === 'undefined') return;
	try {
		localStorage.setItem(key(workflowStepId), JSON.stringify(config));
	} catch (e) {
		console.error('thinking_space: failed to save config', e);
	}
}

export function newQuestionId(): string {
	return `q-${Date.now()}-${Math.random().toString(36).slice(2, 7)}`;
}

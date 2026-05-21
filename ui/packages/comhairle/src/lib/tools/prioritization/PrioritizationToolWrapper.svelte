<script lang="ts">
	import { getLanguageName } from '$lib/config/languages';
	import { createComhairleAdapter } from './adapters/comhairleAdapter';
	import { provideAdapterRef, provideStepContextRef } from './context';
	import PrioritizationTool from './PrioritizationTool.svelte';
	import type { Mode, Question, QuestionType, ToolConfig } from './types';

	// Not sure about this, but Stuart mentioned we want to have this in a wrapper, so we can use as a standalone component at some point? Idk if this is overkill?

	/** Backend stores QuestionType as a key-tagged union: { text: '...' } | { likert_scale: {...} } | { continuous: {...} }. The portable tool uses a `kind`-discriminated union. Normalise here so modes/components never see the raw backend shape. */
	function normaliseQuestionType(raw: unknown): QuestionType {
		if (raw && typeof raw === 'object') {
			const r = raw as Record<string, unknown>;
			if ('likert_scale' in r) {
				const ls = r.likert_scale as { categories?: { label: string; value: number }[] };
				return { kind: 'likert', categories: ls.categories ?? [] };
			}
			if ('continuous' in r) {
				const c = r.continuous as {
					sub_steps?: number;
					min_value?: number;
					max_value?: number;
					min_label?: string;
					max_label?: string;
				};
				return {
					kind: 'continuous',
					subSteps: c.sub_steps ?? 10,
					minValue: c.min_value ?? 0,
					maxValue: c.max_value ?? 10,
					minLabel: c.min_label ?? '',
					maxLabel: c.max_label ?? ''
				};
			}
		}
		return { kind: 'text' };
	}

	function normaliseQuestion(raw: unknown): Question {
		const r = (raw ?? {}) as { id?: string; text?: string; type?: unknown };
		return {
			id: r.id ?? crypto.randomUUID(),
			text: r.text ?? '',
			type: normaliseQuestionType(r.type)
		};
	}

	type Props = {
		mode: Mode;
		conversationId: string;
		workflowId: string;
		/** Loosely typed to avoid a hard dependency on the exact api-client WorkflowStep shape — the host page already knows the strict type. */
		workflowStep: {
			id: string;
			name?: string;
			description?: string;
			toolConfig?: unknown;
			previewToolConfig?: unknown;
		};
		conversation: {
			primaryLocale?: string;
			isLive?: boolean;
			supportedLanguages?: string[];
		};
		currentLocale?: string;
		participantId?: string;
		onDone?: () => void;
	};

	let {
		mode,
		conversationId,
		workflowId,
		workflowStep,
		conversation,
		currentLocale,
		participantId,
		onDone
	}: Props = $props();

	/** IMPORTANT: setContext must run synchronously during init, BEFORE the child <PrioritizationTool> renders. So we hand the children mutable ref objects now, then mutate `ref.current` from an effect once props have settled.
	 * The refs are $state-wrapped so consumer Proxies in context.ts pick up reassignments to `ref.current` reactively (e.g. after invalidateAll). Without this, $derived reads of ctx.toolConfig wouldn't re-run after the wrapper rebuilt the context object. */
	let adapterRefState = $state<{ current: ReturnType<typeof createComhairleAdapter> | null }>({
		current: null
	});
	let stepRefState = $state<{ current: ReturnType<typeof buildStepContext> | null }>({
		current: null
	});
	provideAdapterRef(adapterRefState);
	provideStepContextRef(stepRefState);

	let resolvedToolConfig = $derived(
		(conversation.isLive ? workflowStep.toolConfig : workflowStep.previewToolConfig) as {
			type: string;
			questions?: unknown[];
			randomize_order?: boolean;
		} | null
	);

	let toolConfig = $derived<ToolConfig>(
		resolvedToolConfig?.type === 'prioritization'
			? {
					questions: (resolvedToolConfig.questions ?? []).map(normaliseQuestion),
					randomizeOrder: Boolean(resolvedToolConfig.randomize_order)
				}
			: { questions: [], randomizeOrder: false }
	);

	let adapter = $derived(
		createComhairleAdapter({
			conversationId,
			workflowId,
			workflowStepId: workflowStep.id,
			stepId: workflowStep.id
		})
	);

	function buildStepContext() {
		const primaryLocale = conversation.primaryLocale ?? 'en';
		const supportedLocales =
			conversation.supportedLanguages && conversation.supportedLanguages.length > 0
				? conversation.supportedLanguages
				: [primaryLocale];
		return {
			stepId: workflowStep.id,
			stepTitle: workflowStep.name ?? '',
			stepDescription: workflowStep.description ?? '',
			toolConfig,
			primaryLocale,
			currentLocale: currentLocale ?? primaryLocale,
			supportedLocales,
			participantId: participantId ?? '',
			formatLocale: (locale: string) => getLanguageName(locale),
			onDone
		};
	}

	/** Populate refs synchronously so the first child render sees real values. We intentionally read the $derived `adapter` here just to seed the initial ref; the $effect below keeps it in sync afterwards. */
	// svelte-ignore state_referenced_locally
	adapterRefState.current = adapter;
	stepRefState.current = buildStepContext();

	$effect(() => {
		adapterRefState.current = adapter;
		stepRefState.current = buildStepContext();
	});
</script>

<PrioritizationTool {mode} />

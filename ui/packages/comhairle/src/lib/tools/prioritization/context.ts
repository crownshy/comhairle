import { getContext, setContext } from 'svelte';
import type { PrioritizationAdapter } from './adapter';
import type { Question, ToolConfig } from './types';

/** Context must be set synchronously during a component's init, BEFORE children render. Effects fire later, so we hand children a mutable ref object at init and let the wrapper update `ref.current` from $effect. */

const ADAPTER_KEY = Symbol('prioritization.adapter');
const CONFIG_KEY = Symbol('prioritization.stepContext');

export type StepContext = {
	stepId: string;
	stepTitle: string;
	stepDescription: string;
	toolConfig: ToolConfig;
	primaryLocale: string;
	currentLocale: string;
	supportedLocales: string[];
	participantId: string;
	/** Injected by the wrapper so the portable tool doesn't import comhairle language config directly. Falls back to the locale code if unset. */
	formatLocale: (locale: string) => string;
	/** Called by User mode when the participant finishes the step. Lets the host page advance to the next workflow step. */
	onDone?: () => void;
};

type AdapterRef = { current: PrioritizationAdapter | null };
type StepRef = { current: StepContext | null };

/** Wrapper-side helpers */

/** Accepts an optional caller-supplied ref so the wrapper can pass in a $state-wrapped object. The same ref is stored in context and returned to the caller; consumer Proxies read `ref.current` on every access, so a $state ref gives them reactivity when the wrapper reassigns `current` after invalidateAll. */
export function provideAdapterRef(ref: AdapterRef = { current: null }): AdapterRef {
	setContext(ADAPTER_KEY, ref);
	return ref;
}

export function provideStepContextRef(ref: StepRef = { current: null }): StepRef {
	setContext(CONFIG_KEY, ref);
	return ref;
}

/** Back-compat helpers for any test harness that still uses the old API. */
export function setAdapter(adapter: PrioritizationAdapter): void {
	const existing = getContext<AdapterRef | undefined>(ADAPTER_KEY);
	if (existing) {
		existing.current = adapter;
		return;
	}
	setContext(ADAPTER_KEY, { current: adapter } satisfies AdapterRef);
}

export function setStepContext(ctx: StepContext): void {
	const existing = getContext<StepRef | undefined>(CONFIG_KEY);
	if (existing) {
		existing.current = ctx;
		return;
	}
	setContext(CONFIG_KEY, { current: ctx } satisfies StepRef);
}

/** Consumer-side helpers */

export function getAdapter(): PrioritizationAdapter {
	const ref = getContext<AdapterRef | undefined>(ADAPTER_KEY);
	if (!ref) {
		throw new Error(
			'PrioritizationAdapter missing from context. Render <PrioritizationToolWrapper> (or a test harness with provideAdapterRef) above <PrioritizationTool>.'
		);
	}
	/** Return a Proxy that reads `ref.current` on every access. Combined with a $state-wrapped ref in the wrapper, this keeps consumer $derived reactive when the wrapper reassigns `ref.current` after invalidateAll. */
	return new Proxy({} as PrioritizationAdapter, {
		get(_target, prop) {
			const current = ref.current;
			if (!current) {
				throw new Error('PrioritizationAdapter not yet initialised.');
			}
			const value = (current as unknown as Record<string | symbol, unknown>)[prop];
			return typeof value === 'function'
				? (value as (...a: unknown[]) => unknown).bind(current)
				: value;
		}
	});
}

export function getStepContext(): StepContext {
	const ref = getContext<StepRef | undefined>(CONFIG_KEY);
	if (!ref) {
		throw new Error(
			'Step context missing. Render <PrioritizationToolWrapper> above <PrioritizationTool>.'
		);
	}
	/** See getAdapter for rationale: live proxy backed by a $state ref. */
	return new Proxy({} as StepContext, {
		get(_target, prop) {
			const current = ref.current;
			if (!current) {
				throw new Error('Step context not yet initialised.');
			}
			const value = (current as unknown as Record<string | symbol, unknown>)[prop];
			return typeof value === 'function'
				? (value as (...a: unknown[]) => unknown).bind(current)
				: value;
		}
	});
}

export type { Question };

import type { AutosaveSource } from '$lib/components/Translation/translationUtils';
import { defaultDataProtocolDoc, isDefaultDataProtocol } from '$lib/dataProtocol';

type Options = {
	/** The page stays mounted when moving between steps, so typed text is kept per step. */
	getStepId: () => string | undefined;
	getToolType: () => string | undefined;
	getPrimaryLocale: () => string;
};

/**
 * Shows the tool default in the editor as editable text, but only saves what differs from it.
 * An untouched default stays blank in the database, so it keeps getting the shared, translated
 * wording (ADR-0050).
 */
export function withEditableDefault(source: AutosaveSource, options: Options) {
	// After the first edit the editor shows exactly what was typed, so saving blank for a
	// default-matching edit never swaps the text out from under the cursor.
	let typedFor = $state<{ stepId: string | undefined; content: string }>();

	const typed = () =>
		typedFor && typedFor.stepId === options.getStepId() ? typedFor.content : undefined;
	const saved = () => source.contents[options.getPrimaryLocale()];
	const shown = () =>
		typed() ??
		(isDefaultDataProtocol(saved(), options.getToolType())
			? defaultDataProtocolDoc(options.getToolType())
			: (saved() ?? ''));

	const wrapped: AutosaveSource = Object.create(source, {
		contents: {
			get: () => ({ ...source.contents, [options.getPrimaryLocale()]: shown() })
		},
		saveSource: {
			value: (content: string) => {
				typedFor = { stepId: options.getStepId(), content };
				source.saveSource(
					isDefaultDataProtocol(content, options.getToolType()) ? '' : content
				);
			}
		}
	});

	return {
		source: wrapped,
		get usingDefault() {
			return isDefaultDataProtocol(shown(), options.getToolType());
		},
		resetToDefault() {
			typedFor = undefined;
			source.saveSource('');
		}
	};
}

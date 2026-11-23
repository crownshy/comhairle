import type { Editor } from '@tiptap/core';

/**
 * Bumps a counter on every editor transaction, so a toolbar that reads the counter re-checks
 * which of its tools are active. Call it during component setup.
 */
export function trackEditorTransactions(getEditor: () => Editor | undefined) {
	let revision = $state(0);

	$effect(() => {
		const current = getEditor();
		if (!current) return;
		const bump = () => queueMicrotask(() => (revision += 1));
		current.on('transaction', bump);
		return () => current.off('transaction', bump);
	});

	return {
		isActive(check: (editor: Editor) => boolean): boolean {
			void revision;
			const current = getEditor();
			return !!current && !current.isDestroyed && check(current);
		}
	};
}

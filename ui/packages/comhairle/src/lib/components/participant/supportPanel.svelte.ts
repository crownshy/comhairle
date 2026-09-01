/**
 * The support panel's open state, shared because its trigger and its content live in different
 * components: the step chrome's menu is rendered by the step page, the panel itself by the
 * workflow layout. Nothing else needs to reach it, so it stays a module, not a context.
 */
export type SupportPanelTab = 'learningAssistant' | 'faqs' | 'privacyPolicy';

let open = $state(false);
/** The assistant is the panel's most useful tab, so an unspecific open lands there; the panel
 *  falls back to the FAQs when the conversation has no assistant. */
let tab = $state<SupportPanelTab>('learningAssistant');

export const supportPanel = {
	get open() {
		return open;
	},
	set open(next: boolean) {
		open = next;
	},
	/** Which tab the panel shows when it opens. */
	get tab() {
		return tab;
	},
	openAt(next: SupportPanelTab) {
		tab = next;
		open = true;
	}
};

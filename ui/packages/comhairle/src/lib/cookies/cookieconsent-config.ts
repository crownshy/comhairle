import type { CookieConsentConfig } from 'vanilla-cookieconsent';
import { m } from '$lib/paraglide/messages';
import { getLocale } from '$lib/paraglide/runtime';

const MODAL_SELECTORS = {
	consentModal: '#cc-main .cm',
	preferencesModal: '#cc-main .pm'
} as const;

const FOCUS_RETRY_MS = 50;
const FOCUS_MAX_TRIES = 40;

/**
 * The library's own focus call can run before the modal is visible, which leaves focus
 * on the page behind it. Focusing the dialog itself also makes screen readers read its title.
 */
function focusModalWhenVisible(selector: string, triesLeft = FOCUS_MAX_TRIES) {
	const modal = document.querySelector<HTMLElement>(selector);
	if (!modal || triesLeft === 0) return;
	const style = getComputedStyle(modal);
	if (style.visibility === 'hidden' || style.opacity !== '1') {
		setTimeout(() => focusModalWhenVisible(selector, triesLeft - 1), FOCUS_RETRY_MS);
		return;
	}
	modal.tabIndex = -1;
	modal.focus();
}

// Built on demand so the banner text follows the locale at the moment it opens.
const buildCookieConsentConfig = (): CookieConsentConfig => ({
	autoShow: true,
	disablePageInteraction: true,
	categories: {
		necessary: {
			enabled: true,
			readOnly: true
		}
	},

	onModalShow: ({ modalName }) => {
		focusModalWhenVisible(MODAL_SELECTORS[modalName]);
	},

	guiOptions: {
		consentModal: {
			layout: 'box inline',
			position: 'bottom left',
			equalWeightButtons: true,
			flipButtons: false
		},
		preferencesModal: {
			layout: 'box',
			equalWeightButtons: true,
			flipButtons: false
		}
	},

	language: {
		default: getLocale(),
		translations: {
			[getLocale()]: {
				consentModal: {
					title: m.cookie_consent_title(),
					description: m.cookie_consent_description(),
					acceptAllBtn: m.cookie_accept_all(),
					acceptNecessaryBtn: m.cookie_reject_all(),
					showPreferencesBtn: m.cookie_manage_preferences(),
					footer: `
							<a href="/rights/tos" target="_blank">${m.cookie_terms_of_service()}</a>
							<a href="/rights/privacy" target="_blank">${m.cookie_privacy_policy()}</a>
					`
				},
				preferencesModal: {
					title: m.cookie_preferences_title(),
					acceptAllBtn: m.cookie_accept_all(),
					acceptNecessaryBtn: m.cookie_reject_all(),
					savePreferencesBtn: m.cookie_save_selection(),
					closeIconLabel: m.cookie_close_modal(),
					serviceCounterLabel: m.cookie_service_counter(),
					sections: [
						{
							title: m.cookie_privacy_choices_title(),
							description: m.cookie_privacy_choices_description()
						},
						{
							title: m.cookie_necessary_title(),
							description: m.cookie_necessary_description(),
							linkedCategory: 'necessary'
						},
						{
							title: m.cookie_more_info_title(),
							description: `${m.cookie_more_info_description()} <a href="/rights/cookies">${m.cookie_more_info_link()}</a>`
						}
					]
				}
			}
		}
	}
});

export default buildCookieConsentConfig;

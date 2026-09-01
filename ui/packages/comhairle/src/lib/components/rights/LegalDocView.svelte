<script lang="ts">
	import PrivacyPolicyContent from './PrivacyPolicyContent.svelte';
	import TermsOfServiceContent from './TermsOfServiceContent.svelte';
	import CookiesSettingsContent from './CookiesSettingsContent.svelte';
	import type { LegalDocId } from './legalDocs';

	let {
		doc,
		onSelect
	}: {
		doc: LegalDocId;
		/** Keeps in-body cross-links inside the reader instead of navigating the page away. */
		onSelect?: (id: LegalDocId) => void;
	} = $props();

	function intercept(id: LegalDocId) {
		if (!onSelect) return undefined;
		return (event: MouseEvent) => {
			event.preventDefault();
			onSelect(id);
		};
	}
</script>

{#if doc === 'privacy'}
	<PrivacyPolicyContent />
{:else if doc === 'tos'}
	<TermsOfServiceContent onPrivacyClick={intercept('privacy')} />
{:else}
	<CookiesSettingsContent />
{/if}

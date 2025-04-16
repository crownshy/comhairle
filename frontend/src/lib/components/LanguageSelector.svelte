<script>
	import { availableLanguageTags, languageTag } from '$lib/paraglide/runtime';
	import * as Select from '$lib/components/ui/select';
	import { i18n } from '$lib/i18n';
	import { goto } from '$app/navigation';
	import { page } from '$app/stores';
	import { get } from 'svelte/store';

	/**
	 * @param { import("$lib/paraglide/runtime").AvailableLanguageTag } newLanguage
	 */
	function switchToLanguage(newLanguage) {
		const canonicalPath = i18n.route(get(page).url.pathname);
		const localisedPath = i18n.resolveRoute(canonicalPath, newLanguage);
		goto(localisedPath);
	}

	/**
	 * @type {Record<import("$lib/paraglide/runtime").AvailableLanguageTag, string>}
	 */
	const labels = {
		en: 'English',
		gd: 'Gaelic'
	};
</script>

<select
	class="border-1px border-grey bg-white"
	on:change={(e) => switchToLanguage(/** @type {any} */ (e).target.value)}
>
	{#each availableLanguageTags as langTag}
		<option value={langTag} selected={languageTag() === langTag}>{labels[langTag]}</option>
	{/each}
</select>

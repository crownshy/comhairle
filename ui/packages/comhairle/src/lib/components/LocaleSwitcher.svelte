<script lang="ts">
	import { getLocale, locales, setLocale, type Locale } from '$lib/paraglide/runtime';
	import * as Select from '$lib/components/ui/select';
	import { Day } from '$lib/utils/units';
	import { getLanguageName } from '$lib/config/languages';

	interface Props {
		class?: string;
	}

	let { class: className }: Props = $props();

	let currentLanguage = $state<Locale>(getLocale());
	let languageName = $derived(getLanguageName(currentLanguage, 'native'));

	function setCookie(name: string, value: string, days: number = 365) {
		// eslint-disable-next-line svelte/prefer-svelte-reactivity
		const date = new Date();
		date.setTime(date.getTime() + days * Day);
		const expires = `expires=${date.toUTCString()}`;
		document.cookie = `${name}=${value};${expires};path=/;SameSite=Lax`;
	}

	function switchToLanguage(newLanguage: Locale) {
		setCookie('COMHAIRLE_LOCALE', newLanguage);
		setLocale(newLanguage);
		currentLanguage = newLanguage;
	}
</script>

<Select.Root type="single" onValueChange={(locale) => switchToLanguage(locale as Locale)}>
	<Select.Trigger class="{className} [/&_svg]:opacity-100">
		<span class="text-center">{languageName}</span>
	</Select.Trigger>
	<Select.Content>
		{#each locales as locale (locale)}
			<Select.Item value={locale}>{getLanguageName(locale, 'native')}</Select.Item>
		{/each}
	</Select.Content>
</Select.Root>

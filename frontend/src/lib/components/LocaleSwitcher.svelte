<script lang="ts">
	import { getLocale, locales, setLocale, type Locale } from '$lib/paraglide/runtime';
	import * as Select from '$lib/components/ui/select';

	let { class: className = '' }: { class?: string } = $props();

	const labels = {
		en: 'English',
		es: 'Spanish',
		gd: 'Gaelic',
		cy: 'Welsh',
		zh: 'Chinese',
		es: 'Spanish',
		fr: 'French',
		ar: 'Arabic',
		pt: 'Portuguese'
	};

	function setCookie(name: string, value: string, days: number = 365) {
		const date = new Date();
		date.setTime(date.getTime() + days * 24 * 60 * 60 * 1000);
		const expires = `expires=${date.toUTCString()}`;
		document.cookie = `${name}=${value};${expires};path=/;SameSite=Lax`;
	}

	function switchToLanguage(newLanguage: Locale) {
		setCookie('COMHAIRLE_LOCALE', newLanguage);
		setLocale(newLanguage);
	}
	let currentLanguage = getLocale();
</script>

<Select.Root type="single" onValueChange={(locale) => switchToLanguage(locale as Locale)}>
	<Select.Trigger class="{className} [/&_svg]:opacity-100">
		<span class="text-center">{labels[currentLanguage]}</span>
	</Select.Trigger>
	<Select.Content>
		{#each locales as langTag}
			<Select.Item value={langTag}>{labels[langTag]}</Select.Item>
		{/each}
	</Select.Content>
</Select.Root>

<script lang="ts">
	import { getLocale, locales, type Locale } from '$lib/paraglide/runtime';
	import * as Select from '$lib/components/ui/select';
	import { getLanguageName } from '$lib/config/languages';
	import { switchLocale } from '$lib/utils/locale';

	interface Props {
		class?: string;
	}

	let { class: className }: Props = $props();

	let currentLanguage = $state<Locale>(getLocale());
	let languageName = $derived(getLanguageName(currentLanguage, 'native'));

	function switchToLanguage(newLanguage: Locale) {
		switchLocale(newLanguage);
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

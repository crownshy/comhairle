<script lang="ts">
	import { getLocale, locales, setLocale, type Locale } from '$lib/paraglide/runtime';
	import * as Select from '$lib/components/ui/select';

	const labels = {
		en: 'English',
		gd: 'Gaelic'
	};

	function switchToLanguage(newLanguage: Locale) {
		setLocale(newLanguage);
	}
	let currentLanguage = getLocale();
</script>

<Select.Root type="single" onValueChange={(locale) => switchToLanguage(locale as Locale)}>
	<Select.Trigger class="w-[180px]">
		{labels[currentLanguage]}
	</Select.Trigger>
	<Select.Content>
		{#each locales as langTag}
			<Select.Item value={langTag}>{labels[langTag]}</Select.Item>
		{/each}
	</Select.Content>
</Select.Root>

<script lang="ts">
	import * as Select from '$lib/components/ui/select';
	import { Label } from '$lib/components/ui/label';
	import { getLanguageName } from '$lib/config/languages';
	import MultiSelect from '$lib/components/ui/mutli-select/multi-select.svelte';
	import type { Option } from '$lib/components/ui/mutli-select/multi-select.svelte';
	import { locales, type Locale } from '$lib/paraglide/runtime';

	interface Props {
		primaryLanguage: Locale;
		supportedLanguages: Locale[];
		disabled?: boolean;
		onPrimaryChange?: (language: string) => void;
		onSupportedChange?: (languages: string[]) => void;
	}

	let {
		primaryLanguage = $bindable('en'),
		supportedLanguages = $bindable(['en']),
		disabled = false,
		onPrimaryChange,
		onSupportedChange
	}: Props = $props();

	let otherLanguageOptions = $derived<Option[]>(
		locales
			.filter((locale) => locale !== primaryLanguage)
			.map((locale) => ({ value: locale, label: getLanguageName(locale) }))
	);

	let selectedOtherLanguages = $derived<Option[]>(
		supportedLanguages
			.filter((code) => code !== primaryLanguage)
			.map((code) => ({
				value: code,
				label: getLanguageName(code)
			}))
	);

	function handlePrimaryChange(value: string | undefined) {
		if (disabled || !value) return;
		const languageCode = value as Locale;
		primaryLanguage = languageCode;
		if (!supportedLanguages.includes(languageCode)) {
			supportedLanguages = [languageCode].concat(supportedLanguages);
		}
		onPrimaryChange?.(languageCode);
	}

	function handleOtherLanguagesChange(options: Option[]) {
		if (disabled) return;
		supportedLanguages = [primaryLanguage].concat(options.map((o) => o.value as Locale));
		onSupportedChange?.(supportedLanguages);
	}
</script>

<div class="flex flex-col gap-4">
	<!-- Primary Language -->
	<div class="flex flex-col gap-2">
		<Label class="font-semibold">Primary language</Label>
		<Select.Root
			type="single"
			value={primaryLanguage}
			onValueChange={handlePrimaryChange}
			{disabled}
		>
			<Select.Trigger class="w-full">
				{getLanguageName(primaryLanguage)}
			</Select.Trigger>
			<Select.Content>
				{#each locales as locale (locale)}
					<Select.Item value={locale}>{getLanguageName(locale)}</Select.Item>
				{/each}
			</Select.Content>
		</Select.Root>
	</div>

	<!-- Other Languages -->
	<div class="flex flex-col gap-2">
		<Label class="font-semibold">Supported languages</Label>
		<MultiSelect
			{disabled}
			defaultOptions={otherLanguageOptions}
			selected={selectedOtherLanguages}
			onSelectedChange={handleOtherLanguagesChange}
			placeholder="Select languages..."
			ariaLabel="Supported languages"
			emptyMessage="No languages found"
			class="w-full"
		/>
	</div>
</div>

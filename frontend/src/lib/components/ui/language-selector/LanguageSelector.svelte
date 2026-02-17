<script lang="ts">
	import * as Select from '$lib/components/ui/select';
	import { Label } from '$lib/components/ui/label';
	import { allLanguages } from '$lib/config/languages';
	import MultiSelect from '$lib/components/ui/mutli-select/multi-select.svelte';
	import type { Option } from '$lib/components/ui/mutli-select/multi-select.svelte';

	interface Props {
		primaryLanguage: string;
		supportedLanguages: string[];
		onPrimaryChange?: (language: string) => void;
		onSupportedChange?: (languages: string[]) => void;
	}

	let {
		primaryLanguage = $bindable('en'),
		supportedLanguages = $bindable(['en']),
		onPrimaryChange,
		onSupportedChange
	}: Props = $props();

	// All languages except primary, as Option[]
	let otherLanguageOptions = $derived<Option[]>(
		allLanguages
			.filter((lang) => lang.code !== primaryLanguage)
			.map((lang) => ({ value: lang.code, label: lang.name }))
	);

	// Currently selected other languages as Option[]
	let selectedOtherLanguages = $derived<Option[]>(
		supportedLanguages
			.filter((code) => code !== primaryLanguage)
			.map((code) => ({
				value: code,
				label: allLanguages.find((l) => l.code === code)?.name ?? code
			}))
	);

	function handlePrimaryChange(value: string | undefined) {
		if (!value) return;
		primaryLanguage = value;
		if (!supportedLanguages.includes(value)) {
			supportedLanguages = [value, ...supportedLanguages];
		}
		onPrimaryChange?.(value);
	}

	function handleOtherLanguagesChange(options: Option[]) {
		supportedLanguages = [primaryLanguage, ...options.map((o) => o.value)];
		onSupportedChange?.(supportedLanguages);
	}
</script>

<div class="flex flex-col gap-4">
	<!-- Primary Language -->
	<div class="flex flex-col gap-2">
		<Label class="font-semibold">Primary language</Label>
		<Select.Root type="single" value={primaryLanguage} onValueChange={handlePrimaryChange}>
			<Select.Trigger class="w-full">
				{allLanguages.find((l) => l.code === primaryLanguage)?.name ?? primaryLanguage}
			</Select.Trigger>
			<Select.Content>
				{#each allLanguages as lang (lang.code)}
					<Select.Item value={lang.code}>{lang.name}</Select.Item>
				{/each}
			</Select.Content>
		</Select.Root>
	</div>

	<!-- Other Languages -->
	<div class="flex flex-col gap-2">
		<Label class="font-semibold">Supported languages</Label>
		<MultiSelect
			defaultOptions={otherLanguageOptions}
			selected={selectedOtherLanguages}
			onSelectedChange={handleOtherLanguagesChange}
			placeholder="Select languages..."
			emptyMessage="No languages found"
			class="w-full"
		/>
	</div>
</div>
<script lang="ts">
	import * as Select from '$lib/components/ui/select';
	import * as Popover from '$lib/components/ui/popover';
	import { Checkbox } from '$lib/components/ui/checkbox';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import { ChevronDown, Search } from 'lucide-svelte';
	import { allLanguages } from '$lib/config/languages';
	import * as ScrollArea from '$lib/components/ui/scroll-area';

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

	let searchQuery = $state('');
	let popoverOpen = $state(false);

	let otherLanguageOptions = $derived(
		allLanguages.filter(lang => lang.code !== primaryLanguage)
	);

	let filteredLanguages = $derived(
		otherLanguageOptions.filter(lang => 
			lang.name.toLowerCase().includes(searchQuery.toLowerCase())
		)
	);

	let selectedOtherLanguages = $derived(
		supportedLanguages.filter(code => code !== primaryLanguage)
	);

	function handlePrimaryChange(value: string | undefined) {
		if (!value) return;
		primaryLanguage = value;
		
		if (!supportedLanguages.includes(value)) {
			supportedLanguages = [value, ...supportedLanguages];
		}
		
		onPrimaryChange?.(value);
	}

	function toggleLanguage(code: string) {
		if (selectedOtherLanguages.includes(code)) {
			supportedLanguages = supportedLanguages.filter(c => c !== code);
		} else {
			supportedLanguages = [...supportedLanguages, code];
		}
		onSupportedChange?.(supportedLanguages);
	}

	function getSelectedLabel() {
		const count = selectedOtherLanguages.length;
		if (count === 0) return 'Other languages';
		if (count === 1) {
			const lang = allLanguages.find(l => l.code === selectedOtherLanguages[0]);
			return lang?.name ?? selectedOtherLanguages[0];
		}
		return `${count} languages selected`;
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
		>
			<Select.Trigger class="w-full">
				{allLanguages.find(l => l.code === primaryLanguage)?.name ?? primaryLanguage}
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
		<Label class="font-semibold">Other languages</Label>
		<Popover.Root bind:open={popoverOpen}>
			<Popover.Trigger class="w-full flex items-center justify-between px-3 py-2 border rounded-lg bg-background">
				<span class="text-sm {selectedOtherLanguages.length === 0 ? 'text-gray-500' : 'text-gray-900'}">
					{getSelectedLabel()}
				</span>
				<ChevronDown class="h-4 w-4 text-gray-500" />
			</Popover.Trigger>
			<Popover.Content class="w-[var(--bits-popover-trigger-width)] p-0" align="start">
				<!-- Search -->
				<div class="p-2 border-b">
					<div class="relative">
						<Search class="absolute left-2 top-1/2 -translate-y-1/2 h-4 w-4 text-gray-400" />
						<Input 
							type="text"
							placeholder="Search Language..."
							class="pl-8"
							bind:value={searchQuery}
						/>
					</div>
				</div>
				
				<!-- Language list -->
				<ScrollArea.Root>
					<div class="max-h-48 p-1">
						{#each filteredLanguages as lang (lang.code)}
							<button
								type="button"
								class="w-full flex items-center gap-2 px-2 py-1.5 rounded hover:bg-gray-100 text-left"
								onclick={() => toggleLanguage(lang.code)}
							>
								<Checkbox checked={selectedOtherLanguages.includes(lang.code)} />
								<span class="text-sm">{lang.name}</span>
							</button>
						{/each}
						{#if filteredLanguages.length === 0}
							<div class="px-2 py-4 text-center text-sm text-gray-500">
								No languages found
							</div>
						{/if}
					</div>
				</ScrollArea.Root>
			</Popover.Content>
		</Popover.Root>
	</div>
</div>

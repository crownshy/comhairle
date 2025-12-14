<script lang="ts">
	import { Input } from '$lib/components/ui/input';
	import { Textarea } from '$lib/components/ui/textarea';
	import * as Form from '$lib/components/ui/form/';
	import { Button } from '$lib/components/ui/button';
	import LanguageStatusBadge from './LanguageStatusBadge.svelte';
	import type { TranslationEntry } from './useTranslations.svelte';
	import type { SuperForm } from 'sveltekit-superforms';
	import {Languages} from 'lucide-svelte';

	interface Props {
		form: SuperForm<any>;
		name: string;
		label: string;
		value: string;
		onValueChange: (value: string) => void;
		onEditTranslations: (language?: string) => void;
		onPrimaryChange?: () => void;
		translations: TranslationEntry[];
		inputType?: 'input' | 'textarea';
	}

	let { 
		form, 
		name, 
		label, 
		value,
		onValueChange,
		onEditTranslations,
		onPrimaryChange,
		translations,
		inputType = 'input'
	}: Props = $props();

	function handleInput(newValue: string) {
		onValueChange(newValue);
		if (translations.length > 0) {
			onPrimaryChange?.();
		}
	}
</script>

<Form.Field {form} {name}>
	<Form.Control>
		{#snippet children({ props })}
			<div class="grid grid-cols-[200px_1fr] gap-6 border-t py-6">
				<Form.Label class="font-semibold pt-2">{label}</Form.Label>
				<div class="flex flex-col gap-2">
					<div class="relative">
						{#if inputType === 'textarea'}
							<Textarea 
								class="bg-white pr-32" 
								{...props} 
								{value}
								oninput={(e) => handleInput(e.currentTarget.value)}
							/>
						{:else}
							<Input 
								class="pr-32"
								{...props} 
								{value}
								oninput={(e) => handleInput(e.currentTarget.value)}
							/>
						{/if}
						{#if translations.length > 0}
							<Button
								type="button"
                                variant="link"
								class="absolute right-0 top-0 focus:outline-none"
								onclick={onEditTranslations}
							>
                                <Languages />					
                            </Button>
						{/if}
					</div>
					{#if translations.length > 0}
						<div class="flex items-center gap-2 flex-wrap">
							{#each translations as t (t.language)}
								<LanguageStatusBadge 
									language={t.language} 
									languageName={t.languageName} 
									status={t.status} 
									onclick={(lang) => onEditTranslations(lang)}
								/>
							{/each}
						</div>
					{/if}
					<Form.FieldErrors />
				</div>
			</div>
		{/snippet}
	</Form.Control>
</Form.Field>

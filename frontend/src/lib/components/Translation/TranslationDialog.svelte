<script lang="ts">
	import * as Dialog from '$lib/components/ui/dialog';
	import { Button } from '$lib/components/ui/button';
	import { Badge } from '$lib/components/ui/badge';
	import { X, Sparkles, Check, MoreHorizontal } from 'lucide-svelte';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import type { TranslationEntry, TranslationStatus } from './useTranslations.svelte';

	interface Props {
		open?: boolean;
		translations?: TranslationEntry[];
		initialLanguage?: string | null;
		onClose?: () => void;
		onUpdate?: (language: string, content: string, status: TranslationStatus) => void;
		onAiTranslate?: (sourceLanguage: string, targetLanguage: string) => Promise<string>;
	}
	
	let { 
		open = $bindable(false),
		translations = [],
		initialLanguage = null,
		onClose,
		onUpdate,
		onAiTranslate
	}: Props = $props();
	
	let activeLanguage = $state<string | null>(null);
	let isTranslating = $state(false);
	let hasEdited = $state(false);
	let previousOpen = false;

	$effect(() => {
		if (open && !previousOpen) {
			// Initialize active language when dialog opens
			if (initialLanguage && translations.some(t => t.language === initialLanguage)) {
				activeLanguage = initialLanguage;
			} else {
				const nonPrimary = translations.find(t => t.status !== 'primary');
				activeLanguage = nonPrimary?.language ?? null;
			}
			hasEdited = false;
		}
		previousOpen = open;
	});
	
	let primaryTranslation = $derived(translations.find(t => t.status === 'primary'));
	let activeTranslation = $derived(translations.find(t => t.language === activeLanguage));
	
	function handleClose() {
		open = false;
		onClose?.();
	}
	
	function setActiveLanguage(language: string | null) {
		if (!language || primaryTranslation?.language === language) return;
		activeLanguage = language;
		hasEdited = false;
	}
	
	function handleContentChange(language: string | null, content: string) {
		if (!language) return;
		const translation = translations.find(t => t.language === language);
		if (!translation) return;
		
		const newStatus = translation.status === 'primary' ? 'primary' : 'draft';
		hasEdited = true;
		onUpdate?.(language, content, newStatus);
	}

	function approveTranslation(language: string | null) {
		if (!language) return;
		const translation = translations.find(t => t.language === language);
		if (translation && translation.status !== 'primary') {
			onUpdate?.(language, translation.content, 'approved');
		}
	}

	function unapproveTranslation(language: string | null) {
		if (!language) return;
		const translation = translations.find(t => t.language === language);
		if (translation && translation.status === 'approved') {
			onUpdate?.(language, translation.content, 'draft');
		}
	}
	
	async function handleAiTranslate() {
		if (!primaryTranslation || !activeTranslation || !onAiTranslate) return;
		
		isTranslating = true;
		try {
			const translated = await onAiTranslate(
				primaryTranslation.language,
				activeTranslation.language
			);
			handleContentChange(activeLanguage, translated);
		} catch (error) {
			console.error('Translation failed:', error);
		} finally {
			isTranslating = false;
		}
	}

	const statusToBadgeVariant = {
		primary: 'newPrimary',
		draft: 'secondary',
		approved: 'newApproved'
	} as const;
</script>

<Dialog.Root bind:open>
	<Dialog.Content class="max-h-[90vh] min-w-[70vw] p-12" showCloseButton={false}>
		
		<!-- Header -->
		<Dialog.Header class="flex items-center justify-between pr-0 flex-row">
			<Dialog.Title class="justify-start text-black text-3xl font-semibold leading-8">
				Content Translation
			</Dialog.Title>
			<button
				type="button"
				onclick={handleClose}
				class="rounded-sm opacity-70 transition-opacity hover:opacity-100"
			>
				<X />
				<span class="sr-only">Close</span>
			</button>
		</Dialog.Header>
		
		{#if primaryTranslation}
		<div class="self-stretch relative inline-flex flex-col justify-center items-start gap-7">
			
			<!-- Language Tabs -->
			<div class="self-stretch border-b flex flex-col justify-start items-center gap-6 overflow-hidden">
				<div class="self-stretch px-3 border-b border-base-border inline-flex justify-center items-start">
					<div class="flex-1 max-w-[1304px] flex justify-start items-center">
						
						{#each translations as translation (translation.language)}
							<button
							  type="button"
							  class="py-1.5 border-b-[3px] {activeLanguage === translation.language ? 'border-colors-CS_Blue-500' : 'border-transparent'} flex justify-center items-center gap-2"
							  onclick={() => setActiveLanguage(translation.language)}
							>
							  <div class="px-3 py-2 rounded-lg flex justify-center items-center gap-2">
							    <div class="justify-start {activeLanguage === translation.language ? 'text-base-foreground' : 'text-base-accent-foreground'} text-lg font-semibold leading-7">
							      {translation.languageName}
							    </div>
							    <Badge variant={statusToBadgeVariant[translation.status]}>
							      <span class="capitalize">{translation.status}</span>
							    </Badge>
							  </div>
							</button>
						{/each}     
					</div>
				</div>
			</div>

			<!-- Current Language Title (only show if we have an active translation) -->
			{#if activeTranslation}
				<div class="self-stretch px-6 flex flex-col justify-start items-center gap-6">
					<div class="w-full max-w-[1280px] inline-flex justify-between items-center overflow-hidden">
						<div class="flex-1 inline-flex flex-col justify-start items-start gap-2">
							<h2 class="text-base-foreground text-2xl font-semibold leading-7">
							  {activeTranslation.languageName}
							</h2>
						</div>
					</div>
				</div>
			{/if}

			<!-- Two Column Layout -->
			<div class="w-full max-w-[1328px] inline-flex justify-center items-start">
				<div class="flex-1 px-6 flex justify-center items-start gap-12">
					
					<!-- Left Column: Primary Language -->
					<div class="flex-1 inline-flex flex-col justify-start items-start gap-6">
						<div class="self-stretch flex flex-col justify-start items-start gap-2">
							<div class="inline-flex justify-start items-center gap-2">
							  <div class="justify-start text-base-card-foreground text-base font-semibold leading-6">
							    {primaryTranslation.languageName}
							  </div>
							  <Badge variant="newPrimary">Primary</Badge>
							</div>
						</div>
						
						<!-- Editor Box (Primary)-->
						<div class="self-stretch bg-white rounded-md outline outline-1 outline-offset-[-1px] outline-colors-CS_grey-300 flex flex-col justify-start items-start overflow-hidden">
							
							<!-- Content -->
							<div class="self-stretch px-4 pt-3 pb-4 flex flex-col justify-start items-start overflow-hidden">
							  <textarea
							    class="self-stretch min-h-[200px] w-full resize-none border-none outline-none text-neutral-800 text-sm font-normal leading-5 bg-transparent"
							    bind:value={primaryTranslation.content}
							    oninput={(e) => handleContentChange(primaryTranslation.language, e.currentTarget.value)}
							    placeholder="Primary content..."
							  ></textarea>
							</div>
						</div>
					</div>

					<!-- Right Column: Active Translation or Empty State -->
					<div class="flex-1 self-stretch inline-flex flex-col justify-start items-start gap-6">
						{#if activeTranslation}
							<!-- Header with Auto-save indicator and AI button -->
							<div class="self-stretch h-7 inline-flex justify-between items-start">
							  <div class="w-64 flex justify-start items-center gap-2">
							    <div class="justify-start text-base-card-foreground text-base font-semibold leading-6">
							      {activeTranslation.languageName}
							    </div>
							    
							    {#if hasEdited}
							      <div class="h-7 flex justify-center items-center gap-1 overflow-hidden">
							        <div class="w-4 h-4 flex justify-center items-center gap-2.5">
							          <div class="w-2 h-2 bg-blue-500 rounded-full"></div>
							        </div>
							        <div class="justify-start text-blue-500 font-normal text-sm leading-5">
							          Auto saved
							        </div>
							      </div>
							    {/if}
							  </div>

							  {#if onAiTranslate}
							    <Button onclick={handleAiTranslate} disabled={isTranslating}>
							      {isTranslating ? 'Translating...' : 'AI translation'}
							      <Sparkles class="size-4" />
							    </Button>
							  {/if}
							</div>
							
							<!-- Editor Box (Active/Editable) -->
							<div class="self-stretch flex-1 bg-white rounded-md outline outline-[1.50px] outline-offset-[-1.50px] outline-colors-CS_Blue-500 flex flex-col justify-start items-start overflow-hidden">
							  
							  <!-- Content (Editable) -->
							  <div class="self-stretch px-4 pt-3 pb-4 flex flex-col justify-start items-start overflow-hidden">
							    <textarea
							      class="self-stretch min-h-[200px] w-full resize-none border-none outline-none text-neutral-800 text-sm font-normal leading-5 bg-transparent"
							      bind:value={activeTranslation.content}
							      oninput={(e) => handleContentChange(activeLanguage, e.currentTarget.value)}
							      placeholder="Translation content..."
							    ></textarea>
							  </div>
							</div>

							<!-- Approve Button & Menu -->
							<div class="self-stretch flex justify-center items-center gap-2 pt-4">
							  <Button 
							    onclick={() => approveTranslation(activeLanguage)}
							    disabled={activeTranslation.status === 'approved' || !activeTranslation.content}
							    class="gap-2"
							    variant={activeTranslation.status === 'approved' ? 'outline' : 'default'}
							  >
							    <Check class="size-4" />
							    {activeTranslation.status === 'approved' ? 'Approved' : 'Approve'}
							  </Button>

							  <!-- Three dots menu -->
							  <DropdownMenu.Root>
							    <DropdownMenu.Trigger>
							      <Button variant="outline" size="icon" class="h-10 w-10">
							        <MoreHorizontal class="size-4" />
							      </Button>
							    </DropdownMenu.Trigger>
							    <DropdownMenu.Content>
							      <DropdownMenu.Item 
							        onclick={() => unapproveTranslation(activeLanguage)}
							        disabled={activeTranslation.status !== 'approved'}
							      >
							        Mark as draft
							      </DropdownMenu.Item>
							    </DropdownMenu.Content>
							  </DropdownMenu.Root>
							</div>
						{:else}
							<!-- Empty State: No translation language selected -->
							<div class="self-stretch flex-1 flex flex-col items-center justify-center gap-4 min-h-[300px] bg-gray-50 rounded-md border-2 border-dashed border-gray-200">
							  <div class="text-gray-400 text-center">
							    <p class="text-lg font-medium">No translation selected</p>
							    <p class="text-sm mt-1">Select a language tab above to edit its translation</p>
							  </div>
							</div>
						{/if}
					</div>

				</div>
			</div>
		</div>
		{:else}
			<div class="p-8 text-center text-gray-500">
				No translations available
			</div>
		{/if}

	</Dialog.Content>
</Dialog.Root>

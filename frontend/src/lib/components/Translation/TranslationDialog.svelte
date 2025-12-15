<script lang="ts">
	import * as Dialog from '$lib/components/ui/dialog';
	import { Button } from '$lib/components/ui/button';
	import { Badge } from '$lib/components/ui/badge';
	import { X, Sparkles, Check, MoreHorizontal } from 'lucide-svelte';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import { statusToBadgeVariant, type TranslationEntry, type TranslationStatus } from './useTranslations.svelte';

	interface Props {
		open?: boolean;
		translations: TranslationEntry[];
		activeLanguage: string | null;
		isTranslating?: boolean;
		onClose: () => void;
		onContentChange: (language: string, content: string) => void;
		onStatusChange: (language: string, status: TranslationStatus) => void;
		onActiveLanguageChange: (language: string) => void;
		onAiTranslate: () => void;
	}
	
	let { 
		open = $bindable(false),
		translations,
		activeLanguage,
		isTranslating = false,
		onClose,
		onContentChange,
		onStatusChange,
		onActiveLanguageChange,
		onAiTranslate
	}: Props = $props();
	
	let primaryTranslation = $derived(translations.find(t => t.status === 'primary'));
	let activeTranslation = $derived(translations.find(t => t.language === activeLanguage));
	
	function handleTabClick(language: string) {
		if (primaryTranslation?.language === language) return;
		onActiveLanguageChange(language);
	}

</script>

<Dialog.Root bind:open>
	<Dialog.Content class="scot-gov max-h-[90vh] min-w-[70vw] p-12 rounded-[12px]" showCloseButton={false}>
		
		<!-- Header -->
		<Dialog.Header class="flex items-center justify-between pr-0 flex-row">
			<Dialog.Title class="justify-start text-black text-3xl font-semibold leading-8">
				Content Translation
			</Dialog.Title>
			<button
				type="button"
				onclick={onClose}
				class="rounded-sm opacity-70 transition-opacity hover:opacity-100"
			>
				<X />
				<span class="sr-only">Close</span>
			</button>
		</Dialog.Header>
		
		{#if primaryTranslation}
		<div class="self-stretch relative inline-flex flex-col justify-center items-start gap-7 overflow-y-auto max-h-[calc(90vh-120px)]">
			
			<!-- Language Tabs -->
			<div class="self-stretch flex flex-col justify-start items-center gap-6">
				<div class="self-stretch px-3 border-b border-base-border inline-flex justify-center items-start">
					<div class="flex-1 max-w-[1304px] flex justify-start items-center">
						
						{#each translations as translation (translation.language)}
							<button
							  type="button"
							  class="py-1.5 border-b-[3px] -mb-[1px] {activeLanguage === translation.language ? 'border-primary' : 'border-transparent'} flex justify-center items-center gap-2"
							  onclick={() => handleTabClick(translation.language)}
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
						<div class="self-stretch h-7 flex justify-start items-center gap-2">
							<div class="justify-start text-base-card-foreground text-base font-semibold leading-6">
							  {primaryTranslation.languageName}
							</div>
							<Badge variant="outline">Primary</Badge>
						</div>
						
						<!-- Editor Box (Primary)-->
							<div class="self-stretch bg-white rounded-[12px] outline outline-1 outline-offset-[-1px] outline-colors-CS_grey-300 flex flex-col justify-start items-start overflow-hidden">
								
								<!-- Content -->
								<div class="self-stretch px-4 pt-3 pb-4 flex flex-col justify-start items-start overflow-hidden">
								  <textarea
								    class="self-stretch min-h-[200px] w-full resize-none border-none outline-none text-neutral-800 text-sm font-normal leading-5 bg-transparent"
								    value={primaryTranslation.content}
								    oninput={(e) => onContentChange(primaryTranslation.language, e.currentTarget.value)}
								    placeholder="Primary content..."
								  ></textarea>
								</div>
							</div>
					</div>

					<!-- Right Column: Active Translation   -->
					<div class="flex-1 self-stretch inline-flex flex-col justify-start items-start gap-6">
						{#if activeTranslation}
							<!-- Header with Auto-save indicator and AI button -->
							<div class="self-stretch h-7 inline-flex justify-between items-start">
							  <div class="w-64 flex justify-start items-center gap-2">
							    <div class="justify-start text-base-card-foreground text-base font-semibold leading-6">
							      {activeTranslation.languageName}
							    </div>
							  </div>

							  <Button onclick={onAiTranslate} disabled={isTranslating}>
							    {isTranslating ? 'Translating...' : 'AI translation'}
							    <Sparkles class="size-4" />
							  </Button>
							</div>
							
							<!-- Editor Box (Active/Editable) -->
								<div class="self-stretch flex-1 bg-white rounded-[12px] outline outline-[1.50px] outline-offset-[-1.50px] outline-colors-CS_Blue-500 flex flex-col justify-start items-start overflow-hidden">
								  
								  <!-- Content (Editable) -->
								  <div class="self-stretch px-4 pt-3 pb-4 flex flex-col justify-start items-start overflow-hidden">
								    <textarea
								      class="self-stretch min-h-[200px] w-full resize-none border-none outline-none text-neutral-800 text-sm font-normal leading-5 bg-transparent"
								      value={activeTranslation.content}
								      oninput={(e) => onContentChange(activeTranslation.language, e.currentTarget.value)}
								      placeholder="Translation content..."
								    ></textarea>
								  </div>
								</div>

							<!-- Approve Button & Menu -->
								<div class="self-stretch flex justify-center items-center gap-2 pt-4">
								  <Button 
								    onclick={() => onStatusChange(activeTranslation.language, 'approved')}
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
								        onclick={() => onStatusChange(activeTranslation.language, 'draft')}
								        disabled={activeTranslation.status !== 'approved'}
								      >
								        Mark as draft
								      </DropdownMenu.Item>
								    </DropdownMenu.Content>
								  </DropdownMenu.Root>
								</div>
						{/if}
					</div>

				</div>
			</div>
		</div>
		{/if}

	</Dialog.Content>
</Dialog.Root>

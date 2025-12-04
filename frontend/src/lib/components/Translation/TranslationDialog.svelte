<script lang="ts">
  import * as Dialog from '$lib/components/ui/dialog';
  import { Button } from '$lib/components/ui/button';
  import { Badge } from '$lib/components/ui/badge';
  import { X, Sparkles } from 'lucide-svelte';
  
  type TranslationStatus = 'draft' | 'approved' | 'primary';
  type Language = 'en' | 'gd' | 'cy';
  
  interface Translation {
    language: Language;
    languageName: string;
    status: TranslationStatus;
    content: string;
    lastSaved?: Date;
    isAutoSaved?: boolean;
  }
  
  interface Props {
    open?: boolean;
    translations?: Translation[];
    onClose?: () => void;
    onSave?: (translations: Translation[]) => void;
    onAiTranslate?: (sourceLanguage: Language, targetLanguage: Language) => Promise<string>;
  }
  
  let { 
    open = $bindable(false),
    translations: propTranslations = [],
    onClose,
    onSave,
    onAiTranslate
  }: Props = $props();
  
  let translations = $state<Translation[]>([...propTranslations]);
  let activeLanguage = $state<Language>('gd');
  let isTranslating = $state(false);
  
  $effect(() => {
    if (propTranslations.length > 0) {
      translations = [...propTranslations];
    }
  });
  
  let primaryTranslation = $derived(
    translations.find(t => t.status === 'primary')
  );
  
  let activeTranslation = $derived(
    translations.find(t => t.language === activeLanguage)
  );
  
  function handleClose() {
    open = false;
    onClose?.();
  }
  
  function setActiveLanguage(language: Language) {
    if (primaryTranslation?.language === language) return;
    activeLanguage = language;
  }
  
  function updateContent(language: Language, content: string) {
    const index = translations.findIndex(t => t.language === language);
    if (index !== -1) {
      translations[index] = {
        ...translations[index],
        content,
        lastSaved: new Date(),
        isAutoSaved: true
      };
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
      updateContent(activeLanguage, translated);
    } catch (error) {
      console.error('Translation failed:', error);
    } finally {
      isTranslating = false;
    }
  }
  
  function handleSave() {
    onSave?.(translations);
    handleClose();
  }
  
  function getStatusBadgeVariant(status: TranslationStatus) {
    switch (status) {
      case 'primary':
        return 'newPrimary' as const;
      case 'draft':
        return 'newSecondary' as const;
      case 'approved':
        return 'newApproved' as const;
    }
  }
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
    
    <div class="self-stretch relative inline-flex flex-col justify-center items-start gap-7">
      
      <!-- Language Tabs -->
      <div class="self-stretch border-b flex flex-col justify-start items-center gap-6 overflow-hidden">
        <div class="self-stretch px-3 border-b border-base-border inline-flex justify-center items-start">
          <div class="flex-1 max-w-[1304px] flex justify-start items-center">
            
            {#each translations as translation (translation.language)}
              <button
                type="button"
                class="py-1.5 border-b-[3px] {activeLanguage === translation.language ? 'border-base-primary' : 'border-transparent'} flex justify-center items-center gap-2"
                onclick={() => setActiveLanguage(translation.language)}
              >
                <div class="px-3 py-2 rounded-lg flex justify-center items-center gap-2">
                  <div class="justify-start {activeLanguage === translation.language ? 'text-base-foreground' : 'text-base-accent-foreground'} text-lg font-semibold leading-7">
                    {translation.languageName}
                  </div>
                  <Badge variant={getStatusBadgeVariant(translation.status)}>
                    {translation.status.charAt(0).toUpperCase() + translation.status.slice(1)}
                  </Badge>
                </div>
              </button>
            {/each}
            
            <div class="py-1.5 flex justify-center items-center gap-2">
              <button type="button" class="px-3 py-2 rounded-full flex justify-center items-center gap-2">
                <div class="justify-start text-base-accent-foreground text-lg font-semibold leading-7">
                  + Add new
                </div>
              </button>
            </div>
            
          </div>
        </div>
      </div>

      <!-- Current Language Title -->
      <div class="self-stretch px-6 flex flex-col justify-start items-center gap-6">
        <div class="w-full max-w-[1280px] inline-flex justify-between items-center overflow-hidden">
          <div class="flex-1 inline-flex flex-col justify-start items-start gap-2">
            <h2 class="text-base-foreground text-2xl font-semibold leading-7">
              {activeTranslation?.languageName}
            </h2>
          </div>
        </div>
      </div>

      <!-- Two Column Layout -->
      <div class="w-full max-w-[1328px] inline-flex justify-center items-start">
        <div class="flex-1 px-6 flex justify-center items-start gap-12">
          
          <!-- Left Column: Primary Language -->
          <div class="flex-1 inline-flex flex-col justify-start items-start gap-6">
            <div class="self-stretch flex flex-col justify-start items-start gap-2">
              <div class="inline-flex justify-start items-center gap-2">
                <div class="justify-start text-base-card-foreground text-base font-semibold leading-6">
                  {primaryTranslation?.languageName}
                </div>
                <Badge variant="newPrimary">Primary</Badge>
              </div>
            </div>
            
            <!-- Editor Box (Primary/Editable) -->
            <div class="self-stretch bg-white rounded-md outline outline-1 outline-offset-[-1px] outline-colors-CS_grey-300 flex flex-col justify-start items-start overflow-hidden">
              
              <!-- TODO: we need to be able to render tiptap or textare depending on the type of text? -->
              <!-- Content -->
              <div class="self-stretch px-4 pt-3 pb-4 flex flex-col justify-start items-start overflow-hidden">
                <textarea
                  class="self-stretch min-h-[200px] w-full resize-none border-none outline-none text-neutral-800 text-sm font-normal leading-5 bg-transparent"
                  bind:value={primaryTranslation.content}
                  oninput={(e) => updateContent(primaryTranslation.language, e.currentTarget.value)}
                  placeholder="Primary content..."
                ></textarea>
              </div>
            </div>
          </div>

          <!-- Right Column: Active Translation -->
          <div class="flex-1 self-stretch inline-flex flex-col justify-start items-start gap-6">
            
            <!-- Header with Auto-save indicator and AI button -->
            <div class="self-stretch h-7 inline-flex justify-between items-start">
              <div class="w-64 flex justify-start items-center gap-2">
                <div class="justify-start text-base-card-foreground text-base font-semibold leading-6">
                  {activeTranslation?.languageName}
                </div>
                
                {#if activeTranslation?.isAutoSaved}
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

              {#if onAiTranslate && activeTranslation?.status !== 'primary'}
                <Button variant="newPrimary" onclick={handleAiTranslate} disabled={isTranslating}>
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
                  oninput={(e) => updateContent(activeLanguage, e.currentTarget.value)}
                  placeholder="Translation content..."
                ></textarea>
              </div>
            </div>
          </div>

        </div>
      </div>
    </div>

  </Dialog.Content>
</Dialog.Root>

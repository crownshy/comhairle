<script lang="ts">
	import ContentRenderer from '$lib/components/RichTextEditor/ContentRenderer/ContentRenderer.svelte';
	import { firstImageSrc, withoutFirstImage } from '$lib/step-brief/splitSlides';
	import { stepMeta, type MetaToolConfig } from '$lib/step-brief/slideMeta';
	import { TOOL_META, type ToolType } from '$lib/tool_meta';
	import { Clock } from 'lucide-svelte';
	import type { ComhairleDocument } from '@crownshy/api-client/api';

	let {
		slide,
		title,
		showTitle = false,
		showMeta = false,
		toolConfig,
		availableDocuments = [],
		conversationId
	}: {
		slide: string;
		title?: string;
		/** The cover names the step; later slides do not repeat it. */
		showTitle?: boolean;
		/** Only the cover carries the derived duration and count line. */
		showMeta?: boolean;
		toolConfig?: MetaToolConfig | null;
		availableDocuments?: ComhairleDocument[];
		conversationId?: string;
	} = $props();

	let illustration = $derived(firstImageSrc(slide));
	// The hero is lifted out of the content, so the body must not render it a second time.
	let body = $derived(illustration ? withoutFirstImage(slide) : slide);
	let toolType = $derived(toolConfig?.type as ToolType | undefined);
	let FallbackIcon = $derived(toolType ? TOOL_META[toolType]?.icon : undefined);
	let meta = $derived(showMeta ? stepMeta(toolConfig) : []);
</script>

<div class="flex w-full max-w-xl flex-col items-center gap-6 text-center">
	{#if illustration}
		<div class="w-full overflow-hidden rounded-[20px] rounded-br-[100px] bg-black/5">
			<img src={illustration} alt="" class="h-[156px] w-full object-cover md:h-[220px]" />
		</div>
	{:else if FallbackIcon}
		<div
			class="bg-accent flex size-20 items-center justify-center rounded-full md:size-24"
			aria-hidden="true"
		>
			<FallbackIcon class="text-primary size-10 md:size-12" />
		</div>
	{/if}

	{#if showTitle && title}
		<h1 class="text-primary text-3xl leading-10 font-bold md:text-4xl">{title}</h1>
	{/if}

	<div class="prose prose-p:text-base w-full max-w-none text-base leading-6">
		<ContentRenderer content={body} {availableDocuments} {conversationId} />
	</div>

	{#if meta.length > 0}
		<div class="flex flex-col items-center gap-2">
			{#each meta as item (item.label)}
				<p class="text-muted-foreground flex items-center gap-2 text-base">
					{#if item.kind === 'duration'}
						<Clock class="size-5 shrink-0" aria-hidden="true" />
					{:else if FallbackIcon}
						<FallbackIcon class="size-5 shrink-0" aria-hidden="true" />
					{/if}
					{item.label}
				</p>
			{/each}
		</div>
	{/if}
</div>

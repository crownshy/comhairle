<script lang="ts">
	import Globe from 'lucide-svelte/icons/globe';
	import Languages from 'lucide-svelte/icons/languages';
	import LoaderCircle from 'lucide-svelte/icons/loader-circle';
	import TriangleAlert from 'lucide-svelte/icons/triangle-alert';
	import { Button } from '$lib/components/ui/button';
	import type { PublicStepResults } from '$lib/reports/embeds';
	import type { SaveState } from '$lib/components/Translation/translationUtils';
	import { report_url } from '$lib/urls';
	import { cn } from '$lib/utils';

	type Props = {
		conversationId: string;
		published: boolean;
		canEdit: boolean;
		results: PublicStepResults[];
		saveState: SaveState;
		hasTranslations: boolean;
		onRetry: () => void;
		onReview: () => void;
		onTranslate: () => void;
		onUnpublish: () => void;
	};
	let {
		conversationId,
		published,
		canEdit,
		results,
		saveState,
		hasTranslations,
		onRetry,
		onReview,
		onTranslate,
		onUnpublish
	}: Props = $props();

	const chartCount = $derived(results.reduce((sum, step) => sum + step.sectionBlocks.length, 0));
	const stepNames = $derived(results.map((step) => step.stepName));

	function listNames(names: string[]) {
		if (names.length <= 1) return names.join('');
		return `${names.slice(0, -1).join(', ')} and ${names[names.length - 1]}`;
	}

	const sharing = $derived(
		`${published ? 'Public now:' : 'Not published. Publishing shares'} the text and ${chartCount} ${
			chartCount === 1 ? 'chart' : 'charts'
		}${chartCount > 0 ? ` from ${listNames(stepNames)}` : ''}`
	);
</script>

<!-- What the report shares is always on screen (ADR-0061); clicking it shows the full list. -->
<header
	data-tour="report-publish"
	class="flex flex-wrap items-center gap-x-4 gap-y-2 border-b px-6 py-2"
>
	<span class="text-base font-semibold">Final report</span>
	<button
		type="button"
		class={cn(
			'flex min-w-0 items-center gap-2 rounded-full px-3 py-1 text-base hover:underline',
			published ? 'bg-primary/10 text-foreground' : 'text-muted-foreground'
		)}
		title="See exactly what is shared"
		onclick={onReview}
	>
		<Globe class="text-primary size-4 shrink-0" />
		<span class="truncate">{sharing}</span>
	</button>
	{#if saveState === 'saving'}
		<span class="text-muted-foreground flex items-center gap-1 text-sm">
			<LoaderCircle class="size-3.5 animate-spin motion-reduce:animate-none" />Saving
		</span>
	{:else if saveState === 'error'}
		<span class="text-destructive flex items-center gap-1 text-sm">
			<TriangleAlert class="size-3.5" />Not saved
			<Button variant="link" class="h-auto p-0 text-sm" onclick={onRetry}>Retry</Button>
		</span>
	{/if}
	<div class="ml-auto flex items-center gap-2">
		{#if hasTranslations}
			<Button variant="ghost" onclick={onTranslate}
				><Languages class="size-4" />Translate</Button
			>
		{/if}
		{#if published}
			<Button variant="ghost" href={report_url(conversationId, '')}>Open public report</Button
			>
			{#if canEdit}
				<Button variant="outline" onclick={onUnpublish}>Unpublish</Button>
			{/if}
		{:else}
			<Button
				variant="ghost"
				href={report_url(conversationId, '', true)}
				target="_blank"
				rel="noopener">Preview</Button
			>
			{#if canEdit}
				<Button onclick={onReview}>Review and publish</Button>
			{/if}
		{/if}
	</div>
</header>

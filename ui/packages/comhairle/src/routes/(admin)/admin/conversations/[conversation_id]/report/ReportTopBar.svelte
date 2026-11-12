<script lang="ts">
	import Check from 'lucide-svelte/icons/check';
	import EllipsisVertical from 'lucide-svelte/icons/ellipsis-vertical';
	import EyeOff from 'lucide-svelte/icons/eye-off';
	import ExternalLink from 'lucide-svelte/icons/external-link';
	import Globe from 'lucide-svelte/icons/globe';
	import Languages from 'lucide-svelte/icons/languages';
	import LoaderCircle from 'lucide-svelte/icons/loader-circle';
	import PencilLine from 'lucide-svelte/icons/pencil-line';
	import TriangleAlert from 'lucide-svelte/icons/triangle-alert';
	import { Button } from '$lib/components/ui/button';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
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

	const charts = $derived(`${chartCount} ${chartCount === 1 ? 'chart' : 'charts'}`);
	const sharing = $derived(
		`${published ? 'Shares' : 'Will share'} the text and ${charts}${
			chartCount > 0 ? ` from ${listNames(stepNames)}` : ''
		}`
	);
</script>

<!-- What the report shares is always on screen (ADR-0061); clicking it lists each chart. -->
<header
	data-tour="report-publish"
	class="px-gutter flex flex-wrap items-center gap-x-4 gap-y-2 border-b py-2"
>
	<button
		type="button"
		class="hover:bg-accent/60 flex min-w-0 items-center gap-2 rounded-full py-1 ps-1 pe-3 text-start text-base transition-colors"
		title={sharing}
		onclick={onReview}
	>
		<span
			class={cn(
				'flex shrink-0 items-center gap-1.5 rounded-full px-2.5 py-0.5 font-medium',
				published ? 'bg-primary text-primary-foreground' : 'bg-muted text-foreground'
			)}
		>
			{#if published}
				<Globe class="size-4" aria-hidden="true" />Published
			{:else}
				<PencilLine class="size-4" aria-hidden="true" />Draft
			{/if}
		</span>
		<span class="text-muted-foreground truncate">{sharing}</span>
	</button>

	{#if canEdit}
		<!-- Edits save as you type, so on a published report they reach readers straight away. -->
		<span class="text-muted-foreground flex items-center gap-1.5 text-sm" aria-live="polite">
			{#if saveState === 'saving'}
				<LoaderCircle class="size-3.5 animate-spin motion-reduce:animate-none" />Saving
			{:else if saveState === 'error'}
				<span class="text-destructive flex items-center gap-1.5">
					<TriangleAlert class="size-3.5" />Not saved
				</span>
				<Button variant="link" class="h-auto p-0 text-sm" onclick={onRetry}>Retry</Button>
			{:else}
				<Check class="size-3.5" />{published ? 'Saved, changes are live' : 'Saved'}
			{/if}
		</span>
	{/if}

	<div class="ml-auto flex items-center gap-2">
		{#if hasTranslations}
			<Button variant="ghost" onclick={onTranslate}>
				<Languages class="size-4" />Translate
			</Button>
		{/if}
		{#if published}
			<Button
				variant="outline"
				href={report_url(conversationId, '')}
				target="_blank"
				rel="noopener"
			>
				Open public report<ExternalLink class="size-4" />
			</Button>
			{#if canEdit}
				<DropdownMenu.Root>
					<DropdownMenu.Trigger>
						{#snippet child({ props })}
							<Button
								{...props}
								variant="ghost"
								size="icon"
								aria-label="More report actions"
							>
								<EllipsisVertical class="size-5" />
							</Button>
						{/snippet}
					</DropdownMenu.Trigger>
					<DropdownMenu.Content align="end">
						<DropdownMenu.Item onclick={onUnpublish}>
							<EyeOff class="size-4" />Unpublish
						</DropdownMenu.Item>
					</DropdownMenu.Content>
				</DropdownMenu.Root>
			{/if}
		{:else}
			<Button
				variant="ghost"
				href={report_url(conversationId, '', true)}
				target="_blank"
				rel="noopener"
			>
				Preview<ExternalLink class="size-4" />
			</Button>
			{#if canEdit}
				<Button onclick={onReview}>Review and publish</Button>
			{/if}
		{/if}
	</div>
</header>

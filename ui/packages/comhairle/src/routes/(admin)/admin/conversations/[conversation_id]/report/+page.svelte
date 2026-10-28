<script lang="ts">
	import { untrack } from 'svelte';
	import type { Editor } from '@tiptap/core';
	import Sparkles from 'lucide-svelte/icons/sparkles';
	import X from 'lucide-svelte/icons/x';
	import PenLine from 'lucide-svelte/icons/pen-line';
	import { Button } from '$lib/components/ui/button';
	import { apiClient } from '@crownshy/api-client/client';
	import { ReportTranslations } from '@crownshy/api-client/api';
	import { invalidate } from '$app/navigation';
	import { notifications } from '$lib/notifications.svelte.js';
	import { createTextContentSource } from '$lib/components/Translation/translationSource.svelte.js';
	import { registerAutosaveSource } from '$lib/components/Translation/autosaveRegistry.svelte';
	import type { Locale } from '$lib/paraglide/runtime.js';
	import { key } from '$lib/utils/invalidationKey.js';
	import { permissions } from '$lib/permissions.svelte';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import { cn } from '$lib/utils';
	import { publicResultsByStep, sectionBlocksInDocument } from '$lib/reports/embeds';
	import {
		REPORT_COLUMN_CLASS,
		REPORT_EDITOR_HOVER_CLASS,
		REPORT_EDITOR_SECTION_CLASS,
		REPORT_EDITOR_TEXT_CLASS,
		REPORT_EYEBROW_CLASS
	} from '$lib/reports/styles';
	import { useTour } from '$lib/tours/useTour.svelte';
	import { reportTour } from './tour';
	import ReportTopBar from './ReportTopBar.svelte';
	import ReportTitle from './ReportTitle.svelte';
	import ReportTextEditor from './ReportTextEditor.svelte';
	import EditingRail from './EditingRail.svelte';
	import SectionPills from './SectionPills.svelte';
	import AddChartDialog from './AddChartDialog.svelte';
	import PublishDialog from './PublishDialog.svelte';
	import TranslationsDialog from './TranslationsDialog.svelte';
	import ImpactsAndNotes from './ImpactsAndNotes.svelte';
	import {
		countDraftPrompts,
		draftReport,
		isUnwritten,
		reportSections,
		type ReportSection
	} from './reportDocument';

	useTour(() => reportTour);

	let { data } = $props();
	let report = $derived(data.report);
	let translations = $derived(ReportTranslations.parse(report.translations));
	let conversation = $derived(data.conversation);
	let primaryLocale = $derived(conversation.primaryLocale as Locale);
	let supportedLanguages = $derived(conversation.supportedLanguages as Locale[]);
	const canEdit = $derived(
		permissions.can('conversation', 'conversation_update', conversation.id)
	);

	const summarySource = createTextContentSource({
		getTranslation: () => translations.summary,
		getPrimaryLocale: () => primaryLocale,
		getSupportedLanguages: () => supportedLanguages,
		refresh: () => invalidate(key('admin/conversation/report'))
	});

	const bodySource = createTextContentSource({
		getTranslation: () => translations.body ?? undefined,
		getPrimaryLocale: () => primaryLocale,
		getSupportedLanguages: () => supportedLanguages,
		refresh: () => invalidate(key('admin/conversation/report')),
		ensureTextContentId: upsertBodyTranslation
	});

	registerAutosaveSource(() => summarySource);
	registerAutosaveSource(() => bodySource);

	async function upsertBodyTranslation(content: string): Promise<string | undefined> {
		const textContentRes = await tryCatchAsync(() =>
			apiClient.CreateTextContent({
				content,
				format: 'rich',
				primary_locale: conversation.primaryLocale
			})
		);

		if (textContentRes.err !== null) {
			notifications.send({
				message: 'Failed to create translatable content',
				priority: 'ERROR'
			});
			return;
		}

		const updateReportRes = await tryCatchAsync(() =>
			apiClient.UpdateReport(
				{
					body: textContentRes.ok.id
				},
				{ params: { conversation_id: conversation.id } }
			)
		);

		if (updateReportRes.err !== null) {
			notifications.send({
				message: 'Failed to update report',
				priority: 'ERROR'
			});
			return;
		}

		return textContentRes.ok.id;
	}

	// A report nobody has written opens on a generated starter body (ADR-0066). It is only shown,
	// not saved: the first edit saves it, so opening the page never writes anything.
	const savedSummary = $derived(summarySource.contents[primaryLocale] ?? '');
	const savedBody = $derived(bodySource.contents[primaryLocale] ?? '');
	const bodyIsUnwritten = $derived(isUnwritten(savedBody));
	const initialSummary = $derived(isUnwritten(savedSummary) ? '' : savedSummary);
	const initialBody = $derived(
		bodyIsUnwritten ? JSON.stringify(draftReport(data.reportEmbedSteps)) : savedBody
	);

	// What's on screen right now, including a draft that hasn't been saved yet. Plain state, not
	// derived from the saved text: clearing the body saves it as empty, and the pills must keep
	// showing the empty page rather than the draft.
	let summaryOnScreen = $state(untrack(() => initialSummary));
	let bodyOnScreen = $state(untrack(() => initialBody));
	let draftNoteDismissed = $state(false);

	let published = $derived(report.isPublic);
	let summaryEditor = $state<Editor>();
	let bodyEditor = $state<Editor>();
	let activeEditor = $state<Editor>();
	let addingChart = $state(false);
	let reviewing = $state(false);
	let translating = $state(false);
	// Bumped after the translations dialog, which can also edit the main language.
	let editorsRevision = $state(0);

	const otherLocaleContents = $derived(
		supportedLanguages
			.filter((locale) => locale !== primaryLocale)
			.flatMap((locale) => [
				summarySource.contents[locale] ?? '',
				bodySource.contents[locale] ?? ''
			])
	);
	const publicResults = $derived(
		publicResultsByStep(
			[summaryOnScreen, bodyOnScreen, ...otherLocaleContents],
			data.reportEmbedSteps
		)
	);
	const embeddedKeys = $derived(
		new Set(
			[summaryOnScreen, bodyOnScreen]
				.flatMap(sectionBlocksInDocument)
				.map((block) => `${block.workflowStepId}:${block.sectionBlock}`)
		)
	);
	const sections = $derived(reportSections(bodyOnScreen, 'Introduction'));
	const saveState = $derived(
		[summarySource.saveState, bodySource.saveState].find((state) => state !== 'idle') ?? 'idle'
	);

	function saveSummary(json: string) {
		summaryOnScreen = json;
		summarySource.saveSource(json);
	}

	function saveBody(json: string) {
		bodyOnScreen = json;
		bodySource.saveSource(json);
	}

	/** The document position just before the body's top-level node at `index`. */
	function positionBefore(editor: Editor, index: number): number {
		let position = 0;
		for (let i = 0; i < index; i++) position += editor.state.doc.child(i).nodeSize;
		return position;
	}

	function jumpToSection(section: ReportSection) {
		if (!bodyEditor) return;
		const element = bodyEditor.view.nodeDOM(positionBefore(bodyEditor, section.firstNodeIndex));
		if (!(element instanceof HTMLElement)) return;
		const reduceMotion = window.matchMedia('(prefers-reduced-motion: reduce)').matches;
		element.scrollIntoView({ behavior: reduceMotion ? 'auto' : 'smooth', block: 'start' });
	}

	// A delete rather than replacing the document, so Ctrl+Z brings the section back.
	function removeSection(section: ReportSection) {
		if (!bodyEditor) return;
		const from = positionBefore(bodyEditor, section.firstNodeIndex);
		const to = positionBefore(bodyEditor, section.firstNodeIndex + section.nodeCount);
		bodyEditor.chain().focus().deleteRange({ from, to }).run();
	}

	function startBlank() {
		bodyEditor?.chain().clearContent(true).focus().run();
		draftNoteDismissed = true;
	}

	async function setPublished(isPublic: boolean) {
		// Publish what's on screen: save an untouched draft, and finish any pending typing.
		if (isPublic && bodyIsUnwritten) bodySource.saveSource(bodyOnScreen);
		await Promise.all([summarySource.flush(), bodySource.flush()]);
		const result = await tryCatchAsync(() =>
			apiClient.UpdateReport(
				{ is_public: isPublic },
				{ params: { conversation_id: conversation.id } }
			)
		);
		if (result.err !== null) {
			notifications.send({
				message: isPublic
					? "The report couldn't be published."
					: "The report couldn't be unpublished.",
				priority: 'ERROR'
			});
			return;
		}
		published = isPublic;
	}
</script>

<svelte:head>
	<title>Conversation Report - Comhairle Admin</title>
</svelte:head>

<div class="bg-background flex min-h-0 grow flex-col">
	<ReportTopBar
		conversationId={conversation.id}
		{published}
		{canEdit}
		results={publicResults}
		{saveState}
		hasTranslations={supportedLanguages.length > 1}
		onRetry={() => Promise.all([summarySource.retry(), bodySource.retry()])}
		onReview={() => (reviewing = true)}
		onTranslate={() => (translating = true)}
		onUnpublish={() => setPublished(false)}
	/>

	<div
		class="grid min-h-0 grow grid-cols-1 overflow-y-auto md:grid-cols-[4.5rem_minmax(0,1fr)_4.5rem]"
	>
		<div class="sticky top-0 hidden self-start py-8 ps-4 md:block">
			{#if canEdit}
				<EditingRail
					editor={activeEditor ?? bodyEditor}
					onAddChart={() => (addingChart = true)}
				/>
			{/if}
		</div>

		<div class={cn(REPORT_COLUMN_CLASS, 'flex flex-col gap-5 py-8')}>
			{#if canEdit && bodyIsUnwritten && !draftNoteDismissed}
				<div
					class="border-primary/40 bg-primary/5 flex flex-wrap items-center gap-x-3 gap-y-1 rounded-full border border-dashed py-1.5 ps-4 pe-1.5 text-base"
				>
					<Sparkles class="text-primary size-4 shrink-0" />
					<span class="grow">Starter draft: write over the prompts in italics.</span>
					<Button size="sm" variant="ghost" onclick={startBlank}>Start blank</Button>
					<Button
						size="icon"
						variant="ghost"
						class="size-8 rounded-full"
						aria-label="Hide this note"
						onclick={() => (draftNoteDismissed = true)}
					>
						<X class="size-4" />
					</Button>
				</div>
			{/if}

			<ReportTitle title={conversation.title} conversationId={conversation.id} />

			{#key editorsRevision}
				<section
					class={cn(
						'bg-card/80 group rounded-t-lg rounded-b-3xl border px-4 py-6 transition-colors',
						canEdit && 'hover:border-primary/40 focus-within:border-primary'
					)}
				>
					<p class="flex items-center justify-between">
						<span class={REPORT_EYEBROW_CLASS}>Executive summary</span>
						{#if canEdit}
							<span
								class="text-muted-foreground flex items-center gap-1 text-sm opacity-0 transition-opacity group-focus-within:opacity-0 group-hover:opacity-100"
							>
								<PenLine class="size-3.5" />Click to edit
							</span>
						{/if}
					</p>
					{#if canEdit && isUnwritten(summaryOnScreen)}
						<p class="text-muted-foreground pt-2 text-base italic">
							A few sentences on what this conversation found. Readers see this first.
						</p>
					{/if}
					<div
						class={cn(
							'pt-2',
							REPORT_EDITOR_TEXT_CLASS,
							canEdit && REPORT_EDITOR_HOVER_CLASS
						)}
					>
						<ReportTextEditor
							initialValue={initialSummary}
							editable={canEdit}
							{published}
							label="Executive summary"
							onChange={saveSummary}
							onFocus={(editor) => (activeEditor = editor)}
							bind:editor={summaryEditor}
						/>
					</div>
				</section>

				<SectionPills
					{sections}
					editable={canEdit}
					onJump={jumpToSection}
					onRemove={removeSection}
				/>

				<div
					class={cn(
						REPORT_EDITOR_TEXT_CLASS,
						REPORT_EDITOR_SECTION_CLASS,
						canEdit && REPORT_EDITOR_HOVER_CLASS
					)}
				>
					<ReportTextEditor
						initialValue={initialBody}
						editable={canEdit}
						{published}
						label="Report"
						onChange={saveBody}
						onFocus={(editor) => (activeEditor = editor)}
						bind:editor={bodyEditor}
					/>
				</div>
			{/key}

			<section class="mt-24 flex flex-col gap-4 border-t pt-10">
				<p class="text-muted-foreground text-base">Not part of the public report</p>
				<ImpactsAndNotes {report} {canEdit} />
			</section>
		</div>
	</div>
</div>

{#if canEdit}
	<AddChartDialog
		bind:open={addingChart}
		steps={data.reportEmbedSteps}
		{embeddedKeys}
		{published}
		onPick={(step, block) =>
			(activeEditor ?? bodyEditor)
				?.chain()
				.focus()
				.setReportComponentEmbed({ toolStepId: step.id, componentType: block.type })
				.run()}
	/>

	<TranslationsDialog
		bind:open={translating}
		{summarySource}
		{bodySource}
		{primaryLocale}
		{supportedLanguages}
		conversationId={conversation.id}
		disabled={!canEdit}
		onClosed={() => {
			summaryOnScreen = initialSummary;
			bodyOnScreen = initialBody;
			editorsRevision += 1;
		}}
	/>
{/if}

<PublishDialog
	bind:open={reviewing}
	{published}
	results={publicResults}
	steps={data.reportEmbedSteps}
	promptsLeft={countDraftPrompts(bodyOnScreen)}
	canPublish={canEdit}
	onPublish={() => setPublished(true)}
/>

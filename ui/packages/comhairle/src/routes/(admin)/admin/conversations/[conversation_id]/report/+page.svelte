<script lang="ts">
	import type { Editor } from '@tiptap/core';
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
	import { scrollBehavior } from '$lib/utils/reducedMotion';
	import { publicResultsByStep, sectionBlocksInDocument } from '$lib/reports/embeds';
	import {
		REPORT_COLUMN_CLASS,
		REPORT_EDITOR_HOVER_CLASS,
		REPORT_EDITOR_PAGE_BREAK_CLASS,
		REPORT_EDITOR_TEXT_CLASS,
		REPORT_EYEBROW_CLASS
	} from '$lib/reports/styles';
	import { useTour } from '$lib/tours/useTour.svelte';
	import ImageControls from '$lib/components/RichTextEditor/ImageControls.svelte';
	import { reportTour } from './tour';
	import ReportTopBar from './ReportTopBar.svelte';
	import ReportTitle from './ReportTitle.svelte';
	import ReportTextEditor from './ReportTextEditor.svelte';
	import ReportToolbar from './ReportToolbar.svelte';
	import PagePills from './PagePills.svelte';
	import PagesPanel from './PagesPanel.svelte';
	import AddChartDialog from './AddChartDialog.svelte';
	import PublishDialog from './PublishDialog.svelte';
	import TranslationsDialog from './TranslationsDialog.svelte';
	import ImpactsAndNotes from './ImpactsAndNotes.svelte';
	import { isUnwritten, reportPages, type ReportPage } from './reportDocument';
	import { KeepPageBreaksOut } from './pageBreaks';

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

	// A save puts the text in its source straight away, so these follow typing too. The API fills
	// a new report with a placeholder sentence; the editor opens that as empty.
	const savedSummary = $derived(summarySource.contents[primaryLocale] ?? '');
	const savedBody = $derived(bodySource.contents[primaryLocale] ?? '');
	const summaryOnScreen = $derived(isUnwritten(savedSummary) ? '' : savedSummary);
	const bodyOnScreen = $derived(isUnwritten(savedBody) ? '' : savedBody);

	let published = $derived(report.isPublic);
	let summaryEditor = $state<Editor>();
	let bodyEditor = $state<Editor>();
	let activeEditor = $state<Editor>();
	let addingChart = $state(false);
	let scroller = $state<HTMLElement>();
	// Readers never see an empty summary, so it shows only to someone who can fill it.
	const hasSummary = $derived(!isUnwritten(summaryOnScreen));
	const showSummary = $derived(canEdit || hasSummary);
	// -1 is the summary, above the first page.
	let currentPageIndex = $state(-1);
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
	const embedded = $derived([summaryOnScreen, bodyOnScreen].flatMap(sectionBlocksInDocument));
	const pages = $derived(
		reportPages(bodyOnScreen, (pageNumber) => `Page ${pageNumber}`, 'Untitled page')
	);
	const saveState = $derived(
		[summarySource.saveState, bodySource.saveState].find((state) => state !== 'idle') ?? 'idle'
	);

	/** The document position just before the body's top-level node at `index`, clamped to the end. */
	function positionBefore(editor: Editor, index: number): number {
		const document = editor.state.doc;
		let position = 0;
		for (let i = 0; i < Math.min(index, document.childCount); i++) {
			position += document.child(i).nodeSize;
		}
		return position;
	}

	function pageElement(editor: Editor, reportPage: ReportPage): HTMLElement | undefined {
		if (reportPage.firstNodeIndex >= editor.state.doc.childCount) return undefined;
		const element = editor.view.nodeDOM(positionBefore(editor, reportPage.firstNodeIndex));
		return element instanceof HTMLElement ? element : undefined;
	}

	// Counts as the end of the report, so the last page is current even when it's too short to
	// reach the top.
	const AT_END_TOLERANCE_PX = 4;
	// A page is current once its top passes this far below the top of the report.
	const CURRENT_PAGE_LINE_PX = 96;

	/** The last page whose top has scrolled past the top of the report, or the last page at the end. */
	function followScroll() {
		if (!bodyEditor || !scroller) return;
		const atEnd =
			scroller.scrollTop + scroller.clientHeight >=
			scroller.scrollHeight - AT_END_TOLERANCE_PX;
		if (atEnd) {
			currentPageIndex = pages.length - 1;
			return;
		}
		const line = scroller.getBoundingClientRect().top + CURRENT_PAGE_LINE_PX;
		const editor = bodyEditor;
		const passed = pages.findLastIndex(
			(reportPage) =>
				(pageElement(editor, reportPage)?.getBoundingClientRect().top ?? 0) <= line
		);
		currentPageIndex = showSummary ? passed : Math.max(passed, 0);
	}

	function jumpToSummary() {
		scroller?.scrollTo({ top: 0, behavior: scrollBehavior() });
	}

	/** Reads the page from the editor itself, which can be a keystroke ahead of `pages`. */
	function jumpToPage(reportPage: ReportPage) {
		if (!bodyEditor) return;
		const livePages = reportPages(JSON.stringify(bodyEditor.getJSON()), String, '');
		const livePage = livePages[pages.indexOf(reportPage)] ?? reportPage;
		pageElement(bodyEditor, livePage)?.scrollIntoView({
			behavior: scrollBehavior(),
			block: 'start'
		});
	}

	function addPage() {
		if (!bodyEditor) return;
		// An empty report is already one empty page, so a break would list a second.
		if (pages.length === 1 && pages[0].empty) {
			bodyEditor.chain().focus('end').scrollIntoView().run();
			currentPageIndex = 0;
			return;
		}
		bodyEditor.chain().focus('end').setHorizontalRule().scrollIntoView().run();
		// The new page is the last one, one past the pages listed before it was added.
		currentPageIndex = pages.length;
	}

	// A delete rather than replacing the document, so Ctrl+Z brings the page back.
	function removePage(reportPage: ReportPage) {
		if (!bodyEditor) return;
		const { firstNodeIndex, nodeCount } = reportPage.removal;
		const from = positionBefore(bodyEditor, firstNodeIndex);
		const to = positionBefore(bodyEditor, firstNodeIndex + nodeCount);
		bodyEditor.chain().focus().deleteRange({ from, to }).run();
	}

	async function setPublished(isPublic: boolean) {
		// Publish what's on screen, including typing the autosave hasn't sent yet.
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

	{#if canEdit}
		<!-- Centred over the report rather than flush with the page edge, so the tools sit above
			the text they change. -->
		<div class="border-b">
			<div class="mx-auto w-fit max-w-full px-5 py-2">
				<ReportToolbar
					editor={activeEditor ?? bodyEditor}
					inBody={(activeEditor ?? bodyEditor) === bodyEditor}
					onAddChart={() => (addingChart = true)}
				/>
			</div>
		</div>
	{/if}

	<div
		bind:this={scroller}
		onscroll={followScroll}
		class="grid min-h-0 grow grid-cols-1 overflow-y-auto xl:grid-cols-[16rem_minmax(0,1fr)]"
	>
		<!-- Wide screens list the pages beside the report, like tabs in a document editor. -->
		<div class="sticky top-0 hidden self-start py-8 ps-4 xl:block">
			<PagesPanel
				{pages}
				currentIndex={Math.min(currentPageIndex, pages.length - 1)}
				editable={canEdit}
				onJumpToSummary={showSummary ? jumpToSummary : undefined}
				onJump={jumpToPage}
				onAdd={addPage}
				onRemove={removePage}
			/>
		</div>

		<div class={cn(REPORT_COLUMN_CLASS, 'flex flex-col gap-5 py-8')}>
			<ReportTitle title={conversation.title} />

			{#key editorsRevision}
				{#if showSummary}
					<!-- Readers get the summary on its own opening screen, so a rule ends it here. -->
					<section class="border-b pb-8">
						<h2 class={REPORT_EYEBROW_CLASS}>Executive summary</h2>
						<div
							class={cn(
								'pt-2',
								REPORT_EDITOR_TEXT_CLASS,
								canEdit && REPORT_EDITOR_HOVER_CLASS
							)}
						>
							<ReportTextEditor
								initialValue={summaryOnScreen}
								editable={canEdit}
								label="Executive summary"
								placeholder="A few sentences on what this conversation found. Readers see this first."
								extensions={[KeepPageBreaksOut]}
								onChange={(json) => summarySource.saveSource(json)}
								onFocus={(editor) => (activeEditor = editor)}
								bind:editor={summaryEditor}
							/>
						</div>
						{#if canEdit}
							<ImageControls editor={summaryEditor} />
						{/if}
					</section>
				{/if}

				<PagePills
					{pages}
					editable={canEdit}
					onJump={jumpToPage}
					onRemove={removePage}
					class="bg-background/95 sticky top-0 z-10 -mx-5 px-5 py-3 backdrop-blur xl:hidden"
				/>

				<div
					class={cn(
						REPORT_EDITOR_TEXT_CLASS,
						REPORT_EDITOR_PAGE_BREAK_CLASS,
						canEdit && REPORT_EDITOR_HOVER_CLASS
					)}
				>
					<ReportTextEditor
						initialValue={bodyOnScreen}
						editable={canEdit}
						label="Report"
						placeholder="Start writing, or add a chart from the toolbar."
						onChange={(json) => bodySource.saveSource(json)}
						onFocus={(editor) => (activeEditor = editor)}
						bind:editor={bodyEditor}
					/>
				</div>
				{#if canEdit}
					<ImageControls editor={bodyEditor} />
				{/if}
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
		{embedded}
		{published}
		onPick={(step, block, item) =>
			(activeEditor ?? bodyEditor)
				?.chain()
				.focus()
				.setReportComponentEmbed({
					toolStepId: step.id,
					componentType: block.type,
					config: item ? { item } : {}
				})
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
			// Both editors remount, so the one last focused no longer exists.
			activeEditor = undefined;
			editorsRevision += 1;
		}}
	/>
{/if}

<PublishDialog
	bind:open={reviewing}
	{published}
	{hasSummary}
	results={publicResults}
	steps={data.reportEmbedSteps}
	canPublish={canEdit}
	onPublish={() => setPublished(true)}
/>

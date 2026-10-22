<!--
	@component Generic theme/statement explorer. Harvests a tool's free-text data into
	`statement` rows, runs the sensemaker model over them, and renders the resulting
	themes (each expandable to its classified statements). Same job-polling shape as
	PolisSummaryPanel: POST .../theme_collections/generate returns a job id, polled via
	GET /jobs/{job_id} until completion, then the theme collection is fetched directly.
	`initialThemeCollection` (from GET .../theme_collections/latest, loaded by the page)
	seeds the display so a previously generated collection survives a page refresh.
-->
<script lang="ts">
	import type { ThemeCollectionDto } from '@crownshy/api-client/api';
	import { apiClient } from '@crownshy/api-client/client';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import { notifications } from '$lib/notifications.svelte';
	import { LoadingButton, Button } from '$lib/components/ui/button';
	import { Textarea } from '$lib/components/ui/textarea';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import { Badge } from '$lib/components/ui/badge';
	import * as Accordion from '$lib/components/ui/accordion';
	import * as Dialog from '$lib/components/ui/dialog';
	import ThemeTag from './ThemeTag.svelte';
	import { Sparkles, MessagesSquare, Plus, Pin, PinOff, Trash2 } from '@lucide/svelte';

	import * as Card from '$lib/components/ui/card';

	type Props = {
		conversationId: string;
		workflowId: string;
		workflowStepId: string;
		/** Which tool to harvest statements from. */
		sourceTool: 'heyform' | 'polis' | 'thinking_space' | 'prioritization';
		/** Narrows prioritization to one proposal/section; ignored by other tools. */
		subScope?: string;
		/** Seeds the context box; falls back to a generic "submitted through <tool>" line. */
		defaultContext?: string;
		/** The most recently generated theme collection, loaded server-side, so themes
		 * survive a page refresh instead of only appearing after a generation run in
		 * this session. */
		initialThemeCollection?: ThemeCollectionDto | null;
		/**
		 * Called whenever a theme collection loads (including on mount, from
		 * `initialThemeCollection`), with each statement's theme title keyed by its
		 * `sourceSubId` (e.g. the Polis statement id), so the parent can overlay theme
		 * tags onto its own statement display.
		 */
		onThemesLoaded?: (themeTitleBySourceSubId: Record<string, string>) => void;
	};

	let {
		conversationId,
		workflowId,
		workflowStepId,
		sourceTool,
		subScope,
		defaultContext,
		initialThemeCollection,
		onThemesLoaded
	}: Props = $props();

	let context = $state(defaultContext ?? `Statements submitted through ${sourceTool}`);
	let collection = $state<ThemeCollectionDto | null>(initialThemeCollection ?? null);
	let pendingThemeCollectionId = $state<string | null>(null);

	let requesting = $state(false);
	let jobId = $state<string | null>(null);
	let jobStatus = $state<string | null>(null);

	let generateDialogOpen = $state(false);
	let addThemeDialogOpen = $state(false);
	let newTitle = $state('');
	let newDescription = $state('');
	let addingTheme = $state(false);

	const isWorking = $derived(requesting || (jobId !== null && jobStatus !== 'completed'));

	const pathParams = $derived({
		conversation_id: conversationId,
		workflow_id: workflowId,
		workflow_step_id: workflowStepId
	});

	function notifyThemesLoaded(current: ThemeCollectionDto) {
		const themeTitleBySourceSubId: Record<string, string> = {};
		for (const theme of current.themes) {
			for (const statement of theme.statements) {
				if (statement.sourceSubId)
					themeTitleBySourceSubId[statement.sourceSubId] = theme.title;
			}
		}
		onThemesLoaded?.(themeTitleBySourceSubId);
	}

	/** Gets the step's theme collection, creating it (manual, empty) if none exists
	 * yet - so an organizer can add themes before any generation has run. */
	async function ensureCollection(): Promise<ThemeCollectionDto | null> {
		if (collection) return collection;

		const res = await tryCatchAsync(() =>
			apiClient.CreateThemeCollection(
				{ context, originTool: sourceTool, originSubId: subScope ?? null },
				{ params: pathParams }
			)
		);

		if (res.err !== null) {
			console.error('CreateThemeCollection failed', res.err);
			notifications.send({ priority: 'ERROR', message: 'Failed to create theme collection' });
			return null;
		}

		collection = res.ok;
		return res.ok;
	}

	async function addTheme() {
		const title = newTitle.trim();
		const description = newDescription.trim();
		if (!title || !description) return;

		addingTheme = true;
		const target = await ensureCollection();
		if (!target) {
			addingTheme = false;
			return;
		}

		const res = await tryCatchAsync(() =>
			apiClient.AddTheme(
				{ title, description, createdByAi: false },
				{ params: { ...pathParams, theme_collection_id: target.id } }
			)
		);
		addingTheme = false;

		if (res.err !== null) {
			console.error('AddTheme failed', res.err);
			notifications.send({ priority: 'ERROR', message: 'Failed to add theme' });
			return;
		}

		collection = { ...target, themes: [...target.themes, res.ok] };
		newTitle = '';
		newDescription = '';
		addThemeDialogOpen = false;
	}

	async function togglePinned(theme: ThemeCollectionDto['themes'][number]) {
		if (!collection) return;

		const res = await tryCatchAsync(() =>
			apiClient.SetThemePinned(
				{ pinned: !theme.pinned },
				{
					params: {
						...pathParams,
						theme_collection_id: collection!.id,
						theme_id: theme.id
					}
				}
			)
		);

		if (res.err !== null) {
			console.error('SetThemePinned failed', res.err);
			notifications.send({ priority: 'ERROR', message: 'Failed to update pin' });
			return;
		}

		// SetThemePinned's response has an empty `statements` array (that conversion
		// is shared with other routes that don't need them) - merge just the
		// `pinned` flag onto the existing theme rather than replacing the whole
		// object, so its statements aren't wiped from the display.
		collection = {
			...collection,
			themes: collection.themes.map((t) =>
				t.id === theme.id ? { ...t, pinned: res.ok.pinned } : t
			)
		};
	}

	async function removeTheme(theme: ThemeCollectionDto['themes'][number]) {
		if (!collection) return;

		const res = await tryCatchAsync(() =>
			apiClient.DeleteTheme(undefined, {
				params: { ...pathParams, theme_collection_id: collection!.id, theme_id: theme.id }
			})
		);

		if (res.err !== null) {
			console.error('DeleteTheme failed', res.err);
			notifications.send({ priority: 'ERROR', message: 'Failed to delete theme' });
			return;
		}

		collection = { ...collection, themes: collection.themes.filter((t) => t.id !== theme.id) };
		notifyThemesLoaded(collection);
	}

	async function requestGenerate() {
		requesting = true;

		const res = await tryCatchAsync(() =>
			apiClient.GenerateThemeCollection(
				{ sourceTool, subScope: subScope ?? null, context, additionalInstructions: null },
				{ params: pathParams }
			)
		);

		requesting = false;

		if (res.err !== null) {
			console.error('GenerateThemeCollection failed', res.err);
			notifications.send({ priority: 'ERROR', message: 'Failed to start theme generation' });
			return;
		}

		jobId = res.ok.job_id;
		jobStatus = 'pending';
		pendingThemeCollectionId = res.ok.theme_collection_id;
		generateDialogOpen = false;
	}

	async function poll() {
		if (!jobId) return;

		const res = await tryCatchAsync(() => apiClient.GetJob({ params: { job_id: jobId! } }));

		if (res.err !== null) {
			console.error('GetJob failed', res.err);
			return;
		}

		jobStatus = res.ok.status ?? null;

		if (jobStatus === 'completed') {
			jobId = null;
			await loadCollection();
			return;
		}

		if (jobStatus === 'error') {
			notifications.send({
				priority: 'ERROR',
				message: res.ok.error ?? 'Theme generation failed'
			});
			jobId = null;
		}
	}

	async function loadCollection() {
		if (!pendingThemeCollectionId) return;

		const res = await tryCatchAsync(() =>
			apiClient.ListThemeCollectionThemes({
				params: { ...pathParams, theme_collection_id: pendingThemeCollectionId! }
			})
		);

		if (res.err !== null) {
			console.error('ListThemeCollectionThemes failed', res.err);
			notifications.send({ priority: 'ERROR', message: 'Failed to load generated themes' });
			return;
		}

		collection = res.ok;
		notifyThemesLoaded(collection);
	}

	$effect(() => {
		if (!jobId || jobStatus === 'completed' || jobStatus === 'error') return;
		const timeout = setTimeout(poll, 5000);
		return () => clearTimeout(timeout);
	});
</script>

<Card.Root class="gap-0 rounded-md p-0 shadow-none">
	<header class="flex flex-col gap-0.5 px-4 pt-3.5">
		<div class="flex flex-row gap-x-2">
			<h2 class="text-foreground grow text-lg font-bold">Themes</h2>

			<Dialog.Root bind:open={generateDialogOpen}>
				<Dialog.Trigger>
					<LoadingButton size="sm" loading={isWorking}>
						<Sparkles class="size-4" />
						{isWorking
							? `Generating themes${jobStatus ? ` (${jobStatus})` : ''}…`
							: collection
								? 'Regenerate themes'
								: 'Generate themes'}
					</LoadingButton>
				</Dialog.Trigger>
				<Dialog.Content>
					<Dialog.Header>
						<Dialog.Title
							>{collection ? 'Regenerate themes' : 'Generate themes'}</Dialog.Title
						>
						<Dialog.Description>
							Tells the model what these statements are, so it can group them into
							relevant themes.
						</Dialog.Description>
					</Dialog.Header>
					<div class="flex flex-col gap-1.5 py-2">
						<Label for="theme-explorer-context">Context for the model</Label>
						<Textarea
							id="theme-explorer-context"
							bind:value={context}
							rows={3}
							placeholder="What are these statements from? (shown to the model as context)"
						/>
					</div>
					<Dialog.Footer>
						<Button variant="outline" onclick={() => (generateDialogOpen = false)}>
							Cancel
						</Button>
						<LoadingButton loading={isWorking} onclick={requestGenerate}>
							<Sparkles class="size-4" />
							{collection ? 'Regenerate themes' : 'Generate themes'}
						</LoadingButton>
					</Dialog.Footer>
				</Dialog.Content>
			</Dialog.Root>

			<!-- Manual themes are always carried into the next generation run as "existing
		 themes" (pinned AI themes too), so the model skips statements that already
		 fit one instead of inventing a duplicate. -->
			<div>
				<Dialog.Root bind:open={addThemeDialogOpen}>
					<Dialog.Trigger>
						<Button size="sm" variant="outline">
							<Plus class="size-4" />
							Add theme
						</Button>
					</Dialog.Trigger>
					<Dialog.Content>
						<Dialog.Header>
							<Dialog.Title>Add a theme</Dialog.Title>
							<Dialog.Description>
								Manually added themes are always carried into the next generation
								run, so the model reuses them instead of inventing a duplicate.
							</Dialog.Description>
						</Dialog.Header>
						<div class="flex flex-col gap-4 py-2">
							<div class="flex flex-col gap-1.5">
								<Label for="theme-explorer-new-title">Title</Label>
								<Input
									id="theme-explorer-new-title"
									bind:value={newTitle}
									placeholder="Title"
								/>
							</div>
							<div class="flex flex-col gap-1.5">
								<Label for="theme-explorer-new-description">Description</Label>
								<Textarea
									id="theme-explorer-new-description"
									bind:value={newDescription}
									rows={3}
									placeholder="Description"
								/>
							</div>
						</div>
						<Dialog.Footer>
							<Button variant="outline" onclick={() => (addThemeDialogOpen = false)}>
								Cancel
							</Button>
							<LoadingButton
								loading={addingTheme}
								disabled={!newTitle.trim() || !newDescription.trim()}
								onclick={addTheme}
							>
								Add theme
							</LoadingButton>
						</Dialog.Footer>
					</Dialog.Content>
				</Dialog.Root>
			</div>
		</div>
		<p class="text-muted-foreground text-sm font-medium">
			Group statements into themes, then click a theme to see all of its statements. <br />You
			can also add your own themes manually and pin existing ones to ensure they are included
			when new themes are generated
		</p>
	</header>

	{#if collection && collection.themes.length > 0}
		<Accordion.Root type="multiple" class="w-full space-y-2">
			{#each collection.themes as theme (theme.id)}
				<Accordion.Item value={theme.id} class="w-full border-b-0">
					<div class="flex w-full items-center gap-1">
						<Accordion.Trigger class="w-full flex-1 px-4 py-3 hover:no-underline">
							<div class="flex w-full flex-1 flex-col items-start gap-1 text-left">
								<div class="flex w-full flex-row items-center">
									<span class=" grow text-base font-semibold"
										>{theme.title}

										<Badge variant="secondary">{theme.statements.length}</Badge>
									</span>
									<Button
										variant="ghost"
										size="icon"
										title={theme.pinned ? 'Unpin theme' : 'Pin theme'}
										onclick={() => togglePinned(theme)}
									>
										{#if theme.pinned}
											<Pin class="size-4 fill-current" />
										{:else}
											<PinOff class="size-4" />
										{/if}
									</Button>
									<Button
										variant="ghost"
										size="icon"
										title="Delete theme"
										onclick={() => removeTheme(theme)}
									>
										<Trash2 class="size-4" />
									</Button>
								</div>
								<span class="text-muted-foreground text-sm"
									>{theme.description}</span
								>
							</div>
						</Accordion.Trigger>
					</div>
					<Accordion.Content class="flex flex-col gap-2 px-4 py-2">
						{#each theme.statements as statement (statement.id)}
							<div class="bg-card flex flex-col gap-1 rounded border p-3 text-sm">
								<ThemeTag label={theme.title} />
								<p>{statement.content}</p>
								<p class="text-muted-foreground mt-1 text-xs">
									{statement.sourceTool}{statement.sourceSubId
										? ` · ${statement.sourceSubId}`
										: ''}
								</p>
							</div>
						{/each}
					</Accordion.Content>
				</Accordion.Item>
			{/each}
		</Accordion.Root>
	{:else if collection}
		<p class="text-muted-foreground text-center text-sm">No themes yet.</p>
	{:else if !isWorking}
		<div
			class="border-border bg-card text-muted-foreground flex flex-col items-center justify-center gap-2 rounded-xl border border-dashed p-8 text-center"
		>
			<MessagesSquare class="text-muted-foreground/60 size-6" />
			<p class="text-sm">Generate themes to explore the statements grouped under them.</p>
		</div>
	{/if}
</Card.Root>

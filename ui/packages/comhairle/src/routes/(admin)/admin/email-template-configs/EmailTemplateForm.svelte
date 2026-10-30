<script lang="ts">
	import { untrack, type Snippet } from 'svelte';
	import { useDebounce } from 'runed';
	import { apiClient } from '@crownshy/api-client/client';
	import type { EmailTypeSchema } from '@crownshy/api-client/api';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import RichTextEditor from '$lib/components/RichTextEditor/RichTextEditor.svelte';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import {
		buildSlotPayload,
		fillKnownVariables,
		applyPreviewConversation,
		fillSlotVariables,
		markVariables,
		revealVariables,
		type EmailTemplateSubmission,
		type KnownVariables,
		type PreviewConversation,
		type SaveStatus,
		type SlotValues
	} from '$lib/utils/emailTemplateForm';
	import type { PreviewMode } from './EmailPreview.svelte';
	import EmailPreview from './EmailPreview.svelte';
	import VariableChips from './VariableChips.svelte';

	type Props = {
		/** Lets a save button outside the form, in the page header, submit it. */
		formId: string;
		schema: EmailTypeSchema;
		initialSubject: string;
		initialSlots: SlotValues;
		/** Bound by the page so the header can show it beside the save button. */
		saveStatus?: SaveStatus;
		/** Real values the preview can use instead of examples, such as the chosen conversation. */
		knownVariables?: KnownVariables;
		/** The conversation this version is for, so the preview shows its title and banner. */
		previewConversation?: PreviewConversation | null;
		/** Saves the email. Resolve to false when it failed so the form can say so. */
		onSubmit: (submission: EmailTemplateSubmission) => Promise<boolean>;
		/** Which email and who it applies to, shown above the fields. */
		settings?: Snippet;
		/** Shown under the save button, such as the remove action. */
		footer?: Snippet;
	};

	let {
		formId,
		schema,
		initialSubject,
		initialSlots,
		saveStatus = $bindable('idle'),
		knownVariables = {},
		previewConversation = null,
		onSubmit,
		settings,
		footer
	}: Props = $props();

	const PREVIEW_DELAY_MS = 400;
	const FIELD_CLASS = 'border-border flex flex-col gap-2 border-t py-6';

	// Read once: the parent keys this form, so a different email remounts it. Plain $state
	// (not derived) so one editor reporting a value never resets the others.
	let subject = $state(untrack(() => initialSubject));
	let slotValues = $state<SlotValues>(untrack(() => ({ ...initialSlots })));
	// The editor reports a value while it normalises the HTML it was loaded with, so only
	// count changes once the admin has typed or clicked in the form.
	let adminHasEdited = false;
	let previewHtml = $state('');
	let previewFailed = $state(false);
	let previewMode = $state<PreviewMode>('sample');

	const refreshPreview = useDebounce(async () => {
		const showVariables = previewMode === 'variables';
		const payload = buildSlotPayload(schema, slotValues);
		const previewSlots = showVariables
			? markVariables(payload)
			: fillSlotVariables(schema, payload, knownVariables);
		const result = await tryCatchAsync(() =>
			apiClient.PreviewEmailTemplateConfig({
				slots: { type: schema.email_type, ...previewSlots }
			})
		);
		previewFailed = result.err !== null;
		if (result.err === null) {
			const html = showVariables ? revealVariables(result.ok.html) : result.ok.html;
			previewHtml = applyPreviewConversation(html, previewConversation);
		}
	}, PREVIEW_DELAY_MS);

	function changePreviewMode(mode: PreviewMode) {
		previewMode = mode;
		refreshPreview();
	}

	let previewSubject = $derived(
		previewMode === 'variables' ? subject : fillKnownVariables(subject, knownVariables)
	);

	// Runs on mount, and again when a conversation is chosen (its title and banner).
	$effect(() => {
		void knownVariables;
		void previewConversation;
		untrack(() => refreshPreview());
	});

	function markChanged() {
		saveStatus = 'unsaved';
		refreshPreview();
	}

	function noteEdit() {
		adminHasEdited = true;
	}

	function setSlot(slotKey: string, value: string) {
		slotValues[slotKey] = value;
		if (adminHasEdited) markChanged();
	}

	async function handleSubmit(event: SubmitEvent) {
		event.preventDefault();
		saveStatus = 'saving';
		const saved = await onSubmit({ subject, slots: buildSlotPayload(schema, slotValues) });
		saveStatus = saved ? 'saved' : 'error';
	}
</script>

<div class="grid items-start gap-8 xl:grid-cols-[minmax(0,1fr)_minmax(22rem,30rem)]">
	<form
		id={formId}
		onsubmit={handleSubmit}
		onkeydowncapture={noteEdit}
		onpointerdowncapture={noteEdit}
		class="flex min-w-0 flex-col"
	>
		{#if settings}
			<div class="flex flex-col gap-4 pb-6">{@render settings()}</div>
		{/if}

		{#if schema.variables.length > 0}
			<div class="pb-6"><VariableChips variables={schema.variables} /></div>
		{/if}

		<div class={FIELD_CLASS}>
			<Label for="email-subject" class="text-base font-semibold">Subject line</Label>
			<Input
				id="email-subject"
				value={subject}
				oninput={(event) => {
					subject = event.currentTarget.value;
					markChanged();
				}}
				class="text-base"
			/>
		</div>

		{#each schema.slots as slot (slot.key)}
			<div class={FIELD_CLASS}>
				{#if slot.content_type === 'plain_text'}
					<Label for="slot-{slot.key}" class="text-base font-semibold">{slot.label}</Label>
					<span class="text-muted-foreground text-sm">{slot.hint}</span>
					<Input
						id="slot-{slot.key}"
						value={slotValues[slot.key]}
						oninput={(event) => setSlot(slot.key, event.currentTarget.value)}
						class="text-base"
					/>
				{:else}
					<span class="text-foreground text-base font-semibold">{slot.label}</span>
					<span class="text-muted-foreground text-sm">{slot.hint}</span>
					<RichTextEditor
						width="100%"
						minHeight="140px"
						value={slotValues[slot.key]}
						onChange={(json) => setSlot(slot.key, json)}
					/>
				{/if}
			</div>
		{/each}

		{@render footer?.()}
	</form>

	<aside class="min-w-0 xl:sticky xl:top-4">
		<EmailPreview
			subject={previewSubject}
			html={previewHtml}
			failed={previewFailed}
			variables={schema.variables}
			knownVariables={Object.keys(knownVariables)}
			mode={previewMode}
			onModeChange={changePreviewMode}
		/>
	</aside>
</div>

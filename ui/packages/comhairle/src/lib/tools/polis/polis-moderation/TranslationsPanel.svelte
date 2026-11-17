<script lang="ts">
	import { Badge } from '$lib/components/ui/badge';
	import { Button, LoadingButton } from '$lib/components/ui/button';
	import { Textarea } from '$lib/components/ui/textarea';
	import { notifications } from '$lib/notifications.svelte';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import { guardUnsavedChanges } from '$lib/utils/unsavedChangesGuard.svelte';
	import { apiClient } from '@crownshy/api-client/client';
	import type { PolisStatementTranslation } from '@crownshy/api-client/api';
	import Check from '@lucide/svelte/icons/check';
	import { languageName, needsReview } from './statementTranslations';

	type Props = {
		translations: PolisStatementTranslation[];
		onChange: (translation: PolisStatementTranslation) => void;
	};

	let { translations, onChange }: Props = $props();

	type SaveState = 'saving' | 'saved' | 'failed';

	// Edits live here until they save, so a slow save never overwrites what is being typed.
	let drafts = $state<Record<string, string>>({});
	let saveStates = $state<Record<string, SaveState>>({});
	let verifying = $state<Record<string, boolean>>({});

	const textOf = (translation: PolisStatementTranslation) =>
		drafts[translation.id] ?? translation.content;

	guardUnsavedChanges(() =>
		translations.some((t) => saveStates[t.id] === 'saving' || saveStates[t.id] === 'failed')
	);

	async function save(translation: PolisStatementTranslation) {
		const content = textOf(translation);
		if (content === translation.content || saveStates[translation.id] === 'saving') return;
		saveStates = { ...saveStates, [translation.id]: 'saving' };

		const res = await tryCatchAsync(() =>
			apiClient.PolisUpdateStatementTranslation(
				{ content },
				{ params: { id: translation.id } }
			)
		);
		if (res.err !== null) {
			saveStates = { ...saveStates, [translation.id]: 'failed' };
			notifications.send({ priority: 'ERROR', message: 'Failed to save translation' });
			return;
		}
		saveStates = { ...saveStates, [translation.id]: 'saved' };
		onChange(res.ok);
	}

	async function verify(translation: PolisStatementTranslation) {
		if (verifying[translation.id]) return;
		verifying = { ...verifying, [translation.id]: true };

		const res = await tryCatchAsync(() =>
			apiClient.PolisVerifyStatementTranslation(undefined, {
				params: { id: translation.id }
			})
		);
		verifying = { ...verifying, [translation.id]: false };

		if (res.err !== null) {
			notifications.send({
				priority: 'ERROR',
				message: 'Failed to mark translation as correct'
			});
			return;
		}
		onChange(res.ok);
	}
</script>

<div class="flex max-w-prose flex-col gap-4 border-l-2 pl-4">
	{#if translations.length === 0}
		<p class="text-muted-foreground text-base">No translations yet.</p>
	{:else}
		{#each translations as translation (translation.id)}
			{@const fieldId = `translation-${translation.id}`}
			{@const saveState = saveStates[translation.id]}
			<div class="flex flex-col gap-2">
				<div class="flex flex-wrap items-center gap-2">
					<label for={fieldId} class="text-base font-medium">
						{languageName(translation.locale)}
					</label>
					{#if needsReview(translation)}
						<Badge variant="secondary">Needs review</Badge>
					{:else}
						<Badge variant="outline">Reviewed</Badge>
					{/if}
					<span class="text-muted-foreground ml-auto text-base" aria-live="polite">
						{#if saveState === 'saving'}
							Saving…
						{:else if saveState === 'saved'}
							Saved
						{/if}
					</span>
				</div>
				<Textarea
					id={fieldId}
					value={textOf(translation)}
					oninput={(e) =>
						(drafts = { ...drafts, [translation.id]: e.currentTarget.value })}
					onblur={() => save(translation)}
					class="min-h-11 md:text-base"
				/>
				{#if saveState === 'failed'}
					<div class="flex items-center gap-2">
						<p class="text-destructive text-base">Couldn't save this translation.</p>
						<Button variant="outline" size="sm" onclick={() => save(translation)}>
							Retry
						</Button>
					</div>
				{:else if needsReview(translation)}
					<div>
						<LoadingButton
							size="sm"
							variant="outline"
							loading={!!verifying[translation.id]}
							onclick={() => verify(translation)}
						>
							<Check class="size-4" />
							Mark as correct
						</LoadingButton>
					</div>
				{/if}
			</div>
		{/each}
	{/if}
</div>

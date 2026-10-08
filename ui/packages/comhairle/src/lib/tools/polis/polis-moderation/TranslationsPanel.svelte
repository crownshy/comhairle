<script lang="ts">
	import { Badge } from '$lib/components/ui/badge';
	import { LoadingButton } from '$lib/components/ui/button';
	import { Textarea } from '$lib/components/ui/textarea';
	import { Spinner } from '$lib/components/ui/spinner';
	import { notifications } from '$lib/notifications.svelte';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import { apiClient } from '@crownshy/api-client/client';
	import type { PolisStatementTranslation } from '@crownshy/api-client/api';
	import { getLocale } from '$lib/paraglide/runtime';

	type Props = {
		polisStatementAuxId: string;
	};

	let { polisStatementAuxId }: Props = $props();

	let loading = $state(true);
	let translations = $state<PolisStatementTranslation[]>([]);
	// Local edit buffer, keyed by translation id, so typing doesn't fight the
	// server-synced `translations` array until a save actually lands.
	let drafts = $state<Record<string, string>>({});
	let saving = $state<Record<string, boolean>>({});
	let verifying = $state<Record<string, boolean>>({});

	function localeLabel(locale: string): string {
		try {
			return new Intl.DisplayNames([getLocale()], { type: 'language' }).of(locale) ?? locale;
		} catch {
			return locale;
		}
	}

	async function load() {
		loading = true;
		const res = await tryCatchAsync(() =>
			apiClient.PolisListStatementTranslations({ params: { id: polisStatementAuxId } })
		);
		loading = false;
		if (res.err !== null) {
			console.error('PolisListStatementTranslations failed', res.err);
			notifications.send({ priority: 'ERROR', message: 'Failed to load translations' });
			return;
		}
		translations = res.ok;
		drafts = Object.fromEntries(res.ok.map((t) => [t.id, t.content]));
	}

	load();

	async function save(translation: PolisStatementTranslation) {
		const content = drafts[translation.id];
		if (content === undefined || content === translation.content || saving[translation.id]) {
			return;
		}
		saving = { ...saving, [translation.id]: true };

		const res = await tryCatchAsync(() =>
			apiClient.PolisUpdateStatementTranslation(
				{ content },
				{ params: { id: translation.id } }
			)
		);
		saving = { ...saving, [translation.id]: false };

		if (res.err !== null) {
			console.error('PolisUpdateStatementTranslation failed', res.err);
			notifications.send({ priority: 'ERROR', message: 'Failed to save translation' });
			return;
		}
		translations = translations.map((t) => (t.id === translation.id ? res.ok : t));
		drafts = { ...drafts, [translation.id]: res.ok.content };
		notifications.send({ priority: 'INFO', message: 'Translation saved' });
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
			console.error('PolisVerifyStatementTranslation failed', res.err);
			notifications.send({ priority: 'ERROR', message: 'Failed to verify translation' });
			return;
		}
		translations = translations.map((t) => (t.id === translation.id ? res.ok : t));
		notifications.send({ priority: 'INFO', message: 'Translation verified' });
	}
</script>

<div class="bg-muted/30 flex flex-col gap-3 rounded-lg border p-4">
	{#if loading}
		<div class="flex items-center gap-2 text-sm">
			<Spinner class="size-4" />
			Loading translations…
		</div>
	{:else if translations.length === 0}
		<p class="text-muted-foreground text-sm italic">No translations generated yet.</p>
	{:else}
		{#each translations as translation (translation.id)}
			<div class="flex flex-col gap-2">
				<div class="flex items-center gap-2">
					<span class="text-sm font-medium">{localeLabel(translation.locale)}</span>
					{#if translation.ai_generated && translation.requires_validation}
						<Badge variant="secondary">Needs review</Badge>
						<LoadingButton
							size="sm"
							variant="ghost"
							loading={!!verifying[translation.id]}
							onclick={() => verify(translation)}
						>
							Looks correct
						</LoadingButton>
					{/if}
				</div>
				<Textarea
					value={drafts[translation.id] ?? translation.content}
					oninput={(e) =>
						(drafts = { ...drafts, [translation.id]: e.currentTarget.value })}
					rows={2}
				/>
				<div class="flex justify-end">
					<LoadingButton
						size="sm"
						variant="outline"
						loading={!!saving[translation.id]}
						disabled={(drafts[translation.id] ?? translation.content) ===
							translation.content}
						onclick={() => save(translation)}
					>
						Save
					</LoadingButton>
				</div>
			</div>
		{/each}
	{/if}
</div>

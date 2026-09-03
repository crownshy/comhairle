<!-- src/lib/components/UserDemographicsForm/UserDemographicsForm.svelte -->
<script lang="ts">
	import { untrack } from 'svelte';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import { Button } from '$lib/components/ui/button';
	import { apiClient } from '@crownshy/api-client/client';
	import { notifications } from '$lib/notifications.svelte';
	import { superForm } from 'sveltekit-superforms';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import type { DemographicsQuestion, DemographicsResponse } from '@crownshy/api-client/api';

	type Props = {
		questions: DemographicsQuestion[];
		responses: DemographicsResponse[];
		userId: string;
		onSaved: () => Promise<void>;
	};

	let { questions, responses, userId, onSaved }: Props = $props();

	let saving = $state(false);

	const buildInitialData = () => {
		const initialData: Record<string, string> = {};

		for (const q of questions) {
			// Find existing answer if it exists
			const existingResponse = responses.find((r) => r.questionSlug === q.slug);
			initialData[q.slug] = existingResponse ? existingResponse.value : '';
		}

		return initialData;
	};

	const form = untrack(() => {
		const initialData = buildInitialData();

		return superForm(initialData, {
			validators: false,
			taintedMessage: false,
			validationMethod: 'onsubmit'
		});
	});

	const { form: formData, validateForm, errors } = form;

	// Helper to format slugs like "political_party" into "Political Party"
	const formatSlugLabel = (slug: string) => {
		return slug
			.split('_')
			.map((word) => word.charAt(0).toUpperCase() + word.slice(1))
			.join(' ');
	};

	async function saveDemographics(e: Event) {
		e.preventDefault();
		const result = await validateForm({ update: false });

		if (!result.valid) {
			$errors = result.errors;
			return;
		}

		let apiCalls = await tryCatchAsync(async () => {
			saving = true;

			// 2. Iterate dynamically over the available questions to dispatch API calls
			const apiCalls = questions.map(async (q) => {
				const slug = q.slug;
				const val = result.data[slug];
				const existing = responses.find((r) => r.questionSlug === slug);

				const hasNewValue = val !== null && val !== '' && val !== undefined;
				const stringVal = hasNewValue ? String(val).trim() : '';

				if (hasNewValue && !existing) {
					// Create
					return apiClient.CreateDemographicsResponse({
						questionSlug: slug,
						userId: userId,
						value: stringVal
					});
				} else if (hasNewValue && existing && existing.value !== stringVal) {
					// Update
					return apiClient.UpdateDemographicsResponse(
						{ value: stringVal },
						{ params: { question_slug: slug, user_id: userId } }
					);
				} else if (!hasNewValue && existing) {
					// Delete (user cleared the field)
					return apiClient.DeleteDemographicsResponse(undefined, {
						params: { question_slug: slug, user_id: userId }
					});
				}

				return Promise.resolve();
			});

			await Promise.all(apiCalls);

			notifications.send({
				message: 'Demographics updated successfully',
				priority: 'INFO'
			});
			$errors = {};

			await onSaved();
		});

		if (apiCalls.err !== null) {
			const error = apiCalls.err;
			const errorMessage =
				error instanceof Error
					? error.message
					: 'Failed to update demographics. Please try again.';

			notifications.send({
				message: errorMessage,
				priority: 'ERROR'
			});
		}

		saving = false;
	}
</script>

<div class="space-y-6 border-b py-5">
	<div>
		<h3 class="text-lg font-medium">Demographics</h3>
		<p class="text-muted-foreground text-sm">
			Update your demographic information here. This helps us contextualize conversation
			results.
		</p>
	</div>

	{#if questions.length === 0}
		<p class="text-muted-foreground mt-4 text-sm italic">
			No demographic questions are currently available.
		</p>
	{:else}
		<form method="POST" class="grid gap-4 md:grid-cols-2" onsubmit={saveDemographics}>
			{#each questions as question (question.slug)}
				<div class="space-y-2">
					<!-- Use question.label or question.text if your backend provides it, otherwise format the slug -->
					<Label for={question.slug}>
						{(question as any).label || (question as any).text || question.displayName}
					</Label>

					<Input
						id={question.slug}
						bind:value={$formData[question.slug]}
						placeholder={`Enter ${question.displayName.toLowerCase()}`}
						disabled={saving}
					/>

					{#if $errors[question.slug]}
						<span class="text-destructive block text-xs">{$errors[question.slug]}</span>
					{/if}
				</div>
			{/each}

			<div class="mt-4 flex justify-end md:col-span-2">
				<Button type="submit" disabled={saving}>
					{saving ? 'Saving...' : 'Save Demographics'}
				</Button>
			</div>
		</form>
	{/if}
</div>

<!-- src/lib/components/UserDemographicsForm/UserDemographicsForm.svelte -->
<script lang="ts">
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import { Button } from '$lib/components/ui/button';
	import { apiClient } from '@crownshy/api-client/client';
	import { notifications } from '$lib/notifications.svelte';
	import { superForm } from 'sveltekit-superforms';
	import { zodClient } from 'sveltekit-superforms/adapters';
	import { z } from 'zod';
	import type { DemographicsQuestion, DemographicsResponse } from '@crownshy/api-client/api';
	import { invalidateAll } from '$app/navigation';

	let {
		questions,
		responses,
		userId
	}: {
		questions: DemographicsQuestion[];
		responses: DemographicsResponse[];
		userId: string;
	} = $props();

	let saving = $state(false);

	// 1. Build initial data and dynamic Zod schema based on the provided questions
	const buildFormConfig = () => {
		const initialData: Record<string, string> = {};
		const schemaShape: Record<string, z.ZodTypeAny> = {};

		for (const q of questions) {
			// Find existing answer if it exists
			const existingResponse = responses.find((r) => r.questionSlug === q.slug);
			initialData[q.slug] = existingResponse ? existingResponse.value : '';

			// Require strings but allow empty values to represent deletion
			schemaShape[q.slug] = z.string().nullable().optional();
		}

		return {
			initialData,
			schema: z.object(schemaShape)
		};
	};

	const config = buildFormConfig();

	const form = superForm(config.initialData, {
		validators: zodClient(config.schema),
		taintedMessage: false,
		validationMethod: 'onsubmit'
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

		try {
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
					return apiClient.UpdateDemographicsResponse(slug, userId, {
						value: stringVal
					});
				} else if (!hasNewValue && existing) {
					// Delete (user cleared the field)
					return apiClient.DeleteDemographicsResponse(slug, userId);
				}

				return Promise.resolve();
			});

			await Promise.all(apiCalls);

			notifications.send({
				message: 'Demographics updated successfully',
				priority: 'INFO'
			});
			$errors = {};

			// Re-run the page load function to sync fresh responses into the UI state
			await invalidateAll();
		} catch (error: any) {
			console.error('Demographics Save Error:', error?.response?.data || error); // <-- Add this to see the exact backend rejection

			notifications.send({
				message:
					error?.response?.data?.err ||
					error?.response?.data?.message ||
					'Failed to update demographics. Please try again.',
				priority: 'ERROR'
			});
		} finally {
			saving = false;
		}
	}
</script>

<div class="space-y-6 border-t pt-8">
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
						{(question as any).label ||
							(question as any).text ||
							formatSlugLabel(question.slug)}
					</Label>

					<Input
						id={question.slug}
						bind:value={$formData[question.slug]}
						placeholder={`Enter ${formatSlugLabel(question.slug).toLowerCase()}`}
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

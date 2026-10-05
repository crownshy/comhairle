<script lang="ts">
	import { invalidate } from '$app/navigation';
	import { key } from '$lib/utils/invalidationKey';
	import { page } from '$app/state';
	import Input from '$lib/components/ui/input/input.svelte';
	import Label from '$lib/components/ui/label/label.svelte';
	import Switch from '$lib/components/ui/switch/switch.svelte';
	import { Textarea } from '$lib/components/ui/textarea';
	import { notifications } from '$lib/notifications.svelte';
	import { apiClient } from '@crownshy/api-client/client';
	import { useDebounce } from 'runed';
	import type { InstancedToolConfig } from '$lib/tools/types';

	let {
		toolConfig,
		conversationId,
		workflowId,
		workflowStepId,
		isLive
	}: {
		toolConfig: InstancedToolConfig<'polis'>;
		conversationId: string;
		workflowId: string;
		workflowStepId: string;
		isLive: boolean;
	} = $props();

	const {
		required_votes: requiredVotes,
		show_remaining_statements: showRemaining = true,
		topic = '',
		description = '',
		strict_moderation: strictModeration = false
	} = $derived(toolConfig);

	// Local copies for the text inputs. Writable `$derived`: the user's keystrokes
	// (via bind:value) override these until the load re-runs, at which point they
	// resync to the freshly loaded config.
	let requiredVotesInput = $derived(requiredVotes);
	let topicInput = $derived(topic);
	let descriptionInput = $derived(description);

	// Config keys that must be proxied to Polis (the rest are comhairle-only
	// display flags that live in tool_config).
	const POLIS_KEYS = new Set(['topic', 'description', 'is_active', 'strict_moderation']);

	/**
	 * Persist a single Setup field. Polis-owned fields (see POLIS_KEYS) are
	 * written to the Polis conversation first (enforcement) and then mirrored
	 * into tool_config so the form can pre-fill next time; display-only flags
	 * skip Polis. Order is Polis-then-mirror so a Polis failure doesn't leave a
	 * tool_config value Polis never accepted.
	 */
	async function saveField(field: string, value: unknown) {
		try {
			if (POLIS_KEYS.has(field)) {
				await apiClient.PolisUpdateConfig({
					workflow_step_id: workflowStepId,
					[field]: value
				});
			}
			await apiClient.UpdateConversationWorkflowStep(
				{
					[isLive ? 'tool_config' : 'preview_tool_config']: {
						...toolConfig,
						[field]: value
					}
				},
				{
					params: {
						conversation_id: conversationId,
						workflow_id: workflowId,
						workflow_step_id: workflowStepId
					}
				}
			);
			await invalidate(key('admin/conversation/workflow'));
		} catch (e) {
			console.error(e);
			notifications.send({ priority: 'ERROR', message: 'Failed to update setup' });
		}
	}

	const saveTopic = useDebounce((v: string) => saveField('topic', v), 500);
	const saveDescription = useDebounce((v: string) => saveField('description', v), 500);
	const saveRequiredVotes = useDebounce((raw: string) => {
		const value = Number(raw);
		// Don't persist an empty, invalid, or non-positive number.
		if (raw.trim() === '' || !Number.isFinite(value) || value < 1) return;
		saveField('required_votes', value);
	}, 500);

	// Link back to the Moderation subtab on this same step, preserving the path.
	const moderationHref = $derived.by(() => {
		const url = new URL(page.url);
		url.searchParams.set('subtab', 'moderation');
		return url.pathname + url.search;
	});
</script>

<div class="mb-8 flex max-w-2xl flex-col gap-8">
	<h2 class="text-2xl font-bold">Setup</h2>

	<!-- Content -->
	<div class="flex flex-col gap-4">
		<div class="flex flex-col gap-1">
			<h3 class="text-base font-bold">Content</h3>
			<span class="text-muted-foreground text-sm"
				>This configures the Polis conversation.</span
			>
		</div>

		<div class="flex flex-col gap-1">
			<Label for="topic" class="text-muted-foreground text-xs tracking-tight uppercase"
				>Topic</Label
			>
			<Input
				id="topic"
				bind:value={topicInput}
				placeholder="Conversation topic"
				oninput={(e) => saveTopic((e.currentTarget as HTMLInputElement).value)}
			/>
		</div>

		<div class="flex flex-col gap-1">
			<Label for="description" class="text-muted-foreground text-xs tracking-tight uppercase">
				Description
			</Label>
			<Textarea
				id="description"
				bind:value={descriptionInput}
				rows={3}
				placeholder="What is this conversation about?"
				oninput={(e) => saveDescription((e.currentTarget as HTMLTextAreaElement).value)}
			/>
		</div>
	</div>

	<!-- Settings -->
	<div class="flex flex-col gap-4">
		<div class="flex flex-col gap-1">
			<h3 class="text-base font-bold">Settings</h3>
			<span class="text-muted-foreground text-sm">Customise what participants will see.</span>
		</div>

		<div class="flex flex-col gap-1">
			<Label for="requiredVotes" class="text-sm font-semibold">Required votes</Label>
			<span class="text-muted-foreground mb-1 text-xs">
				Number of votes required before a participant can progress to the next step.
			</span>
			<Input
				id="requiredVotes"
				name="requiredVotes"
				type="number"
				min="1"
				step="1"
				class="w-32"
				bind:value={requiredVotesInput}
				oninput={(e) => saveRequiredVotes((e.currentTarget as HTMLInputElement).value)}
			/>
		</div>

		<div class="flex items-start justify-between gap-4">
			<div class="flex flex-col gap-0.5">
				<Label for="showRemaining" class="text-sm font-medium"
					>Show remaining statements</Label
				>
				<span class="text-muted-foreground text-xs">
					Display the number of remaining statements to participants during voting.
				</span>
			</div>
			<Switch
				id="showRemaining"
				checked={showRemaining}
				onCheckedChange={(checked) => saveField('show_remaining_statements', checked)}
			/>
		</div>

		<div class="flex items-start justify-between gap-4">
			<div class="flex flex-col gap-0.5">
				<Label for="strictModeration" class="text-sm font-medium">
					No comments shown without moderator approval
				</Label>
				<span class="text-muted-foreground text-xs">
					When on, every statement must be accepted or rejected at
					<!-- eslint-disable svelte/no-navigation-without-resolve -->
					<a
						href={moderationHref}
						class="text-primary font-medium underline underline-offset-2">Moderation</a
					>
					<!-- eslint-enable svelte/no-navigation-without-resolve -->
					before participants see it.
				</span>
			</div>
			<Switch
				id="strictModeration"
				checked={strictModeration ?? false}
				onCheckedChange={(checked) => saveField('strict_moderation', checked)}
			/>
		</div>
	</div>
</div>

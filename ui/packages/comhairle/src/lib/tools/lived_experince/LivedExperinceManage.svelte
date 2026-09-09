<script lang="ts">
	import { invalidate } from '$app/navigation';
	import { Button } from '$lib/components/ui/button';
	import {
		Card,
		CardContent,
		CardDescription,
		CardHeader,
		CardTitle
	} from '$lib/components/ui/card';
	import { Checkbox } from '$lib/components/ui/checkbox';
	import { Label } from '$lib/components/ui/label';
	import DraggableList from '$lib/components/DraggableList.svelte';
	import MediaLibraryDialog, {
		addToCache
	} from '$lib/components/Media/MediaLibraryDialog.svelte';
	import MediaUpload from '$lib/components/Media/MediaUpload.svelte';
	import { notifications } from '$lib/notifications.svelte';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import { apiClient } from '@crownshy/api-client/client';
	import type { MediaDto, StoryClip, WorkflowStepsListResponse } from '@crownshy/api-client/api';
	import { GripVertical, Mic, Trash2, Video } from 'lucide-svelte';

	type Props = {
		conversationId: string;
		workflowId: string;
		workflowStep: WorkflowStepsListResponse[number];
		isLive: boolean;
	};

	let { conversationId, workflowId, workflowStep, isLive }: Props = $props();

	/** A chosen clip with a row id of its own, so the same upload can be listed twice. */
	type ClipRow = StoryClip & { id: string };

	let toolConfig = $derived(isLive ? workflowStep.toolConfig : workflowStep.previewToolConfig);

	// Writable $derived: edits replace the list, and a fresh config after save resets it.
	let rows = $derived<ClipRow[]>(
		((toolConfig?.clips ?? []) as StoryClip[]).map((clip) => ({
			...clip,
			id: crypto.randomUUID()
		}))
	);
	let saving = $state(false);

	function addClip(media: MediaDto) {
		const kind = media.contentType.split('/')[0];
		if (kind !== 'video' && kind !== 'audio') {
			notifications.send({
				message: 'Choose a video or audio file.',
				priority: 'ERROR'
			});
			return;
		}
		rows = [
			...rows,
			{ id: crypto.randomUUID(), media_id: media.id, url: media.url, audio: kind === 'audio' }
		];
	}

	function removeClip(id: string) {
		rows = rows.filter((row) => row.id !== id);
	}

	function setAudio(id: string, audio: boolean) {
		rows = rows.map((row) => (row.id === id ? { ...row, audio } : row));
	}

	/** The upload's filename, which the storage key ends with. */
	function clipName(url: string): string {
		return decodeURIComponent(url.slice(url.lastIndexOf('/') + 1));
	}

	async function save() {
		saving = true;
		const config = {
			type: 'stories' as const,
			max_time: (toolConfig?.max_time as number | undefined) ?? 10,
			to_see: (toolConfig?.to_see as number | undefined) ?? 3,
			clips: rows.map(({ media_id, url, audio }): StoryClip => ({ media_id, url, audio }))
		};
		const update = isLive ? { tool_config: config } : { preview_tool_config: config };
		const result = await tryCatchAsync(() =>
			apiClient.UpdateConversationWorkflowStep(update, {
				params: {
					conversation_id: conversationId,
					workflow_id: workflowId,
					workflow_step_id: workflowStep.id
				}
			})
		);
		saving = false;
		if (result.err !== null) {
			notifications.send({
				message: 'Failed to save the clips.',
				priority: 'ERROR'
			});
			return;
		}
		notifications.send({
			message: 'Lived Experience clips saved.',
			priority: 'INFO'
		});
		await invalidate('conversation:workflow');
	}
</script>

<div class="flex flex-col gap-6">
	<Card>
		<CardHeader>
			<CardTitle>Clips</CardTitle>
			<CardDescription class="text-base">
				What participants watch, in order, before they are invited to record their own. Pick
				from the media library or upload a new video or audio file.
			</CardDescription>
		</CardHeader>
		<CardContent class="flex flex-col gap-4">
			<div class="flex flex-wrap gap-4">
				<MediaLibraryDialog onconfirm={addClip} />
				<MediaUpload
					clientSide
					size="sm"
					oncomplete={(media) => {
						addToCache(media);
						addClip(media);
					}}
				/>
			</div>

			{#if rows.length === 0}
				<p class="text-muted-foreground text-base">
					No clips chosen yet. Participants see the placeholder clips until some are.
				</p>
			{:else}
				<DraggableList
					items={rows}
					onReorder={(next) => (rows = next)}
					dragDisabled={saving}
					class="space-y-3"
				>
					{#snippet children(row: ClipRow, index: number)}
						<Card class="bg-card">
							<CardContent class="flex items-center gap-3 p-4">
								<button
									type="button"
									aria-label="Drag to reorder"
									class="text-muted-foreground hover:text-foreground shrink-0 cursor-grab active:cursor-grabbing"
								>
									<GripVertical class="size-5" />
								</button>
								<div
									class="bg-accent text-accent-foreground flex size-10 shrink-0 items-center justify-center rounded-full"
								>
									{#if row.audio}
										<Mic class="size-5" aria-hidden="true" />
									{:else}
										<Video class="size-5" aria-hidden="true" />
									{/if}
								</div>
								<div class="flex min-w-0 grow flex-col gap-1">
									<p class="truncate text-base font-medium" title={row.url}>
										{index + 1}. {clipName(row.url)}
									</p>
									<div class="flex items-center gap-2">
										<Checkbox
											id="audio-{row.id}"
											checked={row.audio}
											onCheckedChange={(checked) =>
												setAudio(row.id, checked === true)}
										/>
										<Label
											for="audio-{row.id}"
											class="text-muted-foreground text-base"
										>
											Audio only
										</Label>
									</div>
								</div>
								<Button
									variant="ghost"
									size="icon"
									aria-label="Remove clip"
									onclick={() => removeClip(row.id)}
								>
									<Trash2 class="size-5" />
								</Button>
							</CardContent>
						</Card>
					{/snippet}
				</DraggableList>
			{/if}

			<div class="flex justify-end">
				<Button onclick={save} disabled={saving}>
					{saving ? 'Saving...' : 'Save clips'}
				</Button>
			</div>
		</CardContent>
	</Card>
</div>

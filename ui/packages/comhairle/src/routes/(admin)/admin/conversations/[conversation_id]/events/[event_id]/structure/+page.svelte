<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import type { EventAgendaItem } from '@crownshy/api-client/api';
	import type { AgendaItemData } from '../agenda-types';
	import AgendaEditor from './AgendaEditor.svelte';
	import { apiClient } from '@crownshy/api-client/client';
	import { notifications } from '$lib/notifications.svelte';
	import { invalidate } from '$app/navigation';
	import { key } from '$lib/utils/invalidationKey';

	const { data } = $props();
	const { conversation, event } = $derived(data);

	let items = $derived<AgendaItemData[]>(apiAgendaToEditor(event.agenda ?? []));
	let dirty = $state(false);
	let saving = $state(false);

	function apiAgendaToEditor(items: EventAgendaItem[]): AgendaItemData[] {
		return items.map((item) => {
			if ('Basic' in item) {
				return {
					id: crypto.randomUUID(),
					type: 'standard',
					title: item.Basic.title
				};
			} else {
				return {
					id: crypto.randomUUID(),
					type: 'breakout',
					title: '',
					duration: item.BreakoutRoom.estimated_time,
					groupSize: item.BreakoutRoom.max_per_room ?? 4,
					prompts: [
						{
							title: item.BreakoutRoom.prompt,
							instructions: item.BreakoutRoom.instructions
						}
					],
					assignmentMode: 'random',
					balanceBy: []
				};
			}
		});
	}

	function editorAgendaToApi(items: AgendaItemData[]): EventAgendaItem[] {
		return items.map((item) => {
			if (item.type === 'standard') {
				return {
					Basic: {
						title: item.title || '',
						description: '',
						estimated_time: 0
					}
				};
			} else {
				const firstPrompt = item.prompts?.[0];
				return {
					BreakoutRoom: {
						prompt: firstPrompt?.title || '',
						instructions: firstPrompt?.instructions || '',
						estimated_time: item.duration ?? 10,
						time_limit: item.duration ? item.duration * 60 : null,
						max_per_room: item.groupSize ?? null
					}
				};
			}
		});
	}

	function handleAgendaUpdate(newItems: AgendaItemData[]) {
		items = newItems;
		dirty = true;
	}

	async function handleSaveAgenda() {
		saving = true;
		try {
			await apiClient.UpdateEvent(
				{ agenda: editorAgendaToApi(items) },
				{
					params: {
						conversation_id: conversation.id,
						event_id: event.id
					}
				}
			);
			await invalidate(key('conversation/event'));
			dirty = false;
			notifications.send({ message: 'Agenda saved', priority: 'INFO' });
		} catch (e) {
			console.error(e);
			notifications.send({ message: 'Failed to save agenda', priority: 'ERROR' });
		} finally {
			saving = false;
		}
	}
</script>

<div class="flex flex-col gap-10 py-6">
	<div class="flex flex-col gap-2">
		<h2 class="text-3xl font-bold">
			Event structure <span class="font-bold">(for facilitator)</span>
		</h2>
		<p class="text-muted-foreground text-base">Plan how your meeting will run</p>
	</div>

	<AgendaEditor bind:items onUpdate={handleAgendaUpdate} />

	<div class="border-border flex justify-center border-t py-6">
		<Button
			variant="default"
			class="px-12"
			disabled={!dirty || saving}
			onclick={handleSaveAgenda}
		>
			{saving ? 'Saving...' : 'Save Agenda'}
		</Button>
	</div>
</div>

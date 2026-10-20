<script lang="ts">
	import { page } from '$app/state';
	import { CalendarDays, Plus } from 'lucide-svelte';
	import TabStripShell from '$lib/components/TabStripShell.svelte';
	import { sectionTabClass } from './sectionTabClass';
	import type { LocalizedEventDto } from '@crownshy/api-client/api';

	let {
		conversationId,
		events
	}: {
		conversationId: string;
		events: LocalizedEventDto[];
	} = $props();

	let basePath = $derived(`/admin/conversations/${conversationId}/events`);
	let manageActive = $derived(page.url.pathname === basePath);
	let newPath = $derived(`${basePath}/new`);

	function isEventActive(eventId: string, currentPath: string): boolean {
		return currentPath.startsWith(`${basePath}/${eventId}`);
	}
</script>

<TabStripShell ariaLabel="Events">
	<li>
		<a
			href={basePath}
			class={sectionTabClass(manageActive)}
			aria-current={manageActive ? 'page' : undefined}
		>
			<CalendarDays class="size-4" />
			All events
		</a>
	</li>
	{#each events as event (event.id)}
		{@const active = isEventActive(event.id, page.url.pathname)}
		<li>
			<a
				href={`${basePath}/${event.id}`}
				title={event.name || 'Unnamed event'}
				class="{sectionTabClass(active)} max-w-[220px]"
				aria-current={active ? 'page' : undefined}
			>
				<span class="truncate">{event.name || 'Unnamed event'}</span>
			</a>
		</li>
	{/each}
	<li>
		<a href={newPath} class={sectionTabClass(false)}>
			<Plus class="size-4" />
			Add event
		</a>
	</li>
</TabStripShell>

<script lang="ts">
	import { page } from '$app/state';
	import { CalendarDays, Plus } from 'lucide-svelte';
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

<nav
	class="border-border bg-muted/50 scrollbar-none w-full overflow-x-auto border-b"
	aria-label="Events"
>
	<ul class="flex min-w-max items-center gap-1.5 px-5 py-1">
		<li>
			<a
				href={basePath}
				class="text-foreground inline-flex h-9 items-center gap-1.5 px-3.5 text-sm font-medium whitespace-nowrap transition-opacity"
				class:text-primary={manageActive}
				class:opacity-70={!manageActive}
				class:hover:opacity-100={!manageActive}
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
					class="text-foreground inline-flex h-9 max-w-[220px] items-center px-3.5 text-sm font-medium transition-opacity"
					class:text-primary={active}
					class:opacity-70={!active}
					class:hover:opacity-100={!active}
					aria-current={active ? 'page' : undefined}
				>
					<span class="truncate">{event.name || 'Unnamed event'}</span>
				</a>
			</li>
		{/each}
		<li>
			<a
				href={newPath}
				class="text-foreground/40 hover:text-foreground inline-flex h-9 items-center gap-1 px-3.5 text-sm font-medium whitespace-nowrap"
			>
				<Plus class="size-4" />
				Add event
			</a>
		</li>
	</ul>
</nav>

<style>
	.scrollbar-none {
		scrollbar-width: none;
	}
	.scrollbar-none::-webkit-scrollbar {
		display: none;
	}
</style>

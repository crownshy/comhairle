<script lang="ts">
	import { page } from '$app/state';
	import { conversationSections } from '$lib/config/conversation-steps';

	let {
		conversationId,
		conversationIsLive
	}: { conversationId: string; conversationIsLive: boolean } = $props();

	let basePath = $derived(`/admin/conversations/${conversationId}`);

	function isActive(sectionPath: string, currentPath: string): boolean {
		const sectionUrl = `${basePath}/${sectionPath}`;
		return currentPath === sectionUrl || currentPath.startsWith(`${sectionUrl}/`);
	}
</script>

<nav
	class="border-border bg-background scrollbar-none flex w-full overflow-x-auto border-b"
	aria-label="Conversation sections"
>
	<!-- First tab bleeds left into the gutter ([&>li:first-child]:-ml-4 cancels the
		 link's px-4) so its label aligns to the shared gutter column. -->
	<ul class="pl-gutter flex min-w-full items-center pr-5 [&>li:first-child]:-ml-4">
		{#each conversationSections as section (section.path)}
			{@const active = isActive(section.path, page.url.pathname)}
			{@const disabled = section.requiresLive && !conversationIsLive}
			<li class="shrink-0">
				{#if disabled}
					<span
						class="text-foreground relative flex h-11 cursor-not-allowed items-center px-4 text-sm font-medium whitespace-nowrap opacity-30"
						aria-disabled="true"
						title="Available after launch"
					>
						{section.name}
					</span>
				{:else}
					<a
						href={`${basePath}/${section.path}`}
						class="text-foreground relative flex h-11 items-center px-4 text-sm font-medium whitespace-nowrap transition-opacity"
						class:opacity-50={!active}
						class:hover:opacity-100={!active}
						aria-current={active ? 'page' : undefined}
					>
						{section.name}
						{#if active}
							<span
								class="border-primary absolute right-0 bottom-0 left-0 border-b-[3px]"
								aria-hidden="true"
							></span>
						{/if}
					</a>
				{/if}
			</li>
		{/each}
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

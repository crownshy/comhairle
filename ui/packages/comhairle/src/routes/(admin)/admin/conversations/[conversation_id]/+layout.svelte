<script lang="ts">
	import { resolve } from '$app/paths';
	import { page } from '$app/state';
	import { getTextInLocale } from '$lib/components/Translation/translationUtils';
	import { snakeToSentenceCase } from '$lib/utils/casingUtils';
	import { DEFAULT_LOCALE } from '$lib/utils/constants';
	import ConversationActions from './ConversationActions.svelte';

	const { data, params, children } = $props();

	console.log(page.route.id?.split('conversations/[conversation_id]/'));

	let conversation = $derived(data.conversation);
	let displayTitle = $derived(
		getTextInLocale(
			conversation.translations?.title,
			conversation.primaryLocale ?? DEFAULT_LOCALE,
			conversation.title
		) || conversation.title
	);

	const paths = [
		'configure',
		'design',
		'learning-assistant',
		'events',
		'invites',
		'monitor',
		'notifications',
		'report'
	] as const;

	type Path = (typeof paths)[number];

	function resolveLabel(path: Path): string {
		switch (path) {
			case 'design':
				return 'Process Design';
			case 'invites':
				return 'Recruit';
			case 'notifications':
				return 'Notify';
			case 'configure':
			case 'learning-assistant':
			case 'events':
			case 'monitor':
			case 'report':
				return snakeToSentenceCase(path);
		}
	}

	function needsLiveConversation(path: Path): boolean {
		switch (path) {
			case 'configure':
			case 'design':
			case 'learning-assistant':
			case 'events':
				return false;
			case 'invites':
			case 'monitor':
			case 'notifications':
			case 'report':
				return true;
		}
	}
</script>

<div
	class="border-border bg-background md:pl-gutter flex w-full items-center justify-between border-b py-2 pr-3 pl-14 md:pr-6"
>
	<h1 class="text-primary max-w-[22ch] truncate text-lg leading-7 font-semibold sm:max-w-[40ch]">
		{displayTitle}
	</h1>

	<ConversationActions {conversation} />
</div>

<div class="flex flex-col">
	<nav
		class="bg-background flex min-w-max items-center gap-x-1.5 gap-y-0.5 pr-5 pl-[calc(var(--spacing-gutter)-1rem)] sm:w-full sm:min-w-0 sm:flex-wrap"
		aria-label="Conversation sections"
	>
		<ul class="flex min-w-full items-center pr-5">
			{#each paths as path (path)}
				{@const active =
					page.route.id?.split('conversations/[conversation_id]/')[1]?.startsWith(path) ??
					false}
				{@const disabled = needsLiveConversation(path) && !conversation.isLive}
				<li
					class="border-box border-box border-b-[3px] px-4 pt-3 pb-2 text-sm font-medium select-none border-{active
						? 'primary'
						: 'transparent'}"
				>
					{#if disabled}
						<span
							class="text-foreground cursor-not-allowed whitespace-nowrap opacity-30"
							aria-disabled="true"
							title="Available after launch"
						>
							{resolveLabel(path)}
						</span>
					{:else}
						<a
							href={resolve(
								`/(admin)/admin/conversations/[conversation_id]/${path}`,
								{
									conversation_id: params.conversation_id
								}
							)}
							class="text-foreground h-full w-full whitespace-nowrap transition-opacity"
							class:opacity-50={!active}
							class:hover:opacity-100={!active}
							aria-current={active ? 'page' : undefined}
						>
							{resolveLabel(path)}
						</a>
					{/if}
				</li>
			{/each}
		</ul>
	</nav>

	{#if conversation.isComplete}
		<div class="border-destructive/20 bg-destructive/10 pl-gutter border-b py-2 pr-5">
			<p class="text-destructive text-sm">This conversation has closed</p>
		</div>
	{/if}

	<div class="bg-admin-background h-full w-full">
		{@render children()}
	</div>
</div>

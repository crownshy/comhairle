<script lang="ts">
	import { ChevronRight, MessageSquareText } from 'lucide-svelte';
	import { timingLabel, type LabConversation } from '../mockData';
	import StateBadge from './StateBadge.svelte';

	type Props = { conversation: LabConversation; href: string };
	let { conversation, href }: Props = $props();
</script>

<!-- Links are resolve() plus the scenario query string, which this rule can't see. -->
<!-- eslint-disable svelte/no-navigation-without-resolve -->

<!-- Compact row for long lists. The staging list layout is admin-only, so this is new. -->
<a
	{href}
	class="bg-card border-border hover:border-primary/40 flex items-center gap-4 rounded-2xl border p-3 transition hover:shadow-md"
>
	<div class="bg-primary/10 relative size-20 shrink-0 overflow-hidden rounded-xl">
		{#if conversation.imageUrl}
			<img
				src={conversation.imageUrl as string}
				alt=""
				class="absolute inset-0 size-full object-cover"
			/>
		{:else}
			<MessageSquareText class="text-primary/40 absolute inset-0 m-auto size-8" />
		{/if}
	</div>
	<div class="flex min-w-0 grow flex-col items-start gap-1.5">
		<StateBadge state={conversation.state} />
		<span class="text-base leading-snug font-semibold">{conversation.title}</span>
		<span class="text-muted-foreground text-base">
			{#if conversation.state === 'results'}
				{timingLabel(conversation)}
			{:else}
				{conversation.estimatedMinutes} min · {timingLabel(conversation)}
			{/if}
		</span>
	</div>
	<ChevronRight class="text-muted-foreground size-5 shrink-0" />
</a>

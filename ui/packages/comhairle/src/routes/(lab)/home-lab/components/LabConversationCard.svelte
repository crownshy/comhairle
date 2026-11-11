<script lang="ts">
	import { Clock, MessageSquareText, Users } from 'lucide-svelte';
	import { participantsLabel, timingLabel, type LabConversation } from '../mockData';
	import StateBadge from './StateBadge.svelte';

	type Props = { conversation: LabConversation; href: string };
	let { conversation, href }: Props = $props();
</script>

<!-- Based on the public variant of staging's ConversationCard, plus a fixed status spot on
	the image and a meta row. Phones stack it, tablets put the image on the left, and large
	screens stack it again for a row of three. A real version would be options on that card. -->
<!-- The link is resolve() plus the scenario query string, which this rule cannot see. -->
<!-- eslint-disable svelte/no-navigation-without-resolve -->
<a
	{href}
	class="group bg-card ring-border/60 flex h-full w-full flex-col items-stretch gap-6 rounded-2xl p-4 shadow-sm ring-1 transition-all duration-300 ease-out hover:-translate-y-0.5 hover:shadow-lg md:flex-row md:items-center md:gap-10 md:p-6 lg:flex-col lg:items-stretch lg:gap-5 lg:p-3"
>
	<div class="w-full min-w-0 md:flex-1 lg:flex-none">
		<div
			class="bg-primary/10 relative h-48 w-full overflow-hidden rounded-xl transition-all duration-300 ease-out group-hover:rounded-2xl group-hover:shadow-lg sm:h-64 md:h-72 lg:h-52 lg:rounded-2xl"
		>
			{#if conversation.imageUrl}
				<img
					class="absolute inset-0 h-full w-full object-cover transition-transform duration-500 ease-out group-hover:scale-105"
					src={conversation.imageUrl as string}
					alt=""
				/>
			{:else}
				<MessageSquareText class="text-primary/30 absolute inset-0 m-auto h-32 w-32" />
			{/if}
			<!-- Every card shows its status here, and only here. -->
			<StateBadge
				state={conversation.state}
				class="absolute top-3 left-3 border-transparent py-1.5 shadow-md backdrop-blur lg:top-4 lg:left-4"
			/>
		</div>
	</div>

	<div class="flex w-full flex-1 flex-col items-start gap-4 lg:gap-3 lg:px-2 lg:pb-2">
		<h2
			class="text-foreground group-hover:text-primary text-2xl leading-8 font-semibold transition-colors duration-300 md:text-3xl md:leading-9 lg:text-xl lg:leading-7"
		>
			{conversation.title}
		</h2>
		<p
			class="text-muted-foreground text-base leading-7 font-medium md:text-lg lg:line-clamp-2 lg:text-base"
		>
			{conversation.shortDescription}
		</p>

		<!-- In a row of three, the meta row sits at the bottom so the rows line up. -->
		<div
			class="text-muted-foreground flex flex-wrap gap-x-5 gap-y-2 text-base lg:mt-auto lg:pt-2"
		>
			<span class="flex items-center gap-1.5">
				<Users class="size-4 opacity-70" />
				{conversation.state === 'upcoming'
					? 'Not open yet'
					: participantsLabel(conversation)}
			</span>
			<span class="flex items-center gap-1.5">
				<Clock class="size-4 opacity-70" />
				{timingLabel(conversation)}
			</span>
		</div>
	</div>
</a>

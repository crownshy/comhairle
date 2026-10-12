<script lang="ts">
	import { resolve } from '$app/paths';
	import * as Card from '$lib/components/ui/card';
	import type { RecruitmentChannel } from './sampleMonitorData';

	type Props = { channels: RecruitmentChannel[]; conversationId: string };

	let { channels, conversationId }: Props = $props();

	let mostJoined = $derived(Math.max(1, ...channels.map((channel) => channel.joined)));
</script>

<Card.Root>
	<Card.Header class="flex flex-row flex-wrap items-center justify-between gap-3">
		<div class="flex flex-col gap-2">
			<Card.Title>Recruitment channel</Card.Title>
			<Card.Description>Where people came from</Card.Description>
		</div>
		<a
			href={resolve('/(admin)/admin/conversations/[conversation_id]/invites', {
				conversation_id: conversationId
			})}
			class="text-primary text-base font-semibold hover:underline">Manage in Recruit</a
		>
	</Card.Header>
	<Card.Content>
		<ul class="flex flex-col gap-4">
			{#each channels as channel (channel.name)}
				<li
					class="grid grid-cols-1 items-center gap-2 md:grid-cols-[16rem_minmax(0,1fr)_10rem] md:gap-4"
				>
					<div class="flex flex-col">
						<span class="text-muted-foreground text-sm tracking-wide uppercase"
							>{channel.type}</span
						>
						<span class="text-base font-semibold">{channel.name}</span>
					</div>
					<div class="bg-muted h-5 rounded-md" aria-hidden="true">
						<div
							class="bg-primary h-5 rounded-md"
							style:width="{(channel.joined / mostJoined) * 100}%"
						></div>
					</div>
					<div class="flex flex-col text-sm">
						<span><strong class="text-base">{channel.joined}</strong> joined</span>
						<span class="text-muted-foreground"
							>{channel.conversion ?? 'No conversion data'}</span
						>
					</div>
				</li>
			{/each}
		</ul>
	</Card.Content>
</Card.Root>

<script lang="ts">
	import { resolve } from '$app/paths';
	import { ArrowRight, Check, Eye } from 'lucide-svelte';
	import * as Card from '$lib/components/ui/card';
	import { Badge } from '$lib/components/ui/badge';
	import { Button } from '$lib/components/ui/button';
	import type { PermittedConversationDto } from '@crownshy/api-client/api';
	import {
		conversationStatus,
		CONVERSATION_STATUS_BADGE_VARIANTS,
		CONVERSATION_STATUS_LABELS
	} from '$lib/utils/conversationStatus';
	import { conversationStages, type StageState } from '../sidebar/conversationStages';
	import { nextStep } from './nextStep';

	let { conversation }: { conversation: PermittedConversationDto } = $props();

	let status = $derived(conversationStatus(conversation));
	let next = $derived(nextStep(conversation.id, status));
	let stages = $derived(conversationStages({ conversation, pathname: '', canAdmin: false }));

	const stagePillClass: Record<StageState, string> = {
		done: 'bg-primary/10 text-primary',
		current: 'bg-primary text-primary-foreground',
		locked: 'bg-muted text-muted-foreground'
	};
</script>

<Card.Root class="gap-5">
	<Card.Header>
		<Card.Description class="text-base">Pick up where you left off</Card.Description>
		<Card.Title class="text-2xl">{conversation.title}</Card.Title>
		<Card.Action class="flex gap-2">
			{#if conversation.isShared}<Badge variant="outline" class="text-sm">Shared</Badge>{/if}
			<Badge variant={CONVERSATION_STATUS_BADGE_VARIANTS[status]} class="text-sm">
				{CONVERSATION_STATUS_LABELS[status]}
			</Badge>
		</Card.Action>
	</Card.Header>
	<Card.Content>
		<ol class="flex flex-wrap items-center gap-2 text-base" aria-label="Conversation stages">
			{#each stages as stage (stage.id)}
				<li
					class="flex items-center gap-1.5 rounded-full px-3 py-1 {stagePillClass[
						stage.state
					]}"
				>
					{#if stage.state === 'done'}<Check class="size-4" /><span class="sr-only"
							>Done:</span
						>{/if}
					{stage.label}
				</li>
			{/each}
		</ol>
	</Card.Content>
	<Card.Footer class="flex flex-wrap gap-3">
		<!-- eslint-disable-next-line svelte/no-navigation-without-resolve -->
		<Button href={next.href}>
			{next.label}
			<ArrowRight class="size-4" />
		</Button>
		<Button
			variant="outline"
			target="_blank"
			href={resolve('/(public)/conversations/[conversation_id]/[[preview]]', {
				conversation_id: conversation.id,
				preview: 'preview'
			})}
		>
			<Eye class="size-4" /> Preview
		</Button>
	</Card.Footer>
</Card.Root>

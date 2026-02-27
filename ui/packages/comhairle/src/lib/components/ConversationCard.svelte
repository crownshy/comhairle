<script lang="ts">
	import type { ConversationDto, LocalizedConversationDto } from '@crown-shy/api-client/api';
	import { Button } from '$lib/components/ui/button';
	import { Badge } from '$lib/components/ui/badge';
	import { ArrowRight, MessageSquareText } from 'lucide-svelte';

	let {
		conversation,
		variant = 'admin',
		organizationName
	}: {
		conversation: ConversationDto | LocalizedConversationDto;
		variant?: 'admin' | 'public';
		organizationName?: string;
	} = $props();
</script>

<!-- Image snippet -->
{#snippet image(heightClass: string)}
	<div class="{heightClass} w-full flex-shrink-0 lg:flex-1">
		<div class="bg-primary/10 relative h-full w-full overflow-hidden rounded-xl lg:rounded-3xl">
			{#if conversation.imageUrl}
				<img
					class="absolute inset-0 h-full w-full object-cover"
					src={conversation.imageUrl}
					alt={conversation.title}
				/>
			{:else}
				<div class="bg-primary/10 absolute inset-0 flex items-center justify-center">
					<MessageSquareText class="text-primary/30 h-32 w-32" />
				</div>
			{/if}
		</div>
	</div>
{/snippet}

{#if conversation}
	<!-- Public variant -->
	{#if variant === 'public'}
		<div
			class="flex w-full max-w-[1280px] min-w-[380px] flex-col items-start justify-start gap-6 md:min-w-[480px] lg:flex-row lg:gap-16"
		>
			{@render image('h-48 sm:h-64 lg:h-80')}

			<!-- Text content -->
			<div class="flex flex-1 flex-col items-start justify-start gap-6 lg:gap-8">
				<div class="flex flex-col items-start justify-start gap-3 self-stretch lg:gap-4">
					{#if conversation.isLive}
						<Badge variant="outline" class="h-7 text-sm">Live</Badge>
					{/if}

					{#if organizationName}
						<p class="text-primary self-stretch text-sm leading-5 font-medium">
							{organizationName}
						</p>
					{/if}

					<h2
						class="text-foreground self-stretch text-2xl leading-8 font-semibold lg:text-3xl lg:leading-9"
					>
						{conversation.title}
					</h2>

					<p
						class="text-muted-foreground self-stretch text-base leading-7 font-medium lg:text-lg"
					>
						{conversation.shortDescription}
					</p>
				</div>
			</div>
		</div>
	{:else}
		<!-- Admin variant -->
		<div
			class="flex w-full max-w-[1280px] flex-col-reverse items-start justify-start gap-2 lg:flex-row-reverse lg:gap-16"
		>
			{@render image('h-40 sm:h-56 lg:h-72')}

			<!-- Text content -->
			<div class="flex flex-1 flex-col items-start justify-start gap-6 lg:gap-8">
				<div class="flex flex-col items-start justify-start gap-3 self-stretch lg:gap-4">
					{#if conversation.isLive}
						<Badge variant="default" class="h-7 text-sm">Live</Badge>
					{:else}
						<Badge variant="draft" class="h-7 text-sm">Draft</Badge>
					{/if}

					<h2
						class="text-foreground self-stretch text-xl leading-7 font-semibold lg:text-2xl"
					>
						{conversation.title}
					</h2>

					<p class="text-foreground self-stretch text-sm leading-5 font-medium">
						{conversation.shortDescription}
					</p>
				</div>

				<!-- Edit button -->
				<Button
					variant="default"
					class="bg-sidebar text-sidebar-foreground hover:bg-sidebar/90 rounded-full px-4 py-3"
					href={`/admin/conversations/${conversation.id}/configure`}
				>
					Edit conversation
					<ArrowRight class="ml-1 size-4" />
				</Button>
			</div>
		</div>
	{/if}
{/if}

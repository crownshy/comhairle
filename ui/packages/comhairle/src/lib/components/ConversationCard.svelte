<script lang="ts">
	import type { ConversationDto, LocalizedConversationDto } from '@crownshy/api-client/api';
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
	<div class="w-full min-w-0 lg:flex-1">
		<div
			class="bg-primary/10 relative {heightClass} w-full overflow-hidden rounded-xl transition-all duration-300 ease-out group-hover:rounded-2xl group-hover:shadow-lg lg:rounded-3xl lg:group-hover:rounded-[2rem]"
		>
			{#if conversation.imageUrl}
				<img
					class="absolute inset-0 h-full w-full object-cover transition-transform duration-500 ease-out group-hover:scale-105"
					src={conversation.imageUrl}
					alt={conversation.title}
				/>
			{:else}
				<div class="bg-primary/10 absolute inset-0 flex items-center justify-center">
					<MessageSquareText
						class="text-primary/30 group-hover:text-primary/50 h-32 w-32 transition-all duration-300 group-hover:scale-110"
					/>
				</div>
			{/if}
		</div>
	</div>
{/snippet}

{#if conversation}
	<!-- Public variant -->
	{#if variant === 'public'}
		<div
			class="group hover:bg-accent/50 flex w-full flex-col items-stretch gap-6 rounded-2xl p-4 transition-all duration-300 ease-out hover:shadow-md lg:flex-row lg:items-start lg:gap-16 lg:p-6"
		>
			{@render image('h-48 sm:h-64 lg:h-80')}

			<!-- Text content -->
			<div class="flex w-full flex-1 flex-col items-start justify-start gap-6 lg:gap-8">
				<div class="flex flex-col items-start justify-start gap-3 self-stretch lg:gap-4">
					{#if conversation.isLive}
						<Badge variant="default" class="h-7 text-sm">Live</Badge>
					{/if}

					{#if organizationName}
						<p
							class="text-primary self-stretch text-sm leading-5 font-medium transition-colors duration-300"
						>
							{organizationName}
						</p>
					{/if}

					<h2
						class="text-foreground self-stretch text-2xl leading-8 font-semibold transition-colors duration-300 group-hover:text-[var(--color-brand)] lg:text-3xl lg:leading-9"
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
			class="group hover:bg-accent/50 flex w-full max-w-[1280px] flex-col-reverse items-stretch gap-2 rounded-2xl p-4 transition-all duration-300 ease-out hover:shadow-md lg:flex-row-reverse lg:items-start lg:gap-16 lg:p-6"
		>
			{@render image('h-40 sm:h-56 lg:h-72')}

			<!-- Text content -->
			<div class="flex w-full flex-1 flex-col items-start justify-start gap-6 lg:gap-8">
				<div class="flex flex-col items-start justify-start gap-3 self-stretch lg:gap-4">
					{#if conversation.isLive}
						<Badge variant="default" class="h-7 text-sm">Live</Badge>
					{:else}
						<Badge variant="draft" class="h-7 text-sm">Draft</Badge>
					{/if}

					<h2
						class="text-foreground self-stretch text-xl leading-7 font-semibold transition-colors duration-300 group-hover:text-[var(--color-brand)] lg:text-2xl"
					>
						{conversation.title}
					</h2>

					<p class="text-foreground self-stretch text-sm leading-5 font-medium">
						{conversation.shortDescription}
					</p>
				</div>

				<!-- Edit button -->
				<Button
					variant="primaryDark"
					class="rounded-full px-4 py-3 transition-transform duration-300"
					href={`/admin/conversations/${conversation.id}/configure`}
				>
					Edit conversation
					<ArrowRight class="ml-1 size-4 transition-transform duration-300" />
				</Button>
			</div>
		</div>
	{/if}
{/if}

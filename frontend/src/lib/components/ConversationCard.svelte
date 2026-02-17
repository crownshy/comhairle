<script lang="ts">
	import type { ConversationDto, LocalizedConversationDto } from '$lib/api/api';
	import { Button } from '$lib/components/ui/button';
	import { Badge } from '$lib/components/ui/badge';
	import { ArrowRight } from 'lucide-svelte';

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

{#if variant === 'public'}
	<!-- Public variant: image left, text right -->
	<div
		class="flex w-full max-w-[1280px] flex-col items-start justify-start gap-6 lg:flex-row lg:gap-16"
	>
		<!-- Image -->
		<div class="h-48 w-full flex-shrink-0 sm:h-64 lg:h-80 lg:flex-1">
			<div
				class="relative h-full w-full overflow-hidden rounded-xl bg-primary/10 lg:rounded-3xl"
			>
				{#if conversation.imageUrl}
					<img
						class="absolute inset-0 h-full w-full object-cover"
						src={conversation.imageUrl}
						alt={conversation.title}
					/>
				{/if}
			</div>
		</div>

		<!-- Text content -->
		<div class="flex flex-1 flex-col items-start justify-start gap-6 lg:gap-8">
			<div class="flex flex-col items-start justify-start gap-3 self-stretch lg:gap-4">
				<!-- Status badge -->
				{#if conversation.isLive}
					<Badge variant="outline" class="h-7 text-sm">Live</Badge>
				{/if}

				<!-- Organization name -->
				{#if organizationName}
					<p class="self-stretch text-sm font-medium leading-5 text-primary">
						{organizationName}
					</p>
				{/if}

				<!-- Title -->
				<h2
					class="self-stretch text-2xl font-semibold leading-8 text-foreground lg:text-3xl lg:leading-9"
				>
					{conversation.title}
				</h2>

				<!-- Description -->
				<p class="self-stretch text-base font-medium leading-7 text-muted-foreground lg:text-lg">
					{conversation.shortDescription}
				</p>
			</div>
		</div>
	</div>
{:else}
	<!-- Admin variant: text left, image right -->
	<div
		class="flex w-full max-w-[1280px] flex-col-reverse items-start justify-start gap-6 lg:flex-row-reverse lg:gap-16"
	>
		<!-- Image -->
		<div class="h-40 w-full flex-shrink-0 sm:h-56 lg:h-72 lg:flex-1">
			<div
				class="relative h-full w-full overflow-hidden rounded-xl bg-stone-200 lg:rounded-3xl"
			>
				{#if conversation.imageUrl}
					<img
						class="absolute inset-0 h-full w-full object-cover"
						src={conversation.imageUrl}
						alt={conversation.title}
					/>
				{/if}
			</div>
		</div>

		<!-- Text content -->
		<div class="flex flex-1 flex-col items-start justify-start gap-6 lg:gap-8">
			<div class="flex flex-col items-start justify-start gap-3 self-stretch lg:gap-4">
				<!-- Status badge -->
				{#if conversation.isLive}
					<Badge variant="default" class="h-7 text-sm">Live</Badge>
				{:else}
					<Badge variant="draft" class="h-7 text-sm">Draft</Badge>
				{/if}

				<!-- Title -->
				<h2 class="self-stretch text-xl font-semibold leading-7 text-foreground lg:text-2xl">
					{conversation.title}
				</h2>

				<!-- Description -->
				<p class="self-stretch text-sm font-medium leading-5 text-black">
					{conversation.shortDescription}
				</p>
			</div>

			<!-- Edit button -->
			<Button
				variant="default"
				class="rounded-full bg-slate-900 px-4 py-3 hover:bg-slate-800"
				href={`/admin/conversations/${conversation.id}/configure`}
			>
				Edit conversation
				<ArrowRight class="ml-1 size-4" />
			</Button>
		</div>
	</div>
{/if}
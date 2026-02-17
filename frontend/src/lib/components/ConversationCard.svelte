<script lang="ts">
	import type { ConversationDto } from '$lib/api/api';
	import { Button } from '$lib/components/ui/button';
	import { ArrowRight } from 'lucide-svelte';

	let { conversation }: { conversation: ConversationDto } = $props();
</script>

<div
	class="flex w-full max-w-[1280px] flex-col-reverse items-start justify-start gap-8 lg:flex-row lg:gap-16"
>
	<!-- Text content -->
	<div class="flex flex-1 flex-col items-start justify-start gap-8">
		<div class="flex flex-col items-start justify-start gap-4 self-stretch">
			<!-- Status badge -->
			{#if conversation.isLive}
			<div
				class="inline-flex h-7 items-center justify-center gap-1 overflow-hidden rounded-full bg-blue-100 px-2.5 shadow-[0px_1px_2px_0px_rgba(0,0,0,0.05)] outline-1 outline-offset-[-1px] outline-blue-200"
			>
				<span class="text-sm font-medium leading-5 text-foreground">Live</span>
			</div>	
			{:else}
			<div
				class="inline-flex h-7 items-center justify-center gap-1 overflow-hidden rounded-full bg-stone-100 px-2.5 shadow-[0px_1px_2px_0px_rgba(0,0,0,0.05)] outline-1 outline-offset-[-1px] outline-stone-200"
			>
				<span class="text-sm font-medium leading-5 text-foreground">Draft</span>
			
			</div>
			{/if}

			<!-- Title -->
			<h2 class="self-stretch text-2xl font-semibold leading-7 text-foreground">
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

	<!-- Image -->
	<div class="flex h-56 flex-1 flex-col items-start justify-start sm:h-72">
		<div class="relative flex-1 self-stretch overflow-hidden rounded-3xl bg-stone-200">
			{#if conversation.imageUrl}
				<img
					class="absolute inset-0 h-full w-full object-cover"
					src={conversation.imageUrl}
					alt={conversation.title}
				/>
			{/if}
		</div>
	</div>
</div>
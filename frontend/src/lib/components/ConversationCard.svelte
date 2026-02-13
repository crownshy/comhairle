<script lang="ts">
	import type { ConversationDto } from '$lib/api/api';
	let { conversation }: { conversation: ConversationDto } = $props();
</script>

<div
	class="flex w-full max-w-[1280px] flex-col items-start justify-start gap-5 md:gap-16 lg:flex-row"
>
	<img
		class="mx-auto h-[408px] w-[612.15px] rounded-3xl object-cover lg:mx-0"
		src={conversation.imageUrl}
		alt={conversation.title}
	/>
	<div class="inline-flex flex-1 flex-col items-start justify-start gap-4 px-4 lg:py-5">
		<div
			class="inline-flex flex-wrap content-center items-center justify-start gap-2 self-stretch"
		>
			<div
				class="flex h-8 items-center justify-center gap-1 overflow-hidden rounded-full bg-stone-300 px-2.5 shadow-[0px_1px_2px_0px_rgba(0,0,0,0.05)]"
			>
				<div
					class="justify-start font-['Inter'] text-sm leading-tight font-medium text-neutral-900"
				>
					Live
				</div>
			</div>
		</div>
		<div
			class="inline-flex flex-wrap content-start items-start justify-start gap-3 self-stretch"
		>
			<div class="flex items-center justify-start gap-1">
				<div
					class="justify-start font-['Inter'] text-sm leading-tight font-normal text-zinc-600"
				>
					Scottish Government
				</div>
			</div>
		</div>
		<div class="flex flex-col items-start justify-center gap-2 self-stretch">
			<div
				class="justify-start self-stretch font-['Inter'] text-3xl leading-9 font-semibold text-neutral-900"
			>
				{conversation.title}
			</div>
			<div
				class="justify-start self-stretch font-['Inter'] text-lg leading-normal font-medium text-zinc-600"
			>
				{conversation.shortDescription}
			</div>
		</div>
	</div>
</div>

<script lang="ts">
	import type { Conversation } from '$lib/api/api';
	import * as Card from '$lib/components/ui/card/index';
	import { Button } from '$lib/components/ui/button';
	let { conversation }: { conversation: Conversation } = $props();

	import * as m from '$lib/paraglide/messages';
</script>

<a href={`/conversations/${conversation.id}`}>
	<div class="inline-flex w-full max-w-[1280px] items-center justify-start gap-16 pr-6">
		<img
			class="h-[408] w-[612.15px] flex-1 rounded-3xl object-cover"
			src={conversation.image_url}
		/>
		<div class="inline-flex flex-1 flex-col items-start justify-start gap-4 px-4">
			<div
				class="inline-flex flex-wrap content-center items-center justify-start gap-2 self-stretch"
			>
				<div
					class="flex h-8 items-center justify-center gap-1 overflow-hidden rounded-full bg-stone-300 px-2.5 shadow-[0px_1px_2px_0px_rgba(0,0,0,0.05)]"
				>
					<div
						class="justify-start font-['Inter'] text-sm leading-tight font-medium text-neutral-900"
					>
						Upcoming
					</div>
				</div>
			</div>
			<div class="inline-flex flex-wrap content-start items-start justify-start gap-3 self-stretch">
				<div class="flex items-center justify-start gap-1">
					<div class="justify-start font-['Inter'] text-sm leading-tight font-normal text-zinc-600">
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
					{conversation.short_description}
				</div>
			</div>
			<div class="inline-flex items-start justify-start gap-2 self-stretch">
				<div
					data-property-1="Default"
					class="flex h-7 items-center justify-center gap-1 overflow-hidden rounded-lg bg-white px-2.5 shadow-[0px_1px_2px_0px_rgba(0,0,0,0.05)] outline outline-1 outline-offset-[-1px] outline-stone-200"
				>
					<div
						class="justify-start font-['Inter'] text-sm leading-none font-medium text-neutral-900"
					>
						Land
					</div>
				</div>
				<div
					data-property-1="Default"
					class="flex h-7 items-center justify-center gap-1 overflow-hidden rounded-lg bg-white px-2.5 shadow-[0px_1px_2px_0px_rgba(0,0,0,0.05)] outline outline-1 outline-offset-[-1px] outline-stone-200"
				>
					<div
						class="justify-start font-['Inter'] text-sm leading-none font-medium text-neutral-900"
					>
						Natural resources
					</div>
				</div>
			</div>
		</div>
	</div>
</a>

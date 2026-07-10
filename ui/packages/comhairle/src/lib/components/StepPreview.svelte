<script lang="ts">
	import type { ToolType } from '$lib/tool_meta';
	import {
		ThumbsUp,
		ThumbsDown,
		ArrowRight,
		Check,
		Pencil,
		Mic,
		Camera,
		Sparkles
	} from 'lucide-svelte';

	let { type, class: className = '' }: { type: string | undefined; class?: string } = $props();

	// Two-tone only: `card` for the white surfaces, `secondary` for the light-grey shapes.
</script>

<!--
	Decorative, non-interactive skeletons that hint at what a participant sees for each
	tool. Intentionally low-fidelity placeholders for the redesigned board; refine per Figma.
-->
<div
	class="bg-card border-border relative aspect-square w-full overflow-hidden rounded border p-6 {className}"
	aria-hidden="true"
>
	{#if type === 'learn'}
		<div class="flex h-full flex-col gap-2">
			<div class="bg-secondary mb-2 h-24 w-2/5 rounded"></div>
			<div class="bg-secondary h-1.5 w-full rounded-md"></div>
			<div class="bg-secondary h-1.5 w-full rounded-md"></div>
			<div class="bg-secondary h-1.5 w-3/4 rounded-md"></div>
			<div class="bg-secondary mt-2 h-1.5 w-full rounded-md"></div>
			<div class="bg-secondary h-1.5 w-full rounded-md"></div>
			<div class="bg-secondary h-1.5 w-3/4 rounded-md"></div>
			<div class="mt-auto flex gap-2">
				<div class="bg-secondary h-3 w-3.5 rounded-sm"></div>
				<div class="bg-secondary h-1.5 w-6 self-center rounded-md"></div>
				<div class="bg-secondary h-1.5 w-6 self-center rounded-md"></div>
				<div class="bg-secondary h-1.5 w-6 self-center rounded-md"></div>
			</div>
		</div>
	{:else if type === 'heyform'}
		<div class="flex h-full flex-col justify-center gap-6">
			{#each [1, 2, 3] as n (n)}
				<div class="flex items-start gap-4">
					<div
						class="bg-secondary text-card flex size-9 shrink-0 items-center justify-center rounded-full text-lg font-medium"
					>
						{n}
					</div>
					<div class="flex flex-1 flex-col gap-1.5 pt-1">
						<div class="bg-secondary h-1.5 w-3/4 rounded-md"></div>
						<div class="bg-secondary h-1.5 w-1/2 rounded-md"></div>
						{#if n === 1}
							<div class="mt-1 flex gap-1">
								{#each Array(5) as _, i (i)}
									<div class="bg-secondary size-5 rounded-md"></div>
								{/each}
							</div>
						{:else if n === 2}
							<div
								class="border-secondary mt-1 h-6 w-2/3 rounded-md border-[3px]"
							></div>
						{:else}
							<div class="bg-secondary mt-2 h-1.5 w-2/3 rounded-md"></div>
						{/if}
					</div>
				</div>
			{/each}
		</div>
	{:else if type === 'polis'}
		<div class="flex h-full flex-col items-center justify-center gap-4">
			<div class="border-secondary w-3/4 rounded-md border-2 p-3">
				<div class="mb-1 flex items-center gap-2">
					<ThumbsUp class="text-secondary size-4" />
					<ThumbsDown class="text-secondary size-4" />
					<ArrowRight class="text-secondary size-4" />
				</div>
				<div class="bg-secondary mt-2 h-1.5 w-3/4 rounded-md"></div>
				<div class="bg-secondary mt-1 h-1.5 w-2/3 rounded-md"></div>
				<div class="bg-secondary mt-1 h-1.5 w-1/2 rounded-md"></div>
			</div>
			<div class="bg-secondary flex size-14 items-center justify-center rounded-full">
				<div class="border-card size-6 rounded-full border-2"></div>
			</div>
		</div>
	{:else if type === 'prioritization'}
		<div class="flex h-full items-center gap-4">
			<div class="border-secondary flex flex-1 flex-col gap-1.5 rounded border-2 p-2">
				<div class="bg-secondary h-20 w-full rounded"></div>
				<div class="bg-secondary h-1.5 w-3/4 rounded-md"></div>
				<div class="bg-secondary h-1.5 w-2/3 rounded-md"></div>
				<div class="bg-secondary h-1.5 w-1/2 rounded-md"></div>
			</div>
			<div class="flex flex-col gap-3">
				<div class="bg-secondary flex size-12 items-center justify-center rounded-full">
					<Check class="text-card size-6" />
				</div>
				<div class="bg-secondary flex size-12 items-center justify-center rounded-full">
					<Pencil class="text-card size-5" />
				</div>
			</div>
		</div>
	{:else if type === 'stories'}
		<div class="grid h-full grid-cols-2 grid-rows-2 gap-2">
			{#each Array(4) as _, i (i)}
				<div class="bg-secondary relative flex items-center justify-center rounded">
					<div
						class="border-card flex size-8 items-center justify-center rounded-full border-2"
					>
						{#if i === 0}<Camera class="text-card size-4" />{:else if i === 3}<Mic
								class="text-card size-4"
							/>{/if}
					</div>
				</div>
			{/each}
		</div>
	{:else}
		<!-- thinkingspace + elicitationbot: coaching-chat vibe -->
		<div class="flex h-full flex-col justify-center gap-3">
			<div class="flex items-start gap-2">
				<Sparkles class="text-secondary mt-1 size-4 shrink-0" />
				<div class="border-secondary flex-1 rounded-md border-2 p-2">
					<div class="bg-secondary h-1.5 w-3/4 rounded-md"></div>
					<div class="bg-secondary mt-1 h-1.5 w-1/2 rounded-md"></div>
				</div>
			</div>
			<div class="ml-8 flex items-start justify-end gap-2">
				<div class="bg-secondary flex-1 rounded-md p-2">
					<div class="bg-card/60 h-1.5 w-2/3 rounded-md"></div>
					<div class="bg-card/60 mt-1 h-1.5 w-1/2 rounded-md"></div>
				</div>
				<div class="bg-secondary size-8 shrink-0 rounded-full"></div>
			</div>
			<div class="flex items-start gap-2">
				<Sparkles class="text-secondary mt-1 size-4 shrink-0" />
				<div class="border-secondary flex-1 rounded-md border-2 p-2">
					<div class="bg-secondary h-1.5 w-2/3 rounded-md"></div>
				</div>
			</div>
		</div>
	{/if}
</div>

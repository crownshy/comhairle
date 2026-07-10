<script lang="ts">
	import { PALETTE_TOOLS, toolInfoUrl } from '$lib/tool_meta';
	import StepPreview from '$lib/components/StepPreview.svelte';
	import { Button } from '$lib/components/ui/button';
	import { LinkPreview } from 'bits-ui';
	import { Plus, ArrowRight } from 'lucide-svelte';

	let { onAdd, adding = false }: { onAdd: (creationKey: string) => void; adding?: boolean } =
		$props();
</script>

<div
	class="bg-muted border-border flex h-full flex-col gap-3 overflow-y-auto border-r p-3"
	aria-label="Tool palette"
>
	{#each PALETTE_TOOLS as tool (tool.type)}
		<div class="group relative shrink-0">
			<div
				class="border-border bg-card/40 relative aspect-square w-44 overflow-hidden rounded-lg border shadow-sm"
			>
				<!-- Name pill: a LinkPreview (hover-card) so its content stays open while the
				     pointer moves onto it, making "Learn more" reliably clickable. Portaled,
				     so the palette's overflow-y-auto can't clip it. -->
				<LinkPreview.Root openDelay={100} closeDelay={200}>
					<LinkPreview.Trigger>
						{#snippet child({ props })}
							<button
								{...props}
								type="button"
								class="bg-card text-primary absolute top-3.5 left-1/2 z-20 -translate-x-1/2 rounded-full px-3 py-1.5 text-xs font-medium whitespace-nowrap shadow"
							>
								{tool.displayName}
							</button>
						{/snippet}
					</LinkPreview.Trigger>
					<LinkPreview.Portal>
						<LinkPreview.Content
							side="right"
							sideOffset={8}
							class="bg-primary text-primary-foreground z-50 w-80 rounded-lg p-3 text-left shadow-lg"
						>
							<p class="text-sm font-medium">{tool.displayName}</p>
							<p class="mt-2 text-sm leading-5">{tool.tagline}</p>
							<a
								href={toolInfoUrl(tool.type)}
								target="_blank"
								rel="noopener noreferrer"
								class="mt-2 inline-flex items-center gap-1 text-xs font-medium underline"
							>
								Learn more
								<ArrowRight class="size-3" />
							</a>
							<LinkPreview.Arrow />
						</LinkPreview.Content>
					</LinkPreview.Portal>
				</LinkPreview.Root>

				<!-- Preview backdrop -->
				<div class="pointer-events-none absolute inset-6 top-12 opacity-60">
					<StepPreview type={tool.type} class="border-0 !p-2" />
				</div>

				<!-- Hover overlay + Add -->
				<div
					class="absolute inset-0 z-10 flex items-end justify-center bg-black/5 pb-6 opacity-0 transition-opacity group-hover:opacity-100"
				>
					<Button
						size="sm"
						class="rounded-full"
						disabled={adding}
						onclick={() => onAdd(tool.creationKey)}
					>
						Add
						<Plus class="size-3" />
					</Button>
				</div>
			</div>
		</div>
	{/each}
</div>

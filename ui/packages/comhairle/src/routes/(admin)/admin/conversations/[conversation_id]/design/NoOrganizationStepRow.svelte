<script lang="ts">
	import { Popover } from 'bits-ui';
	import { resolve } from '$app/paths';
	import ArrowRight from '@lucide/svelte/icons/arrow-right';
	import SelectableOptionRow from '$lib/components/SelectableOptionRow.svelte';

	// A step type greyed out because the conversation has no hosting organisation.
	// Hover, focus or tap explains why, with a link to set one.
	let { name, conversationId }: { name: string; conversationId: string } = $props();
</script>

<Popover.Root>
	<Popover.Trigger openOnHover openDelay={100} closeDelay={150}>
		{#snippet child({ props })}
			<div
				{...props}
				tabindex="0"
				aria-label={`${name} (unavailable)`}
				class="rounded-lg max-md:h-full"
			>
				<!-- The disabled button would swallow hover, so the wrapper takes the pointer. -->
				<SelectableOptionRow
					{name}
					disabled
					onSelect={() => {}}
					class="pointer-events-none max-md:h-full max-md:p-3"
				/>
			</div>
		{/snippet}
	</Popover.Trigger>
	<Popover.Portal>
		<Popover.Content
			side="right"
			sideOffset={8}
			collisionPadding={12}
			trapFocus={false}
			onOpenAutoFocus={(e) => e.preventDefault()}
			class="bg-popover text-popover-foreground border-border data-[state=open]:animate-in data-[state=open]:fade-in-0 data-[state=open]:zoom-in-95 z-50 w-80 max-w-[90vw] rounded-lg border p-4 text-base shadow-md"
		>
			<p class="text-muted-foreground">
				Demographic steps need a hosting organisation, because their questions and forms
				come from it. This conversation doesn't have one yet.
			</p>
			<div class="border-border mt-3 border-t pt-3">
				<a
					href={resolve('/(admin)/admin/conversations/[conversation_id]/configure/team', {
						conversation_id: conversationId
					})}
					class="text-primary inline-flex items-center gap-1 font-semibold hover:underline"
				>
					Configure one
					<ArrowRight class="size-3.5" aria-hidden="true" />
				</a>
			</div>
		</Popover.Content>
	</Popover.Portal>
</Popover.Root>

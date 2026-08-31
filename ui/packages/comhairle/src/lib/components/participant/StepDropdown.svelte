<script lang="ts">
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import { ChevronDown, Check, Lock } from 'lucide-svelte';
	import { cn } from '$lib/utils';
	import * as m from '$lib/paraglide/messages';
	import type { StepItem } from './stepItems';

	let {
		steps,
		currentIndex,
		label,
		legalLinks = []
	}: {
		steps: StepItem[];
		currentIndex: number;
		/** The trigger's text: the current step's name, prefixed with its position. */
		label: string;
		/** Site links the workflow routes no longer render a footer for. */
		legalLinks?: { href: string; label: string }[];
	} = $props();
</script>

<!-- Step hrefs are built by the step page from `workflow_step_url`, and the legal links are
	plain site paths. Neither is a typed route id, so resolve() has nothing to resolve. -->
<!-- eslint-disable svelte/no-navigation-without-resolve -->
<DropdownMenu.Root>
	<DropdownMenu.Trigger
		class="text-primary flex min-w-0 items-center justify-end gap-2 text-sm font-medium md:text-base"
		aria-label={m.step_dropdown_open()}
	>
		<span class="truncate">{label}</span>
		<ChevronDown class="size-5 shrink-0" aria-hidden="true" />
	</DropdownMenu.Trigger>

	<DropdownMenu.Content align="end" class="w-72">
		<DropdownMenu.GroupHeading>{m.step_dropdown_heading()}</DropdownMenu.GroupHeading>
		{#each steps as step, index (step.id)}
			{@const position = m.step_position_label({ current: index + 1, total: steps.length })}
			{#snippet row()}
				<span class="flex w-full items-center gap-3">
					<span
						class={cn(
							'flex size-5 shrink-0 items-center justify-center rounded-full',
							step.status === 'current'
								? 'bg-primary'
								: step.status === 'upcoming'
									? 'bg-primary/20'
									: 'bg-primary'
						)}
					>
						{#if step.status === 'completed' || step.status === 'completed-locked'}
							<Check class="text-primary-foreground size-3" />
						{:else if step.status === 'current'}
							<span class="bg-background size-2 rounded-full"></span>
						{/if}
					</span>
					<span class="flex min-w-0 flex-col">
						<span class="text-ring text-xs leading-4 font-medium uppercase">
							{position}
						</span>
						<span
							class={cn(
								'truncate text-base leading-5 font-medium',
								step.status === 'completed-locked' || step.status === 'upcoming'
									? 'text-muted-foreground'
									: 'text-foreground'
							)}
						>
							{step.name}
						</span>
					</span>
					{#if step.status === 'completed-locked'}
						<Lock class="text-muted-foreground ml-auto size-4 shrink-0" />
					{/if}
				</span>
			{/snippet}

			{#if step.href}
				<DropdownMenu.Item>
					{#snippet child({ props })}
						<a
							{...props}
							href={step.href}
							class={cn(props.class as string, 'cursor-pointer')}
						>
							{@render row()}
						</a>
					{/snippet}
				</DropdownMenu.Item>
			{:else}
				<!-- Inert rather than absent: an unreachable step still tells you where you are. -->
				<div class="px-2 py-1.5" aria-current={index === currentIndex ? 'step' : undefined}>
					{@render row()}
				</div>
			{/if}
		{/each}

		{#if legalLinks.length > 0}
			<DropdownMenu.Separator />
			{#each legalLinks as link (link.href)}
				<DropdownMenu.Item>
					{#snippet child({ props })}
						<a
							{...props}
							href={link.href}
							class={cn(
								props.class as string,
								'text-muted-foreground cursor-pointer text-sm'
							)}
						>
							{link.label}
						</a>
					{/snippet}
				</DropdownMenu.Item>
			{/each}
		{/if}
	</DropdownMenu.Content>
</DropdownMenu.Root>

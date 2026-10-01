<script lang="ts">
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import * as Drawer from '$lib/components/ui/drawer';
	import { ChevronDown, Check, Lock, Moon, Sun } from 'lucide-svelte';
	import { IsMobile } from '$lib/hooks/is-mobile.svelte';
	import { themeStore } from '$lib/stores/theme.svelte';
	import { cn } from '$lib/utils';
	import * as m from '$lib/paraglide/messages';
	import type { StepItem } from './stepItems';

	type Props = {
		steps: StepItem[];
		currentIndex: number;
		/** The trigger's text: the viewed step's name, prefixed with its position. */
		label: string;
	};

	let { steps, currentIndex, label }: Props = $props();

	const isMobile = new IsMobile();

	let open = $state(false);

	let viewedStep = $derived(steps[currentIndex]);

	// The count goes in the heading once rather than on every row.
	let heading = $derived(m.step_x_of_y({ current: currentIndex + 1, total: steps.length }));

	let themeLabel = $derived(themeStore.isDark ? m.theme_light_mode() : m.theme_dark_mode());
	let ThemeIcon = $derived(themeStore.isDark ? Sun : Moon);
</script>

{#snippet triggerInner()}
	<span class="truncate">{label}</span>
	<ChevronDown
		class="size-5 shrink-0 transition-transform duration-150 group-data-[state=open]:rotate-180"
		aria-hidden="true"
	/>
{/snippet}

{#snippet stepRow(step: StepItem, position: number)}
	<span class="flex w-full items-center gap-3">
		<span
			class={cn(
				'flex size-6 shrink-0 items-center justify-center rounded-full text-xs font-medium tabular-nums',
				step.status === 'upcoming'
					? 'bg-muted text-muted-foreground'
					: 'bg-primary text-primary-foreground'
			)}
		>
			{#if step.status === 'completed' || step.status === 'completed-locked'}
				<!-- `text-current` opts out of the menu item's blanket muted colour for icons. -->
				<Check class="size-3.5 text-current" strokeWidth={3} />
			{:else}
				{position}
			{/if}
		</span>
		<span
			class={cn(
				'min-w-0 truncate text-base font-medium',
				step.status === 'completed-locked' || step.status === 'upcoming'
					? 'text-muted-foreground'
					: 'text-foreground'
			)}
		>
			{step.name}
		</span>
		{#if step.status === 'completed-locked'}
			<Lock class="text-muted-foreground ml-auto size-4 shrink-0" />
		{/if}
	</span>
{/snippet}

{#if isMobile.current}
	<Drawer.Root bind:open>
		<Drawer.Trigger
			class="group text-primary active:bg-primary/10 data-[state=open]:bg-primary/10 -mx-2 flex min-w-0 items-center justify-end gap-2 rounded-full px-2 py-1 text-sm font-medium transition-colors duration-75"
			aria-label={m.step_dropdown_open()}
		>
			{@render triggerInner()}
		</Drawer.Trigger>

		<!-- Capped short of full height so Chrome on iOS can't tuck the heading under its
			address bar (ADR-0049). -->
		<Drawer.Content class="data-[vaul-drawer-direction=bottom]:max-h-[75svh]">
			<Drawer.Header class="px-5 pt-2 pb-3 text-left">
				<Drawer.Title class="text-foreground text-lg font-bold">{heading}</Drawer.Title>
			</Drawer.Header>

			<div
				class="min-h-0 flex-1 overflow-y-auto overscroll-contain px-2 pb-[env(safe-area-inset-bottom)]"
			>
				{#each steps as step, index (step.id)}
					{#if step.href}
						<!-- Step hrefs come from workflow_step_url, so resolve() has nothing to check. -->
						<!-- eslint-disable svelte/no-navigation-without-resolve -->
						<a
							href={step.href}
							class="hover:bg-muted active:bg-muted flex min-h-14 items-center rounded-xl px-3 transition-colors"
							onclick={() => (open = false)}
						>
							{@render stepRow(step, index + 1)}
						</a>
						<!-- eslint-enable svelte/no-navigation-without-resolve -->
					{:else}
						<!-- Inert rather than absent: an unreachable step still tells you where you are. -->
						<div
							class="flex min-h-14 items-center px-3"
							aria-current={step === viewedStep ? 'step' : undefined}
						>
							{@render stepRow(step, index + 1)}
						</div>
					{/if}
				{/each}

				<div class="border-border mt-2 border-t pt-2 pb-2">
					<!-- Stays open so you can see the new mode and switch back. -->
					<button
						type="button"
						class="hover:bg-muted active:bg-muted text-foreground flex min-h-14 w-full items-center gap-3 rounded-xl px-3 text-left text-base transition-colors"
						onclick={() => themeStore.toggleMode()}
					>
						<ThemeIcon class="text-muted-foreground size-5 shrink-0 stroke-current" />
						{themeLabel}
					</button>
				</div>
			</div>
		</Drawer.Content>
	</Drawer.Root>
{:else}
	<!-- A soft veil keeps the menu reading as a layer above the step's own text. -->
	{#if open}
		<div
			class="motion-safe:animate-in motion-safe:fade-in-0 bg-background/20 fixed inset-0 z-40 backdrop-blur-[3px] duration-150"
			aria-hidden="true"
		></div>
	{/if}

	<DropdownMenu.Root bind:open>
		<DropdownMenu.Trigger
			class="group text-primary active:bg-primary/10 data-[state=open]:bg-primary/10 relative -mx-2 flex min-w-0 items-center justify-end gap-2 rounded-full px-2 py-1 text-base font-medium transition-colors duration-75 data-[state=open]:z-50"
			aria-label={m.step_dropdown_open()}
		>
			{@render triggerInner()}
		</DropdownMenu.Trigger>

		<!-- Faster than the shadcn default, which reads as lag on a menu you use to move. -->
		<DropdownMenu.Content align="end" class="animation-duration-100 w-80 p-2 shadow-xl">
			<DropdownMenu.Group>
				<DropdownMenu.GroupHeading class="text-sm">
					{heading}
				</DropdownMenu.GroupHeading>
				{#each steps as step, index (step.id)}
					{#if step.href}
						<DropdownMenu.Item class="py-2.5">
							{#snippet child({ props })}
								<!-- eslint-disable svelte/no-navigation-without-resolve -->
								<a
									{...props}
									href={step.href}
									class={cn(props.class as string, 'cursor-pointer')}
								>
									{@render stepRow(step, index + 1)}
								</a>
								<!-- eslint-enable svelte/no-navigation-without-resolve -->
							{/snippet}
						</DropdownMenu.Item>
					{:else}
						<!-- Inert rather than absent: an unreachable step still tells you where you are. -->
						<div
							class="px-2 py-2.5"
							aria-current={step === viewedStep ? 'step' : undefined}
						>
							{@render stepRow(step, index + 1)}
						</div>
					{/if}
				{/each}
			</DropdownMenu.Group>

			<DropdownMenu.Separator />
			<DropdownMenu.Item
				class="cursor-pointer py-2.5 text-base"
				onSelect={(event) => {
					event.preventDefault();
					themeStore.toggleMode();
				}}
			>
				<ThemeIcon class="text-muted-foreground size-4 stroke-current" />
				{themeLabel}
			</DropdownMenu.Item>
		</DropdownMenu.Content>
	</DropdownMenu.Root>
{/if}

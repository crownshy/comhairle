<script lang="ts">
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import * as Drawer from '$lib/components/ui/drawer';
	import { ChevronDown, Check, Lock, Moon, Sun } from 'lucide-svelte';
	import { IsMobile } from '$lib/hooks/is-mobile.svelte';
	import { themeStore } from '$lib/stores/theme.svelte';
	import { cn } from '$lib/utils';
	import { m } from '$lib/paraglide/messages';
	import { HEADER_PILL_CLASS } from './styles';
	import type { StepItem, StepStatus } from './stepItems';

	type Props = {
		steps: StepItem[];
		currentIndex: number;
	};

	let { steps, currentIndex }: Props = $props();

	type RowStyle = { badge: string; name: string; done: boolean; locked: boolean };

	const FILLED_BADGE_CLASS = 'bg-primary text-primary-foreground';
	const ROW_STYLES: Record<StepStatus, RowStyle> = {
		completed: {
			badge: FILLED_BADGE_CLASS,
			name: 'text-foreground',
			done: true,
			locked: false
		},
		'completed-locked': {
			badge: FILLED_BADGE_CLASS,
			name: 'text-muted-foreground',
			done: true,
			locked: true
		},
		current: { badge: FILLED_BADGE_CLASS, name: 'text-foreground', done: false, locked: false },
		upcoming: {
			badge: 'bg-muted text-muted-foreground',
			name: 'text-muted-foreground',
			done: false,
			locked: false
		}
	};

	// Both shells share one pill so the dropdown the server paints can become the sheet
	// on hydration without a visible change (ADR-0049).
	const TRIGGER_CLASS = cn(
		HEADER_PILL_CLASS,
		'group data-[state=open]:bg-primary/10 min-w-0 justify-end'
	);

	const isMobile = new IsMobile();

	let open = $state(false);

	let viewedStep = $derived(steps[currentIndex]);
	let heading = $derived(m.step_x_of_y({ current: currentIndex + 1, total: steps.length }));
	let label = $derived(viewedStep ? `${heading}: ${viewedStep.name}` : heading);

	let themeLabel = $derived(themeStore.isDark ? m.theme_light_mode() : m.theme_dark_mode());
	let ThemeIcon = $derived(themeStore.isDark ? Sun : Moon);

	function ariaCurrent(step: StepItem) {
		return step === viewedStep ? 'step' : undefined;
	}
</script>

{#snippet triggerInner()}
	<span class="truncate">{label}</span>
	<ChevronDown
		class="size-5 shrink-0 transition-transform duration-150 group-data-[state=open]:rotate-180"
		aria-hidden="true"
	/>
{/snippet}

{#snippet stepRow(step: StepItem, position: number)}
	{@const style = ROW_STYLES[step.status]}
	<span class="flex w-full items-center gap-3">
		<span
			class={cn(
				'flex size-6 shrink-0 items-center justify-center rounded-full text-sm font-medium tabular-nums',
				style.badge
			)}
		>
			{#if style.done}
				<!-- `text-current` opts out of the menu item's blanket muted colour for icons. -->
				<Check class="size-3.5 text-current" strokeWidth={3} />
			{:else}
				{position}
			{/if}
		</span>
		<span class={cn('min-w-0 truncate text-base font-medium', style.name)}>
			{step.name}
		</span>
		{#if style.locked}
			<Lock class="text-muted-foreground ml-auto size-4 shrink-0" />
		{/if}
	</span>
{/snippet}

{#snippet themeRowInner()}
	<ThemeIcon class="text-muted-foreground size-5 shrink-0 stroke-current" />
	{themeLabel}
{/snippet}

<!-- eslint-disable svelte/no-navigation-without-resolve -->

{#if isMobile.current}
	<Drawer.Root bind:open>
		<Drawer.Trigger class={TRIGGER_CLASS}>
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
						<a
							href={step.href}
							class="hover:bg-muted active:bg-muted flex min-h-14 items-center rounded-xl px-3 transition-colors"
							aria-current={ariaCurrent(step)}
							onclick={() => (open = false)}
						>
							{@render stepRow(step, index + 1)}
						</a>
					{:else}
						<!-- Inert rather than absent: an unreachable step still tells you where you are. -->
						<div
							class="flex min-h-14 items-center px-3"
							aria-current={ariaCurrent(step)}
						>
							{@render stepRow(step, index + 1)}
						</div>
					{/if}
				{/each}

				<div class="border-border mt-2 border-t pt-2 pb-2">
					<button
						type="button"
						class="hover:bg-muted active:bg-muted text-foreground flex min-h-14 w-full items-center gap-3 rounded-xl px-3 text-left text-base transition-colors"
						onclick={() => themeStore.toggleMode()}
					>
						{@render themeRowInner()}
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
		<DropdownMenu.Trigger class={cn(TRIGGER_CLASS, 'relative data-[state=open]:z-50')}>
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
								<a
									{...props}
									href={step.href}
									class={cn(props.class as string, 'cursor-pointer')}
									aria-current={ariaCurrent(step)}
								>
									{@render stepRow(step, index + 1)}
								</a>
							{/snippet}
						</DropdownMenu.Item>
					{:else}
						<!-- Disabled rather than absent: an unreachable step still tells you where you are.
							Full opacity because the current step is one of these. -->
						<DropdownMenu.Item
							disabled
							class="py-2.5 data-[disabled]:opacity-100"
							aria-current={ariaCurrent(step)}
						>
							{@render stepRow(step, index + 1)}
						</DropdownMenu.Item>
					{/if}
				{/each}
			</DropdownMenu.Group>

			<DropdownMenu.Separator />
			<!-- Stays open so you can see the new mode and switch back. -->
			<DropdownMenu.Item
				class="cursor-pointer py-2.5 text-base"
				onSelect={(event) => {
					event.preventDefault();
					themeStore.toggleMode();
				}}
			>
				{@render themeRowInner()}
			</DropdownMenu.Item>
		</DropdownMenu.Content>
	</DropdownMenu.Root>
{/if}

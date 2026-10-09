<script lang="ts">
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import * as Drawer from '$lib/components/ui/drawer';
	import {
		ChevronDown,
		ChevronRight,
		Check,
		CircleHelp,
		ShieldCheck,
		Languages,
		Lock,
		Moon,
		Sun
	} from 'lucide-svelte';
	import { tick } from 'svelte';
	import { Spinner } from '$lib/components/ui/spinner';
	import { IsMobile } from '$lib/hooks/is-mobile.svelte';
	import { themeStore } from '$lib/stores/theme.svelte';
	import { cn } from '$lib/utils';
	import { m } from '$lib/paraglide/messages';
	import { HEADER_PILL_CLASS } from './styles';
	import { getLocale, locales, type Locale } from '$lib/paraglide/runtime';
	import { getLanguageName } from '$lib/config/languages';
	import { switchLocale } from '$lib/utils/locale';
	import { useSupportDrawer, type SupportTab } from '$lib/components/supportDrawerContext.svelte';
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

	// Opened in a new tab so reading them doesn't take the participant out of their step. The
	// privacy policy is the conversation's own, so it lives in Find out more instead.
	const LEGAL_LINKS = [
		{ href: '/rights/tos', label: m.terms_of_service },
		{ href: '/rights/cookies', label: m.cookies_settings }
	];

	const SHEET_ROW_CLASS =
		'hover:bg-muted active:bg-muted text-foreground flex min-h-14 w-full items-center gap-3 rounded-xl px-3 text-left text-base transition-colors';

	const isMobile = new IsMobile();

	let open = $state(false);

	let viewedStep = $derived(steps[currentIndex]);
	let heading = $derived(m.step_x_of_y({ current: currentIndex + 1, total: steps.length }));
	let label = $derived(viewedStep ? `${heading}: ${viewedStep.name}` : heading);

	let themeLabel = $derived(themeStore.isDark ? m.theme_light_mode() : m.theme_dark_mode());
	let ThemeIcon = $derived(themeStore.isDark ? Sun : Moon);

	const currentLocale = getLocale();
	let languageListOpen = $state(false);
	let pendingLocale = $state<Locale | null>(null);
	let shownLocale = $derived(pendingLocale ?? currentLocale);

	// Switching reloads the whole page, so show the spinner first and let it paint.
	async function chooseLanguage(locale: Locale) {
		if (locale === currentLocale || pendingLocale) return;
		pendingLocale = locale;
		await tick();
		await new Promise((resolve) => requestAnimationFrame(() => requestAnimationFrame(resolve)));
		switchLocale(locale);
	}

	const supportDrawer = useSupportDrawer();

	function openSupport(tab: SupportTab) {
		open = false;
		supportDrawer.openOn(tab);
	}

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

{#snippet languageRowInner()}
	<Languages class="text-muted-foreground size-5 shrink-0" />
	<span>{m.language()}</span>
	<span class="text-muted-foreground ml-auto flex items-center gap-2">
		{#if pendingLocale}
			<Spinner class="size-4" />
		{/if}
		{getLanguageName(shownLocale, 'native')}
	</span>
{/snippet}

{#snippet faqRowInner()}
	<CircleHelp class="text-muted-foreground size-5 shrink-0" />
	{m.faq()}
{/snippet}

{#snippet privacyRowInner()}
	<ShieldCheck class="text-muted-foreground size-5 shrink-0" />
	{m.privacy_policy()}
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

				<div class="border-border mt-2 border-t pt-2">
					<p class="text-foreground px-3 pt-2 pb-1 text-base font-semibold">
						{m.step_menu_about_heading()}
					</p>
					<button
						type="button"
						class={SHEET_ROW_CLASS}
						onclick={() => openSupport('faqs')}
					>
						{@render faqRowInner()}
					</button>
					<button
						type="button"
						class={SHEET_ROW_CLASS}
						onclick={() => openSupport('privacyPolicy')}
					>
						{@render privacyRowInner()}
					</button>
				</div>

				<div class="border-border mt-2 border-t pt-2">
					<button
						type="button"
						class={SHEET_ROW_CLASS}
						onclick={() => themeStore.toggleMode()}
					>
						{@render themeRowInner()}
					</button>
					<button
						type="button"
						class={SHEET_ROW_CLASS}
						aria-expanded={languageListOpen}
						onclick={() => (languageListOpen = !languageListOpen)}
					>
						{@render languageRowInner()}
						<ChevronRight
							class={cn(
								'text-muted-foreground size-5 shrink-0 transition-transform',
								languageListOpen && 'rotate-90'
							)}
						/>
					</button>
					{#if languageListOpen}
						<div class="pl-8">
							{#each locales as locale (locale)}
								<button
									type="button"
									class={SHEET_ROW_CLASS}
									aria-current={locale === shownLocale ? 'true' : undefined}
									disabled={pendingLocale !== null}
									onclick={() => chooseLanguage(locale)}
								>
									{getLanguageName(locale, 'native')}
									{#if locale === pendingLocale}
										<Spinner class="text-primary ml-auto size-5 shrink-0" />
									{:else if locale === shownLocale}
										<Check class="text-primary ml-auto size-5 shrink-0" />
									{/if}
								</button>
							{/each}
						</div>
					{/if}
				</div>

				<div class="border-border mt-2 border-t pt-2 pb-2">
					{#each LEGAL_LINKS as link (link.href)}
						<a
							href={link.href}
							target="_blank"
							rel="noopener"
							class={cn(SHEET_ROW_CLASS, 'text-muted-foreground min-h-12')}
						>
							{link.label()}
						</a>
					{/each}
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
			<DropdownMenu.Group>
				<DropdownMenu.GroupHeading class="text-sm">
					{m.step_menu_about_heading()}
				</DropdownMenu.GroupHeading>
				<DropdownMenu.Item
					class="cursor-pointer py-2.5 text-base"
					onSelect={() => openSupport('faqs')}
				>
					{@render faqRowInner()}
				</DropdownMenu.Item>
				<DropdownMenu.Item
					class="cursor-pointer py-2.5 text-base"
					onSelect={() => openSupport('privacyPolicy')}
				>
					{@render privacyRowInner()}
				</DropdownMenu.Item>
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
			<DropdownMenu.Sub>
				<DropdownMenu.SubTrigger class="cursor-pointer gap-2 py-2.5 text-base">
					{@render languageRowInner()}
				</DropdownMenu.SubTrigger>
				<DropdownMenu.SubContent class="w-56 p-2">
					<DropdownMenu.RadioGroup
						value={shownLocale}
						onValueChange={(value) => chooseLanguage(value as Locale)}
					>
						{#each locales as locale (locale)}
							<DropdownMenu.RadioItem
								value={locale}
								closeOnSelect={false}
								disabled={pendingLocale !== null && locale !== pendingLocale}
								class="cursor-pointer py-2.5 text-base"
							>
								{getLanguageName(locale, 'native')}
								{#if locale === pendingLocale}
									<Spinner class="ml-auto size-4" />
								{/if}
							</DropdownMenu.RadioItem>
						{/each}
					</DropdownMenu.RadioGroup>
				</DropdownMenu.SubContent>
			</DropdownMenu.Sub>

			<DropdownMenu.Separator />
			{#each LEGAL_LINKS as link (link.href)}
				<DropdownMenu.Item class="text-muted-foreground cursor-pointer py-2 text-base">
					{#snippet child({ props })}
						<a {...props} href={link.href} target="_blank" rel="noopener">
							{link.label()}
						</a>
					{/snippet}
				</DropdownMenu.Item>
			{/each}
		</DropdownMenu.Content>
	</DropdownMenu.Root>
{/if}

<script lang="ts">
	import { resolve } from '$app/paths';
	import { page } from '$app/state';
	import { afterNavigate, replaceState } from '$app/navigation';
	import { Button, buttonVariants } from '$lib/components/ui/button';
	import * as Dialog from '$lib/components/ui/dialog';
	import Callout from '$lib/components/Callout.svelte';
	import type { RejectReason } from '$lib/moderation/moderationPolicy';
	import { readStorage, writeStorage } from '$lib/utils/browserStorage';
	import { cn } from '$lib/utils';
	import BookOpen from '@lucide/svelte/icons/book-open';
	import Check from '@lucide/svelte/icons/check';
	import Info from '@lucide/svelte/icons/info';
	import ShieldAlert from '@lucide/svelte/icons/shield-alert';
	import X from '@lucide/svelte/icons/x';
	import ModerationPractice from './ModerationPractice.svelte';
	import { showCloseHint } from './rulesCloseHint';
	import {
		FROM_EMAIL,
		FROM_PARAM,
		MODERATION_GUIDE_PARAMS,
		MODERATION_HEADLINE,
		MODERATION_PAIRS,
		MODERATION_WHY
	} from './moderationGuidance';

	type Props = {
		/** This conversation's reasons, from its moderation policy. */
		rejectReasons: RejectReason[];
		/** The platform's default reasons. The practice round judges against these, so its
		 * feedback stays right whatever an organiser renames or removes in their policy. */
		defaultRejectReasons: RejectReason[];
	};

	let { rejectReasons, defaultRejectReasons }: Props = $props();

	const SECTIONS = [
		{ id: 'moderation-rule', label: 'What’s important' },
		{ id: 'moderation-do-never', label: 'Do and never' },
		{ id: 'moderation-reasons', label: 'Reasons for rejecting' },
		{ id: 'moderation-practice', label: 'Let’s practise' }
	];
	let activeSection = $state(SECTIONS[0].id);
	let scroller = $state<HTMLElement | null>(null);

	function jumpTo(id: string) {
		activeSection = id;
		document.getElementById(id)?.scrollIntoView({ behavior: 'smooth', block: 'start' });
	}

	// Highlights the last section whose heading has scrolled under the quick links.
	function trackActiveSection() {
		if (!scroller) return;
		const top = scroller.getBoundingClientRect().top + 80;
		const atBottom = scroller.scrollTop + scroller.clientHeight >= scroller.scrollHeight - 4;
		if (atBottom) {
			activeSection = SECTIONS[SECTIONS.length - 1].id;
			return;
		}
		for (const section of SECTIONS) {
			const el = document.getElementById(section.id);
			if (el && el.getBoundingClientRect().top <= top) activeSection = section.id;
		}
	}

	// Opens by itself on a moderator's first visit, and whenever they arrive from the
	// "statements are waiting" email. Otherwise it's one click away.
	// Remembered per browser; private windows just show it again.
	const SEEN_KEY = 'comhairle:moderation-rules-seen';
	let open = $state(false);

	// After navigation rather than on mount: replaceState needs the router to be ready.
	afterNavigate(() => {
		const fromEmail = page.url.searchParams.get(FROM_PARAM) === FROM_EMAIL;
		if (fromEmail) {
			// Drop the marker so a reload doesn't open the rules again. Only the query string
			// of the current page changes, so there is nothing for `resolve()` to do.
			const url = new URL(page.url);
			url.searchParams.delete(FROM_PARAM);
			// eslint-disable-next-line svelte/no-navigation-without-resolve
			replaceState(url, page.state);
		}
		if (fromEmail || readStorage(SEEN_KEY) !== 'true') open = true;
	});

	let triggerEl = $state<HTMLElement | null>(null);

	function onOpenChange(next: boolean) {
		if (next) {
			activeSection = SECTIONS[0].id;
			return;
		}
		writeStorage(SEEN_KEY, 'true');
		showCloseHint(triggerEl);
	}

	function finish() {
		onOpenChange(false);
		open = false;
	}
</script>

{#snippet sectionHeading(number: number, title: string)}
	<h3 class="flex items-center gap-3 text-xl font-semibold">
		<span
			class="bg-primary/10 text-primary flex size-8 items-center justify-center rounded-full text-base tabular-nums"
		>
			{number}
		</span>
		{title}
	</h3>
{/snippet}

<Dialog.Root bind:open {onOpenChange}>
	<Dialog.Trigger
		bind:ref={triggerEl}
		data-tour="moderation-guidance"
		class={cn(
			buttonVariants({ variant: 'ghost' }),
			'text-primary text-base font-medium underline underline-offset-4'
		)}
	>
		<ShieldAlert class="size-4" />
		Moderation rules
	</Dialog.Trigger>
	<Dialog.Content
		class="flex h-[calc(100dvh-2rem)] max-h-[56rem] flex-col gap-0 overflow-hidden p-0 sm:max-w-5xl"
	>
		<Dialog.Header class="gap-1 border-b px-6 pt-6 pb-4 text-start">
			<Dialog.Title class="text-2xl">Moderation rules</Dialog.Title>
			<Dialog.Description class="text-base">
				How to moderate fairly. Read it through once, or jump to what you need.
			</Dialog.Description>
			<nav aria-label="Moderation rules sections" class="-mx-1 mt-3 overflow-x-auto px-1">
				<ul class="flex gap-2">
					{#each SECTIONS as section, i (section.id)}
						<li>
							<button
								type="button"
								onclick={() => jumpTo(section.id)}
								aria-current={activeSection === section.id ? 'true' : undefined}
								class={[
									'flex items-center gap-2 rounded-full border px-3 py-1.5 text-base whitespace-nowrap transition-colors',
									activeSection === section.id
										? 'border-primary bg-primary text-primary-foreground'
										: 'hover:bg-muted'
								]}
							>
								<span class="tabular-nums opacity-70">{i + 1}</span>
								{section.label}
							</button>
						</li>
					{/each}
				</ul>
			</nav>
		</Dialog.Header>

		<div
			bind:this={scroller}
			onscroll={trackActiveSection}
			class="flex flex-1 flex-col gap-20 overflow-y-auto px-8 pt-10 pb-16"
		>
			<section id={SECTIONS[0].id} class="flex scroll-mt-10 flex-col gap-6">
				{@render sectionHeading(1, SECTIONS[0].label)}
				<!-- Softer than Callout's default, to match the Do and Never tints below. -->
				<Callout class="border-primary/20! bg-primary/5! flex items-start gap-3 px-5 py-4">
					<Info class="text-primary mt-1 size-5 shrink-0" />
					<div class="flex flex-col gap-1">
						<p class="text-lg font-semibold">{MODERATION_HEADLINE}</p>
						<p class="text-base">{MODERATION_WHY}</p>
					</div>
				</Callout>
			</section>

			<section id={SECTIONS[1].id} class="flex scroll-mt-10 flex-col gap-6">
				{@render sectionHeading(2, SECTIONS[1].label)}
				<div>
					<div
						class="hidden grid-cols-[11rem_1fr_1fr] gap-x-4 text-base font-semibold sm:grid"
					>
						<span></span>
						<span
							class="bg-primary/5 text-primary flex items-center gap-2 rounded-t-lg px-4 pt-4 pb-3"
						>
							<Check class="size-4" /> Do
						</span>
						<span
							class="bg-destructive/5 text-destructive flex items-center gap-2 rounded-t-lg px-4 pt-4 pb-3"
						>
							<X class="size-4" /> Never
						</span>
					</div>
					<ul>
						{#each MODERATION_PAIRS as pair (pair.topic)}
							<li
								class="group border-border/60 grid gap-y-2 border-t py-4 sm:grid-cols-[11rem_1fr_1fr] sm:gap-x-4 sm:gap-y-0 sm:py-0"
							>
								<p
									class="text-muted-foreground text-base font-medium sm:py-5 sm:pr-2"
								>
									{pair.topic}
								</p>
								<p
									class="bg-primary/5 flex items-start gap-2 rounded-lg px-4 py-3 text-base leading-7 sm:rounded-none sm:py-5 sm:group-last:rounded-b-lg"
								>
									<Check
										class="text-primary mt-1.5 size-4 shrink-0 sm:hidden"
										aria-label="Do"
									/>
									{pair.do}
								</p>
								<p
									class="bg-destructive/5 flex items-start gap-2 rounded-lg px-4 py-3 text-base leading-7 sm:rounded-none sm:py-5 sm:group-last:rounded-b-lg"
								>
									<X
										class="text-destructive mt-1.5 size-4 shrink-0 sm:hidden"
										aria-label="Never"
									/>
									{pair.never}
								</p>
							</li>
						{/each}
					</ul>
				</div>
			</section>

			<section id={SECTIONS[2].id} class="flex scroll-mt-10 flex-col gap-6">
				{@render sectionHeading(3, SECTIONS[2].label)}
				<p class="text-muted-foreground text-base">
					Pick one of these for every reject, so decisions can be checked later. If a
					statement doesn’t fit any of them, accept it.
				</p>
				<dl class="grid gap-2 sm:grid-cols-2">
					{#each rejectReasons as reason (reason.label)}
						<div class="flex flex-col gap-0.5 rounded-lg border p-3">
							<dt class="text-base font-semibold">{reason.label}</dt>
							{#if reason.description}
								<dd class="text-muted-foreground text-base">
									{reason.description}
								</dd>
							{/if}
						</div>
					{/each}
				</dl>
			</section>

			<section id={SECTIONS[3].id} class="flex scroll-mt-10 flex-col gap-6">
				{@render sectionHeading(4, SECTIONS[3].label)}
				<p class="text-muted-foreground text-base">
					This is the real moderation list with made-up statements. Accept or reject each
					one and see if you made the right call.
				</p>
				<ModerationPractice rejectReasons={defaultRejectReasons} />
			</section>
		</div>

		<Dialog.Footer class="flex-row items-center gap-2 border-t px-6 py-4 sm:justify-between">
			<a
				href={resolve(
					'/(admin)/admin/info/how-to/[guide_id]/[topic_id]',
					MODERATION_GUIDE_PARAMS
				)}
				target="_blank"
				rel="noopener"
				class={buttonVariants({ variant: 'ghost' })}
			>
				<BookOpen class="size-4" />
				Read the full guide
			</a>
			<Button onclick={finish}>Start moderating</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>

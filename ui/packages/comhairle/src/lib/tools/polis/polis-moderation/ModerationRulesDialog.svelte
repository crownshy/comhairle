<script lang="ts">
	import type { Snippet } from 'svelte';
	import type { Attachment } from 'svelte/attachments';
	import { resolve } from '$app/paths';
	import * as Dialog from '$lib/components/ui/dialog';
	import { Button } from '$lib/components/ui/button';
	import type { RejectReason } from '$lib/moderation/moderationPolicy';
	import { cn } from '$lib/utils';
	import { scrollBehavior } from '$lib/utils/reducedMotion';
	import BookOpen from '@lucide/svelte/icons/book-open';
	import Check from '@lucide/svelte/icons/check';
	import Info from '@lucide/svelte/icons/info';
	import X from '@lucide/svelte/icons/x';
	import ModerationPractice from './ModerationPractice.svelte';
	import { activeSectionIndex } from './activeSection';
	import {
		MODERATION_GUIDE_PARAMS,
		MODERATION_HEADLINE,
		MODERATION_HEADLINE_REASON,
		MODERATION_RULES
	} from './moderationGuidance';

	type Props = {
		open: boolean;
		/** The button that opens the dialog. Spread `props` onto it so focus returns there on close. */
		trigger: Snippet<[{ props: Record<string, unknown> }]>;
		/** The step's moderation policy reasons. */
		rejectReasons: RejectReason[];
		conversationId: string;
		/** Shows a link to edit the moderation policy. */
		canEditPolicy: boolean;
		/** Runs once the dialog has finished closing. */
		onClosed?: () => void;
	};

	let {
		open = $bindable(),
		trigger,
		rejectReasons,
		conversationId,
		canEditPolicy,
		onClosed
	}: Props = $props();

	const SECTIONS = [
		{ id: 'important', label: 'What’s important' },
		{ id: 'do-and-never', label: 'Do and never' },
		{ id: 'reasons', label: 'Reasons for rejecting' },
		{ id: 'practise', label: 'Let’s practise' }
	];

	const instanceId = $props.id();
	const sectionId = (id: string) => `${instanceId}-${id}`;

	let scroller = $state<HTMLElement>();
	let sectionElements = $state<HTMLElement[]>([]);
	let activeIndex = $state(0);

	// Sections are direct children of the `relative` scroller, so offsetTop is measured from its top.
	function updateActiveSection() {
		if (!scroller) return;
		activeIndex = activeSectionIndex(
			sectionElements.map((section) => section.offsetTop),
			scroller
		);
	}

	const trackActiveSection: Attachment<HTMLElement> = (element) => {
		scroller = element;
		updateActiveSection();
		element.addEventListener('scroll', updateActiveSection, { passive: true });
		return () => element.removeEventListener('scroll', updateActiveSection);
	};

	function jumpTo(index: number) {
		const section = sectionElements[index];
		if (!scroller || !section) return;
		activeIndex = index;
		scroller.scrollTo({ top: section.offsetTop, behavior: scrollBehavior() });
	}

	// The pill row scrolls sideways on a phone, so keep the current pill in view.
	const revealActivePill: Attachment<HTMLElement> = (pill) => {
		pill.scrollIntoView({ inline: 'center', block: 'nearest', behavior: scrollBehavior() });
	};
</script>

{#snippet sectionHeading(index: number)}
	<h3
		id="{sectionId(SECTIONS[index].id)}-title"
		class="flex items-center gap-3 text-2xl font-semibold"
	>
		<span
			class="bg-primary/10 text-primary flex size-10 shrink-0 items-center justify-center rounded-full text-lg tabular-nums"
			aria-hidden="true"
		>
			{index + 1}
		</span>
		{SECTIONS[index].label}
	</h3>
{/snippet}

<Dialog.Root
	bind:open
	onOpenChangeComplete={(isOpen) => {
		if (!isOpen) onClosed?.();
	}}
>
	<Dialog.Trigger>
		{#snippet child({ props })}
			{@render trigger({ props })}
		{/snippet}
	</Dialog.Trigger>
	<Dialog.Content
		class="flex h-[calc(100dvh-2rem)] max-h-[60rem] flex-col gap-0 p-0 sm:max-w-5xl"
	>
		<Dialog.Header class="gap-4 border-b px-6 pt-6 pb-4 text-start md:px-8">
			<div class="flex flex-col gap-1 pe-8">
				<Dialog.Title class="text-2xl">Moderation rules</Dialog.Title>
				<Dialog.Description class="text-base">
					How to moderate fairly. Read it through once, or jump to what you need.
				</Dialog.Description>
			</div>
			<nav
				aria-label="Moderation rules sections"
				class="-mx-6 overflow-x-auto px-6 [scrollbar-width:none] md:-mx-8 md:px-8"
			>
				<ul class="flex w-max gap-2">
					{#each SECTIONS as section, index (section.id)}
						{@const active = index === activeIndex}
						<li>
							<Button
								variant={active ? 'default' : 'outline'}
								onclick={() => jumpTo(index)}
								aria-current={active ? 'location' : undefined}
								class={cn(
									'min-h-11 px-4 text-base',
									!active && 'text-foreground border-border hover:text-foreground'
								)}
								{@attach active && revealActivePill}
							>
								<span
									class={cn('tabular-nums', !active && 'text-muted-foreground')}
								>
									{index + 1}
								</span>
								{section.label}
							</Button>
						</li>
					{/each}
				</ul>
			</nav>
		</Dialog.Header>

		<div
			class="relative flex min-h-0 flex-1 flex-col gap-14 overflow-y-auto px-6 py-8 md:px-8"
			{@attach trackActiveSection}
		>
			<section
				bind:this={sectionElements[0]}
				aria-labelledby="{sectionId('important')}-title"
				class="flex flex-col gap-5"
			>
				{@render sectionHeading(0)}
				<div class="bg-primary/5 flex items-start gap-3 rounded-lg px-5 py-4">
					<Info class="text-primary mt-1 size-5 shrink-0" />
					<div class="flex flex-col gap-1">
						<p class="text-lg font-semibold">{MODERATION_HEADLINE}</p>
						<p class="text-base">{MODERATION_HEADLINE_REASON}</p>
					</div>
				</div>
			</section>

			<section
				bind:this={sectionElements[1]}
				aria-labelledby="{sectionId('do-and-never')}-title"
				class="flex flex-col gap-5"
			>
				{@render sectionHeading(1)}
				<!-- A table from md up; below that each row stacks its Do and Never. Not the shadcn
				     Table: its row hover tints every cell over the Do/Never colours, and it won't wrap. -->
				<table class="block w-full border-collapse text-start md:table">
					<thead class="sr-only md:not-sr-only md:table-header-group">
						<tr>
							<td class="w-48"></td>
							<th
								scope="col"
								class="bg-primary/5 text-primary rounded-t-lg px-5 py-4 text-start text-lg font-semibold"
							>
								<span class="flex items-center gap-2"
									><Check class="size-5" />Do</span
								>
							</th>
							<th
								scope="col"
								class="bg-destructive/5 text-destructive rounded-t-lg px-5 py-4 text-start text-lg font-semibold"
							>
								<span class="flex items-center gap-2"
									><X class="size-5" />Never</span
								>
							</th>
						</tr>
					</thead>
					<tbody class="block md:table-row-group">
						{#each MODERATION_RULES as rule (rule.topic)}
							<tr class="flex flex-col gap-2 border-t py-4 md:table-row md:py-0">
								<th
									scope="row"
									class="text-muted-foreground text-start text-base font-medium md:py-5 md:pe-6 md:align-top"
								>
									{rule.topic}
								</th>
								<td
									class="bg-primary/5 rounded-lg px-4 py-3 text-base leading-7 md:rounded-none md:px-5 md:py-5 md:align-top"
								>
									<span
										class="text-primary flex items-center gap-1 font-semibold md:hidden"
									>
										<Check class="size-4" />Do
									</span>
									{rule.do}
								</td>
								<td
									class="bg-destructive/5 rounded-lg px-4 py-3 text-base leading-7 md:rounded-none md:px-5 md:py-5 md:align-top"
								>
									<span
										class="text-destructive flex items-center gap-1 font-semibold md:hidden"
									>
										<X class="size-4" />Never
									</span>
									{rule.never}
								</td>
							</tr>
						{/each}
					</tbody>
				</table>
			</section>

			<section
				bind:this={sectionElements[2]}
				aria-labelledby="{sectionId('reasons')}-title"
				class="flex flex-col gap-5"
			>
				{@render sectionHeading(2)}
				{#if rejectReasons.length > 0}
					<p class="text-base">
						Pick one of these for every reject, so decisions can be checked later. If a
						statement doesn’t fit any of them, accept it.
					</p>
					<dl class="grid gap-3 sm:grid-cols-2">
						{#each rejectReasons as reason (reason.label)}
							<div class="bg-card flex flex-col gap-1 rounded-lg border p-4">
								<dt class="text-base font-semibold">{reason.label}</dt>
								{#if reason.description}
									<dd class="text-muted-foreground text-base">
										{reason.description}
									</dd>
								{/if}
							</div>
						{/each}
					</dl>
				{:else}
					<p class="text-base">
						This step’s moderation policy has no reasons for rejecting yet. Add a note
						to every reject so decisions can be checked later.
					</p>
				{/if}
				{#if canEditPolicy}
					<Button
						variant="link"
						class="h-auto self-start p-0 text-base"
						href={resolve(
							'/(admin)/admin/conversations/[conversation_id]/configure/moderation-policy',
							{ conversation_id: conversationId }
						)}
					>
						Edit the moderation policy
					</Button>
				{/if}
			</section>

			<section
				bind:this={sectionElements[3]}
				aria-labelledby="{sectionId('practise')}-title"
				class="flex flex-col gap-5"
			>
				{@render sectionHeading(3)}
				<p class="text-base">
					This is the real moderation list with made-up statements. Accept or reject each
					one and see if you made the right call.
				</p>
				<ModerationPractice {rejectReasons} />
			</section>
		</div>

		<Dialog.Footer
			class="flex-row items-center justify-between gap-3 border-t px-6 py-4 md:px-8"
		>
			<Button
				variant="ghost"
				class="text-base"
				href={resolve(
					'/(admin)/admin/info/how-to/[guide_id]/[topic_id]',
					MODERATION_GUIDE_PARAMS
				)}
				target="_blank"
				rel="noopener"
			>
				<BookOpen class="size-4" />
				Read the full guide
			</Button>
			<Button class="text-base" onclick={() => (open = false)}>Start moderating</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>

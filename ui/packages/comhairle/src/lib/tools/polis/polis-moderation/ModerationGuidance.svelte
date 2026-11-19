<script lang="ts">
	import { resolve } from '$app/paths';
	import { Button, buttonVariants } from '$lib/components/ui/button';
	import * as Sheet from '$lib/components/ui/sheet';
	import Callout from '$lib/components/Callout.svelte';
	import type { RejectReason } from '$lib/moderation/moderationPolicy';
	import { cn } from '$lib/utils';
	import BookOpen from '@lucide/svelte/icons/book-open';
	import Check from '@lucide/svelte/icons/check';
	import ShieldAlert from '@lucide/svelte/icons/shield-alert';
	import ShieldCheck from '@lucide/svelte/icons/shield-check';
	import X from '@lucide/svelte/icons/x';
	import {
		MODERATION_DO,
		MODERATION_GUIDE_PARAMS,
		MODERATION_HEADLINE,
		MODERATION_NEVER
	} from './moderationGuidance';

	type Props = {
		rejectReasons: RejectReason[];
		conversationId: string;
		/** Shows a link to edit the moderation policy. */
		canEditPolicy: boolean;
		/** Only the "Moderation rules" link, for once the banner has been hidden. */
		compact?: boolean;
		/** Hides the banner. Without it there is no close button. */
		onDismiss?: () => void;
	};

	let {
		rejectReasons,
		conversationId,
		canEditPolicy,
		compact = false,
		onDismiss
	}: Props = $props();
</script>

{#snippet controls()}
	<div class="flex items-center gap-1">
		<Sheet.Root>
			<Sheet.Trigger
				class={cn(
					buttonVariants({ variant: 'ghost', size: 'sm' }),
					'text-base font-medium underline underline-offset-4',
					compact ? 'text-primary' : 'hover:bg-amber-100 dark:hover:bg-amber-900/40'
				)}
			>
				{#if compact}
					<ShieldAlert class="size-4" />
				{/if}
				Moderation rules
			</Sheet.Trigger>
			<Sheet.Content side="right" class="flex w-full flex-col gap-0 p-0 sm:max-w-2xl">
				<Sheet.Header class="border-b px-6 py-5">
					<Sheet.Title class="text-2xl">Moderation rules</Sheet.Title>
					<Sheet.Description class="text-base">
						Moderators decide what participants get to vote on. If people suspect
						moderation is one-sided, they stop trusting the results. These rules keep it
						fair.
					</Sheet.Description>
				</Sheet.Header>

				<div class="flex flex-1 flex-col gap-8 overflow-y-auto px-6 py-6">
					<div class="grid gap-4 md:grid-cols-2">
						<section class="bg-card flex flex-col gap-3 rounded-xl border p-4">
							<h3 class="flex items-center gap-2 text-lg font-semibold">
								<span
									class="bg-primary/15 text-primary flex size-7 items-center justify-center rounded-full"
								>
									<Check class="size-4" />
								</span>
								Do
							</h3>
							<ul class="flex flex-col gap-3">
								{#each MODERATION_DO as rule (rule)}
									<li class="text-base leading-6">{rule}</li>
								{/each}
							</ul>
						</section>
						<section
							class="border-destructive/30 bg-destructive/5 flex flex-col gap-3 rounded-xl border p-4"
						>
							<h3 class="flex items-center gap-2 text-lg font-semibold">
								<span
									class="bg-destructive/15 text-destructive flex size-7 items-center justify-center rounded-full"
								>
									<X class="size-4" />
								</span>
								Never
							</h3>
							<ul class="flex flex-col gap-3">
								{#each MODERATION_NEVER as rule (rule)}
									<li class="text-base leading-6">{rule}</li>
								{/each}
							</ul>
						</section>
					</div>

					{#if rejectReasons.length > 0}
						<section class="flex flex-col gap-3">
							<div class="flex flex-col gap-1">
								<h3 class="text-lg font-semibold">Your reasons for rejecting</h3>
								<p class="text-muted-foreground text-base">
									From this conversation's moderation policy. If a statement
									doesn't fit one of these, accept it.
								</p>
							</div>
							<dl class="grid gap-3 sm:grid-cols-2">
								{#each rejectReasons as reason (reason.label)}
									<div class="bg-muted/50 flex flex-col gap-1 rounded-lg p-3">
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
					{/if}
				</div>

				<Sheet.Footer class="flex-row flex-wrap gap-2 border-t px-6 py-4 sm:justify-start">
					<a
						href={resolve(
							'/(admin)/admin/info/how-to/[guide_id]/[topic_id]',
							MODERATION_GUIDE_PARAMS
						)}
						target="_blank"
						rel="noopener"
						class={buttonVariants({ variant: 'default' })}
					>
						<BookOpen class="size-4" />
						Read the full guide
					</a>
					{#if canEditPolicy}
						<a
							href={resolve(
								'/(admin)/admin/conversations/[conversation_id]/configure/moderation-policy',
								{ conversation_id: conversationId }
							)}
							class={buttonVariants({ variant: 'outline' })}
						>
							<ShieldCheck class="size-4" />
							Edit moderation policy
						</a>
					{/if}
				</Sheet.Footer>
			</Sheet.Content>
		</Sheet.Root>
		{#if onDismiss && !compact}
			<Button
				variant="ghost"
				size="icon"
				onclick={onDismiss}
				aria-label="Hide this tip"
				title="Hide this tip"
				class="size-9 hover:bg-amber-100 dark:hover:bg-amber-900/40"
			>
				<X class="size-5" />
			</Button>
		{/if}
	</div>
{/snippet}

{#if compact}
	<div data-tour="moderation-guidance" class="flex items-center">
		{@render controls()}
	</div>
{:else}
	<Callout
		variant="warning"
		data-tour="moderation-guidance"
		class="flex flex-wrap items-center justify-between gap-x-4 gap-y-2 px-4 py-2"
	>
		<p class="flex items-center gap-2 text-base">
			<ShieldAlert class="size-4 shrink-0" />
			{MODERATION_HEADLINE}
		</p>
		{@render controls()}
	</Callout>
{/if}

<script lang="ts">
	import { buttonVariants } from '$lib/components/ui/button';
	import * as Drawer from '$lib/components/ui/drawer';
	import { Skeleton } from '$lib/components/ui/skeleton';
	import { ShieldCheck } from 'lucide-svelte';
	import { m } from '$lib/paraglide/messages';
	import { apiClient } from '@crownshy/api-client/client';
	import { type PublishedReason } from '@crownshy/api-client/api';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import { cn } from '$lib/utils';

	type Props = { workflowStepId: string; isPreview: boolean };

	let { workflowStepId, isPreview }: Props = $props();

	let reasons = $state<PublishedReason[] | null>(null);
	let loadFailed = $state(false);

	async function loadReasons() {
		if (reasons !== null) return;
		loadFailed = false;
		const result = await tryCatchAsync(() =>
			apiClient.PolisGetPublishedModerationPolicy({
				queries: { workflow_step_id: workflowStepId, preview: isPreview }
			})
		);
		if (result.err !== null) {
			loadFailed = true;
			return;
		}
		reasons = result.ok;
	}
</script>

<Drawer.Root onOpenChange={(open) => open && loadReasons()}>
	<p class="text-card-foreground text-base sm:px-4">
		{m.polis_moderation_policy_notice()}
		<Drawer.Trigger
			class="text-primary inline-flex items-center gap-1.5 font-medium underline underline-offset-4"
		>
			<ShieldCheck class="size-5" aria-hidden="true" />
			{m.polis_moderation_policy_read()}
		</Drawer.Trigger>
	</p>
	<Drawer.Content>
		<div class="mx-auto flex w-full max-w-2xl flex-col gap-4 overflow-y-auto px-6 pt-4 pb-8">
			<Drawer.Title class="text-card-foreground text-xl font-semibold">
				{m.polis_moderation_policy_title()}
			</Drawer.Title>
			<Drawer.Description class="text-card-foreground text-base">
				{m.polis_moderation_policy_intro()}
			</Drawer.Description>

			{#if loadFailed}
				<p class="text-destructive text-base">{m.something_went_wrong()}</p>
			{:else if reasons === null}
				<div class="flex flex-col gap-3">
					<Skeleton class="h-12 w-full" />
					<Skeleton class="h-12 w-full" />
					<Skeleton class="h-12 w-full" />
				</div>
			{:else}
				<ul class="flex flex-col gap-3">
					{#each reasons as reason (reason.label)}
						<li class="text-card-foreground text-base">
							<p class="font-semibold">{reason.label}</p>
							{#if reason.description}
								<p class="text-muted-foreground">{reason.description}</p>
							{/if}
						</li>
					{/each}
				</ul>
			{/if}

			<p class="text-card-foreground text-base">{m.polis_moderation_policy_outcome()}</p>

			<Drawer.Close class={cn(buttonVariants({ size: 'lg' }), 'mt-2 w-full text-lg')}>
				{m.polis_close()}
			</Drawer.Close>
		</div>
	</Drawer.Content>
</Drawer.Root>

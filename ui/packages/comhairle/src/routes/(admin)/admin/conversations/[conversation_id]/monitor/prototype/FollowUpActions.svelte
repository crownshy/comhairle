<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { Download, Send } from 'lucide-svelte';
	import { cn } from '$lib/utils';

	type FollowUp = {
		count: number;
		who: string;
		why: string;
		label: string;
		href: string;
		kind: 'message' | 'download';
		/** The most urgent follow-up: highlighted and given the primary button. */
		primary?: boolean;
		disabled?: boolean;
	};

	type Props = { actions: FollowUp[] };

	let { actions }: Props = $props();

	let groups = $derived(
		[
			{
				title: 'Message people',
				items: actions.filter((action) => action.kind === 'message')
			},
			{ title: 'Export data', items: actions.filter((action) => action.kind === 'download') }
		].filter((group) => group.items.length > 0)
	);
</script>

<!-- Next steps for people in this conversation, grouped into messaging and exports. -->
<section aria-label="Follow up" class="bg-card flex flex-col gap-6 rounded-3xl border p-6">
	{#each groups as group (group.title)}
		<div class="flex flex-col gap-2">
			<h3 class="text-primary text-sm font-semibold tracking-wide">
				{group.title}
			</h3>
			<ul class="flex flex-col gap-2">
				{#each group.items as action (action.label)}
					<li
						class={cn(
							'flex flex-col gap-3 rounded-2xl p-4',
							action.primary ? 'bg-accent' : 'bg-muted/50'
						)}
					>
						<div class="flex items-start gap-3">
							<span
								class={cn(
									'inline-flex min-w-12 shrink-0 items-center justify-center rounded-xl px-2 py-1.5 text-2xl font-bold tabular-nums',
									action.primary
										? 'bg-primary text-primary-foreground'
										: 'bg-card'
								)}
							>
								{action.count}
							</span>
							<div class="flex min-w-0 flex-col">
								<span class="text-base font-semibold">{action.who}</span>
								<span class="text-muted-foreground text-sm">{action.why}</span>
							</div>
						</div>
						<Button
							href={action.href}
							variant={action.primary ? 'default' : 'outline'}
							disabled={action.disabled}
							download={action.kind === 'download' ? true : undefined}
							class="self-start rounded-full"
						>
							{#if action.kind === 'download'}
								<Download class="mr-2 size-4" aria-hidden="true" />
							{:else}
								<Send class="mr-2 size-4" aria-hidden="true" />
							{/if}
							{action.label}
						</Button>
					</li>
				{/each}
			</ul>
		</div>
	{/each}
</section>

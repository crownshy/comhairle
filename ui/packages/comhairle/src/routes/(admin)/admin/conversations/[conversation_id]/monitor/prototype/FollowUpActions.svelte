<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { Download } from 'lucide-svelte';
	import { cn } from '$lib/utils';

	type DataExport = {
		/** How many people the export covers, when known. */
		count?: number;
		title: string;
		description: string;
		label: string;
		href: string;
		/** Highlighted and given the primary button. */
		primary?: boolean;
		disabled?: boolean;
	};

	type Props = { exports: DataExport[] };

	let { exports }: Props = $props();
</script>

<!-- Data the admin can download about the people in this conversation. -->
<section aria-labelledby="follow-up-heading" class="flex flex-col gap-4">
	<div class="flex flex-col gap-1">
		<h2 id="follow-up-heading" class="text-2xl font-semibold">Follow up</h2>
		<p class="text-muted-foreground text-base">
			Export data about people in this conversation.
		</p>
	</div>
	<ul class="grid grid-cols-1 gap-4 md:grid-cols-2 lg:grid-cols-3">
		{#each exports as item (item.href + item.label)}
			<li
				class={cn(
					'flex flex-col justify-between gap-4 rounded-2xl border p-5',
					item.primary ? 'bg-accent border-transparent' : 'bg-card'
				)}
			>
				<div class="flex items-start gap-3">
					{#if item.count !== undefined}
						<span
							class={cn(
								'inline-flex min-w-12 shrink-0 items-center justify-center rounded-xl px-2 py-1.5 text-2xl font-bold tabular-nums',
								item.primary ? 'bg-primary text-primary-foreground' : 'bg-muted'
							)}
						>
							{item.count}
						</span>
					{/if}
					<div class="flex min-w-0 flex-col gap-1">
						<span class="text-base font-semibold">{item.title}</span>
						<span class="text-muted-foreground text-sm">{item.description}</span>
					</div>
				</div>
				<Button
					href={item.href}
					variant={item.primary ? 'default' : 'outline'}
					disabled={item.disabled}
					download
					class="self-start rounded-full"
				>
					<Download class="mr-2 size-4" aria-hidden="true" />
					{item.label}
				</Button>
			</li>
		{/each}
	</ul>
</section>

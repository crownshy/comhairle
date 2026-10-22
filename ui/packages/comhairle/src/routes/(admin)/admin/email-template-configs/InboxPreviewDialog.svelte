<script lang="ts">
	import { Inbox, Search, Send, Star } from 'lucide-svelte';
	import * as Dialog from '$lib/components/ui/dialog';
	import * as Tabs from '$lib/components/ui/tabs';
	import { inboxSnippet } from '$lib/utils/emailInboxSnippet';

	type Props = {
		open: boolean;
		subject: string;
		html: string;
		variables: string[];
	};

	let { open = $bindable(), subject, html, variables }: Props = $props();

	// The From address the mailer sets (api/src/mailer.rs). It has no display name, so
	// participants see the bare address.
	const SENDER_ADDRESS = 'noreply@comhairle.scot';
	const SENDER_INITIAL = 'N';
	const PLACEHOLDER_ROWS = [0, 1, 2];

	let view = $state<'desktop' | 'phone'>('desktop');
	let snippet = $derived(inboxSnippet(html));
	let shownSubject = $derived(subject || 'No subject');
	let variableNames = $derived(variables.map((variable) => `{{${variable}}}`).join(', '));
</script>

{#snippet senderAvatar(size: string)}
	<div
		class="bg-primary text-primary-foreground flex shrink-0 items-center justify-center rounded-full font-semibold {size}"
		aria-hidden="true"
	>
		{SENDER_INITIAL}
	</div>
{/snippet}

{#snippet placeholderRow()}
	<li class="flex flex-col gap-2 px-4 py-3" aria-hidden="true">
		<div class="bg-muted h-3 w-24 rounded"></div>
		<div class="bg-muted h-3 w-40 rounded"></div>
		<div class="bg-muted/70 h-3 w-full rounded"></div>
	</li>
{/snippet}

{#snippet inboxRow()}
	<div class="bg-accent flex gap-3 px-4 py-3">
		<span class="bg-primary mt-2 size-2 shrink-0 rounded-full" aria-hidden="true"></span>
		<div class="flex min-w-0 flex-1 flex-col">
			<div class="flex items-baseline justify-between gap-2">
				<span class="text-foreground truncate text-sm font-bold">{SENDER_ADDRESS}</span>
				<span class="text-muted-foreground shrink-0 text-sm">Now</span>
			</div>
			<span class="text-foreground truncate text-sm font-semibold">{shownSubject}</span>
			<span class="text-muted-foreground line-clamp-2 text-sm">{snippet}</span>
		</div>
	</div>
{/snippet}

{#snippet messageHeader(compact: boolean)}
	<div class="border-border flex flex-col gap-3 border-b px-4 py-3">
		<h3 class="text-foreground font-semibold {compact ? 'text-base' : 'text-xl'}">
			{shownSubject}
		</h3>
		<div class="flex items-center gap-3">
			{@render senderAvatar('size-9 text-sm')}
			<div class="flex min-w-0 flex-col">
				<span class="text-foreground truncate text-sm font-semibold">{SENDER_ADDRESS}</span>
				<span class="text-muted-foreground text-sm">to me</span>
			</div>
		</div>
	</div>
{/snippet}

<Dialog.Root bind:open>
	<Dialog.Content class="flex max-h-[92vh] w-full flex-col gap-4 overflow-hidden sm:max-w-6xl">
		<Dialog.Header>
			<Dialog.Title>How participants receive it</Dialog.Title>
			<Dialog.Description class="text-base">
				A mock inbox using your current wording.
				{#if variables.length > 0}
					Variables ({variableNames}) show example values here. Participants see the real ones.
				{/if}
			</Dialog.Description>
		</Dialog.Header>

		<Tabs.Root bind:value={view} class="flex min-h-0 flex-1 flex-col gap-4">
			<Tabs.List class="self-start">
				<Tabs.Trigger value="desktop">Desktop</Tabs.Trigger>
				<Tabs.Trigger value="phone">Phone</Tabs.Trigger>
			</Tabs.List>

			<Tabs.Content value="desktop" class="min-h-0 flex-1">
				<div
					class="border-border bg-card flex h-[68vh] flex-col overflow-hidden rounded-xl border"
				>
					<div class="border-border flex items-center gap-3 border-b px-4 py-2.5">
						<div
							class="bg-muted text-muted-foreground flex h-9 max-w-md flex-1 items-center gap-2 rounded-full px-4 text-sm"
							aria-hidden="true"
						>
							<Search class="size-4" />
							Search mail
						</div>
					</div>
					<div class="grid min-h-0 flex-1 grid-cols-[10rem_18rem_minmax(0,1fr)]">
						<nav
							aria-label="Mock mail folders"
							class="border-border flex flex-col gap-1 border-r p-3 text-sm"
						>
							<span
								class="bg-accent text-foreground flex items-center gap-2 rounded-md px-2 py-1.5 font-semibold"
							>
								<Inbox class="size-4" aria-hidden="true" /> Inbox
								<span class="ml-auto">1</span>
							</span>
							<span class="text-muted-foreground flex items-center gap-2 px-2 py-1.5">
								<Star class="size-4" aria-hidden="true" /> Starred
							</span>
							<span class="text-muted-foreground flex items-center gap-2 px-2 py-1.5">
								<Send class="size-4" aria-hidden="true" /> Sent
							</span>
						</nav>
						<ul class="border-border divide-border min-h-0 divide-y overflow-y-auto border-r">
							<li>{@render inboxRow()}</li>
							{#each PLACEHOLDER_ROWS as row (row)}
								{@render placeholderRow()}
							{/each}
						</ul>
						<div class="flex min-h-0 flex-col">
							{@render messageHeader(false)}
							<iframe
								title="Email as opened on a computer"
								srcdoc={html}
								sandbox=""
								class="min-h-0 w-full flex-1 bg-white"
							></iframe>
						</div>
					</div>
				</div>
			</Tabs.Content>

			<Tabs.Content value="phone" class="min-h-0 flex-1">
				<div class="flex flex-wrap items-start justify-center gap-8">
					<figure class="flex flex-col items-center gap-2">
						<div
							class="border-foreground/80 bg-card flex h-[60vh] w-[22rem] flex-col overflow-hidden rounded-[2.5rem] border-[10px]"
						>
							<div class="flex flex-col gap-1 px-4 pt-6 pb-2">
								<span class="text-foreground text-xl font-bold">Inbox</span>
							</div>
							<ul class="divide-border divide-y">
								<li>{@render inboxRow()}</li>
								{#each PLACEHOLDER_ROWS as row (row)}
									{@render placeholderRow()}
								{/each}
							</ul>
						</div>
						<figcaption class="text-muted-foreground text-sm">In the inbox</figcaption>
					</figure>
					<figure class="flex flex-col items-center gap-2">
						<div
							class="border-foreground/80 bg-card flex h-[60vh] w-[22rem] flex-col overflow-hidden rounded-[2.5rem] border-[10px]"
						>
							<div class="pt-4">{@render messageHeader(true)}</div>
							<iframe
								title="Email as opened on a phone"
								srcdoc={html}
								sandbox=""
								class="min-h-0 w-full flex-1 bg-white"
							></iframe>
						</div>
						<figcaption class="text-muted-foreground text-sm">Opened</figcaption>
					</figure>
				</div>
			</Tabs.Content>
		</Tabs.Root>
	</Dialog.Content>
</Dialog.Root>

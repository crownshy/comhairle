<script lang="ts">
	import * as AlertDialog from '$lib/components/ui/alert-dialog';
	import { buttonVariants } from '$lib/components/ui/button';
	import { Check, Copy } from 'lucide-svelte';
	import { cn } from '$lib/utils';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import { goto } from '$app/navigation';
	import * as m from '$lib/paraglide/messages';

	let {
		open = $bindable(false),
		returnUrl,
		anonymousId,
		leaveHref = '/'
	}: {
		open?: boolean;
		/** Absolute link back into the participant's current position in the workflow. */
		returnUrl: string;
		/** The anonymous id a participant signs back in with, when they have one. */
		anonymousId?: string;
		leaveHref?: string;
	} = $props();

	let copied = $state(false);
	let clearCopied: ReturnType<typeof setTimeout> | undefined;
	let field = $state<HTMLInputElement | null>(null);

	async function copyLink() {
		const result = await tryCatchAsync(() => navigator.clipboard.writeText(returnUrl));
		if (result.err) {
			// No clipboard (insecure origin, or the browser said no). Select the address so the
			// participant can still take it manually.
			field?.select();
			return;
		}
		copied = true;
		clearTimeout(clearCopied);
		clearCopied = setTimeout(() => (copied = false), 2000);
	}
</script>

<!-- The leave link is a plain site path, so resolve() has nothing to resolve. -->
<!-- eslint-disable svelte/no-navigation-without-resolve -->
<AlertDialog.Root
	bind:open
	onOpenChange={(next) => {
		if (!next) copied = false;
	}}
>
	<AlertDialog.Content class="gap-5">
		<AlertDialog.Header>
			<AlertDialog.Title>{m.step_leave_title()}</AlertDialog.Title>
			<AlertDialog.Description class="text-base">
				{m.step_leave_body()}
			</AlertDialog.Description>
		</AlertDialog.Header>

		<div class="flex flex-col gap-2">
			<span class="text-muted-foreground text-sm font-medium">
				{m.step_leave_link_label()}
			</span>
			<div class="flex items-center gap-2">
				<!-- Readonly rather than a link: the point is to carry the address away, not to
					follow it from here. -->
				<input
					bind:this={field}
					type="text"
					readonly
					value={returnUrl}
					aria-label={m.step_leave_link_label()}
					class="border-border bg-muted text-foreground h-10 min-w-0 flex-1 truncate rounded-md border px-3 text-sm"
					onfocus={(event) => event.currentTarget.select()}
				/>
				<button
					type="button"
					class={cn(buttonVariants({ variant: 'outline' }), 'shrink-0')}
					onclick={copyLink}
				>
					{#if copied}
						<Check class="size-4" aria-hidden="true" />
						{m.step_leave_copied()}
					{:else}
						<Copy class="size-4" aria-hidden="true" />
						{m.step_leave_copy()}
					{/if}
				</button>
			</div>
			{#if anonymousId}
				<p class="text-muted-foreground text-sm">
					{m.step_leave_anonymous_id({ id: anonymousId })}
				</p>
			{/if}
		</div>

		<AlertDialog.Footer>
			<!-- Staying is the emphasised choice, so leaving is the quiet one. `bg-transparent`
				and `shadow-none` because the ghost variant has no background of its own to
				override the action's default. -->
			<AlertDialog.Action
				onclick={() => goto(leaveHref)}
				class={cn(
					buttonVariants({ variant: 'ghost' }),
					'text-muted-foreground bg-transparent shadow-none'
				)}
			>
				{m.step_leave_confirm()}
			</AlertDialog.Action>
			<AlertDialog.Cancel>
				{m.step_leave_stay()}
			</AlertDialog.Cancel>
		</AlertDialog.Footer>
	</AlertDialog.Content>
</AlertDialog.Root>

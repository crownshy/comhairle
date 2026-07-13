<script lang="ts">
	/**
	 * Inline theme chip + dropdown picker. Renders the current themes on a
	 * statement; clicking opens a dropdown of themes already used elsewhere
	 * on this conversation, with toggle checkmarks and an "Add new" affordance.
	 *
	 * The component is dumb — the parent owns the source of truth and the
	 * persistence call. We just emit per-theme add/remove events that map 1:1
	 * onto the comhairle /themes endpoints.
	 *
	 * The dropdown is a shadcn `Popover`, which portals its content to the body
	 * and handles collision/flip via Floating UI. That's why the ancestor `Card`
	 * / scroll container overflow doesn't clip it, with no manual positioning.
	 */
	import * as Popover from '$lib/components/ui/popover';

	interface Props {
		/** Themes currently assigned to this statement. */
		themes: string[];
		/** Every theme already used on this conversation (for the dropdown). */
		availableThemes: string[];
		/** Disabled when there's no aux row to write to. */
		disabled?: boolean;
		onAddTheme?: (theme: string) => void | Promise<void>;
		onRemoveTheme?: (theme: string) => void | Promise<void>;
	}

	let { themes, availableThemes, disabled = false, onAddTheme, onRemoveTheme }: Props = $props();

	let newDraft = $state('');

	const themeSet = $derived(new Set(themes));

	function applyToggle(theme: string) {
		if (themeSet.has(theme)) {
			onRemoveTheme?.(theme);
		} else {
			onAddTheme?.(theme);
		}
	}

	function applyRemove(theme: string) {
		if (disabled) return;
		onRemoveTheme?.(theme);
	}

	function applyAdd() {
		const t = newDraft.trim();
		if (!t || themeSet.has(t)) return;
		onAddTheme?.(t);
		newDraft = '';
	}
</script>

<div class="inline-flex flex-wrap items-center gap-2">
	{#each themes as t (t)}
		<span
			class="group/chip bg-muted text-foreground inline-flex items-center gap-1 rounded-[3px] px-[5px] py-[3px] text-base font-medium"
		>
			{t}
			{#if !disabled}
				<button
					type="button"
					onclick={() => applyRemove(t)}
					aria-label={`Remove theme ${t}`}
					class="hover:text-destructive text-foreground/40 hidden leading-none group-hover/chip:inline"
				>
					×
				</button>
			{/if}
		</span>
	{/each}

	{#if !disabled}
		<Popover.Root>
			<Popover.Trigger
				class="text-primary bg-primary/10 hover:bg-primary/20 inline-flex items-center rounded-[3px] px-[5px] py-0.5 text-base font-medium"
			>
				Add new+
			</Popover.Trigger>
			<Popover.Content align="start" class="w-80 overflow-hidden p-0 shadow-lg">
				{#each availableThemes as t (t)}
					{@const checked = themeSet.has(t)}
					<button
						type="button"
						onclick={() => applyToggle(t)}
						class="hover:bg-muted/60 border-border flex w-full items-center justify-between gap-2 border-b px-4 py-3 text-left text-base last:border-b-0"
					>
						<span>#{t}</span>
						{#if checked}
							<span class="text-primary text-lg">✓</span>
						{/if}
					</button>
				{/each}
				<div class="border-border flex items-center gap-2 border-t p-2.5">
					<input
						type="text"
						bind:value={newDraft}
						onkeydown={(e) => e.key === 'Enter' && applyAdd()}
						placeholder="Add new…"
						class="border-border focus:ring-primary/40 min-w-0 flex-1 rounded-md border px-3 py-2 text-base focus:ring-2 focus:outline-none"
					/>
					<button
						type="button"
						onclick={applyAdd}
						disabled={!newDraft.trim()}
						class="bg-primary text-primary-foreground rounded-md px-4 py-2 text-base font-medium disabled:opacity-40"
					>
						Add
					</button>
				</div>
			</Popover.Content>
		</Popover.Root>
	{/if}
</div>

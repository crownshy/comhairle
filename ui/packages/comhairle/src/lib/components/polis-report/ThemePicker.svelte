<script lang="ts">
	/**
	 * Inline theme chip + dropdown picker. Renders the current themes on a
	 * statement; clicking opens a dropdown of themes already used elsewhere
	 * on this conversation, with toggle checkmarks and an "Add new" affordance.
	 *
	 * The component is dumb — the parent owns the source of truth and the
	 * persistence call. We just emit per-theme add/remove events that map 1:1
	 * onto the comhairle /themes endpoints.
	 */
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

	let open = $state(false);
	let newDraft = $state('');
	let containerEl: HTMLDivElement | undefined = $state();
	let dropdownEl: HTMLDivElement | undefined = $state();
	let dropdownPos = $state<{ top: number; left: number } | null>(null);

	const themeSet = $derived(new Set(themes));

	function toggleOpen() {
		if (disabled) return;
		open = !open;
		if (open) reposition();
	}

	/**
	 * Dropdown uses `position: fixed` to escape ancestor `overflow: hidden`
	 * (e.g. the Card wrapping the statements list). So we have to position it
	 * manually from the trigger's bbox, and re-position on scroll/resize.
	 *
	 * When there isn't room below the trigger (statement near the bottom of the
	 * viewport), flip the menu above it so it never gets clipped off-screen.
	 */
	function reposition() {
		if (!containerEl) return;
		const rect = containerEl.getBoundingClientRect();
		const gap = 4;
		const menuHeight = dropdownEl?.offsetHeight ?? 260;
		const spaceBelow = window.innerHeight - rect.bottom;
		const top =
			spaceBelow < menuHeight + gap && rect.top > spaceBelow
				? Math.max(gap, rect.top - menuHeight - gap)
				: rect.bottom + gap;
		dropdownPos = { top, left: rect.left };
	}

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

	// Once the menu is in the DOM we know its real height, so re-measure to
	// decide whether it should flip above the trigger.
	$effect(() => {
		if (open && dropdownEl) reposition();
	});

	// Close on outside click + reposition on scroll/resize while open.
	$effect(() => {
		if (!open) return;
		function onDocClick(e: MouseEvent) {
			if (containerEl && !containerEl.contains(e.target as Node)) {
				open = false;
			}
		}
		document.addEventListener('mousedown', onDocClick);
		window.addEventListener('scroll', reposition, true);
		window.addEventListener('resize', reposition);
		return () => {
			document.removeEventListener('mousedown', onDocClick);
			window.removeEventListener('scroll', reposition, true);
			window.removeEventListener('resize', reposition);
		};
	});
</script>

<div class="relative inline-flex flex-wrap items-center gap-2" bind:this={containerEl}>
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
		<button
			type="button"
			onclick={toggleOpen}
			class="text-primary bg-primary/10 hover:bg-primary/20 inline-flex items-center rounded-[3px] px-[5px] py-0.5 text-base font-medium"
		>
			Add new+
		</button>
	{/if}

	{#if open && dropdownPos}
		<div
			bind:this={dropdownEl}
			style:top="{dropdownPos.top}px"
			style:left="{dropdownPos.left}px"
			class="border-border bg-popover text-popover-foreground fixed z-50 w-80 overflow-hidden rounded-lg border shadow-lg"
		>
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
		</div>
	{/if}
</div>

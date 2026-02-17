<script lang="ts">
	import { X } from "lucide-svelte";
	import { cn } from "$lib/utils.js";

	export type Option = {
		value: string;
		label: string;
		disable?: boolean;
		fixed?: boolean;
	};

	type Props = {
		defaultOptions?: Option[];
		selected?: Option[];
		onSelectedChange?: (options: Option[]) => void;
		placeholder?: string;
		disabled?: boolean;
		emptyMessage?: string;
		class?: string;
		badgeClass?: string;
	};

	let {
		defaultOptions = [],
		selected = $bindable([]),
		onSelectedChange,
		placeholder = "Select...",
		disabled = false,
		emptyMessage = "No results found",
		class: className,
		badgeClass,
	}: Props = $props();

	let inputRef: HTMLInputElement | null = $state(null);
	let open = $state(false);
	let inputValue = $state("");
	let onScrollbar = $state(false);
	let containerRef: HTMLDivElement | null = $state(null);

	let selectables = $derived(
		defaultOptions
			.filter((opt) => !selected.find((s) => s.value === opt.value))
			.filter((opt) => opt.label.toLowerCase().includes(inputValue.toLowerCase()))
	);

	function updateSelected(newSelected: Option[]) {
		selected = newSelected;
		onSelectedChange?.(newSelected);
	}

	function handleUnselect(option: Option) {
		if (option.fixed) return;
		updateSelected(selected.filter((s) => s.value !== option.value));
	}

	function handleSelect(option: Option) {
		inputValue = "";
		updateSelected([...selected, option]);
	}

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === "Backspace" && inputValue === "" && selected.length > 0) {
			const last = selected[selected.length - 1];
			if (!last.fixed) {
				handleUnselect(last);
			}
		}
		if (e.key === "Escape") {
			open = false;
			inputRef?.blur();
		}
	}

	function handleClickOutside(e: MouseEvent) {
		if (containerRef && !containerRef.contains(e.target as Node)) {
			open = false;
		}
	}

	$effect(() => {
		if (open) {
			document.addEventListener("mousedown", handleClickOutside);
		} else {
			document.removeEventListener("mousedown", handleClickOutside);
		}
		return () => {
			document.removeEventListener("mousedown", handleClickOutside);
		};
	});
</script>

<div bind:this={containerRef} class="relative">
	<div
		class={cn(
			"border-input focus-within:border-ring focus-within:ring-ring/50 relative min-h-[38px] rounded-md border bg-background text-sm transition-[color,box-shadow] outline-none focus-within:ring-[3px]",
			selected.length > 0 && "p-1 cursor-text",
			disabled && "pointer-events-none cursor-not-allowed opacity-50",
			className
		)}
		onclick={() => {
			if (!disabled) inputRef?.focus();
		}}
	>
		<div class="flex flex-wrap gap-1">
			{#each selected as option (option.value)}
				<div
					class={cn(
						"animate-fadeIn bg-background text-secondary-foreground hover:bg-background relative inline-flex h-7 cursor-default items-center rounded-md border pl-2 text-xs font-medium transition-all",
						option.fixed ? "pr-2" : "pr-7",
						badgeClass
					)}
				>
					{option.label}
					{#if !option.fixed}
						<button
							type="button"
							class="text-muted-foreground/80 hover:text-foreground absolute -inset-y-px -right-px flex size-7 items-center justify-center rounded-r-md border border-transparent p-0 outline-none transition-[color,box-shadow]"
							onclick={(e) => {
								e.stopPropagation();
								handleUnselect(option);
							}}
							aria-label="Remove"
						>
							<X class="size-3.5" />
						</button>
					{/if}
				</div>
			{/each}

			<input
				bind:this={inputRef}
				bind:value={inputValue}
				{disabled}
				{placeholder}
				class={cn(
					"placeholder:text-muted-foreground/70 flex-1 bg-transparent outline-none disabled:cursor-not-allowed",
					selected.length === 0 ? "px-3 py-2" : "ml-1"
				)}
				onkeydown={handleKeydown}
				onfocus={() => (open = true)}
				onblur={() => {
					if (!onScrollbar) open = false;
				}}
			/>
		</div>
	</div>

	{#if open}
		<div
			class="border-input bg-popover text-popover-foreground absolute top-[calc(100%+8px)] z-50 w-full overflow-hidden rounded-md border shadow-lg animate-in fade-in-0 zoom-in-95"
			onmouseenter={() => (onScrollbar = true)}
			onmouseleave={() => (onScrollbar = false)}
			onmouseup={() => inputRef?.focus()}
		>
			<div class="overflow-y-auto p-1">
				{#each selectables as option (option.value)}
					<button
						type="button"
						class={cn(
							"w-full flex items-center rounded-sm px-2 py-1.5 text-sm text-left cursor-pointer hover:bg-accent hover:text-accent-foreground outline-none",
							option.disable && "pointer-events-none cursor-not-allowed opacity-50"
						)}
						disabled={option.disable}
						onmousedown={(e) => {
							e.preventDefault();
							e.stopPropagation();
						}}
						onclick={() => handleSelect(option)}
					>
						{option.label}
					</button>
				{/each}

				{#if selectables.length === 0}
					<div class="px-2 py-4 text-center text-sm text-muted-foreground">
						{emptyMessage}
					</div>
				{/if}
			</div>
		</div>
	{/if}
</div>
<script lang="ts" module>
	import { cn, type WithElementRef } from '$lib/utils.js';
	import type { Icon as LucideIcon } from 'lucide-svelte';
	import type { ComponentType } from 'svelte';
	import type { HTMLAnchorAttributes, HTMLButtonAttributes } from 'svelte/elements';
	import { type VariantProps, tv } from 'tailwind-variants';

	export const buttonVariants = tv({
		base: "w-fit  aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive inline-flex shrink-0 items-center justify-center gap-2 whitespace-nowrap rounded-full text-sm font-medium transition-all disabled:pointer-events-none disabled:opacity-50 aria-disabled:pointer-events-none aria-disabled:opacity-50 [&_svg:not([class*='size-'])]:size-4 [&_svg]:pointer-events-none [&_svg]:shrink-0",
		variants: {
			variant: {
				default: 'bg-primary text-primary-foreground shadow-xs hover:bg-primary/90',
				primaryDark: 'bg-sidebar text-sidebar-foreground hover:bg-sidebar/90',
				destructive:
					'bg-destructive shadow-xs hover:bg-destructive/90 focus-visible:ring-destructive/20 dark:focus-visible:ring-destructive/40 dark:bg-destructive/60 text-white',
				outline:
					'bg-background shadow-xs hover:bg-accent hover:text-accent-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 border border-primary text-primary hover:bg-primary/10 hover:text-primary',
				destructiveOutline:
					'bg-transparent shadow-xs border border-destructive text-destructive hover:bg-destructive/20',
				secondary: 'bg-secondary text-secondary-foreground shadow-xs hover:bg-secondary/80',
				ghost: 'hover:bg-accent hover:text-accent-foreground dark:hover:bg-accent/50',
				link: 'text-primary underline-offset-4 hover:underline',
				nav: 'text-primary-foreground shadow-xs hover:bg-white/10'
			},
			size: {
				default: 'h-9 px-4 py-2 has-[>svg]:px-3',
				sm: 'h-8 gap-1.5 px-3 has-[>svg]:px-2.5',
				lg: 'h-10 px-6 has-[>svg]:px-4',
				xl: 'h-12 rounded-lg px-6 text-lg font-bold has-[>svg]:px-4',
				icon: 'size-9'
			}
		},
		defaultVariants: {
			variant: 'default',
			size: 'default'
		}
	});

	export type ButtonVariant = VariantProps<typeof buttonVariants>['variant'];
	export type ButtonSize = VariantProps<typeof buttonVariants>['size'];

	type DialogId = string;

	// Using "," to have multiple dialogs be affected
	type Command = `open-dialog:${DialogId}` | `close-dialog:${DialogId}`;

	export type ButtonProps = Omit<WithElementRef<HTMLButtonAttributes>, 'command'> &
		WithElementRef<HTMLAnchorAttributes> & {
			variant?: ButtonVariant;
			size?: ButtonSize;
			icon?: ComponentType<LucideIcon>;
			plain?: boolean;
			command?: Command;
		};
</script>

<script lang="ts">
	let {
		class: className,
		variant = 'default',
		size = 'default',
		ref = $bindable(),
		href = undefined,
		type = 'button',
		disabled,
		command,
		icon: Icon,
		plain,
		onclick,
		children,
		...restProps
	}: ButtonProps = $props();
</script>

{#if href}
	<a
		bind:this={ref}
		data-slot="button"
		class={plain ? className : cn(buttonVariants({ variant, size }), className)}
		href={disabled ? undefined : href}
		aria-disabled={disabled}
		role={disabled ? 'link' : undefined}
		tabindex={disabled ? -1 : undefined}
		{onclick}
		{...restProps}
	>
		{#if Icon}
			<Icon class="h-4 w-4" />
		{/if}
		{@render children?.()}
	</a>
{:else}
	<button
		bind:this={ref}
		data-slot="button"
		class={plain ? className : cn(buttonVariants({ variant, size }), className)}
		{type}
		{disabled}
		onclick={(e) => {
			onclick?.(e);
			if (command) {
				const [cmd, idStr] = command.split(':');
				const ids = idStr.split(',');

				for (const id of ids) {
					const Dialog = document.getElementById(id) as HTMLDialogElement | null;
					if (!Dialog) {
						return;
					}
					switch (cmd) {
						case 'open-dialog':
							Dialog.showModal();
							break;
						case 'close-dialog':
							Dialog.close();
					}
				}
			}
		}}
		{...restProps}
	>
		{#if Icon}
			<Icon class="h-4 w-4" />
		{/if}
		{@render children?.()}
	</button>
{/if}

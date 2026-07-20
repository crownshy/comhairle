<script lang="ts">
	/** Pill-shaped selectable theme chip used by the Theme Explorer. */
	interface Props {
		label: string;
		selected: boolean;
		/**
		 * Colour scheme — both variants use the comhairle primary accent. `default`
		 * renders primary text on a neutral muted surface; `primary` uses the solid
		 * primary fill when selected and the subtle secondary surface when idle.
		 */
		variant?: 'default' | 'primary';
		onclick: () => void;
	}

	let { label, selected, variant = 'default', onclick }: Props = $props();

	const palette = {
		default: {
			on: 'bg-primary text-primary-foreground hover:bg-primary/90',
			off: 'bg-muted text-primary hover:bg-muted-foreground/15'
		},
		primary: {
			on: 'bg-primary text-primary-foreground hover:bg-primary/90',
			off: 'bg-secondary text-primary hover:bg-secondary/80'
		}
	};
</script>

<button
	type="button"
	{onclick}
	class={`cursor-pointer rounded-[30px] px-3 py-2 text-lg leading-6 font-medium transition-all duration-150 hover:scale-[1.05] hover:shadow-sm active:scale-[0.96] ${
		selected ? palette[variant].on : palette[variant].off
	}`}
>
	{label}
</button>

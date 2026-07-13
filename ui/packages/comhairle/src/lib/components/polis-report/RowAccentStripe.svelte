<script lang="ts">
	interface Props {
		/** Tailwind background-color class for the bar, e.g. "bg-primary". */
		accent: string;
	}

	let { accent }: Props = $props();
</script>

<!-- Left accent stripe. Requires the parent row to have `group` + `relative`;
     the bar grows from 1.5 → 2 when the row is hovered. -->
<div
	class={`absolute top-0 bottom-0 left-0 w-1.5 transition-all duration-150 group-hover:w-2 ${accent}`}
></div>

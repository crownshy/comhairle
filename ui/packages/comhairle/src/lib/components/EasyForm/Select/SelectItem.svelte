<script lang="ts">
	type Props = {
		name: string;
		label: string;
		value: string;
		onselect: (value: string) => void;
	};

	let { name, label, value, onselect }: Props = $props();
</script>

<label
	class=" border-card bg-card has-checked:border-primary has-checked:bg-primary/30 cursor-pointer rounded-xl border-2 p-5 hover:border-gray-500/20 hover:bg-gray-500/20"
>
	<input
		type="radio"
		class="hidden"
		{name}
		{value}
		onselect={(e) => onselect((e.target as HTMLInputElement).value)}
	/>
	{label}
</label>

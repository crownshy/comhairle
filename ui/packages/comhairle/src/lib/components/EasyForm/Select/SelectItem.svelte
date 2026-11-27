<script lang="ts">
	import { Check } from '@lucide/svelte';

	type Props = {
		name: string;
		label: string;
		value: string;
		onselect: (value: string) => void;
		defaultSelected?: boolean;
		multi?: boolean;
	};

	let { name, label, value, onselect, defaultSelected, multi }: Props = $props();
</script>

<label
	class="has-checked:border-primary has-checked:bg-primary/30 has-focus-visible:border-ring has-focus-visible:ring-ring/50 flex cursor-pointer items-center rounded-xl border-2 border-slate-500/10 bg-slate-500/10 p-5 pl-2 hover:border-gray-500/20 hover:bg-gray-500/20 has-focus-visible:ring-[3px]"
>
	<input
		type={multi ? 'checkbox' : 'radio'}
		class="peer opacity-0 not-checked:mr-3"
		{name}
		{value}
		defaultChecked={defaultSelected}
		onchange={(e) => onselect((e.target as HTMLInputElement).value)}
	/>
	<Check class="mr-2 hidden size-4 peer-checked:block" strokeWidth={3} aria-hidden />
	{label}
</label>

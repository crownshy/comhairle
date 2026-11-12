<script lang="ts">
	import { Checkbox } from '$lib/components/ui/checkbox';

	type Props = {
		id: string;
		text: string;
		checked: boolean;
		onChange: (checked: boolean) => void;
	};

	let { id, text, checked, onChange }: Props = $props();
</script>

<div class="flex items-start gap-3">
	<Checkbox {id} {checked} onCheckedChange={(value) => onChange(value === true)} class="mt-1" />
	<label for={id} class="text-base">{text}</label>
</div>

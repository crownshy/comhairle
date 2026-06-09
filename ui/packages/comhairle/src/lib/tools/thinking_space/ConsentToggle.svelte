<script lang="ts">
	import { Switch } from '$lib/components/ui/switch';

	type Props = {
		shared: boolean;
		onChange: (next: boolean) => void;
	};

	let { shared, onChange }: Props = $props();

	const id = $props.id();
</script>

<div
	class="bg-muted/40 border-border inline-flex items-center gap-3 rounded-full border px-3 py-1.5"
>
	<label for={id} class="text-foreground cursor-pointer text-xs font-medium">
		Share my responses with the organizers
	</label>
	<Switch
		{id}
		checked={shared}
		onCheckedChange={(v) => onChange(v)}
		aria-label="Share my responses with the organizers"
	/>
</div>

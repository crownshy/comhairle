<script lang="ts">
	import { Spinner } from '$lib/components/ui/spinner';

	type Props = {
		items: string;
	};

	let { items }: Props = $props();
</script>

<div class="text-muted-foreground flex flex-row items-center gap-x-2">
	<Spinner class="size-5" />
	<span>Loading {items}</span>
</div>

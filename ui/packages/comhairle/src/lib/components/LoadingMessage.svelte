<script lang="ts">
	import { Spinner } from '$lib/components/ui/spinner';

	type Props = {
		subject: string;
	};

	let { subject }: Props = $props();
</script>

<div class="text-muted-foreground flex flex-row items-center gap-x-2">
	<Spinner class="size-5" />
	<span>Loading {subject}</span>
</div>

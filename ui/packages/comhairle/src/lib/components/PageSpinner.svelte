<script lang="ts">
	import { Spinner } from '$lib/components/ui/spinner';

	type Props = {
		subject: string;
	};

	let { subject }: Props = $props();
</script>

<div
	class="text-muted-foreground mt-5 flex h-full w-full flex-col items-center justify-center"
	aria-label="Loading {subject}"
	aria-live="polite"
>
	<Spinner class="h-20 w-20" strokeWidth={1.2} />
</div>

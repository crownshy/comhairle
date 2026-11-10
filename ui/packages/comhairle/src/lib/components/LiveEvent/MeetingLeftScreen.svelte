<script lang="ts">
	import Button from '$lib/components/ui/button/button.svelte';
	import { PhoneOff } from 'lucide-svelte';

	interface Props {
		title: string;
		onRejoin: () => void;
	}

	let { title, onRejoin }: Props = $props();
</script>

<div class="bg-background flex min-h-dvh w-full items-center justify-center">
	<div class="flex max-w-lg flex-col items-center gap-12 px-6 text-center">
		<div class="bg-muted flex h-20 w-20 items-center justify-center rounded-full">
			<PhoneOff class="text-muted-foreground h-10 w-10" />
		</div>

		<div class="flex flex-col items-center gap-4">
			<h1 class="text-foreground text-4xl leading-10 font-bold">You left the meeting</h1>
			<p class="text-muted-foreground text-lg leading-7">
				<span class="text-foreground font-medium">{title}</span> is still going. You can rejoin
				until it ends.
			</p>
		</div>

		<Button variant="primaryDark" class="h-12 px-8 text-base font-semibold" onclick={onRejoin}>
			Rejoin
		</Button>
	</div>
</div>

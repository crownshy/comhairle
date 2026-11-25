<script lang="ts">
	import Button from '$lib/components/ui/button/button.svelte';
	import { m } from '$lib/paraglide/messages';
	import { focusOnScreenChange } from '$lib/utils/focusOnScreenChange';
	import PhoneOff from '@lucide/svelte/icons/phone-off';

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
			<h1
				tabindex="-1"
				{@attach focusOnScreenChange}
				class="text-foreground text-4xl leading-10 font-bold outline-none"
			>
				{m.live_event_left_title()}
			</h1>
			<p class="text-muted-foreground text-lg leading-7">
				{m.live_event_left_body({ title })}
			</p>
		</div>

		<Button variant="primaryDark" class="h-12 px-8 text-base font-semibold" onclick={onRejoin}>
			{m.live_event_rejoin()}
		</Button>
	</div>
</div>

<script lang="ts">
	import { notifications } from '$lib/notifications.svelte';
	import * as Tooltip from '$lib/components/ui/tooltip';
	import Icon from '@iconify/svelte';
	import type { Snippet } from 'svelte';
	import { copy } from 'svelte-copy';
	type Props = {
		copyText: string;
		children: Snippet;
	};
	let props: Props = $props();
	let { copyText, children } = props;
</script>

<Tooltip.Provider>
	<Tooltip.Root>
		<Tooltip.Trigger class="w-full">
			<button
				use:copy={{
					text: copyText,
					onCopy() {
						notifications.send({ priority: 'INFO', message: 'Copied' });
					}
				}}
				class="flex w-full cursor-pointer flex-row items-center justify-center gap-x-2 outline-hidden"
			>
				{@render children()}
				<Icon icon="solar:copy-bold" />
			</button>
		</Tooltip.Trigger>
		<Tooltip.Content>"Click to copy"</Tooltip.Content>
	</Tooltip.Root>
</Tooltip.Provider>

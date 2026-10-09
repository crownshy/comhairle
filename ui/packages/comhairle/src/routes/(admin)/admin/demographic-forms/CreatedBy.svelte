<script lang="ts">
	import * as Avatar from '$lib/components/ui/avatar';
	import { userInitials } from '$lib/utils';

	type Props = { name: string };

	let { name }: Props = $props();
</script>

<span class="flex items-center gap-2">
	<Avatar.Root class="size-8">
		<Avatar.Fallback class="text-sm">{userInitials(name)}</Avatar.Fallback>
	</Avatar.Root>
	<span class="text-muted-foreground text-sm">{name}</span>
</span>

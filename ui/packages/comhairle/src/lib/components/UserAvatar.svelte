<script lang="ts">
	import type { UserDto } from '@crown-shy/api-client/api';
	import * as Avatar from '$lib/components/ui/avatar';
	import { userInitials } from '$lib/utils';
	type Props = {
		user: UserDto;
	};
	let { user }: Props = $props();
	let user_initials = $derived(userInitials(user?.username ?? ''));
</script>

<div class="bg-primary mt-12 flex flex-col items-center">
	<Avatar.Root class="mb-4 h-20 w-20">
		{#if user.avatarUrl}
			<Avatar.Image src={user.avatarUrl} alt="@shadcn" />
		{/if}
		<Avatar.Fallback>{user_initials}</Avatar.Fallback>
	</Avatar.Root>
	<h2 class="text-2xl capitalize">{user.username}</h2>
</div>

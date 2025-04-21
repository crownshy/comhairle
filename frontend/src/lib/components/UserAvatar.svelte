<script lang="ts">
	import * as Avatar from '$lib/components/ui/avatar';
	import { userInitals } from '$lib/utils';
	let { user } = $props();
	let user_initals = $derived(userInitals(user?.username));
</script>

<div class="mt-12 flex flex-col items-center">
	<Avatar.Root class="mb-4 h-40 w-40">
		{#if user.avatar_url}
			<Avatar.Image src={user.avatar_url} alt="@shadcn" />
		{/if}
		<Avatar.Fallback>{user_initals}</Avatar.Fallback>
	</Avatar.Root>
	<h2 class="text-2xl capitalize">{user.username}</h2>
</div>

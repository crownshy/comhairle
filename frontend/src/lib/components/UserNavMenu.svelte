<script lang="ts">
	import { Button } from './ui/button';
	import UserAvatar from './UserAvatar.svelte';
	import * as m from '$lib/paraglide/messages';
	import type { User } from '$lib/api/api';
	import { LoginButtons } from '$lib/profile';

	type Props = {
		user: User;
	};

	let props: Props = $props();
	let user = props.user;
</script>

<div class="flex w-full flex-col items-center gap-4">
	{#if user}
		<div class="flex flex-col gap-4">
			<UserAvatar {user} />
			<form method="POST" action="/auth/logout">
				<Button type="submit" variant="outline" class="text-gray-700 hover:text-black">
					{m.logout()}
				</Button>
			</form>
		</div>
	{:else}
		<LoginButtons />
	{/if}
</div>

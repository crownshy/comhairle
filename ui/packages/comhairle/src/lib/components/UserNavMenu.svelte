<script lang="ts">
	import { logOut } from '$lib/utils/logout';
	import { Button } from './ui/button';
	import UserAvatar from './UserAvatar.svelte';
	import { m } from '$lib/paraglide/messages';
	import type { UserDto } from '@crownshy/api-client/api';
	import { LoginButtons } from '$lib/profile';

	type Props = {
		user: UserDto;
	};

	let props: Props = $props();
	let user = props.user;
</script>

<div class="flex w-full flex-col items-center gap-4">
	{#if user}
		<div class="flex flex-col gap-4">
			<UserAvatar {user} />
			<form
				method="POST"
				onsubmit={(e) => {
					e.preventDefault();
					logOut();
				}}
			>
				<Button type="submit" variant="outline" class="text-gray-700 hover:text-black">
					{m.logout()}
				</Button>
			</form>
		</div>
	{:else}
		<LoginButtons />
	{/if}
</div>

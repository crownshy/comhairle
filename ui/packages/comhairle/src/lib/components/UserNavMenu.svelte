<script lang="ts">
	import { Button } from './ui/button';
	import UserAvatar from './UserAvatar.svelte';
	import * as m from '$lib/paraglide/messages';
	import type { UserDto } from '@crownshy/api-client/api';
	import { LoginButtons } from '$lib/profile';
	import { apiClient } from '@crownshy/api-client/client';
	import { notifications } from '$lib/notifications.svelte';
	import { goto } from '$app/navigation';

	type Props = {
		user: UserDto;
	};

	let props: Props = $props();
	let user = props.user;

	async function attemptLogout() {
		try {
			await apiClient.LogoutUser(undefined);

			await goto('/', { invalidate: ['user'] });
		} catch (e) {
			console.error(e);
			notifications.send({
				priority: 'ERROR',
				message: 'An error occurred when attempting to logout '
			});
		}
	}
</script>

<div class="flex w-full flex-col items-center gap-4">
	{#if user}
		<div class="flex flex-col gap-4">
			<UserAvatar {user} />
			<form
				method="POST"
				onsubmit={(e) => {
					e.preventDefault();
					attemptLogout();
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

<script lang="ts">
	import * as Avatar from '$lib/components/ui/avatar';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import { Button } from '$lib/components/ui/form';
	import { buttonVariants } from '$lib/components/ui/button';
	import LoginButtons from './LoginButtons.svelte';
	import { userInitials } from '$lib/utils';
	const { user } = $props();

	let user_initials = $derived(userInitials(user?.username));
</script>

{#if user}
	<DropdownMenu.Root>
		<DropdownMenu.Trigger class={buttonVariants({ variant: 'outline' })}>
			<Avatar.Root class="mr-4 h-6 w-6">
				{#if user.avatar_url}
					<Avatar.Image src={user.avatar_url} alt="@shadcn" />
				{/if}
				<Avatar.Fallback class="text-foreground">{user_initials}</Avatar.Fallback>
			</Avatar.Root>
			<p class="text-foreground">
				{#if user.auth_type === 'annon'}
					Anonymous
				{:else}
					{user.username}
				{/if}
			</p>
		</DropdownMenu.Trigger>
		<DropdownMenu.Content>
			<DropdownMenu.Group>
				<DropdownMenu.Item>
					<h2>Id {user.username}</h2>
				</DropdownMenu.Item>
				<DropdownMenu.Item>
					<form method="POST" action="/auth/logout">
						<Button type="submit" variant="ghost" fullWidth>Logout</Button>
					</form>
				</DropdownMenu.Item>
			</DropdownMenu.Group>
		</DropdownMenu.Content>
	</DropdownMenu.Root>
{:else}
	<LoginButtons />
{/if}

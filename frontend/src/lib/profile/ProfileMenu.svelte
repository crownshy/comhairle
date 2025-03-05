<script lang="ts">
  import * as Avatar from "$lib/components/ui/avatar";
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import { Button } from '$lib/components/ui/form';
	import { buttonVariants } from '$lib/components/ui/button';
	import LoginButtons from './LoginButtons.svelte';
	const { user } = $props();
	let user_initals = $derived(user?.username.split(/\s/).map(s=>s.charAt(0).toUpperCase()).join(""));
</script>

{#if user}
	<DropdownMenu.Root>
		<DropdownMenu.Trigger class={buttonVariants({ variant: 'outline' })}
			>
				<Avatar.Root class="h-6 w-6 mr-4">
					{#if user.avatar_url}
					  <Avatar.Image src={user.avatar_url} alt="@shadcn" />
				  {/if}
				  <Avatar.Fallback>{user_initals}</Avatar.Fallback>
				</Avatar.Root>
				{user.username}
			</DropdownMenu.Trigger
		>
		<DropdownMenu.Content>
			<DropdownMenu.Group>
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

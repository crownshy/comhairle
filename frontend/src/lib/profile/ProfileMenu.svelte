<script lang="ts">
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import { Button } from '$lib/components/ui/form';
	import { buttonVariants } from '$lib/components/ui/button';
	import LoginButtons from './LoginButtons.svelte';
	const { user } = $props();
</script>

{#if user}
	<DropdownMenu.Root>
		<DropdownMenu.Trigger class={buttonVariants({ variant: 'outline' })}
			>{user.username}</DropdownMenu.Trigger
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

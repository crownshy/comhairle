<script lang="ts">
	import type { UserDto } from '@crownshy/api-client/api';
	import * as Avatar from '$lib/components/ui/avatar';
	import { cn, userInitials } from '$lib/utils';

	type Props = Pick<UserDto, 'username' | 'avatarUrl'> & {
		class?: string;
		withUsername?: boolean;
	};
	let { username, avatarUrl, class: avatarClasses, withUsername }: Props = $props();

	let user_initials = $derived(userInitials(username ?? ''));
</script>

<Avatar.Root class={cn('size-8', avatarClasses)}>
	{#if avatarUrl}
		<Avatar.Image src={avatarUrl} alt="Avatar for {username}" />
	{/if}
	<Avatar.Fallback>{user_initials}</Avatar.Fallback>
</Avatar.Root>
{#if withUsername}
	<h2 class="text-2xl capitalize">{username}</h2>
{/if}

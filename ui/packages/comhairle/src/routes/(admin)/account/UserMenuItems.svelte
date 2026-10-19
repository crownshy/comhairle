<script lang="ts">
	import { goto } from '$app/navigation';
	import { resolve } from '$app/paths';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import { Home, Info, LogOut, Moon, Settings, Sun } from 'lucide-svelte';
	import { themeStore } from '$lib/stores/theme.svelte';
	import { apiClient } from '@crownshy/api-client/client';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import { key } from '$lib/utils/invalidationKey';
	import { notifications } from '$lib/notifications.svelte';
	import { m } from '$lib/paraglide/messages';

	async function signOut() {
		const logout = await tryCatchAsync(async () => {
			await apiClient.LogoutUser(undefined);
			await goto(resolve('/'), { invalidate: [key('user')] });
		});
		if (logout.err) {
			notifications.send({ priority: 'ERROR', message: m.logout_failed() });
		}
	}
</script>

<DropdownMenu.Item>
	{#snippet child({ props })}
		<a {...props} href={resolve('/settings')}>
			<Settings class="size-4" /> Settings
		</a>
	{/snippet}
</DropdownMenu.Item>
<DropdownMenu.Item>
	{#snippet child({ props })}
		<a {...props} href={resolve('/')}>
			<Home class="size-4" /> Back to home
		</a>
	{/snippet}
</DropdownMenu.Item>
<DropdownMenu.Item>
	{#snippet child({ props })}
		<a {...props} href={resolve('/about')}>
			<Info class="size-4" /> About Comhairle
		</a>
	{/snippet}
</DropdownMenu.Item>
<DropdownMenu.Item closeOnSelect={false} onclick={() => themeStore.toggleMode()}>
	{#if themeStore.isDark}
		<Sun class="size-4" /> Light mode
	{:else}
		<Moon class="size-4" /> Dark mode
	{/if}
</DropdownMenu.Item>
<DropdownMenu.Separator />
<DropdownMenu.Item onclick={signOut}>
	<LogOut class="size-4" /> Sign out
</DropdownMenu.Item>

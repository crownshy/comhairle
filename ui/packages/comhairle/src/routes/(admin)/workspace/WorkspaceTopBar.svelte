<script lang="ts">
	import { resolve } from '$app/paths';
	import * as Avatar from '$lib/components/ui/avatar';
	import * as Breadcrumb from '$lib/components/ui/breadcrumb';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import type { UserDto } from '@crownshy/api-client/api';
	import { userInitials } from '$lib/utils';
	import { m } from '$lib/paraglide/messages';
	import ComhairleLogo from '$lib/components/ComhairleLogo.svelte';
	import UserMenuItems from '../account/UserMenuItems.svelte';
	import { workspaceCrumbs } from './workspaceLinks';

	type Props = { user: UserDto; path: string };
	let { user, path }: Props = $props();

	let crumbs = $derived(workspaceCrumbs(path));
</script>

<!-- Crumb hrefs come from workspaceCrumbs, which only builds /admin section paths. -->
<!-- eslint-disable svelte/no-navigation-without-resolve -->
<header class="border-border bg-background flex h-14 w-full shrink-0 border-b">
	<div
		class="px-gutter mx-auto flex w-full max-w-336 items-center justify-between gap-4 sm:pr-8 lg:pr-16"
	>
		<div class="flex min-w-0 items-center gap-4">
			<div class="flex shrink-0 items-center gap-2">
				<ComhairleLogo href={resolve('/admin')} logoSize="sm" color="text-primary" />
				<span
					class="bg-primary/10 text-primary rounded-full px-2 py-0.5 text-sm font-semibold"
				>
					Admin
				</span>
			</div>
			<span class="bg-border hidden h-6 w-px sm:block" aria-hidden="true"></span>
			<Breadcrumb.Root class="hidden min-w-0 sm:block">
				<Breadcrumb.List class="text-base">
					<Breadcrumb.Item>
						{#if crumbs.length === 0}
							<Breadcrumb.Page>{m.workspace()}</Breadcrumb.Page>
						{:else}
							<Breadcrumb.Link href={resolve('/admin')}
								>{m.workspace()}</Breadcrumb.Link
							>
						{/if}
					</Breadcrumb.Item>
					{#each crumbs as crumb (crumb.label)}
						<Breadcrumb.Separator />
						<Breadcrumb.Item>
							{#if crumb.href}
								<Breadcrumb.Link href={crumb.href}>{crumb.label}</Breadcrumb.Link>
							{:else}
								<Breadcrumb.Page>{crumb.label}</Breadcrumb.Page>
							{/if}
						</Breadcrumb.Item>
					{/each}
				</Breadcrumb.List>
			</Breadcrumb.Root>
		</div>

		<DropdownMenu.Root>
			<DropdownMenu.Trigger
				class="hover:bg-muted flex items-center gap-2 rounded-full p-1"
				aria-label="Account menu"
			>
				<Avatar.Root class="size-8 rounded-full">
					{#if user?.avatarUrl}
						<Avatar.Image src={user.avatarUrl} alt={user.username} />
					{/if}
					<Avatar.Fallback class="rounded-full">
						{userInitials(user?.username ?? '')}
					</Avatar.Fallback>
				</Avatar.Root>
			</DropdownMenu.Trigger>
			<DropdownMenu.Content align="end" class="w-56">
				<DropdownMenu.Label class="flex flex-col">
					<span class="truncate text-base font-semibold">{user?.username}</span>
					{#if user?.email}
						<span class="text-muted-foreground truncate text-sm font-normal">
							{user.email}
						</span>
					{/if}
				</DropdownMenu.Label>
				<DropdownMenu.Separator />
				<UserMenuItems />
			</DropdownMenu.Content>
		</DropdownMenu.Root>
	</div>
</header>

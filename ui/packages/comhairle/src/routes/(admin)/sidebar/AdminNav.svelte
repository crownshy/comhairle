<script lang="ts">
	import { resolve } from '$app/paths';
	import * as SideBar from '$lib/components/ui/sidebar';
	import { ArrowLeft, PanelLeftClose, PanelLeftOpen } from 'lucide-svelte';
	import { Button } from '$lib/components/ui/button';
	import ComhairleLogo from '$lib/components/ComhairleLogo.svelte';
	import SidebarResizeHandle from '$lib/components/SidebarResizeHandle.svelte';
	import { useSidebar } from '$lib/components/ui/sidebar/context.svelte.js';
	import { SIDEBAR_KEYBOARD_SHORTCUT } from '$lib/components/ui/sidebar/constants';
	import { useSidebarWidth } from '$lib/components/sidebarWidthContext.svelte.js';
	import { EXPAND_WIDTH } from '$lib/components/sidebarWidth.js';
	import { hasSiteAdminRole, hasSuperAdminRole } from '$lib/utils/permissions';
	import type {
		LocalizedConversationDto,
		UserOrganizationAccess,
		UserDto,
		UserRoles
	} from '@crownshy/api-client/api';
	import type { StageConversation } from './conversationStages';
	import ConversationSection from './ConversationSection.svelte';
	import OrganizationSwitcher from './OrganizationSwitcher.svelte';
	import UserMenu from './UserMenu.svelte';
	import WorkspaceLinks from './WorkspaceLinks.svelte';
	import WorkspaceSection from './WorkspaceSection.svelte';

	type Props = {
		latestConversations: LocalizedConversationDto[];
		organizations: UserOrganizationAccess[];
		/** The conversation being edited, which switches the sidebar to its stages. */
		conversation: StageConversation | undefined;
		stepCount: number;
		user: UserDto;
		userRoles: readonly UserRoles[] | undefined;
		path: string;
	};

	let props: Props = $props();
	let isSuperAdmin = $derived(hasSuperAdminRole(props.userRoles));
	let isSiteAdmin = $derived(hasSiteAdminRole(props.userRoles));

	const sidebar = useSidebar();
	const sidebarWidth = useSidebarWidth();
	const toggleTitle = `Open / close (Ctrl+${SIDEBAR_KEYBOARD_SHORTCUT} or Cmd+${SIDEBAR_KEYBOARD_SHORTCUT})`;

	function expandSidebar() {
		if (sidebarWidth.width < EXPAND_WIDTH) sidebarWidth.setWidth(EXPAND_WIDTH);
		sidebar.setOpen(true);
		sidebarWidth.persist();
	}
</script>

<SideBar.Root collapsible="icon">
	<SideBar.Header
		class="flex flex-row items-center justify-between gap-2 py-6 pr-3 pl-4 group-data-[collapsible=icon]:flex-col group-data-[collapsible=icon]:px-2 group-data-[collapsible=icon]:py-3"
	>
		<div class="flex min-w-0 flex-1 group-data-[collapsible=icon]:hidden">
			{#if props.conversation}
				<a
					href={resolve('/admin/conversations')}
					class="text-sidebar-foreground/80 hover:text-sidebar-foreground flex items-center gap-2 text-base"
				>
					<ArrowLeft class="size-4" />
					Conversations
				</a>
			{:else if props.organizations.length > 0}
				<OrganizationSwitcher organizations={props.organizations} />
			{:else}
				<ComhairleLogo color="sidebar-foreground" />
			{/if}
		</div>
		<Button
			variant="ghost"
			size="icon"
			class="text-sidebar-foreground/70 hover:text-sidebar size-7 group-data-[collapsible=icon]:hidden"
			title={toggleTitle}
			onclick={() => sidebar.toggle()}
		>
			<PanelLeftClose class="size-4" />
			<span class="sr-only">Collapse sidebar</span>
		</Button>
		<Button
			variant="ghost"
			size="icon"
			class="text-sidebar-foreground/70 hover:text-sidebar mx-auto hidden size-7 group-data-[collapsible=icon]:flex"
			title={toggleTitle}
			onclick={expandSidebar}
		>
			<PanelLeftOpen class="size-4" />
			<span class="sr-only">Expand sidebar</span>
		</Button>
		{#if props.conversation}
			<Button
				variant="ghost"
				size="icon"
				href={resolve('/admin/conversations')}
				class="text-sidebar-foreground/70 hover:text-sidebar mx-auto hidden size-7 group-data-[collapsible=icon]:flex"
				title="Back to conversations"
			>
				<ArrowLeft class="size-4" />
				<span class="sr-only">Back to conversations</span>
			</Button>
		{/if}
	</SideBar.Header>

	<SideBar.Content
		class="flex flex-col gap-4 overflow-x-hidden overflow-y-auto pt-2 pl-4 group-data-[collapsible=icon]:items-center group-data-[collapsible=icon]:overflow-hidden group-data-[collapsible=icon]:p-0 group-data-[collapsible=icon]:pt-2"
	>
		{#if props.conversation}
			<ConversationSection
				conversation={props.conversation}
				stepCount={props.stepCount}
				path={props.path}
			/>
		{:else}
			<WorkspaceSection latestConversations={props.latestConversations} path={props.path} />
		{/if}
	</SideBar.Content>

	<SideBar.Footer class="border-sidebar-border gap-3 border-t">
		<WorkspaceLinks path={props.path} {isSiteAdmin} {isSuperAdmin} />
		<UserMenu user={props.user} />
	</SideBar.Footer>
	<SidebarResizeHandle />
</SideBar.Root>

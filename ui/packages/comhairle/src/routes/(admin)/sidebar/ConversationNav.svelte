<script lang="ts">
	import { resolve } from '$app/paths';
	import * as SideBar from '$lib/components/ui/sidebar';
	import { ArrowLeft, PanelLeftClose, PanelLeftOpen } from 'lucide-svelte';
	import { Button } from '$lib/components/ui/button';
	import SidebarResizeHandle from '$lib/components/SidebarResizeHandle.svelte';
	import { useSidebar } from '$lib/components/ui/sidebar/context.svelte.js';
	import { SIDEBAR_KEYBOARD_SHORTCUT } from '$lib/components/ui/sidebar/constants';
	import { useSidebarWidth } from '$lib/components/sidebarWidthContext.svelte.js';
	import { EXPAND_WIDTH } from '$lib/components/sidebarWidth.js';
	import LaunchConversationModal from '$lib/components/LaunchConversationModal.svelte';
	import EndConversationModal from '$lib/components/EndConversationModal.svelte';
	import type { ConversationWithTranslations, UserDto } from '@crownshy/api-client/api';
	import ConversationHeading from './ConversationHeading.svelte';
	import ConversationSection from './ConversationSection.svelte';
	import UserMenu from './UserMenu.svelte';

	type Props = {
		conversation: ConversationWithTranslations;
		stepCount: number;
		user: UserDto;
		path: string;
	};

	let { conversation, stepCount, user, path }: Props = $props();
	let launchModalOpen = $state(false);
	let endModalOpen = $state(false);

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
		<a
			href={resolve('/admin')}
			class="text-sidebar-foreground/80 hover:text-sidebar-foreground flex min-w-0 flex-1 items-center gap-2 text-base group-data-[collapsible=icon]:hidden"
		>
			<ArrowLeft class="size-4" />
			Workspace
		</a>
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
		<Button
			variant="ghost"
			size="icon"
			href={resolve('/admin')}
			class="text-sidebar-foreground/70 hover:text-sidebar mx-auto hidden size-7 group-data-[collapsible=icon]:flex"
			title="Back to workspace"
		>
			<ArrowLeft class="size-4" />
			<span class="sr-only">Back to workspace</span>
		</Button>
	</SideBar.Header>

	<SideBar.Content
		class="flex flex-col gap-6 overflow-x-hidden overflow-y-auto pt-2 pl-4 group-data-[collapsible=icon]:items-center group-data-[collapsible=icon]:overflow-hidden group-data-[collapsible=icon]:p-0 group-data-[collapsible=icon]:pt-2"
	>
		<ConversationHeading
			{conversation}
			onLaunch={() => (launchModalOpen = true)}
			onToggleComplete={() => (endModalOpen = true)}
		/>
		<ConversationSection
			{conversation}
			{stepCount}
			{path}
			onLaunch={() => (launchModalOpen = true)}
		/>
	</SideBar.Content>

	<SideBar.Footer class="border-sidebar-foreground/20 border-t">
		<UserMenu {user} />
	</SideBar.Footer>
	<SidebarResizeHandle />
</SideBar.Root>

<LaunchConversationModal
	conversation_id={conversation.id}
	bind:open={launchModalOpen}
	hideTrigger
/>
<EndConversationModal {conversation} bind:open={endModalOpen} hideTrigger />

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
	import type { UserDto } from '@crownshy/api-client/api';
	import type { StageConversation } from './conversationStages';
	import ConversationSection from './ConversationSection.svelte';
	import UserMenu from './UserMenu.svelte';

	type Props = {
		conversation: StageConversation;
		stepCount: number;
		hasPolisStep: boolean;
		user: UserDto;
		path: string;
	};

	let { conversation, stepCount, hasPolisStep, user, path }: Props = $props();

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
			href={resolve('/admin/conversations')}
			title="Back to conversations"
			class="text-sidebar-foreground/80 hover:text-sidebar-foreground hover:bg-sidebar-foreground/10 -ml-2 flex h-8 items-center gap-2 rounded-lg px-2 text-base group-data-[collapsible=icon]:hidden"
		>
			<ArrowLeft class="size-4 shrink-0" />
			Conversations
		</a>
		<Button
			variant="ghost"
			size="icon"
			class="text-sidebar-foreground/70 hover:text-sidebar size-7 group-data-[collapsible=icon]:hidden"
			title={toggleTitle}
			aria-label="Collapse sidebar"
			onclick={() => sidebar.toggle()}
		>
			<PanelLeftClose class="size-4" />
		</Button>
		<Button
			variant="ghost"
			size="icon"
			class="text-sidebar-foreground/70 hover:text-sidebar mx-auto hidden size-7 group-data-[collapsible=icon]:flex"
			title={toggleTitle}
			aria-label="Expand sidebar"
			onclick={expandSidebar}
		>
			<PanelLeftOpen class="size-4" />
		</Button>
	</SideBar.Header>

	<SideBar.Content
		class="flex flex-col gap-4 overflow-x-hidden overflow-y-auto pt-2 pl-4 group-data-[collapsible=icon]:items-center group-data-[collapsible=icon]:overflow-hidden group-data-[collapsible=icon]:p-0 group-data-[collapsible=icon]:pt-2"
	>
		<ConversationSection {conversation} {stepCount} {hasPolisStep} {path} />
	</SideBar.Content>

	<SideBar.Footer class="border-sidebar-foreground/20 border-t">
		<UserMenu {user} />
	</SideBar.Footer>
	<SidebarResizeHandle />
</SideBar.Root>

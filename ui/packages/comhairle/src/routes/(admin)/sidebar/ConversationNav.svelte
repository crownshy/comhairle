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
	import type { ConversationWithTranslations, UserDto } from '@crownshy/api-client/api';
	import { conversationTitle } from '$lib/components/Translation/translationUtils';
	import ConversationStatus from './ConversationStatus.svelte';
	import ConversationSection from './ConversationSection.svelte';
	import UserMenu from './UserMenu.svelte';

	type Props = {
		conversation: ConversationWithTranslations;
		stepCount: number;
		hasPolisStep: boolean;
		user: UserDto;
		path: string;
	};

	let { conversation, stepCount, hasPolisStep, user, path }: Props = $props();
	let title = $derived(conversationTitle(conversation));

	const sidebar = useSidebar();
	const sidebarWidth = useSidebarWidth();

	function expandSidebar() {
		if (sidebarWidth.width < EXPAND_WIDTH) sidebarWidth.setWidth(EXPAND_WIDTH);
		sidebar.setOpen(true);
		sidebarWidth.persist();
	}
</script>

<SideBar.Root collapsible="icon">
	<SideBar.Header
		class="flex flex-col gap-2 pt-5 pr-3 pb-3 pl-4 group-data-[collapsible=icon]:items-center group-data-[collapsible=icon]:px-2 group-data-[collapsible=icon]:py-3"
	>
		{@const TOGGLE_TITLE = `Open / close (Ctrl+${SIDEBAR_KEYBOARD_SHORTCUT} or Cmd+${SIDEBAR_KEYBOARD_SHORTCUT})`}
		<div class="flex min-h-8 items-center justify-between gap-2">
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
				class="text-sidebar-foreground/70 hover:text-sidebar size-7 shrink-0 group-data-[collapsible=icon]:hidden"
				title={TOGGLE_TITLE}
				aria-label="Collapse sidebar"
				onclick={() => sidebar.toggle()}
			>
				<PanelLeftClose class="size-4" />
			</Button>
			<Button
				variant="ghost"
				size="icon"
				class="text-sidebar-foreground/70 hover:text-sidebar hidden size-7 group-data-[collapsible=icon]:flex"
				title={TOGGLE_TITLE}
				aria-label="Expand sidebar"
				onclick={expandSidebar}
			>
				<PanelLeftOpen class="size-4" />
			</Button>
		</div>
		<h1
			class="text-sidebar-foreground line-clamp-2 text-lg leading-7 font-semibold group-data-[collapsible=icon]:hidden"
			{title}
		>
			{title}
		</h1>
	</SideBar.Header>

	<SideBar.Content
		class="flex flex-col gap-6 overflow-x-hidden overflow-y-auto pt-2 pl-4 [scrollbar-color:color-mix(in_oklab,var(--sidebar-foreground)_25%,transparent)_transparent] [scrollbar-width:thin] group-data-[collapsible=icon]:items-center group-data-[collapsible=icon]:overflow-hidden group-data-[collapsible=icon]:p-0 group-data-[collapsible=icon]:pt-2"
	>
		<ConversationStatus {conversation} />
		<ConversationSection {conversation} {stepCount} {hasPolisStep} {path} />
	</SideBar.Content>

	<SideBar.Footer class="border-sidebar-foreground/20 border-t">
		<UserMenu {user} />
	</SideBar.Footer>
	<SidebarResizeHandle />
</SideBar.Root>

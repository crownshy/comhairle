<script lang="ts">
	import Logo from '$lib/assets/comhairle_logo.svg';
	import UserAvatar from '$lib/components/UserAvatar.svelte';
	import * as SideBar from '$lib/components/ui/sidebar';
	import {
		Home,
		Info,
		LayoutDashboard,
		MessageSquareText,
		NotebookText,
		Pencil,
		Plus,
		Settings,
		TerminalSquare,
		UsersRound
	} from 'lucide-svelte';
	import { Button } from './ui/button';
	let props = $props();
	let user = $derived(props.user);
	let conversations = $derived(props.conversations);

	// TODO We need to use data-sveltekit-reload as the
	// component isn't relaoading on navigation when we use
	// page.ts for the data fetching
</script>

<SideBar.Provider>
	<SideBar.Root>
		<SideBar.Header>
			<div class="flex flex-row items-center gap-4 p-4">
				<img src={Logo} alt="Comhairle Logo" />
				<h1 class="text-nav-text text-xl font-bold">Comhairle</h1>
			</div>
		</SideBar.Header>
		<SideBar.Content class="radius-nav">
			<SideBar.Group>
				<SideBar.GroupContent>
					<SideBar.Menu>
						<SideBar.MenuItem>
							<SideBar.MenuButton>
								{#snippet child({ props })}
									<a {...props} href="/">
										<Home />
										Home
									</a>
								{/snippet}
							</SideBar.MenuButton>
						</SideBar.MenuItem>
						<SideBar.MenuItem>
							<SideBar.MenuButton>
								{#snippet child({ props })}
									<a {...props} href="/admin/">
										<LayoutDashboard />
										Dashboard
									</a>
								{/snippet}
							</SideBar.MenuButton>
						</SideBar.MenuItem>
					</SideBar.Menu>
				</SideBar.GroupContent>
			</SideBar.Group>
			<SideBar.Group>
				<SideBar.GroupLabel>Conversations</SideBar.GroupLabel>
				<SideBar.GroupContent>
					{#if conversations}
						<SideBar.Menu>
							{#each conversations.records as conversation}
								<SideBar.MenuItem>
									<SideBar.MenuButton>
										{#snippet child({ props })}
											<a {...props} href={`/admin/conversations/${conversation.id}/configure`}>
												<MessageSquareText />
												{conversation.title}
											</a>
										{/snippet}
									</SideBar.MenuButton>
									<SideBar.MenuSub>
										<SideBar.MenuSubItem>
											<SideBar.MenuSubButton
												href={`/admin/conversations/${conversation.id}/configure`}
												><TerminalSquare
													class="stroke-nav-text hover:stroke-sidebar-foreground"
												/>Configure</SideBar.MenuSubButton
											>
										</SideBar.MenuSubItem>
									</SideBar.MenuSub>
									<SideBar.MenuSub>
										<SideBar.MenuSubItem>
											<SideBar.MenuSubButton
												href={`/admin/conversations/${conversation.id}/invites`}
												><UsersRound class="stroke-nav-text hover:stroke-sidebar-foreground" /> Recruit</SideBar.MenuSubButton
											>
										</SideBar.MenuSubItem>
									</SideBar.MenuSub>
									<SideBar.MenuSub>
										<SideBar.MenuSubItem>
											<SideBar.MenuSubButton href={`/admin/conversations/${conversation.id}/design`}
												><Pencil class="stroke-nav-text hover:stroke-sidebar-foreground" /> Design</SideBar.MenuSubButton
											>
										</SideBar.MenuSubItem>
									</SideBar.MenuSub>
									<SideBar.MenuSub>
										<SideBar.MenuSubItem>
											<SideBar.MenuSubButton href={`/admin/conversations/${conversation.id}/report`}
												><NotebookText class="stroke-nav-text hover:stroke-sidebar-foreground" /> Report</SideBar.MenuSubButton
											>
										</SideBar.MenuSubItem>
									</SideBar.MenuSub>
								</SideBar.MenuItem>
							{/each}
							<SideBar.MenuItem>
								<Button href="/admin/conversations/new" class="w-full" variant="secondary">
									<Plus />
									New Conversation
								</Button>
							</SideBar.MenuItem>
						</SideBar.Menu>
					{/if}
				</SideBar.GroupContent>
			</SideBar.Group>
			<SideBar.Group>
				<SideBar.GroupContent>
					<SideBar.Menu>
						<SideBar.MenuItem>
							<SideBar.MenuButton>
								{#snippet child({ props })}
									<a {...props} href="/admin/">
										<Settings />
										Settings
									</a>
								{/snippet}
							</SideBar.MenuButton>
						</SideBar.MenuItem>
						<SideBar.MenuItem>
							<SideBar.MenuButton>
								{#snippet child({ props })}
									<a {...props} href="/admin/">
										<Info />
										About
									</a>
								{/snippet}
							</SideBar.MenuButton>
						</SideBar.MenuItem>
					</SideBar.Menu>
				</SideBar.GroupContent>
			</SideBar.Group>
		</SideBar.Content>
		<SideBar.Footer>
			<UserAvatar {user} />
		</SideBar.Footer>
	</SideBar.Root>
</SideBar.Provider>

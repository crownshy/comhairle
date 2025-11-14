<script lang="ts">
	import Logo from '$lib/assets/comhairle_logo.svg';
	import UserAvatar from '$lib/components/UserAvatar.svelte';
	import * as SideBar from '$lib/components/ui/sidebar';
	import * as Collapsible from '$lib/components/ui/collapsible';
	import * as Tooltip from '$lib/components/ui/tooltip';
	import {
		Home,
		Info,
		LayoutDashboard,
		MessageSquareText,
		NotebookText,
		Pencil,
		Binoculars,
		Plus,
		Settings,
		TerminalSquare,
		UsersRound,
		Bell
	} from 'lucide-svelte';
	import { Button } from './ui/button';
	let props = $props();
	let path = $derived(props.path);
	console.log('Path is ', path);
	let user = $derived(props.user);
	let conversations = $derived(props.conversations);

	// TODO We need to use data-sveltekit-reload as the
	// component isn't relaoading on navigation when we use
	// page.ts for the data fetching
</script>

<SideBar.Provider>
	<SideBar.Root class="w-[400px]">
		<SideBar.Header>
			<a href="/">
				<div class="flex flex-row items-center gap-4 p-4">
					<img class="w-10" src={Logo} alt="Comhairle Logo" />
					<h1 class="text-nav-text text-xl font-bold">Comhairle</h1>
				</div>
			</a>
		</SideBar.Header>
		<SideBar.Content class="radius-nav">
			<SideBar.Group>
				<SideBar.GroupContent>
					<SideBar.Menu>
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
								<Collapsible.Root open={path.includes(conversation.id)} class="group/collapsible">
									<SideBar.MenuItem>
										<Collapsible.Trigger>
											{#snippet child({ props })}
												<Tooltip.Provider>
													<Tooltip.Root>
														<Tooltip.Trigger>
															<SideBar.MenuButton class="text-nowrap text-ellipsis " {...props}>
																{#snippet child({ props })}
																	<a
																		{...props}
																		href={`/admin/conversations/${conversation.id}/configure`}
																	>
																		<MessageSquareText />
																		{conversation.title}
																	</a>
																{/snippet}
															</SideBar.MenuButton>
														</Tooltip.Trigger>
														<Tooltip.Content side="right">
															{conversation.title}
														</Tooltip.Content>
													</Tooltip.Root>
												</Tooltip.Provider>
											{/snippet}
										</Collapsible.Trigger>
										<Collapsible.Content>
											<SideBar.MenuSub>
												<SideBar.MenuSubItem>
													<SideBar.MenuSubButton
														href={`/admin/conversations/${conversation.id}/configure`}
														class={path.includes('configure') ? 'font-bold' : ''}
														><TerminalSquare
															class="stroke-nav-text hover:stroke-sidebar-foreground"
														/>Configure</SideBar.MenuSubButton
													>
												</SideBar.MenuSubItem>
											</SideBar.MenuSub>
											<SideBar.MenuSub>
												<SideBar.MenuSubItem>
													<SideBar.MenuSubButton
														href={`/admin/conversations/${conversation.id}/design`}
														class={path.includes('design') ? 'font-bold' : ''}
														><Pencil class="stroke-nav-text hover:stroke-sidebar-foreground" /> Design</SideBar.MenuSubButton
													>
												</SideBar.MenuSubItem>
											</SideBar.MenuSub>
											<SideBar.MenuSub>
												<SideBar.MenuSubItem>
													<SideBar.MenuSubButton
														href={`/admin/conversations/${conversation.id}/invites`}
														class={path.includes('invites') ? 'font-bold' : ''}
														><UsersRound class="stroke-nav-text hover:stroke-sidebar-foreground" /> Recruit</SideBar.MenuSubButton
													>
												</SideBar.MenuSubItem>
											</SideBar.MenuSub>
											<SideBar.MenuSub>
												<SideBar.MenuSubItem>
													<SideBar.MenuSubButton
														href={`/admin/conversations/${conversation.id}/monitor`}
														class={path.includes('monitor') ? 'font-bold' : ''}
														><Binoculars class="stroke-nav-text hover:stroke-sidebar-foreground" /> Monitor</SideBar.MenuSubButton
													>
												</SideBar.MenuSubItem>
											</SideBar.MenuSub>
											<SideBar.MenuSub>
												<SideBar.MenuSubItem>
													<SideBar.MenuSubButton
														href={`/admin/conversations/${conversation.id}/moderate`}
														class={path.includes('moderate') ? 'font-bold' : ''}
														><UsersRound class="stroke-nav-text hover:stroke-sidebar-foreground" /> Moderate</SideBar.MenuSubButton
													>
												</SideBar.MenuSubItem>
											</SideBar.MenuSub>
											<SideBar.MenuSub>
												<SideBar.MenuSubItem>
													<SideBar.MenuSubButton
														href={`/admin/conversations/${conversation.id}/notifications`}
														class={path.includes('notifications') ? 'font-bold' : ''}
														><Bell class="stroke-nav-text hover:stroke-sidebar-foreground" /> Notify</SideBar.MenuSubButton
													>
												</SideBar.MenuSubItem>
											</SideBar.MenuSub>
											<SideBar.MenuSub>
												<SideBar.MenuSubItem>
													<SideBar.MenuSubButton
														href={`/admin/conversations/${conversation.id}/report`}
														class={path.includes('report') ? 'font-bold' : ''}
														><NotebookText
															class="stroke-nav-text hover:stroke-sidebar-foreground"
														/> Report</SideBar.MenuSubButton
													>
												</SideBar.MenuSubItem>
											</SideBar.MenuSub>
										</Collapsible.Content>
									</SideBar.MenuItem>
								</Collapsible.Root>
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

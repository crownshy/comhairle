<script lang="ts">
	import Logo from '$lib/assets/comhairle_logo.svg';
	import UserAvatar from '$lib/components/UserAvatar.svelte';
	import * as SideBar from '$lib/components/ui/sidebar';
	import * as Collapsible from '$lib/components/ui/collapsible';
	import * as ScrollArea from '$lib/components/ui/scroll-area';
	import {
		Home,
		Info,
		LayoutDashboard,
		MessageSquareText,
		Plus,
		Settings,
		TerminalSquare,
		UsersRound,
		Bell,
		Database,
		ChevronDown
	} from 'lucide-svelte';
	import { conversationSteps } from '$lib/config/conversation-steps';
	import { Button } from './ui/button';
	import { page } from '$app/state';
	let props = $props();
	let path = $derived(props.path);
	console.log('Path is ', path);
	let user = $derived(props.user);
	let conversations = $derived(props.conversations);
	let workflow_steps = $derived(page.data?.workflow_steps ?? []);

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
		<SideBar.Content class="radius-nav overflow-x-hidden">
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
			<SideBar.Group class="flex min-h-0 flex-1 flex-col">
				<SideBar.GroupLabel class="shrink-0">Conversations</SideBar.GroupLabel>
				<SideBar.GroupContent class="min-h-0 flex-1">
					<ScrollArea.Root class="h-full" type="always">
						{#if conversations}
							<SideBar.Menu>
								{#each conversations.records as conversation}
									<Collapsible.Root
										open={path.includes(conversation.id)}
										class="group/collapsible"
									>
										<SideBar.MenuItem>
											<Collapsible.Trigger>
												{#snippet child({ props })}
													<SideBar.MenuButton
														class="w-full overflow-hidden"
														{...props}
													>
														{#snippet child({ props })}
															<a
																{...props}
																href={`/admin/conversations/${conversation.id}/configure`}
																class="flex w-full items-start gap-2"
															>
																<MessageSquareText
																	class="shrink-0"
																/>
																<span
																	class="break-words whitespace-normal"
																	>{conversation.title}</span
																>
															</a>
														{/snippet}
													</SideBar.MenuButton>
												{/snippet}
											</Collapsible.Trigger>
											<Collapsible.Content>
												{#each conversationSteps as step}
													{#if step.path === 'design'}
														<Collapsible.Root
															open={path.includes('design')}
															class="group/design"
														>
															<SideBar.MenuSub>
																<SideBar.MenuSubItem>
																	<Collapsible.Trigger
																		class="w-full"
																	>
																		<SideBar.MenuSubButton
																			href={`/admin/conversations/${conversation.id}/design`}
																			class="{path.includes(
																				'design'
																			)
																				? 'font-bold'
																				: ''} hover:text-black"
																			aria-disabled={!(
																				step.activeOnLive &&
																				conversation.is_live
																			)}
																		>
																			<step.icon
																				class="stroke-nav-text group-hover/menu-button:stroke-black"
																			/>
																			<span
																				class="flex-1 text-left"
																				>{step.name}</span
																			>
																			<ChevronDown
																				class="h-4 w-4 stroke-white transition-transform group-hover/menu-button:stroke-black group-data-[state=open]/design:rotate-180"
																			/>
																		</SideBar.MenuSubButton>
																	</Collapsible.Trigger>
																</SideBar.MenuSubItem>
															</SideBar.MenuSub>
															<Collapsible.Content>
																<div
																	class="border-sidebar-border relative mr-6 ml-6 border-l pl-2"
																>
																	{#if path.includes(conversation.id) && workflow_steps?.length > 0}
																		{#each workflow_steps as wfStep (wfStep.id)}
																			<a
																				href={`/admin/conversations/${conversation.id}/design/step/${wfStep.id}`}
																				class="text-sidebar-foreground hover:bg-sidebar-accent block rounded-lg px-2 py-1.5 text-sm hover:text-black {path.includes(
																					wfStep.id
																				)
																					? 'font-bold'
																					: ''}"
																			>
																				{wfStep.name}
																			</a>
																		{/each}
																	{/if}
																	<a
																		href={`/admin/conversations/${conversation.id}/design?addStep=true`}
																		class="text-sidebar-foreground/40 hover:bg-sidebar-accent block rounded-lg px-2 py-1.5 text-sm hover:text-black"
																	>
																		+ Add new
																	</a>
																</div>
															</Collapsible.Content>
														</Collapsible.Root>
													{:else}
														<SideBar.MenuSub>
															<SideBar.MenuSubItem>
																<SideBar.MenuSubButton
																	href={`/admin/conversations/${conversation.id}/${step.path}`}
																	class="{path.includes(step.path)
																		? 'font-bold'
																		: ''} hover:text-black"
																	><step.icon
																		class="stroke-nav-text group-hover/menu-button:stroke-black"
																	/>
																	{step.name}</SideBar.MenuSubButton
																>
															</SideBar.MenuSubItem>
														</SideBar.MenuSub>
													{/if}
												{/each}
											</Collapsible.Content>
										</SideBar.MenuItem>
									</Collapsible.Root>
								{/each}
							</SideBar.Menu>
						{/if}
					</ScrollArea.Root>
				</SideBar.GroupContent>
				<div class="shrink-0 p-2">
					<Button href="/admin/conversations/new" class="w-full" variant="secondary">
						<Plus />
						New Conversation
					</Button>
				</div>
			</SideBar.Group>
		</SideBar.Content>
		<SideBar.Footer class="flex flex-col gap-1">
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
			<UserAvatar {user} />
		</SideBar.Footer>
	</SideBar.Root>
</SideBar.Provider>

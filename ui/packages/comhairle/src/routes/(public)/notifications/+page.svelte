<script lang="ts">
	import type { PageData } from './$types';
	import { Button } from '$lib/components/ui/button';
	import { Badge } from '$lib/components/ui/badge';
	import {
		Card,
		CardContent,
		CardDescription,
		CardHeader,
		CardTitle
	} from '$lib/components/ui/card';
	import { Separator } from '$lib/components/ui/separator';
	import { apiClient } from '$lib/api/client';
	import { notifications as notificationService } from '$lib/notifications.svelte';
	import { invalidateAll } from '$app/navigation';
	import ConversationContextImage from '$lib/components/ConversationContextImage.svelte';
	import {
		Bell,
		BellRing,
		Check,
		CheckCheck,
		Info,
		AlertCircle,
		AlertTriangle,
		CheckCircle,
		Calendar,
		Eye,
		EyeOff
	} from 'lucide-svelte';
	import type { NotificationWithDeliveryDto } from '$lib/api/api';
	import { formatDistanceToNow } from 'date-fns';

	let { data }: PageData = $props();

	let showAll = $state(false);
	let markingAllAsRead = $state(false);
	let markingAsRead = $state(new Set<string>());

	let displayNotifications = $derived(showAll ? data.allNotifications : data.unreadNotifications);
	let hasUnread = $derived(data.unreadCount.count > 0);

	function getNotificationIcon(type: string) {
		switch (type) {
			case 'info':
				return Info;
			case 'warning':
				return AlertTriangle;
			case 'error':
				return AlertCircle;
			case 'success':
				return CheckCircle;
			default:
				return Bell;
		}
	}

	$effect(() => {
		async function reloadNotifications() {}
		let timeoutId = setTimeout(async () => {
			await invalidateAll();
		}, 5000);
		return () => {
			window.clearTimeout(timeoutId);
		};
	});

	function getNotificationBadgeVariant(type: string) {
		switch (type) {
			case 'info':
				return 'default';
			case 'warning':
				return 'draft';
			case 'error':
				return 'destructive';
			case 'success':
				return 'default';
			default:
				return 'default';
		}
	}

	async function markAsRead(deliveryId: string) {
		if (markingAsRead.has(deliveryId)) return;

		markingAsRead.add(deliveryId);
		try {
			await apiClient.MarkNotificationAsRead(undefined, {
				params: { delivery_id: deliveryId }
			});
			await invalidateAll();
			notificationService.send({
				message: 'Notification marked as read',
				priority: 'SUCCESS'
			});
		} catch (error: any) {
			console.error(error);
			notificationService.send({
				message: 'Failed to mark notification as read',
				priority: 'ERROR'
			});
		} finally {
			markingAsRead.delete(deliveryId);
		}
	}

	async function markAllAsRead() {
		if (markingAllAsRead) return;

		markingAllAsRead = true;
		try {
			await apiClient.MarkAllNotificationsAsRead(undefined);
			await invalidateAll();
			notificationService.send({
				message: 'All notifications marked as read',
				priority: 'SUCCESS'
			});
		} catch (error: any) {
			console.error(error);
			notificationService.send({
				message: 'Failed to mark all notifications as read',
				priority: 'ERROR'
			});
		} finally {
			markingAllAsRead = false;
		}
	}

	function isUnread(notification: NotificationWithDeliveryDto) {
		return !notification.readAt;
	}
</script>

<div class="container mx-auto max-w-4xl px-4 py-6">
	<div class="mb-6 flex flex-col items-start justify-between gap-4 sm:flex-row sm:items-center">
		<div class="flex flex-col">
			<div class="flex items-center gap-3">
				<BellRing class="h-8 w-8" />
				<div>
					<h1 class="text-3xl font-bold">Notifications</h1>
				</div>
			</div>

			{#if hasUnread}
				<p class="text-black">
					You have {data.unreadCount.count} unread notification{data.unreadCount.count !==
					1
						? 's'
						: ''}
				</p>
			{:else}
				<p class="text-muted-foreground">All caught up!</p>
			{/if}
		</div>
		<div class="flex flex-col gap-2 sm:flex-row">
			<Button
				variant="outline"
				onclick={() => (showAll = !showAll)}
				class="flex items-center gap-2"
			>
				{#if showAll}
					<EyeOff class="h-4 w-4" />
					Show Unread Only
				{:else}
					<Eye class="h-4 w-4" />
					Show All
				{/if}
			</Button>

			{#if hasUnread}
				<Button
					onclick={markAllAsRead}
					disabled={markingAllAsRead}
					class="flex items-center gap-2"
				>
					<CheckCheck class="h-4 w-4" />
					{markingAllAsRead ? 'Marking...' : 'Mark All Read'}
				</Button>
			{/if}
		</div>
	</div>

	{#if displayNotifications.records.length === 0}
		<Card>
			<CardContent class="flex flex-col items-center justify-center py-12">
				<Bell class="text-muted-foreground/50 mb-4 h-16 w-16" />
				<h3 class="text-muted-foreground mb-2 text-xl font-semibold">
					{showAll ? 'No notifications' : 'No unread notifications'}
				</h3>
				<p class="text-muted-foreground max-w-md text-center">
					{showAll
						? "You don't have any notifications yet. When you receive notifications, they'll appear here."
						: 'All your notifications are read. Great job staying on top of things!'}
				</p>
			</CardContent>
		</Card>
	{:else}
		<div class="space-y-4">
			{#each displayNotifications.records as notification (notification.id)}
				<Card
					class=" rounded-xl transition-all hover:shadow-md {isUnread(notification)
						? 'border-l-4 '
						: ''}"
				>
					<CardHeader class="pb-3">
						<div class="flex items-start justify-between gap-4">
							<div class="flex flex-1 items-start gap-3">
								<div class="mt-0.5">
									<Info class="text-muted-foreground h-5 w-5" />
								</div>
								<div class="min-w-0 flex-1">
									<div class="mb-2 flex items-center gap-2">
										<CardTitle class="text-lg leading-tight">
											{notification.notification.title}
										</CardTitle>
										<Badge
											variant={getNotificationBadgeVariant(
												notification.notification.notificationType
											)}
										>
											{notification.notification.notificationType}
										</Badge>
										{#if isUnread(notification)}
											<Badge
												variant="live"
												>Unread</Badge
											>
										{/if}
									</div>
									<CardDescription class="flex items-center gap-2 text-sm">
										<Calendar class="h-4 w-4" />
										{formatDistanceToNow(notification.deliveredAt, {
											addSuffix: true
										})}
										{#if notification.readAt}
											• Read {formatDistanceToNow(notification.readAt, {
												addSuffix: true
											})}
										{/if}
									</CardDescription>
								</div>
							</div>

							{#if isUnread(notification)}
								<Button
									variant="outline"
									size="sm"
									onclick={() => markAsRead(notification.id)}
									disabled={markingAsRead.has(notification.id)}
									class="flex shrink-0 items-center gap-1.5"
								>
									<Check class="h-3.5 w-3.5" />
									{markingAsRead.has(notification.id)
										? 'Marking...'
										: 'Mark Read'}
								</Button>
							{/if}
						</div>
					</CardHeader>
					<CardContent>
						<div class="flex flex-row items-center gap-2">
							{#if notification.notification.contextType == 'conversation'}
								<ConversationContextImage
									conversation_id={notification.notification.contextId}
								/>
							{/if}
							<div class="prose prose-sm max-w-none">
								<p class="text-muted-foreground whitespace-pre-wrap text-sm">
									{notification.notification.content}
								</p>
							</div>
						</div>

						<Separator class="my-4" />

						<div
							class="text-muted-foreground flex items-center justify-between text-xs"
						>
							<div class="flex items-center gap-4">
								<span
									>Delivered via {notification.deliveryMethod.replace(
										'_',
										' '
									)}</span
								>
							</div>
						</div>
					</CardContent>
				</Card>
			{/each}
		</div>

		{#if displayNotifications.total > displayNotifications.records.length}
			<div class="mt-6 text-center">
				<Button variant="outline">
					Load More ({displayNotifications.total - displayNotifications.records.length} remaining)
				</Button>
			</div>
		{/if}
	{/if}
</div>

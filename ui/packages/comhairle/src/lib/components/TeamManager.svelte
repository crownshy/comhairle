<script lang="ts">
	import ResourceRoleForm from './Permissions/ResourceRoleForm.svelte';
	import type { UserWithPermissionDto } from '@crownshy/api-client/api';

	type Props = {
		conversationId: string;
		permittedUsers: UserWithPermissionDto[];
	};

	const { conversationId, permittedUsers }: Props = $props();
</script>

<div class="flex flex-col gap-4">
	<ResourceRoleForm
		resourceId={conversationId}
		resourceType="conversation"
		role="content_editor"
		grantReason="Conversation editing"
		{permittedUsers}
	/>
</div>

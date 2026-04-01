<script lang="ts">
	import { invalidateAll } from '$app/navigation';
	import { Button } from '$lib/components/ui/button';
	import { notifications } from '$lib/notifications.svelte';
	import { Import } from 'lucide-svelte';

	let fileInput: HTMLInputElement | null = $state(null);

	function triggerFileSelect() {
		fileInput?.click();
	}

	async function handleFileSelect(e: Event) {
		const target = e.target as HTMLInputElement;
		const files = target.files;

		if (!files)
			return notifications.send({
				priority: 'ERROR',
				message: 'No file selected for import '
			});

		if (files && files.length > 1)
			return notifications.send({
				priority: 'ERROR',
				message: 'Can only import from one file '
			});

		const formData = new FormData();
		formData.append('files', files[0]);

		try {
			const response = await fetch(`/api/conversation/import`, {
				method: 'POST',
				body: formData,
				credentials: 'include'
			});

			if (!response.ok) {
				throw new Error(`Upload failed: ${response.statusText}`);
			}

			notifications.send({
				message: 'Successfully created new conversation from imported JSON file',
				priority: 'INFO'
			});

			await invalidateAll();
		} catch (e) {
			console.error(e);
			notifications.send({
				priority: 'ERROR',
				message: 'Failed to upload JSON file '
			});
		}
	}
</script>

<span>
	<Button variant="outline" onclick={triggerFileSelect}><Import />Import New Conversation</Button>
	<input bind:this={fileInput} hidden type="file" accept=".json" onchange={handleFileSelect} />
</span>

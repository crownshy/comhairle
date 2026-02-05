<script lang="ts">
	import { FileText } from 'lucide-svelte';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { notifications } from '$lib/notifications.svelte';
	import { invalidateAll } from '$app/navigation';

	type Props = {
		conversation_id: string;
		accept?: string;
		maxSizeMB?: number;
	};

	let {
		conversation_id,
		accept = '.jpeg,.jpg,.png,.pdf,.mp4,.txt',
		maxSizeMB = 50
	}: Props = $props();

	let fileInput: HTMLInputElement | null = $state(null);
	let urlInput = $state('');
	let isDragging = $state(false);
	let isUploading = $state(false);

	const maxSizeBytes = maxSizeMB * 1024 * 1024;

	async function uploadFile(file: File) {
		if (file.size > maxSizeBytes) {
			notifications.send({
				message: `File size exceeds ${maxSizeMB}MB limit`,
				priority: 'ERROR'
			});
			return;
		}

		isUploading = true;
		const formData = new FormData();
		formData.append('documents', file);

		try {
			const response = await fetch(`/api/conversation/${conversation_id}/documents`, {
				method: 'POST',
				body: formData,
				credentials: 'include'
			});

			if (!response.ok) {
				throw new Error(`Upload failed: ${response.statusText}`);
			}

			notifications.send({
				message: 'File uploaded successfully',
				priority: 'INFO'
			});
		} catch (e) {
			console.error(e);
			notifications.send({
				message: 'Failed to upload file',
				priority: 'ERROR'
			});
		} finally {
			isUploading = false;
			await invalidateAll();
		}
	}

	async function uploadFromUrl() {
		if (!urlInput.trim()) {
			notifications.send({
				message: 'Please enter a valid URL',
				priority: 'ERROR'
			});
			return;
		}

		isUploading = true;
		try {
			const response = await fetch(`/api/conversation/${conversation_id}/upload_document`, {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json'
				},
				body: JSON.stringify({ url: urlInput }),
				credentials: 'include'
			});

			if (!response.ok) {
				throw new Error(`Upload failed: ${response.statusText}`);
			}

			notifications.send({
				message: 'File uploaded from URL successfully',
				priority: 'INFO'
			});
			urlInput = '';
		} catch (e) {
			console.error(e);
			notifications.send({
				message: 'Failed to upload from URL',
				priority: 'ERROR'
			});
		} finally {
			isUploading = false;
			await invalidateAll();
		}
	}

	function handleFileSelect(event: Event) {
		const target = event.target as HTMLInputElement;
		const files = target.files;
		if (files && files.length > 0) {
			for (const file of files) {
				uploadFile(file);
			}
		}
	}

	function handleDrop(event: DragEvent) {
		event.preventDefault();
		isDragging = false;

		const files = event.dataTransfer?.files;
		if (files && files.length > 0) {
			for (const file of files) {
				uploadFile(file);
			}
		}
	}

	function handleDragOver(event: DragEvent) {
		event.preventDefault();
		isDragging = true;
	}

	function handleDragLeave(event: DragEvent) {
		event.preventDefault();
		isDragging = false;
	}

	function triggerFileSelect() {
		fileInput?.click();
	}
</script>

<div class="flex w-full flex-col gap-2 border-t py-5 lg:flex-row lg:justify-between">
	<div class="flex grow flex-col gap-4">
		<div
			role="button"
			tabindex="0"
			class="flex cursor-pointer flex-col items-center gap-4 rounded-[10px] border border-gray-300 bg-gray-50 p-8 transition-colors"
			class:bg-gray-100={isDragging}
			class:border-primary={isDragging}
			ondrop={handleDrop}
			ondragover={handleDragOver}
			ondragleave={handleDragLeave}
			onkeydown={(e) => e.key === 'Enter' && triggerFileSelect()}
		>
			<div class="h-8 w-8 text-gray-400">
				<FileText class="h-full w-full" />
			</div>
			<div class="flex flex-col items-center gap-2">
				<div class="text-foreground text-center text-base font-medium">
					{isDragging ? 'Drop your files here' : 'Drag and drop your files'}
				</div>
				<div class="text-muted-foreground text-center text-sm">
					JPEG, PNG, PDF, and MP4 formats, up to {maxSizeMB}MB
				</div>
			</div>
			<Button variant="outline" onclick={triggerFileSelect} disabled={isUploading}>
				{isUploading ? 'Uploading...' : 'Select file'}
			</Button>
			<input
				bind:this={fileInput}
				type="file"
				{accept}
				multiple
				class="hidden"
				onchange={handleFileSelect}
			/>
		</div>
		<div class="flex flex-col gap-2">
			<div class="text-muted-foreground text-sm">or upload from URL</div>
			<div class="flex gap-2">
				<Input
					class="flex-1"
					type="text"
					placeholder="Add file URL"
					bind:value={urlInput}
					disabled={isUploading}
				/>
				<Button
					variant="outline"
					onclick={uploadFromUrl}
					disabled={isUploading || !urlInput.trim()}
				>
					Upload
				</Button>
			</div>
		</div>
	</div>
</div>

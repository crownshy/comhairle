<script lang="ts">
	import { FileText } from 'lucide-svelte';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { notifications } from '$lib/notifications.svelte';

	type Props = {
		conversation_id: string;
		accept?: string;
		maxSizeMB?: number;
	};

	let { conversation_id, accept = '.jpeg,.jpg,.png,.pdf,.mp4', maxSizeMB = 50 }: Props = $props();

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
			const response = await fetch(`/api/conversation/${conversation_id}/upload_knowledge_base_document`, {
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
			const response = await fetch(`/api/conversation/${conversation_id}/upload_knowledge_base_document`, {
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

<div class="flex w-full flex-col lg:flex-row lg:justify-between gap-2 border-t py-5">
	<div class="lg:w-60 lg:shrink-0 font-bold">File upload</div>
	<div class="grow flex flex-col gap-4">
		<div
			role="button"
			tabindex="0"
			class="p-8 bg-gray-50 rounded-[10px] border border-gray-300 flex flex-col items-center gap-4 cursor-pointer transition-colors"
			class:bg-gray-100={isDragging}
			class:border-primary={isDragging}
			ondrop={handleDrop}
			ondragover={handleDragOver}
			ondragleave={handleDragLeave}
			onkeydown={(e) => e.key === 'Enter' && triggerFileSelect()}
		>
			<div class="w-8 h-8 text-gray-400">
				<FileText class="w-full h-full" />
			</div>
			<div class="flex flex-col items-center gap-2">
				<div class="text-center text-foreground text-base font-medium">
					{isDragging ? 'Drop your files here' : 'Drag and drop your files'}
				</div>
				<div class="text-center text-muted-foreground text-sm">
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
			<div class="text-sm text-muted-foreground">or upload from URL</div>
			<div class="flex gap-2">
				<Input
					class="flex-1"
					type="text"
					placeholder="Add file URL"
					bind:value={urlInput}
					disabled={isUploading}
				/>
				<Button variant="outline" onclick={uploadFromUrl} disabled={isUploading || !urlInput.trim()}>
					Upload
				</Button>
			</div>
		</div>
	</div>
</div>

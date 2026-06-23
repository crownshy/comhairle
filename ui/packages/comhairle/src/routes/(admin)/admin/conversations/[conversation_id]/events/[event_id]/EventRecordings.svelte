<script lang="ts">
	import { FileAudio, Trash2, Plus, Download, RefreshCw } from 'lucide-svelte';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Badge } from '$lib/components/ui/badge';
	import { Progress } from '$lib/components/ui/progress';
	import { notifications } from '$lib/notifications.svelte';
	import { invalidateAll } from '$app/navigation';
	import { apiClient } from '@crownshy/api-client/client';
	import type {
		AudioRecordingDto,
		AudioRecordingStatus,
		SignedDownloadUrls
	} from '@crownshy/api-client/api';

	type Props = {
		conversation_id: string;
		event_id: string;
		recordings: AudioRecordingDto[];
	};

	let { conversation_id, event_id, recordings }: Props = $props();

	const maxSizeMB = 500;
	const maxSizeBytes = maxSizeMB * 1024 * 1024;
	const accept = '.wav,.mp3,.m4a,.mp4,.ogg,.webm,.flac,audio/*';
	const supportedExtensions = ['wav', 'mp3', 'm4a', 'mp4', 'ogg', 'webm', 'flac'] as const;
	type SupportedExtension = (typeof supportedExtensions)[number];

	function extractExtension(name: string): SupportedExtension | null {
		const dot = name.lastIndexOf('.');
		if (dot < 0) return null;
		const ext = name.slice(dot + 1).toLowerCase();
		const normalised = ext === 'oga' ? 'ogg' : ext;
		return (supportedExtensions as readonly string[]).includes(normalised)
			? (normalised as SupportedExtension)
			: null;
	}

	type RoomSlot = {
		key: string;
		label: string; // "Main" or breakout room ID
		isMain: boolean;
		file: File | null;
		progress: number; // 0-100
		state: 'idle' | 'uploading' | 'done' | 'error';
		error?: string;
	};

	function makeRoomSlot(isMain: boolean, label = ''): RoomSlot {
		return {
			key: crypto.randomUUID(),
			label: isMain ? 'Main room' : label,
			isMain,
			file: null,
			progress: 0,
			state: 'idle'
		};
	}

	const recording = $derived(recordings[0] ?? null);
	const hasPending = $derived(recording?.status === 'pending');

	let slots = $state<RoomSlot[]>([makeRoomSlot(true)]);
	let isUploading = $state(false);
	let isDraggingOver = $state<string | null>(null);

	let downloadUrls = $state<SignedDownloadUrls | null>(null);
	let loadingDownloads = $state(false);

	function addBreakoutRow() {
		const next = slots.filter((s) => !s.isMain).length + 1;
		slots = [...slots, makeRoomSlot(false, `room-${next}`)];
	}

	function removeRow(key: string) {
		slots = slots.filter((s) => s.key !== key);
	}

	function chooseFile(slot: RoomSlot, file: File | null) {
		if (!file) return;
		if (file.size > maxSizeBytes) {
			notifications.send({
				message: `File "${file.name}" exceeds ${maxSizeMB}MB limit`,
				priority: 'ERROR'
			});
			return;
		}
		slot.file = file;
		slot.progress = 0;
		slot.state = 'idle';
		slot.error = undefined;
	}

	function uploadToSignedUrl(
		file: File,
		url: string,
		onProgress: (pct: number) => void
	): Promise<void> {
		return new Promise((resolve, reject) => {
			const xhr = new XMLHttpRequest();
			xhr.open('PUT', url, true);
			xhr.upload.onprogress = (e) => {
				if (e.lengthComputable) {
					onProgress(Math.round((e.loaded / e.total) * 100));
				}
			};
			xhr.onload = () => {
				if (xhr.status >= 200 && xhr.status < 300) {
					onProgress(100);
					resolve();
				} else {
					reject(new Error(`Upload failed (${xhr.status})`));
				}
			};
			xhr.onerror = () => reject(new Error('Network error during upload'));
			xhr.send(file);
		});
	}

	async function startUpload() {
		const main = slots.find((s) => s.isMain);
		if (!main?.file) {
			notifications.send({
				message: 'Please select a main-room recording',
				priority: 'ERROR'
			});
			return;
		}
		const breakouts = slots.filter((s) => !s.isMain);
		const missingBreakout = breakouts.find((b) => !b.file || !b.label.trim());
		if (missingBreakout) {
			notifications.send({
				message: 'Each breakout room needs a name and a file',
				priority: 'ERROR'
			});
			return;
		}
		const ids = breakouts.map((b) => b.label.trim());
		if (new Set(ids).size !== ids.length) {
			notifications.send({
				message: 'Breakout room names must be unique',
				priority: 'ERROR'
			});
			return;
		}

		const mainExt = extractExtension(main.file.name);
		if (!mainExt) {
			notifications.send({
				message: `Unsupported audio format for "${main.file.name}". Use one of: ${supportedExtensions.join(', ')}.`,
				priority: 'ERROR'
			});
			return;
		}
		// Transcription pipeline stores one format per recording — require all
		// breakout files to match the main file's extension.
		const mismatched = breakouts.find(
			(b) => b.file && extractExtension(b.file.name) !== mainExt
		);
		if (mismatched) {
			notifications.send({
				message: `All recordings must share the same file format. "${mismatched.file?.name}" doesn't match the main room's .${mainExt} file.`,
				priority: 'ERROR'
			});
			return;
		}

		isUploading = true;
		try {
			const urls = await apiClient.RequestAudioUploadUrls(
				{ breakoutRooms: ids, fileExtension: mainExt },
				{ params: { conversation_id, event_id } }
			);

			main.state = 'uploading';
			slots = [...slots];
			await uploadToSignedUrl(main.file, urls.main, (pct) => {
				main.progress = pct;
				slots = [...slots];
			});
			main.state = 'done';
			slots = [...slots];

			for (const [roomId, signedUrl] of urls.breakoutRooms as unknown as [string, string][]) {
				const slot = breakouts.find((b) => b.label.trim() === roomId);
				if (!slot?.file) continue;
				slot.state = 'uploading';
				slots = [...slots];
				try {
					await uploadToSignedUrl(slot.file, signedUrl, (pct) => {
						slot.progress = pct;
						slots = [...slots];
					});
					slot.state = 'done';
				} catch (e) {
					slot.state = 'error';
					slot.error = e instanceof Error ? e.message : 'Upload failed';
				}
				slots = [...slots];
			}

			try {
				await apiClient.ProcessVideoCallTranscriptions(undefined, {
					params: { conversation_id, event_id }
				});
				notifications.send({
					message: 'Recordings uploaded — transcription started',
					priority: 'INFO'
				});
			} catch (e) {
				console.error(e);
				notifications.send({
					message:
						'Uploaded, but failed to start transcription. Retry from the Recordings tab.',
					priority: 'ERROR'
				});
			}
			await invalidateAll();
		} catch (e) {
			console.error(e);
			notifications.send({
				message: 'Failed to start upload',
				priority: 'ERROR'
			});
		} finally {
			isUploading = false;
		}
	}

	async function retryTranscription() {
		try {
			await apiClient.ProcessVideoCallTranscriptions(undefined, {
				params: { conversation_id, event_id }
			});
			notifications.send({ message: 'Transcription started', priority: 'INFO' });
			await invalidateAll();
		} catch (e) {
			console.error(e);
			notifications.send({ message: 'Failed to start transcription', priority: 'ERROR' });
		}
	}

	function statusVariant(status: AudioRecordingStatus): 'default' | 'secondary' | 'destructive' {
		if (status === 'completed') return 'default';
		if (status === 'failed') return 'destructive';
		return 'secondary';
	}

	async function refreshStatus() {
		await invalidateAll();
	}

	async function loadDownloadUrls() {
		if (!recording || recording.status !== 'completed') return;
		loadingDownloads = true;
		try {
			downloadUrls = await apiClient.GetAudioDownloadUrls({
				params: { conversation_id, event_id }
			});
		} catch (e) {
			console.error(e);
			notifications.send({ message: 'Failed to load download URLs', priority: 'ERROR' });
		} finally {
			loadingDownloads = false;
		}
	}

	$effect(() => {
		if (recording?.status === 'completed' && !downloadUrls && !loadingDownloads) {
			loadDownloadUrls();
		}
	});

	$effect(() => {
		if (!hasPending) return;
		const interval = setInterval(() => {
			invalidateAll();
		}, 10000);
		return () => clearInterval(interval);
	});
</script>

{#snippet dropZone(slot: RoomSlot)}
	<div
		role="button"
		tabindex="0"
		class="border-input dark:bg-input/30 flex cursor-pointer flex-col items-center gap-3 rounded-xl border bg-gray-50 p-6 transition-colors"
		class:bg-gray-100={isDraggingOver === slot.key}
		class:border-primary={isDraggingOver === slot.key}
		ondrop={(e) => {
			e.preventDefault();
			isDraggingOver = null;
			const f = e.dataTransfer?.files?.[0];
			if (f) chooseFile(slot, f);
		}}
		ondragover={(e) => {
			e.preventDefault();
			isDraggingOver = slot.key;
		}}
		ondragleave={(e) => {
			e.preventDefault();
			isDraggingOver = null;
		}}
		onkeydown={(e) => {
			if (e.key === 'Enter') {
				(
					e.currentTarget.querySelector('input[type=file]') as HTMLInputElement | null
				)?.click();
			}
		}}
	>
		<FileAudio class="h-7 w-7 text-gray-400" />
		{#if slot.file}
			<div class="text-foreground text-sm font-medium">{slot.file.name}</div>
			<div class="text-muted-foreground text-xs">
				{(slot.file.size / 1024 / 1024).toFixed(1)} MB
			</div>
		{:else}
			<div class="text-foreground text-sm">Drag an audio file here, or</div>
		{/if}
		<label class="cursor-pointer">
			<input
				type="file"
				{accept}
				class="hidden"
				disabled={isUploading}
				onchange={(e) => {
					const f = (e.target as HTMLInputElement).files?.[0] ?? null;
					chooseFile(slot, f);
				}}
			/>
			<span
				class="border-input hover:bg-accent bg-background inline-flex items-center rounded-md border px-3 py-1.5 text-sm"
			>
				{slot.file ? 'Choose a different file' : 'Choose file'}
			</span>
		</label>
		{#if slot.state === 'uploading' || slot.state === 'done'}
			<div class="w-full">
				<Progress value={slot.progress} class="h-2" />
				<div class="text-muted-foreground mt-1 text-center text-xs">
					{slot.state === 'done' ? 'Uploaded' : `${slot.progress}%`}
				</div>
			</div>
		{:else if slot.state === 'error'}
			<div class="text-destructive text-xs">{slot.error}</div>
		{/if}
	</div>
{/snippet}

<div class="flex flex-col gap-8 py-6">
	{#if recording}
		<section class="flex flex-col gap-3">
			<div class="flex items-center justify-between">
				<div class="flex items-center gap-3">
					<h2 class="text-2xl font-bold">Recordings</h2>
					<Badge variant={statusVariant(recording.status)} class="capitalize">
						{recording.status}
					</Badge>
				</div>
				<div class="flex items-center gap-2">
					{#if recording.status !== 'completed'}
						<Button variant="outline" size="sm" onclick={retryTranscription}>
							Start transcription
						</Button>
					{/if}
					<Button variant="outline" size="sm" onclick={refreshStatus}>
						<RefreshCw class="mr-2 h-4 w-4" />
						Refresh
					</Button>
				</div>
			</div>

			<div class="border-border overflow-hidden rounded-lg border">
				<table class="w-full text-sm">
					<thead class="bg-muted/50">
						<tr>
							<th class="px-4 py-2 text-left font-medium">Room</th>
							<th class="px-4 py-2 text-left font-medium">Status</th>
							<th class="px-4 py-2 text-right font-medium">Files</th>
						</tr>
					</thead>
					<tbody>
						<tr class="border-t">
							<td class="px-4 py-3 font-medium">Main room</td>
							<td class="px-4 py-3">
								<Badge variant={statusVariant(recording.status)} class="capitalize">
									{recording.status}
								</Badge>
							</td>
							<td class="px-4 py-3 text-right">
								{#if recording.status === 'completed' && downloadUrls}
									<div class="flex justify-end gap-2">
										<a
											href={downloadUrls.main.recordingUrl}
											target="_blank"
											rel="noopener"
											class="text-primary text-xs hover:underline"
										>
											<Download class="inline h-3 w-3" /> Audio
										</a>
										<a
											href={downloadUrls.main.transcriptUrl}
											target="_blank"
											rel="noopener"
											class="text-primary text-xs hover:underline"
										>
											<Download class="inline h-3 w-3" /> Transcript
										</a>
										<a
											href={downloadUrls.main.reportUrl}
											target="_blank"
											rel="noopener"
											class="text-primary text-xs hover:underline"
										>
											<Download class="inline h-3 w-3" /> Report
										</a>
									</div>
								{:else}
									<span class="text-muted-foreground text-xs">—</span>
								{/if}
							</td>
						</tr>
						{#each recording.breakoutRoomIds as roomId (roomId)}
							{@const breakoutUrls = downloadUrls?.breakoutRooms.find(
								(b) => (b as unknown as [string, unknown])[0] === roomId
							) as unknown as
								| [
										string,
										{
											recordingUrl: string;
											transcriptUrl: string;
											reportUrl: string;
										}
								  ]
								| undefined}
							<tr class="border-t">
								<td class="px-4 py-3">{roomId}</td>
								<td class="px-4 py-3">
									<Badge
										variant={statusVariant(recording.status)}
										class="capitalize"
									>
										{recording.status}
									</Badge>
								</td>
								<td class="px-4 py-3 text-right">
									{#if recording.status === 'completed' && breakoutUrls}
										<div class="flex justify-end gap-2">
											<a
												href={breakoutUrls[1].recordingUrl}
												target="_blank"
												rel="noopener"
												class="text-primary text-xs hover:underline"
											>
												<Download class="inline h-3 w-3" /> Audio
											</a>
											<a
												href={breakoutUrls[1].transcriptUrl}
												target="_blank"
												rel="noopener"
												class="text-primary text-xs hover:underline"
											>
												<Download class="inline h-3 w-3" /> Transcript
											</a>
											<a
												href={breakoutUrls[1].reportUrl}
												target="_blank"
												rel="noopener"
												class="text-primary text-xs hover:underline"
											>
												<Download class="inline h-3 w-3" /> Report
											</a>
										</div>
									{:else}
										<span class="text-muted-foreground text-xs">—</span>
									{/if}
								</td>
							</tr>
						{/each}
					</tbody>
				</table>
			</div>

			{#if hasPending}
				<p class="text-muted-foreground text-xs">
					Processing audio… status will refresh automatically every 10 seconds.
				</p>
			{/if}
		</section>
	{/if}

	<section class="flex flex-col gap-4">
		<div class="flex flex-col gap-1">
			<h2 class="text-2xl font-bold">
				{recording ? 'Upload again' : 'Upload recordings'}
			</h2>
			<p class="text-muted-foreground text-sm">
				Audio up to {maxSizeMB}MB per file. Add a row for each breakout room you want to
				upload.
			</p>
		</div>

		{#each slots as slot (slot.key)}
			<div class="border-border flex flex-col gap-3 rounded-lg border p-4">
				<div class="flex items-center justify-between gap-3">
					{#if slot.isMain}
						<div class="text-sm font-semibold">Main room</div>
					{:else}
						<div class="flex flex-1 items-center gap-2">
							<label
								class="text-sm font-semibold whitespace-nowrap"
								for="room-{slot.key}"
							>
								Breakout ID
							</label>
							<Input
								id="room-{slot.key}"
								class="max-w-xs"
								bind:value={slot.label}
								disabled={isUploading}
								placeholder="e.g. room-1"
							/>
						</div>
						<Button
							variant="ghost"
							size="sm"
							onclick={() => removeRow(slot.key)}
							disabled={isUploading}
						>
							<Trash2 class="h-4 w-4" />
						</Button>
					{/if}
				</div>
				{@render dropZone(slot)}
			</div>
		{/each}

		<div class="flex flex-wrap items-center gap-3">
			<Button variant="outline" onclick={addBreakoutRow} disabled={isUploading}>
				<Plus class="mr-2 h-4 w-4" /> Add breakout room
			</Button>
			<Button onclick={startUpload} disabled={isUploading}>
				{isUploading ? 'Uploading…' : 'Upload recordings'}
			</Button>
		</div>
	</section>
</div>

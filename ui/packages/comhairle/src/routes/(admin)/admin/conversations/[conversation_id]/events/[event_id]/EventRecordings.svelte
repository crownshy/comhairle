<script lang="ts">
	import { FileAudio, Trash2, Plus, Download, RefreshCw, Mic } from 'lucide-svelte';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Badge } from '$lib/components/ui/badge';
	import { Progress } from '$lib/components/ui/progress';
	import { notifications } from '$lib/notifications.svelte';
	import { invalidateAll } from '$app/navigation';
	import { onMount } from 'svelte';
	import { apiClient } from '@crownshy/api-client/client';
	import type {
		AudioRecordingDto,
		AudioRecordingStatus,
		RecordingDownloadUrls,
		LiveAudioRecordingDto
	} from '@crownshy/api-client/api';

	type Props = {
		conversation_id: string;
		event_id: string;
		recordings: AudioRecordingDto[];
	};

	let { conversation_id, event_id, recordings }: Props = $props();

	// TODO: Merge with interfaces/Media.ts
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

	type UploadRow = {
		key: string;
		name: string;
		file: File | null;
		progress: number;
		state: 'idle' | 'uploading' | 'done' | 'error';
		error?: string;
	};

	function makeRow(): UploadRow {
		return {
			key: crypto.randomUUID(),
			name: '',
			file: null,
			progress: 0,
			state: 'idle'
		};
	}

	let rows = $state<UploadRow[]>([makeRow()]);
	let isUploading = $state(false);
	let isDraggingOver = $state<string | null>(null);

	let downloads = $state<Record<string, RecordingDownloadUrls>>({});
	let loadingDownloads = $state<Set<string>>(new Set());
	let deletingIds = $state<Set<string>>(new Set());
	let liveRecordings = $state<LiveAudioRecordingDto[]>([]);

	const liveRecordingAudioIds = $derived(
		new Set(liveRecordings.map((recording) => recording.audioRecordingId))
	);

	const isInFlight = (s: AudioRecordingStatus) => s === 'transcribing' || s === 'categorizing';
	const hasInFlight = $derived(recordings.some((r) => isInFlight(r.status)));

	function addRow() {
		rows = [...rows, makeRow()];
	}

	function removeRow(key: string) {
		rows = rows.filter((r) => r.key !== key);
		if (rows.length === 0) rows = [makeRow()];
	}

	function chooseFile(row: UploadRow, file: File | null) {
		if (!file) return;
		if (file.size > maxSizeBytes) {
			notifications.send({
				message: `File "${file.name}" exceeds ${maxSizeMB}MB limit`,
				priority: 'ERROR'
			});
			return;
		}
		row.file = file;
		row.progress = 0;
		row.state = 'idle';
		row.error = undefined;
		if (!row.name.trim()) {
			const dot = file.name.lastIndexOf('.');
			row.name = dot > 0 ? file.name.slice(0, dot) : file.name;
		}
		rows = [...rows];
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

	async function uploadRow(row: UploadRow): Promise<boolean> {
		const name = row.name.trim();
		if (!row.file) {
			row.state = 'error';
			row.error = 'Choose a file';
			rows = [...rows];
			return false;
		}
		const ext = extractExtension(row.file.name);
		if (!ext) {
			row.state = 'error';
			row.error = `Unsupported format. Use: ${supportedExtensions.join(', ')}.`;
			rows = [...rows];
			return false;
		}
		row.state = 'uploading';
		row.progress = 0;
		row.error = undefined;
		rows = [...rows];
		try {
			const created = await apiClient.CreateAudioRecording(
				{ name, fileExtension: ext },
				{ params: { conversation_id, event_id } }
			);
			await uploadToSignedUrl(row.file, created.uploadUrl, (pct) => {
				row.progress = pct;
				rows = [...rows];
			});
			await apiClient.ProcessAudioRecording(undefined, {
				params: { conversation_id, event_id, recording_id: created.recording.id }
			});
			row.state = 'done';
			rows = [...rows];
			return true;
		} catch (e) {
			console.error(e);
			row.state = 'error';
			row.error = e instanceof Error ? e.message : 'Upload failed';
			rows = [...rows];
			return false;
		}
	}

	async function startUpload() {
		const pending = rows.filter((r) => r.state !== 'done');
		const names = pending.map((r) => r.name.trim());
		if (names.some((n) => !n)) {
			notifications.send({ message: 'Each recording needs a name', priority: 'ERROR' });
			return;
		}
		if (new Set(names).size !== names.length) {
			notifications.send({
				message: 'Recording names must be unique within this batch',
				priority: 'ERROR'
			});
			return;
		}
		const existing = new Set(recordings.map((r) => r.name));
		const clash = names.find((n) => existing.has(n));
		if (clash) {
			notifications.send({
				message: `"${clash}" is already used by an existing recording`,
				priority: 'ERROR'
			});
			return;
		}
		if (pending.some((r) => !r.file)) {
			notifications.send({ message: 'Each row needs an audio file', priority: 'ERROR' });
			return;
		}

		isUploading = true;
		let successCount = 0;
		try {
			for (const row of pending) {
				const ok = await uploadRow(row);
				if (ok) successCount += 1;
			}
		} finally {
			isUploading = false;
		}

		if (successCount > 0) {
			notifications.send({
				message: `Uploaded ${successCount} recording${successCount === 1 ? '' : 's'} — transcription started`,
				priority: 'INFO'
			});
			// Keep failed rows so the user can fix and retry; drop successful ones.
			rows = rows.filter((r) => r.state !== 'done');
			if (rows.length === 0) rows = [makeRow()];
			await invalidateAll();
		}
	}

	async function deleteRecording(recording: AudioRecordingDto) {
		const ok = window.confirm(
			`Delete recording "${recording.name}"? This removes its audio, transcript, and report. This cannot be undone.`
		);
		if (!ok) return;
		deletingIds.add(recording.id);
		deletingIds = new Set(deletingIds);
		try {
			await apiClient.DeleteAudioRecording(undefined, {
				params: { conversation_id, event_id, recording_id: recording.id }
			});
			notifications.send({ message: `Deleted "${recording.name}"`, priority: 'INFO' });
			await invalidateAll();
		} catch (e) {
			console.error(e);
			notifications.send({
				message: `Failed to delete "${recording.name}"`,
				priority: 'ERROR'
			});
		} finally {
			deletingIds.delete(recording.id);
			deletingIds = new Set(deletingIds);
		}
	}

	async function retryProcessing(recordingId: string) {
		try {
			await apiClient.ProcessAudioRecording(undefined, {
				params: { conversation_id, event_id, recording_id: recordingId }
			});
			notifications.send({ message: 'Processing restarted', priority: 'INFO' });
			await invalidateAll();
		} catch (e) {
			console.error(e);
			notifications.send({ message: 'Failed to restart processing', priority: 'ERROR' });
		}
	}

	async function refreshStatus() {
		await invalidateAll();
		await loadLiveRecordings();
	}

	async function loadLiveRecordings() {
		try {
			liveRecordings = await apiClient.ListLiveAudioRecordings({
				params: { conversation_id, event_id }
			});
		} catch (e) {
			console.error(e);
			liveRecordings = [];
		}
	}

	function hasLiveRecording(audioRecordingId: string): boolean {
		return liveRecordingAudioIds.has(audioRecordingId);
	}

	async function copyParticipantRecordingLink() {
		try {
			if (!window?.navigator?.clipboard) {
				throw new Error('Clipboard unavailable');
			}
			const link = `${window.location.origin}/conversations/${conversation_id}/events/${event_id}/record`;
			await window.navigator.clipboard.writeText(link);
			notifications.send({
				message: 'Participant recording link copied',
				priority: 'INFO'
			});
		} catch {
			notifications.send({
				message: 'Could not copy participant recording link',
				priority: 'ERROR'
			});
		}
	}

	async function loadDownloads(recordingId: string) {
		if (downloads[recordingId] || loadingDownloads.has(recordingId)) return;
		loadingDownloads.add(recordingId);
		loadingDownloads = new Set(loadingDownloads);
		try {
			const detail = await apiClient.GetAudioRecording({
				params: { conversation_id, event_id, recording_id: recordingId }
			});
			downloads = { ...downloads, [recordingId]: detail.downloads };
		} catch (e) {
			console.error(e);
		} finally {
			loadingDownloads.delete(recordingId);
			loadingDownloads = new Set(loadingDownloads);
		}
	}

	function statusVariant(status: AudioRecordingStatus): 'default' | 'secondary' | 'destructive' {
		if (status === 'complete') return 'default';
		if (status === 'transcription_failed' || status === 'categorization_failed')
			return 'destructive';
		return 'secondary';
	}

	function statusLabel(status: AudioRecordingStatus): string {
		switch (status) {
			case 'awaiting_upload':
				return 'Awaiting upload';
			case 'transcribing':
				return 'Transcribing';
			case 'categorizing':
				return 'Categorizing';
			case 'complete':
				return 'Complete';
			case 'transcription_failed':
				return 'Transcription failed';
			case 'categorization_failed':
				return 'Categorization failed';
		}
	}

	function hasTranscript(status: AudioRecordingStatus): boolean {
		return (
			status === 'categorizing' || status === 'complete' || status === 'categorization_failed'
		);
	}

	function hasReport(status: AudioRecordingStatus): boolean {
		return status === 'complete';
	}

	// Statuses where we know the audio is in storage (so download URLs are
	// worth pre-fetching). `awaiting_upload` and `transcription_failed` rows
	// may have no audio at all, so we skip them.
	function hasStoredAudio(status: AudioRecordingStatus): boolean {
		return (
			status === 'transcribing' ||
			status === 'categorizing' ||
			status === 'complete' ||
			status === 'categorization_failed'
		);
	}

	$effect(() => {
		for (const r of recordings) {
			if (hasStoredAudio(r.status) && !downloads[r.id] && !loadingDownloads.has(r.id)) {
				loadDownloads(r.id);
			}
		}
	});

	$effect(() => {
		if (!hasInFlight) return;
		const interval = setInterval(() => {
			invalidateAll();
		}, 10000);
		return () => clearInterval(interval);
	});

	onMount(() => {
		void loadLiveRecordings();
	});
</script>

{#snippet dropZone(row: UploadRow)}
	<div
		role="button"
		tabindex="0"
		class="border-input dark:bg-input/30 flex cursor-pointer flex-col items-center gap-3 rounded-xl border bg-gray-50 p-6 transition-colors"
		class:bg-gray-100={isDraggingOver === row.key}
		class:border-primary={isDraggingOver === row.key}
		ondrop={(e) => {
			e.preventDefault();
			isDraggingOver = null;
			const f = e.dataTransfer?.files?.[0];
			if (f) chooseFile(row, f);
		}}
		ondragover={(e) => {
			e.preventDefault();
			isDraggingOver = row.key;
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
		{#if row.file}
			<div class="text-foreground text-sm font-medium">{row.file.name}</div>
			<div class="text-muted-foreground text-xs">
				{(row.file.size / 1024 / 1024).toFixed(1)} MB
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
					chooseFile(row, f);
				}}
			/>
			<span
				class="border-input hover:bg-accent bg-background inline-flex items-center rounded-md border px-3 py-1.5 text-sm"
			>
				{row.file ? 'Choose a different file' : 'Choose file'}
			</span>
		</label>
		{#if row.state === 'uploading' || row.state === 'done'}
			<div class="w-full">
				<Progress value={row.progress} class="h-2" />
				<div class="text-muted-foreground mt-1 text-center text-xs">
					{row.state === 'done' ? 'Uploaded' : `${row.progress}%`}
				</div>
			</div>
		{:else if row.state === 'error'}
			<div class="text-destructive text-xs">{row.error}</div>
		{/if}
	</div>
{/snippet}

<div class="flex flex-col gap-8 py-6">
	<section class="flex flex-col gap-3">
		<div class="flex items-center justify-between">
			<h2 class="text-2xl font-bold">Recordings</h2>
			<div class="flex items-center gap-2">
				<Button variant="outline" size="sm" onclick={copyParticipantRecordingLink}>
					Copy participant recording link
				</Button>
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
						<th class="px-4 py-2 text-left font-medium">Name</th>
						<th class="px-4 py-2 text-left font-medium">Status</th>
						<th class="px-4 py-2 text-right font-medium">Files</th>
						<th class="w-12 px-4 py-2"></th>
					</tr>
				</thead>
				<tbody>
					{#if recordings.length === 0}
						<tr class="border-t">
							<td colspan="4" class="text-muted-foreground px-4 py-6 text-center">
								No recordings yet.
							</td>
						</tr>
					{:else}
						{#each recordings as recording (recording.id)}
							{@const urls = downloads[recording.id]}
							{@const isDeleting = deletingIds.has(recording.id)}
							<tr class="border-t" class:opacity-50={isDeleting}>
								<td class="px-4 py-3 font-medium">
									<div class="flex items-center gap-2">
										{#if hasLiveRecording(recording.id)}
											<Mic
												class="text-destructive h-4 w-4"
												aria-label="Live recording in progress"
											/>
										{/if}
										<span>{recording.name}</span>
									</div>
								</td>
								<td class="px-4 py-3">
									<div class="flex items-center gap-2">
										<Badge variant={statusVariant(recording.status)}>
											{statusLabel(recording.status)}
										</Badge>
										{#if recording.status === 'transcription_failed' || recording.status === 'categorization_failed'}
											<Button
												variant="outline"
												size="sm"
												disabled={isDeleting}
												onclick={() => retryProcessing(recording.id)}
											>
												Retry
											</Button>
										{/if}
									</div>
								</td>
								<td class="px-4 py-3 text-right">
									{#if urls}
										<div class="flex justify-end gap-2">
											<a
												href={urls.recordingUrl}
												target="_blank"
												rel="noopener"
												class="text-primary text-xs hover:underline"
											>
												<Download class="inline h-3 w-3" /> Audio
											</a>
											{#if hasTranscript(recording.status)}
												<a
													href={urls.transcriptUrl}
													target="_blank"
													rel="noopener"
													class="text-primary text-xs hover:underline"
												>
													<Download class="inline h-3 w-3" /> Transcript
												</a>
											{/if}
											{#if hasReport(recording.status)}
												<a
													href={urls.reportUrl}
													target="_blank"
													rel="noopener"
													class="text-primary text-xs hover:underline"
												>
													<Download class="inline h-3 w-3" /> Report
												</a>
											{/if}
										</div>
									{:else if hasStoredAudio(recording.status)}
										<span class="text-muted-foreground text-xs">Loading…</span>
									{:else}
										<span class="text-muted-foreground text-xs">—</span>
									{/if}
								</td>
								<td class="px-4 py-3 text-right">
									<Button
										variant="ghost"
										size="sm"
										title="Delete recording"
										aria-label="Delete recording {recording.name}"
										disabled={isDeleting}
										onclick={() => deleteRecording(recording)}
									>
										<Trash2 class="text-muted-foreground h-4 w-4" />
									</Button>
								</td>
							</tr>
						{/each}
					{/if}
				</tbody>
			</table>
		</div>

		{#if hasInFlight}
			<p class="text-muted-foreground text-xs">
				Processing audio… status will refresh automatically every 10 seconds.
			</p>
		{/if}
	</section>

	<section class="flex flex-col gap-4">
		<div class="flex flex-col gap-1">
			<h2 class="text-2xl font-bold">
				{recordings.length > 0 ? 'Add more recordings' : 'Upload recordings'}
			</h2>
			<p class="text-muted-foreground text-sm">
				Audio up to {maxSizeMB}MB per file. Upload one recording per room — add a row for
				each.
			</p>
		</div>

		{#each rows as row (row.key)}
			<div class="border-border flex flex-col gap-3 rounded-lg border p-4">
				<div class="flex items-center justify-between gap-3">
					<div class="flex flex-1 items-center gap-2">
						<label class="text-sm font-semibold whitespace-nowrap" for="name-{row.key}">
							Name
						</label>
						<Input
							id="name-{row.key}"
							class="max-w-xs"
							bind:value={row.name}
							disabled={isUploading}
							placeholder="e.g. Main room, Breakout 1"
						/>
					</div>
					<Button
						variant="ghost"
						size="sm"
						onclick={() => removeRow(row.key)}
						disabled={isUploading || rows.length === 1}
					>
						<Trash2 class="h-4 w-4" />
					</Button>
				</div>
				{@render dropZone(row)}
			</div>
		{/each}

		<div class="flex flex-wrap items-center gap-3">
			<Button variant="outline" onclick={addRow} disabled={isUploading}>
				<Plus class="mr-2 h-4 w-4" /> Add another recording
			</Button>
			<Button onclick={startUpload} disabled={isUploading}>
				{isUploading ? 'Uploading…' : 'Upload'}
			</Button>
		</div>
	</section>
</div>

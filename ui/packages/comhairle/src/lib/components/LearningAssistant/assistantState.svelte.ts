/**
 * Session plumbing for the Learning assistant: history, the Q/A pairing the panel reads,
 * and the source-document viewer's props. Split out of the component so the panel is only
 * layout, and so a second surface (the support drawer) shares the same reactive session
 * rather than duplicating it.
 */

import { getChatSession, type ChatMessage } from '$lib/api/chatSession.svelte';
import type { ChatReference, ReferenceChunk } from '$lib/api/chatClient.svelte';
import { getPreviewKind } from '$lib/utils/previewKind';
import { highlightsFromPositions } from '$lib/components/PdfViewer/highlights';

export type QA = {
	id: string;
	question: string;
	answer: string;
	reference: ChatReference | null;
	streaming: boolean;
	error: string | null;
	timestamp: Date | null;
};

const INTRO_KEY = 'comhairle-assistant-intro';

export function hasSeenIntro(conversationId: string): boolean {
	if (typeof window === 'undefined') return false;
	try {
		return localStorage.getItem(`${INTRO_KEY}-${conversationId}`) === 'seen';
	} catch {
		return false;
	}
}

export function markIntroSeen(conversationId: string): void {
	if (typeof window === 'undefined') return;
	try {
		localStorage.setItem(`${INTRO_KEY}-${conversationId}`, 'seen');
	} catch {
		/* ignore */
	}
}

/** Collapse a reference's chunks down to one entry per document. */
export function uniqueDocs(ref: ChatReference | null): ReferenceChunk[] {
	if (!ref?.chunks?.length) return [];
	const seen = new Set<string>();
	const out: ReferenceChunk[] = [];
	for (const chunk of ref.chunks) {
		const key = chunk.document_id || chunk.document_name;
		if (seen.has(key)) continue;
		seen.add(key);
		out.push(chunk);
	}
	return out;
}

export function formatTimestamp(date: Date | null): string {
	if (!date) return '';
	const sec = Math.round((Date.now() - date.getTime()) / 1000);
	if (sec < 10) return 'just now';
	if (sec < 60) return `${sec}s ago`;
	const min = Math.round(sec / 60);
	if (min < 60) return `${min}m ago`;
	const hr = Math.round(min / 60);
	if (hr < 24) return `${hr}h ago`;
	const day = Math.round(hr / 24);
	if (day < 7) return `${day}d ago`;
	return date.toLocaleDateString();
}

type Options = {
	getConversationId: () => string;
	getPageTitle: () => string;
	/** True while the surrounding page is still resolving. */
	getLoading: () => boolean;
};

export function createAssistantState(options: Options) {
	const { getConversationId, getPageTitle, getLoading } = options;

	const session = $derived(getConversationId() ? getChatSession(getConversationId()) : null);
	const initializing = $derived(session?.initializing ?? false);
	const chatError = $derived(session?.error ?? null);
	const isStreaming = $derived(session?.isStreaming ?? false);
	/** Keyed off `initialized`, not `initializing`, so the skeleton is up before the
	 *  init effect has fired and the empty input never flashes. */
	const loadingHistory = $derived(!!session && !session.initialized && !chatError);

	let initStartedFor = $state<string | null>(null);
	let activeChunk = $state<ReferenceChunk | null>(null);
	let viewerOpen = $state(false);

	$effect(() => {
		if (!session) return;
		if (initStartedFor === session.conversationId) return;
		initStartedFor = session.conversationId;
		void session.init();
	});

	/** Oldest first. Each pair is a user message plus the assistant reply that follows it. */
	const asked = $derived.by<QA[]>(() => {
		const msgs: ChatMessage[] = session?.messages ?? [];
		const out: QA[] = [];
		for (let i = 0; i < msgs.length; i++) {
			const msg = msgs[i];
			if (msg.role !== 'user') continue;
			const next = msgs[i + 1];
			const answered = next && next.role === 'assistant';
			out.push({
				id: msg.id,
				question: msg.content,
				answer: answered ? next.content : '',
				reference: answered ? next.reference : null,
				streaming: answered ? !!next.streaming : isStreaming,
				error: answered ? (next.error ?? null) : null,
				timestamp: msg.timestamp ?? null
			});
			if (answered) i++;
		}
		return out;
	});

	const newestFirst = $derived([...asked].reverse());
	const inputDisabled = $derived(
		getLoading() || isStreaming || initializing || (!!chatError && asked.length === 0)
	);

	const activeSource = $derived.by(() => {
		const chunk = activeChunk;
		if (!chunk) return null;
		// Source chunks are only ever PDFs or uploaded docs, so an unrecognised name still
		// opens in the PDF viewer rather than falling back to a download.
		const kind = getPreviewKind(chunk.document_name) ?? 'pdf';
		const href = `/api/conversation/${getConversationId()}/documents/${chunk.document_id}/download`;
		const highlights = kind === 'pdf' ? highlightsFromPositions(chunk.positions) : [];
		return {
			kind,
			src: href,
			downloadHref: href,
			name: chunk.document_name,
			highlights,
			page: highlights[0]?.page ?? null
		};
	});

	return {
		get asked() {
			return asked;
		},
		get newestFirst() {
			return newestFirst;
		},
		get initializing() {
			return initializing;
		},
		get loadingHistory() {
			return loadingHistory;
		},
		get isStreaming() {
			return isStreaming;
		},
		get chatError() {
			return chatError;
		},
		get inputDisabled() {
			return inputDisabled;
		},
		get fatalError() {
			return chatError && asked.length === 0 && !initializing ? chatError : null;
		},
		get activeSource() {
			return activeSource;
		},
		get viewerOpen() {
			return viewerOpen;
		},
		set viewerOpen(next: boolean) {
			viewerOpen = next;
		},
		openSource(chunk: ReferenceChunk) {
			activeChunk = chunk;
			viewerOpen = true;
		},
		async ask(question: string) {
			const trimmed = question.trim();
			if (!trimmed || !session || inputDisabled) return;
			const title = getPageTitle();
			await session.send(trimmed, title ? `[Reading "${title}"] ${trimmed}` : trimmed);
		},
		async retryLast() {
			if (!session || isStreaming) return;
			await session.retryLast();
		},
		async retryInit() {
			await session?.retryInit();
		}
	};
}

export type AssistantState = ReturnType<typeof createAssistantState>;

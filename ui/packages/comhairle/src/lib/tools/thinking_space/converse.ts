/**
 * Calls the Thinking Space converse endpoint (`POST /tools/thinking_space`)
 * which streams a RAGFlow agent response. The agent is a one-shot task agent:
 * given the topic, starting question, question intent and the Q/A history so
 * far, it emits a JSON object `{ questions: [{ type, question }] }`.
 *
 * The generated api-client cannot consume streaming responses, so this uses a
 * raw fetch and reads the stream incrementally.
 */

export interface FollowUpQuestion {
	type: string;
	question: string;
}

export interface ConverseParams {
	workflowStepId: string;
	startingQuestion: string;
	questionIntent: string;
	history: string;
}

export async function fetchFollowUps(params: ConverseParams): Promise<FollowUpQuestion[]> {
	const res = await fetch('/api/tools/thinking_space', {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		credentials: 'include',
		body: JSON.stringify({
			workflow_step_id: params.workflowStepId,
			starting_question: params.startingQuestion,
			question_intent: params.questionIntent,
			history: params.history
		})
	});

	if (!res.ok || !res.body) {
		throw new Error(`thinking_space converse failed: ${res.status}`);
	}

	const reader = res.body.getReader();
	const decoder = new TextDecoder();
	let buffer = '';
	let latestContent = '';

	for (;;) {
		const { done, value } = await reader.read();
		if (done) break;
		buffer += decoder.decode(value, { stream: true });
		const lines = buffer.split('\n');
		// Keep the last (possibly incomplete) line in the buffer.
		buffer = lines.pop() ?? '';
		for (const line of lines) {
			const content = parseDataLine(line);
			if (content !== null) latestContent = content;
		}
	}
	const tail = parseDataLine(buffer);
	if (tail !== null) latestContent = tail;

	return parseQuestionArray(latestContent);
}

/**
 * RAGFlow streams SSE-style `data:{...}` lines, one JSON event per line, ending
 * with `data:[DONE]`. The agent's answer arrives in a `message` event as
 * `data.content` (and is echoed in the Message node's `data.outputs.content`).
 * Returns that content string if this line carries one.
 */
function parseDataLine(line: string): string | null {
	const trimmed = line.trim();
	if (!trimmed.startsWith('data:')) return null;
	const payload = trimmed.slice(5).trim();
	if (!payload || payload === '[DONE]') return null;

	let json: unknown;
	try {
		json = JSON.parse(payload);
	} catch {
		// Partial or non-JSON line — ignore, the next chunk completes it.
		return null;
	}

	const j = json as {
		event?: string;
		data?: { content?: unknown; answer?: unknown; outputs?: { content?: unknown } };
	};
	if (j?.event === 'message' && typeof j.data?.content === 'string') {
		return j.data.content;
	}
	if (typeof j?.data?.outputs?.content === 'string') {
		return j.data.outputs.content;
	}
	// Older streaming format kept as a fallback.
	if (typeof j?.data?.answer === 'string') {
		return j.data.answer;
	}
	return null;
}

/**
 * The agent's answer is JSON — either `{ questions: [...] }` or a bare array —
 * sometimes wrapped in a markdown ```json fence. Extract and parse it
 * defensively into a flat list of follow-up questions.
 */
function parseQuestionArray(content: string): FollowUpQuestion[] {
	if (!content) return [];
	let text = content.trim();

	const fence = text.match(/```(?:json)?\s*([\s\S]*?)```/i);
	if (fence) text = fence[1].trim();

	let parsed: unknown;
	try {
		parsed = JSON.parse(text);
	} catch {
		// Last resort: slice out the first [...] array and parse that.
		const start = text.indexOf('[');
		const end = text.lastIndexOf(']');
		if (start === -1 || end <= start) return [];
		try {
			parsed = JSON.parse(text.slice(start, end + 1));
		} catch (e) {
			console.error('thinking_space: could not parse follow-up questions', e, content);
			return [];
		}
	}

	const arr = Array.isArray(parsed)
		? parsed
		: Array.isArray((parsed as { questions?: unknown })?.questions)
			? (parsed as { questions: unknown[] }).questions
			: [];

	return (arr as Array<{ question?: unknown; type?: unknown }>)
		.filter((x) => x && typeof x.question === 'string')
		.map((x) => ({ type: String(x.type ?? ''), question: String(x.question) }));
}

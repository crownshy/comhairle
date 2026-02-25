const BLOCK_DELIMITER = '\n\n---\n\n';

export function extractTextFromTiptap(content: string): string {
	if (!content || !content.trim()) return '';
	
	const trimmed = content.trim();
	
	if (!trimmed.startsWith('{')) {
		return trimmed;
	}
	
	try {
		const parsed = JSON.parse(trimmed);
		if (parsed?.type === 'doc' && parsed?.content) {
			return extractBlocksFromNodes(parsed.content).join(BLOCK_DELIMITER);
		}
	} catch {
	}
	
	return trimmed;
}

function extractBlocksFromNodes(nodes: any[]): string[] {
	if (!Array.isArray(nodes)) return [];
	
	const blocks: string[] = [];
	
	for (const node of nodes) {
		if (node.type === 'paragraph' || node.type === 'heading') {
			const text = extractTextFromNode(node);
			if (text.trim()) {
				blocks.push(text);
			}
		} else if (node.content) {
			blocks.push(...extractBlocksFromNodes(node.content));
		}
	}
	
	return blocks;
}

function extractTextFromNode(node: any): string {
	if (!node) return '';
	
	if (node.type === 'text') {
		return node.text || '';
	}
	
	if (node.content && Array.isArray(node.content)) {
		return node.content.map((n: any) => extractTextFromNode(n)).join('');
	}
	
	return '';
}

export function wrapTextInTiptap(text: string): string {
	if (!text || !text.trim()) {
		return JSON.stringify({
			type: 'doc',
			content: [{ type: 'paragraph' }]
		});
	}
	
	const paragraphs = text.split(/\n+/).filter(p => p.trim());
	
	const content = paragraphs.map(p => ({
		type: 'paragraph',
		attrs: { textAlign: null },
		content: [{ type: 'text', text: p.trim() }]
	}));
	
	return JSON.stringify({
		type: 'doc',
		content: content.length > 0 ? content : [{ type: 'paragraph' }]
	});
}


export function isTiptapJson(content: string): boolean {
	if (!content || !content.trim().startsWith('{')) return false;
	
	try {
		const parsed = JSON.parse(content);
		return parsed?.type === 'doc';
	} catch {
		return false;
	}
}

export function translateTiptapContent(sourceContent: string, translatedPlainText: string): string {
	if (!sourceContent || !sourceContent.trim().startsWith('{')) {
		return wrapTextInTiptap(translatedPlainText);
	}
	
	try {
		const parsed = JSON.parse(sourceContent);
		if (parsed?.type !== 'doc' || !parsed?.content) {
			return wrapTextInTiptap(translatedPlainText);
		}
		
		const cloned = JSON.parse(JSON.stringify(parsed));
		
		const translatedBlocks = translatedPlainText
			.split(/\n\n---\n\n|\n---\n|---/)
			.map(b => b.trim())
			.filter(b => b);
		
		if (translatedBlocks.length === 0) {
			return sourceContent;
		}
		
		replaceTextInBlocks(cloned.content, translatedBlocks, { index: 0 });
		
		return JSON.stringify(cloned);
	} catch {
		return wrapTextInTiptap(translatedPlainText);
	}
}

function replaceTextInBlocks(nodes: any[], translatedBlocks: string[], counter: { index: number }): void {
	if (!Array.isArray(nodes)) return;
	
	for (const node of nodes) {
		if ((node.type === 'paragraph' || node.type === 'heading') && node.content) {
			const textNodes = node.content.filter((n: any) => n.type === 'text' && n.text);
			const hasText = textNodes.some((n: any) => n.text?.trim());
			
			if (hasText && counter.index < translatedBlocks.length) {
				const translatedText = translatedBlocks[counter.index];
				if (translatedText) {
					node.content = distributeTextWithMarks(node.content, translatedText);
				}
				counter.index++;
			}
		} else if (node.content) {
			replaceTextInBlocks(node.content, translatedBlocks, counter);
		}
	}
}

/**
 * Replaces the text in a paragraph's content nodes with translated text while preserving formatting marks.
 * If all original text nodes share identical marks (e.g. all bold), the translated text inherits those marks.
 * If marks differ across nodes (e.g. mixed bold/italic), marks are stripped since we can't know how
 * the translated text maps to the original formatting boundaries.
 * Non-text nodes (e.g. hard breaks, images) are preserved and appended after the text.
 */
function distributeTextWithMarks(originalContent: any[], translatedText: string): any[] {
	const textNodes = originalContent.filter((n: any) => n.type === 'text' && n.text);
	const nonTextNodes = originalContent.filter((n: any) => n.type !== 'text' || !n.text);
	
	if (textNodes.length === 0) {
		return originalContent;
	}
	
	const marksKey = (marks: any[] | undefined) => JSON.stringify(marks ?? []);
	const uniqueMarkSets = new Set(textNodes.map((n: any) => marksKey(n.marks)));
	
	const result: any[] = [];
	
	if (uniqueMarkSets.size === 1) {
		const sharedMarks = textNodes[0].marks;
		const newNode: any = { type: 'text', text: translatedText };
		if (sharedMarks && sharedMarks.length > 0) {
			newNode.marks = sharedMarks;
		}
		result.push(newNode);
	} else {
		result.push({ type: 'text', text: translatedText });
	}
	
	result.push(...nonTextNodes);
	
	return result;
}

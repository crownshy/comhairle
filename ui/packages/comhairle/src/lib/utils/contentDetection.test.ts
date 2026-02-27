import { describe, it, expect } from 'vitest';
import { detectContentType } from './contentDetection';
import { CONTENT_TYPES } from '$lib/components/RichTextEditor/types';

describe('detectContentType', () => {
	describe('empty or invalid content', () => {
		it('should return empty markdown for undefined', () => {
			const result = detectContentType(undefined);
			expect(result).toEqual({
				content: '',
				type: CONTENT_TYPES.MARKDOWN
			});
		});

		it('should return empty markdown for empty string', () => {
			const result = detectContentType('');
			expect(result).toEqual({
				content: '',
				type: CONTENT_TYPES.MARKDOWN
			});
		});

		it('should return empty markdown for whitespace only', () => {
			expect(detectContentType('   ')).toEqual({
				content: '',
				type: CONTENT_TYPES.MARKDOWN
			});
			expect(detectContentType('\n\t  \n')).toEqual({
				content: '',
				type: CONTENT_TYPES.MARKDOWN
			});
		});
	});

	describe('valid ProseMirror JSON', () => {
		it('should detect valid ProseMirror doc', () => {
			const jsonContent = JSON.stringify({
				type: 'doc',
				content: [
					{
						type: 'paragraph',
						content: [{ type: 'text', text: 'Hello' }]
					}
				]
			});

			const result = detectContentType(jsonContent);
			expect(result.type).toBe(CONTENT_TYPES.JSON);
			expect(result.content).toEqual(JSON.parse(jsonContent));
		});

		it('should detect minimal valid doc', () => {
			const jsonContent = JSON.stringify({ type: 'doc', content: [] });

			const result = detectContentType(jsonContent);
			expect(result.type).toBe(CONTENT_TYPES.JSON);
			expect(result.content).toEqual({ type: 'doc', content: [] });
		});

		it('should handle doc with complex nested content', () => {
			const jsonContent = JSON.stringify({
				type: 'doc',
				content: [
					{
						type: 'heading',
						attrs: { level: 1 },
						content: [{ type: 'text', text: 'Title' }]
					},
					{
						type: 'paragraph',
						content: [
							{ type: 'text', text: 'Some ' },
							{ type: 'text', marks: [{ type: 'bold' }], text: 'bold' },
							{ type: 'text', text: ' text' }
						]
					}
				]
			});

			const result = detectContentType(jsonContent);
			expect(result.type).toBe(CONTENT_TYPES.JSON);
			expect(result.content.type).toBe('doc');
			expect(result.content.content).toHaveLength(2);
		});

		it('should trim whitespace from JSON strings', () => {
			const jsonContent = '  \n  {"type":"doc","content":[]}  \n  ';

			const result = detectContentType(jsonContent);
			expect(result.type).toBe(CONTENT_TYPES.JSON);
			expect(result.content).toEqual({ type: 'doc', content: [] });
		});
	});

	describe('invalid JSON that looks like JSON', () => {
		it('should fall back to markdown for malformed JSON', () => {
			const malformedJson = '{"type":"doc",invalid}';

			const result = detectContentType(malformedJson);
			expect(result.type).toBe(CONTENT_TYPES.MARKDOWN);
			expect(result.content).toBe(malformedJson);
		});

		it('should fall back to markdown for JSON without type: doc', () => {
			const jsonContent = JSON.stringify({ type: 'paragraph', text: 'hello' });

			const result = detectContentType(jsonContent);
			expect(result.type).toBe(CONTENT_TYPES.MARKDOWN);
			expect(result.content).toBe(jsonContent);
		});

		it('should fall back to markdown for JSON array', () => {
			const jsonContent = JSON.stringify([1, 2, 3]);

			const result = detectContentType(jsonContent);
			expect(result.type).toBe(CONTENT_TYPES.MARKDOWN);
			expect(result.content).toBe(jsonContent);
		});

		it('should fall back to markdown for non-object JSON', () => {
			const jsonContent = JSON.stringify('just a string');

			const result = detectContentType(jsonContent);
			expect(result.type).toBe(CONTENT_TYPES.MARKDOWN);
			expect(result.content).toBe(jsonContent);
		});

		it('should fall back to markdown for null JSON', () => {
			const jsonContent = 'null';

			const result = detectContentType(jsonContent);
			expect(result.type).toBe(CONTENT_TYPES.MARKDOWN);
			expect(result.content).toBe('null');
		});

		it('should fall back to markdown for object without type field', () => {
			const jsonContent = JSON.stringify({ content: [], data: 'test' });

			const result = detectContentType(jsonContent);
			expect(result.type).toBe(CONTENT_TYPES.MARKDOWN);
			expect(result.content).toBe(jsonContent);
		});
	});

	describe('markdown content', () => {
		it('should detect simple markdown text', () => {
			const markdown = 'Hello World';

			const result = detectContentType(markdown);
			expect(result.type).toBe(CONTENT_TYPES.MARKDOWN);
			expect(result.content).toBe('Hello World');
		});

		it('should detect markdown with headers', () => {
			const markdown = '# Hello World\n\nThis is a paragraph.';

			const result = detectContentType(markdown);
			expect(result.type).toBe(CONTENT_TYPES.MARKDOWN);
			expect(result.content).toBe(markdown);
		});

		it('should detect markdown with formatting', () => {
			const markdown = '**Bold** and *italic* text';

			const result = detectContentType(markdown);
			expect(result.type).toBe(CONTENT_TYPES.MARKDOWN);
			expect(result.content).toBe(markdown);
		});

		it('should detect markdown with lists', () => {
			const markdown = '- Item 1\n- Item 2\n- Item 3';

			const result = detectContentType(markdown);
			expect(result.type).toBe(CONTENT_TYPES.MARKDOWN);
			expect(result.content).toBe(markdown);
		});

		it('should detect markdown with code blocks', () => {
			const markdown = '```javascript\nconst x = 1;\n```';

			const result = detectContentType(markdown);
			expect(result.type).toBe(CONTENT_TYPES.MARKDOWN);
			expect(result.content).toBe(markdown);
		});

		it('should trim whitespace from markdown', () => {
			const markdown = '  \n  # Hello World  \n  ';

			const result = detectContentType(markdown);
			expect(result.type).toBe(CONTENT_TYPES.MARKDOWN);
			expect(result.content).toBe('# Hello World');
		});
	});

	describe('edge cases', () => {
		it('should handle content starting with [ but not valid JSON array', () => {
			const content = '[not valid json';

			const result = detectContentType(content);
			expect(result.type).toBe(CONTENT_TYPES.MARKDOWN);
			expect(result.content).toBe(content);
		});

		it('should handle content starting with { but not valid JSON object', () => {
			const content = '{not valid json';

			const result = detectContentType(content);
			expect(result.type).toBe(CONTENT_TYPES.MARKDOWN);
			expect(result.content).toBe(content);
		});

		it('should handle valid JSON array with doc-like objects', () => {
			const jsonContent = JSON.stringify([{ type: 'doc', content: [] }]);

			const result = detectContentType(jsonContent);
			expect(result.type).toBe(CONTENT_TYPES.MARKDOWN);
			expect(result.content).toBe(jsonContent);
		});

		it('should handle markdown that looks like it could be JSON', () => {
			const markdown = 'My document {has} some [brackets] in it';

			const result = detectContentType(markdown);
			expect(result.type).toBe(CONTENT_TYPES.MARKDOWN);
			expect(result.content).toBe(markdown);
		});

		it('should preserve exact content for markdown', () => {
			const markdown = 'Content with\nmultiple\nlines\nand special chars: !@#$%^&*()';

			const result = detectContentType(markdown);
			expect(result.type).toBe(CONTENT_TYPES.MARKDOWN);
			expect(result.content).toBe(markdown);
		});
	});

	describe('real-world examples', () => {
		it('should handle typical TipTap JSON output', () => {
			const tiptapJson = JSON.stringify({
				type: 'doc',
				content: [
					{
						type: 'heading',
						attrs: { level: 1 },
						content: [{ type: 'text', text: 'Welcome' }]
					},
					{
						type: 'paragraph',
						content: [{ type: 'text', text: 'This is my rich text content.' }]
					},
					{
						type: 'bulletList',
						content: [
							{
								type: 'listItem',
								content: [
									{
										type: 'paragraph',
										content: [{ type: 'text', text: 'First item' }]
									}
								]
							}
						]
					}
				]
			});

			const result = detectContentType(tiptapJson);
			expect(result.type).toBe(CONTENT_TYPES.JSON);
			expect(result.content.type).toBe('doc');
			expect(result.content.content).toHaveLength(3);
		});

		it('should handle typical markdown blog post', () => {
			const markdown = `# My Blog Post

This is the introduction paragraph with **bold** and *italic* text.

## Section 1

Some content here with a [link](https://example.com).

- Point one
- Point two
- Point three

\`\`\`javascript
const code = 'example';
\`\`\``;

			const result = detectContentType(markdown);
			expect(result.type).toBe(CONTENT_TYPES.MARKDOWN);
			expect(result.content).toBe(markdown);
		});
	});
});

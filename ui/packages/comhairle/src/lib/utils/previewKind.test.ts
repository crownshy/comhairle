import { describe, it, expect } from 'vitest';
import { getPreviewKind } from './previewKind';

describe('getPreviewKind', () => {
	it('maps PDFs to the pdf viewer', () => {
		expect(getPreviewKind('report.pdf')).toBe('pdf');
	});

	it('maps image extensions to the image viewer', () => {
		for (const name of [
			'a.jpg',
			'a.jpeg',
			'a.png',
			'a.gif',
			'a.webp',
			'a.bmp',
			'a.svg',
			'a.avif'
		]) {
			expect(getPreviewKind(name)).toBe('image');
		}
	});

	it('maps Word documents to the docx viewer', () => {
		expect(getPreviewKind('memo.doc')).toBe('docx');
		expect(getPreviewKind('memo.docx')).toBe('docx');
	});

	it('maps plain-text and markdown to the text viewer', () => {
		expect(getPreviewKind('notes.md')).toBe('text');
		expect(getPreviewKind('notes.markdown')).toBe('text');
		expect(getPreviewKind('notes.txt')).toBe('text');
	});

	it('is case-insensitive on the extension', () => {
		expect(getPreviewKind('SCAN.PDF')).toBe('pdf');
		expect(getPreviewKind('Photo.PNG')).toBe('image');
	});

	it('returns null for types we have no viewer for', () => {
		expect(getPreviewKind('data.csv')).toBeNull();
		expect(getPreviewKind('archive.zip')).toBeNull();
		expect(getPreviewKind('noextension')).toBeNull();
	});

	it('defaults an unknown type to pdf when the caller opts in with ?? "pdf"', () => {
		expect(getPreviewKind('mystery.bin') ?? 'pdf').toBe('pdf');
	});
});

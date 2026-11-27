#!/usr/bin/env node
// Converts the Comhairle Handbook pages (src/routes/(admin)/admin/info) to markdown and
// syncs them into a RAGFlow dataset, replacing any existing document of the same name.
// Crawls a running app from the welcome page, e.g. `pnpm build && pnpm preview`.
//
// Usage:
//   RAGFLOW_API_KEY=... RAGFLOW_DATASET_ID=... node scripts/sync-info-docs-to-ragflow.mjs
//
// Env:
//   RAGFLOW_API_KEY    (required) RAGFlow API key, sent as `Authorization: Bearer <key>`.
//   RAGFLOW_DATASET_ID (required) Target dataset id.
//   RAGFLOW_BASE_URL   (optional) Defaults to https://rag.comhairle.scot.
//   HANDBOOK_APP_URL   (optional) App to crawl. Defaults to http://localhost:4173 (vite preview).

import { JSDOM } from 'jsdom';
import TurndownService from 'turndown';

const HANDBOOK_PATH = '/admin/info';
const APP_URL = process.env.HANDBOOK_APP_URL ?? 'http://localhost:4173';
const BASE_URL = process.env.RAGFLOW_BASE_URL ?? 'https://rag.comhairle.scot';
const API_KEY = process.env.RAGFLOW_API_KEY;
const DATASET_ID = process.env.RAGFLOW_DATASET_ID;

if (!API_KEY) throw new Error('RAGFLOW_API_KEY is required');
if (!DATASET_ID) throw new Error('RAGFLOW_DATASET_ID is required');

const turndown = new TurndownService({ headingStyle: 'atx', bulletListMarker: '-' });

function handbookPathOf(href, pageUrl) {
	const url = new URL(href, pageUrl);
	if (url.origin !== new URL(APP_URL).origin) return null;
	return url.pathname.startsWith(`${HANDBOOK_PATH}/`) ? url.pathname : null;
}

// Fetches every Handbook page reachable from the welcome page. Keyed by the path after
// redirects so a guide's index and its first topic aren't synced twice.
async function crawlHandbook() {
	const pages = new Map();
	const queue = [`${HANDBOOK_PATH}/welcome`];
	const seen = new Set(queue);

	while (queue.length > 0) {
		const res = await fetch(new URL(queue.shift(), APP_URL));
		if (!res.ok) throw new Error(`Handbook fetch failed: ${res.url} -> ${res.status}`);
		const finalPath = new URL(res.url).pathname;
		if (pages.has(finalPath)) continue;

		const html = await res.text();
		pages.set(finalPath, html);

		const { document } = new JSDOM(html).window;
		for (const link of document.querySelectorAll('a[href]')) {
			const path = handbookPathOf(link.getAttribute('href'), res.url);
			if (path && !seen.has(path)) {
				seen.add(path);
				queue.push(path);
			}
		}
	}
	return pages;
}

// Stable, readable document name derived from the route, e.g.
// /admin/info/how-to/getting-started/sign-up -> admin-info-how-to-getting-started-sign-up.md
function docNameFor(pagePath) {
	return `admin-info-${pagePath
		.slice(HANDBOOK_PATH.length + 1)
		.split('/')
		.join('-')}.md`;
}

function extractMarkdown(html) {
	const { window } = new JSDOM(html);
	const main = window.document.querySelector('main');
	if (!main) return null;

	// In-page navs (the glossary's A-Z jump bar, the "Next topic" link) aren't content,
	// and aria-hidden elements are decorative (icons, redundant step-number badges).
	// .sr-only labels (e.g. "Best for" dt) duplicate the visible text next to them.
	main.querySelectorAll('nav').forEach((nav) => nav.remove());
	main.querySelectorAll('[aria-hidden="true"], .sr-only').forEach((el) => el.remove());

	const markdown = turndown.turndown(main.innerHTML).trim();
	return markdown.length > 0 ? markdown : null;
}

async function ragflowFetch(pathname, options = {}) {
	const res = await fetch(`${BASE_URL}${pathname}`, {
		...options,
		headers: {
			Authorization: `Bearer ${API_KEY}`,
			...options.headers
		}
	});
	const body = await res.json().catch(() => null);
	if (!res.ok || body?.code !== 0) {
		throw new Error(
			`RAGFlow request failed: ${options.method ?? 'GET'} ${pathname} -> ${res.status} ${JSON.stringify(body)}`
		);
	}
	return body;
}

// The `name` query filter appears to look up documents globally rather than scoped to
// this dataset, and errors with an ownership check on collision instead of returning
// empty. List this dataset's documents unfiltered and match the name client-side.
async function findExistingDocumentId(name) {
	const body = await ragflowFetch(`/api/v1/datasets/${DATASET_ID}/documents?page_size=1000`);
	const match = body.data?.docs?.find((doc) => doc.name === name);
	return match?.id ?? null;
}

async function deleteDocument(id) {
	await ragflowFetch(`/api/v1/datasets/${DATASET_ID}/documents`, {
		method: 'DELETE',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify({ ids: [id] })
	});
}

async function uploadDocument(name, markdown) {
	const form = new FormData();
	form.append('file', new Blob([markdown], { type: 'text/markdown' }), name);
	const body = await ragflowFetch(`/api/v1/datasets/${DATASET_ID}/documents`, {
		method: 'POST',
		body: form
	});
	return body.data[0].id;
}

async function parseDocument(id) {
	await ragflowFetch(`/api/v1/datasets/${DATASET_ID}/chunks`, {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify({ document_ids: [id] })
	});
}

async function syncOne(pagePath, html) {
	const name = docNameFor(pagePath);
	const markdown = extractMarkdown(html);
	if (!markdown) {
		console.log(`skip (no content): ${name}`);
		return;
	}

	const existingId = await findExistingDocumentId(name);
	if (existingId) await deleteDocument(existingId);

	const id = await uploadDocument(name, markdown);
	await parseDocument(id);
	console.log(`${existingId ? 'replaced' : 'created'}: ${name}`);
}

async function main() {
	const pages = await crawlHandbook();
	for (const [pagePath, html] of pages) {
		await syncOne(pagePath, html);
	}
}

main().catch((err) => {
	console.error(err);
	process.exitCode = 1;
});

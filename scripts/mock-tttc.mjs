#!/usr/bin/env node
// Minimal mock of the Talk-to-the-City (TTTC) categorization service for local
// Comhairle development. Implements the contract Comhairle's TttcCategorizer
// expects (api/src/categorization_service/tttc_categorizer.rs):
//
//   POST /api/v1/jobs   ->  200 { jobId, status, createdAt }
//
// then, after a short delay, POSTs the finished report back to the webhookUrl
// supplied in the job body with a valid HMAC-SHA256 signature that
// submit_report (api/src/routes/events.rs) / verify_webhook_signature
// (api/src/routes/auth.rs) will accept:
//
//   signature = "sha256=" + hex(HMAC_SHA256(webhookSecret, `${timestamp}.${body}`))
//   headers:   X-Webhook-Timestamp (unix seconds), X-Webhook-Signature
//
// Zero dependencies — Node built-ins only. Run: node scripts/mock-tttc.mjs
//
// The report callback always targets the API directly (API_BASE), NOT the
// DOMAIN-derived URL Comhairle sends. That URL points at the UI origin and goes
// through the Vite dev proxy, which strips the X-Webhook-* auth headers — so the
// mock rewrites every callback to API_BASE and drops the leading /api prefix.
//
// Env knobs (all optional):
//   PORT             listen port                 (default 4000)
//   WEBHOOK_DELAY_MS delay before callback       (default 1500)
//   FIXTURE          report body to send back    (default fixtures/tttc-report.json)
//   API_BASE         where to deliver callbacks  (default http://localhost:3000)

import http from 'node:http';
import crypto from 'node:crypto';
import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { dirname, resolve } from 'node:path';

const PORT = Number(process.env.PORT ?? 4000);
const WEBHOOK_DELAY_MS = Number(process.env.WEBHOOK_DELAY_MS ?? 1500);
const API_BASE = process.env.API_BASE ?? 'http://localhost:3000';

const __dirname = dirname(fileURLToPath(import.meta.url));
const FIXTURE = process.env.FIXTURE ?? resolve(__dirname, '..', 'fixtures', 'tttc-report.json');

// Pre-parse the fixture once so we can fail fast on a bad path.
let fixtureReport;
try {
	fixtureReport = JSON.parse(readFileSync(FIXTURE, 'utf8'));
} catch (e) {
	console.error(`[mock-tttc] could not read fixture ${FIXTURE}: ${e.message}`);
	process.exit(1);
}

function log(...args) {
	console.log('[mock-tttc]', ...args);
}

function readBody(req) {
	return new Promise((resolve, reject) => {
		const chunks = [];
		req.on('data', (c) => chunks.push(c));
		req.on('end', () => resolve(Buffer.concat(chunks).toString('utf8')));
		req.on('error', reject);
	});
}

function signAndSend(webhookUrl, webhookSecret, jobId) {
	// Build the body string ONCE; the exact same bytes are both signed and sent.
	const report = { ...fixtureReport, jobId };
	const body = JSON.stringify(report);

	const timestamp = String(Math.floor(Date.now() / 1000));
	const signedPayload = `${timestamp}.${body}`;
	const signature =
		'sha256=' + crypto.createHmac('sha256', webhookSecret).update(signedPayload).digest('hex');

	// Always deliver to the API directly: take Comhairle's DOMAIN-derived callback
	// URL, keep its path (minus the leading /api), and swap the origin to API_BASE.
	// This bypasses the UI's Vite proxy, which would otherwise strip the auth headers.
	let url;
	try {
		url = new URL(webhookUrl);
		const apiBase = new URL(API_BASE);
		url.protocol = apiBase.protocol;
		url.host = apiBase.host; // host includes port
		url.pathname = url.pathname.replace(/^\/api(\/|$)/, '/');
	} catch {
		log(`invalid webhookUrl, skipping callback: ${webhookUrl}`);
		return;
	}
	if (url.href !== webhookUrl) log(`callback rewritten -> ${url.href} (from ${webhookUrl})`);

	const payload = Buffer.from(body);
	const req = http.request(
		url,
		{
			method: 'POST',
			headers: {
				'Content-Type': 'application/json',
				'Content-Length': payload.length,
				'X-Webhook-Timestamp': timestamp,
				'X-Webhook-Signature': signature
			}
		},
		(res) => {
			const chunks = [];
			res.on('data', (c) => chunks.push(c));
			res.on('end', () => {
				const text = Buffer.concat(chunks).toString('utf8');
				const ok = res.statusCode >= 200 && res.statusCode < 300;
				log(
					`${ok ? 'webhook accepted' : 'webhook REJECTED'} (${res.statusCode}) <- ${url.href}`,
					text.slice(0, 200)
				);
			});
		}
	);
	req.on('error', (e) => log(`webhook delivery failed -> ${url.href}: ${e.message}`));
	req.end(payload);

	log(`firing webhook -> ${url.href} (ts=${timestamp}, sig=${signature.slice(0, 23)}…)`);
}

const server = http.createServer(async (req, res) => {
	const path = req.url.split('?')[0].replace(/\/$/, '');
	if (req.method === 'POST' && path === '/api/v1/jobs') {
		const raw = await readBody(req);
		let job;
		try {
			job = JSON.parse(raw || '{}');
		} catch {
			res.writeHead(400, { 'Content-Type': 'application/json' });
			res.end(JSON.stringify({ error: 'invalid JSON body' }));
			return;
		}

		const jobId = crypto.randomUUID();
		const { webhookUrl, webhookSecret } = job;
		log(`job ${jobId}: ${Array.isArray(job.data) ? job.data.length : 0} comments, webhook=${webhookUrl}`);

		res.writeHead(200, { 'Content-Type': 'application/json' });
		res.end(JSON.stringify({ jobId, status: 'processing', createdAt: new Date().toISOString() }));

		if (!webhookUrl || !webhookSecret) {
			log(`job ${jobId}: missing webhookUrl/webhookSecret — no callback will be fired`);
			return;
		}
		setTimeout(() => signAndSend(webhookUrl, webhookSecret, jobId), WEBHOOK_DELAY_MS);
		return;
	}

	res.writeHead(404, { 'Content-Type': 'application/json' });
	res.end(JSON.stringify({ error: 'not found' }));
});

server.listen(PORT, '127.0.0.1', () => {
	log(`listening on http://localhost:${PORT}  (POST /api/v1/jobs)`);
	log(`fixture: ${FIXTURE}`);
	log(`webhook delay: ${WEBHOOK_DELAY_MS}ms, callbacks delivered to ${API_BASE}`);
});

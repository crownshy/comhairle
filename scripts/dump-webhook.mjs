#!/usr/bin/env node
// Tiny debug receiver for isolated verification of the TTTC mock's webhook.
// Stands in for Comhairle's submit_report endpoint: accepts any POST, prints
// the signature headers + body, and returns 201 (matching submit_report's
// success status). It does NOT need S3/Redis/Comhairle.
//
// Optionally, if WEBHOOK_SECRET is set, it independently recomputes the
// expected signature using the SAME scheme as api/src/routes/auth.rs
// (HMAC-SHA256 over `${timestamp}.${body}`, hex, "sha256=" prefix) and reports
// whether it matches — a cross-check that the mock signs correctly.
//
// Run: node scripts/dump-webhook.mjs    (listens on :4100 by default)
// Env: PORT (default 4100), WEBHOOK_SECRET (optional, enables match check)

import http from 'node:http';
import crypto from 'node:crypto';

const PORT = Number(process.env.PORT ?? 4100);
const SECRET = process.env.WEBHOOK_SECRET;

const server = http.createServer((req, res) => {
	const chunks = [];
	req.on('data', (c) => chunks.push(c));
	req.on('end', () => {
		const body = Buffer.concat(chunks).toString('utf8');
		const ts = req.headers['x-webhook-timestamp'];
		const sig = req.headers['x-webhook-signature'];

		console.log('\n[dump-webhook] ──────────────────────────────────────────');
		console.log(`  ${req.method} ${req.url}`);
		console.log(`  X-Webhook-Timestamp: ${ts}`);
		console.log(`  X-Webhook-Signature: ${sig}`);
		console.log(`  body (${body.length} bytes): ${body.slice(0, 120)}${body.length > 120 ? '…' : ''}`);

		if (SECRET && ts !== undefined) {
			const expected =
				'sha256=' +
				crypto.createHmac('sha256', SECRET).update(`${ts}.${body}`).digest('hex');
			const match = expected === sig;
			console.log(`  signature check (secret="${SECRET}"): ${match ? '✅ VALID' : '❌ MISMATCH'}`);
			if (!match) console.log(`    expected: ${expected}`);
		} else if (!SECRET) {
			console.log('  (set WEBHOOK_SECRET to auto-verify the signature here)');
		}

		res.writeHead(201, { 'Content-Type': 'application/json' });
		res.end(JSON.stringify({ success: true, url: 'mock://dump-webhook' }));
	});
});

server.listen(PORT, '127.0.0.1', () => {
	console.log(`[dump-webhook] listening on http://localhost:${PORT} (returns 201 to any POST)`);
	if (SECRET) console.log(`[dump-webhook] will verify signatures with secret "${SECRET}"`);
});

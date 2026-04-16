import { paraglideVitePlugin } from '@inlang/paraglide-js';
import devtoolsJson from 'vite-plugin-devtools-json';

import { defineConfig } from 'vite';
import { sveltekit } from '@sveltejs/kit/vite';
import tailwindcss from '@tailwindcss/vite';

export default defineConfig({
	plugins: [
		tailwindcss(),
		sveltekit(),
		devtoolsJson(),
		paraglideVitePlugin({
			project: './project.inlang',
			outdir: './src/lib/paraglide'
		})
	],
	server: {
		proxy: {
			'/api/ws': {
				target: 'http://localhost:3000',
				ws: true,
				changeOrigin: true,
				rewrite: (path) => path.replace(/^\/api/, ''),
				configure: (proxy, _options) => {
					proxy.on('error', (err, _req, _res) => {
						console.error('❌ WebSocket proxy error:', err);
					});
					proxy.on('proxyReq', (proxyReq, req, _res) => {
						console.log(
							'📤 WebSocket proxying:',
							req.method,
							req.url,
							'→',
							proxyReq.path
						);
					});
					proxy.on('proxyRes', (proxyRes, req, _res) => {
						console.log('📥 WebSocket response:', proxyRes.statusCode, req.url);
					});
					proxy.on('upgrade', (req, socket, head) => {
						console.log('⬆️  WebSocket upgrade:', req.url);
					});
					proxy.on('open', (proxySocket) => {
						console.log('✅ WebSocket proxy connection opened');
					});
					proxy.on('close', (res, socket, head) => {
						console.log('❌ WebSocket proxy connection closed');
					});
				}
			},
			'/api': {
				target: 'http://localhost:3000',
				changeOrigin: true,
				rewrite: (path) => path.replace(/^\/api/, ''),
				configure: (proxy, _options) => {
					proxy.on('error', (err, _req, _res) => {
						console.log('proxy error', err);
					});
					proxy.on('proxyReq', (proxyReq, req, _res) => {
						console.log('Sending Request to the Target:', req.method, req.url);
					});
					proxy.on('proxyRes', (proxyRes, req, _res) => {
						console.log(
							'Received Response from the Target:',
							proxyRes.statusCode,
							req.url
						);
					});
				}
			}
		}
	},
	test: {
		include: ['src/**/*.{test,spec}.{js,ts}']
	}
});

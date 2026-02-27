<script>
	if ('serviceWorker' in navigator) {
		window.addEventListener('load', () => {
			navigator.serviceWorker
				.register('/polis_worker.js')
				.then((registration) => {
					console.log('✅ Service Worker registered:', registration.scope);
				})
				.catch((error) => {
					console.error('❌ Service Worker registration failed:', error);
				});
		});
	}

	setTimeout(() => {
		console.log('sending request');
		fetch('/api/tools/polis/proxy').then(() => {
			console.log('response');
		});
	}, 2000);
</script>

<h1>Polis Rewrite test</h1>

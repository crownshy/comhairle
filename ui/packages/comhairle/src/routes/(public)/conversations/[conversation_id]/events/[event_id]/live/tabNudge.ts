/**
 * Flash the tab title and play a short tone until the tab is visible again.
 * Returns a function that stops the nudge early
 */
export function nudgeBackToTab(message: string): () => void {
	const originalTitle = document.title;
	let showingMessage = false;

	const interval = setInterval(() => {
		showingMessage = !showingMessage;
		document.title = showingMessage ? message : originalTitle;
	}, 1000);

	const stop = () => {
		clearInterval(interval);
		document.title = originalTitle;
		document.removeEventListener('visibilitychange', stopWhenVisible);
	};

	const stopWhenVisible = () => {
		if (!document.hidden) stop();
	};
	document.addEventListener('visibilitychange', stopWhenVisible);

	playChime();
	return stop;
}

function playChime({ notes = [660, 880], noteLength = 0.22, repeats = 2, volume = 0.2 } = {}) {
	const context = new AudioContext();
	const gap = 0.35;
	let start = context.currentTime;

	try {
		for (let r = 0; r < repeats; r++) {
			for (const frequency of notes) {
				const oscillator = context.createOscillator();
				const gain = context.createGain();
				oscillator.type = 'sine';
				oscillator.frequency.value = frequency;
				gain.gain.setValueAtTime(0.0001, start);
				gain.gain.exponentialRampToValueAtTime(volume, start + 0.02);
				gain.gain.exponentialRampToValueAtTime(0.0001, start + noteLength);
				oscillator.connect(gain).connect(context.destination);
				oscillator.start(start);
				oscillator.stop(start + noteLength);
				start += noteLength;
			}
			start += gap;
		}
		setTimeout(() => context.close(), (start - context.currentTime) * 1000 + 100);
	} catch {
		// Title will still flash if the audio is unavailable
	}
}

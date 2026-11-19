<script lang="ts">
	type Props = {
		percentage: string;
		error?: boolean;
	};

	let { percentage, error }: Props = $props();
</script>

<div class="text-muted-foreground flex items-center font-mono font-semibold">
	<progress
		aria-label="Media upload progress"
		data-error={error}
		max="100"
		value={percentage}
		class="border-border bg-muted accent-primary h-3 rounded-full border"
		class:bg-red-600={error}
	></progress>
	<span class="pl-5">{percentage}%</span>
</div>

<style>
	progress::-webkit-progress-value,
	progress::-moz-progress-bar {
		--color-1: var(--color-primary);
		--color-2: oklch(from var(--color-1) l c h / 0.8);
		background-image: repeating-linear-gradient(
			135deg,
			var(--color-1) 0 8px,
			var(--color-2) 8px 16px
		);
		animation: progress-slide 1.5s linear infinite;
	}

	@keyframes progress-slide {
		from {
			background-position: 0 0;
		}
		to {
			background-position: 22px 0;
		}
	}

	progress[value='100']::-webkit-progress-value,
	progress[value='100']::-moz-progress-bar {
		background-image: none;
		background-color: var(--color-green-600);
	}

	progress[data-error='true']::-webkit-progress-value,
	progress[data-error='true']::-moz-progress-bar {
		background-image: none;
		background-color: var(--color-red-600);
	}
</style>

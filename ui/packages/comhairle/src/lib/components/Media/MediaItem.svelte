<script lang="ts">
	type Props = {
		src: string;
	} & ({ type: 'audio' | 'video'; alt?: undefined } | { type: 'image'; alt: string });

	const { src, alt, type }: Props = $props();
</script>

{#if type === 'audio'}
	<audio {src} controls autoplay={false} class="h-full w-full"></audio>
{:else if type === 'video'}
	<video {src} controls autoplay={false} class="h-full w-full"><track kind="captions" /></video>
{:else}
	<img {src} {alt} class="h-full w-full cursor-pointer overflow-hidden object-cover" />
{/if}

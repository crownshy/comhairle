<script lang="ts">
	import 'altcha';
	import type {} from 'altcha/types/svelte';

	interface Props {
		value?: string;
	}

	let { value = $bindable('') }: Props = $props();
</script>

<altcha-widget
	style="--altcha-max-width:100%"
	challenge="/api/captcha/challenge"
	configuration={JSON.stringify({
		debug: true
	})}
	onstatechange={(ev) => {
		const { payload, state } = ev.detail;
		if (state === 'verified' && payload) {
			value = payload;
		} else {
			value = '';
		}
	}}
></altcha-widget>

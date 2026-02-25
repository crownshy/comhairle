<script lang="ts">
	import type { PageData } from './$types.js';
	import SignupForm from './signup-form.svelte';
	import { AuthPage } from '$lib/profile';
	let { data }: PageData = $props();
	let backTo = $derived(data.backTo);
</script>

<AuthPage>
	<SignupForm {backTo} />
</AuthPage>

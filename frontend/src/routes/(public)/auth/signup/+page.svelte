<script lang="ts">
	import type { PageData } from './$types.js';
	import SignupForm from './signup-form.svelte';
	import { AuthPage } from '$lib/profile';
	let { data }: PageData = $props();
</script>

<AuthPage>
	<SignupForm data={data.form} backTo={data.backTo} />
</AuthPage>

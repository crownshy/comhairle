<script lang="ts">
	import type { PageData } from './$types.js';
	import SignupForm from './signup-form.svelte';
	import { AuthPage } from '$lib/profile';
	let { backTo }: PageData = $props();
</script>

<AuthPage>
	<SignupForm {backTo} />
</AuthPage>

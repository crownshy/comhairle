<script lang="ts">
	import type { PageData } from './$types.js';
	import { AuthPage } from '$lib/profile';
	import LoginForm from './login-form.svelte';

	let { data }: PageData = $props();
</script>

<AuthPage>
	<LoginForm data={data.form} backTo={data.backTo} />
</AuthPage>

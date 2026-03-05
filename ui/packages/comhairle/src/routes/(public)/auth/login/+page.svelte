<script lang="ts">
	import type { PageData } from './$types.js';
	import { AuthPage } from '$lib/profile';
	import LoginForm from './login-form.svelte';
	export let data: PageData;
</script>

<svelte:head>
	<title>Login - Comhairle</title>
</svelte:head>

<AuthPage>
	<LoginForm backTo={data.backTo} />
</AuthPage>

<script lang="ts">
	import type { PageData } from './$types.js';
	import { AuthPage } from '$lib/profile';
	import LoginForm from './login-form.svelte';
	export let data: PageData;
</script>

<AuthPage>
	<LoginForm backTo={data.backTo} />
</AuthPage>

<script lang="ts">
	import { AuthPage } from '$lib/profile';
	import type { PageData } from './$types';
	import VerifyUser from './verify-user.svelte';
	let { data }: PageData = $props();
	let token = $derived(data.token);
</script>

<AuthPage>
	<VerifyUser {token} />
</AuthPage>

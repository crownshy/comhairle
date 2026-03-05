<script lang="ts">
	import type { PageData } from './$types.js';
	import { AuthPage } from '$lib/profile';
	import VerificationMessage from './verification-message.svelte';

	export let data: PageData;
</script>

<svelte:head>
	<title>Verify Your Email - Comhairle</title>
</svelte:head>

<AuthPage>
	<VerificationMessage user={data.user} />
</AuthPage>

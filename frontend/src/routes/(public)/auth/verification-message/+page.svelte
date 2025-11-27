<script lang="ts">
	import type { PageData } from './$types.js';
	import { AuthPage } from '$lib/profile';
	import VerificationMessage from './verification-message.svelte';

	export let data: PageData;
</script>

<AuthPage>
	<VerificationMessage user={data.user} />
</AuthPage>

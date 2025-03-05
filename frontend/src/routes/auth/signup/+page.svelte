<script lang="ts">
	import type { PageData } from './$types.js';
	import SignupForm from './signup-form.svelte';
	import { AuthPage } from '$lib/profile';
	export let data: PageData;
</script>

<AuthPage>
	<SignupForm data={data.form} />
</AuthPage>

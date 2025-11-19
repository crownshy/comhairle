<script lang="ts">
	let { token } = $props();

	let verifiedPromise = false; // TODO: make request to verify token
</script>

<section class="">
	{#await verifiedPromise}
		<h1 class="mb-2 text-xl font-bold">Verifying your email</h1>
		<p class="mb-4">Click on the verification link we have sent to your email address.</p>
	{:then}
		<h1 class="mb-2 text-xl font-bold">Successfully verified your email</h1>
		<p class="mb-4">
			Please <a href="/auth/login" class="underline">log in</a> to your verified account to join a conversation.
		</p>
	{:catch}
		<h1 class="mb-2 text-xl font-bold">Unable to verify your email address</h1>
		<p class="mb-4">
			Please click the link in your email again or
			<button type="button" class="underline" onclick={() => alert('Resend verification email')}>
				<!-- TODO: -->
				Send again.
			</button>
		</p>
	{/await}
</section>

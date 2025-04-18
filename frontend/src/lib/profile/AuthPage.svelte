<script lang="ts">
	import Logo from '$lib/assets/comhairle_logo.png';
</script>

<div class="align-center mx-auto max-w-xs md:mt-10">
	<div class="align-center mb-4 hidden flex-col items-center md:flex">
		<img src={Logo} alt="Comhairle Logo" />
		<a href="/" class="text-xl font-bold">Comhairle</a>
	</div>
	<div class="my-8 border border-dotted p-8">
		<slot />
	</div>
</div>

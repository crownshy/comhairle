<script lang="ts">
	import Logo from '$lib/assets/comhairle_logo.png';
</script>

<div class="mx-auto max-w-xs">
	<div class="align-center mb-4 flex flex-col items-center">
		<img src={Logo} alt="Comhairle Logo" />
		<a href="/" class="text-xl font-bold">Comhairle</a>
	</div>
	<div class="my-8 border border-dotted p-8">
		<slot />
	</div>
	<div class="text-muted-foreground w-full py-4 text-center">© 2025 Comhairle</div>
</div>

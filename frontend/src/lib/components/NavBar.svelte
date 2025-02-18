<script>
  import  Button  from "$lib/components/ui/button/button.svelte";
  import { onMount } from"svelte";
  let isOpen = false;
</script>

<nav class="w-full bg-white shadow-md p-4">
  <div class="container mx-auto flex justify-between items-center">
    <a href="#" class="text-xl font-bold">Comhairle</a>
    
    <!-- Desktop Navigation -->
    <div class="hidden md:flex space-x-4">
      <a href="/" class="text-gray-700 hover:text-black">Home</a>
      <a href="#" class="text-gray-700 hover:text-black">About</a>
      <a href="#" class="text-gray-700 hover:text-black">Services</a>
      <a href="#" class="text-gray-700 hover:text-black">Contact</a>
    </div>
    
    <!-- Mobile Menu Button -->
    <Button class="md:hidden" on:click={() => isOpen = !isOpen}>
      {isOpen ? "Close" : "Menu"}
    </Button>
  </div>

  <!-- Mobile Navigation -->
  <div class="md:hidden" class:open={isOpen}>
    {#if isOpen}
      <div class="flex flex-col mt-2 space-y-2">
        <a href="/" class="block px-4 py-2 text-gray-700 hover:text-black">Home</a>
        <a href="#" class="block px-4 py-2 text-gray-700 hover:text-black">About</a>
        <a href="#" class="block px-4 py-2 text-gray-700 hover:text-black">Services</a>
        <a href="#" class="block px-4 py-2 text-gray-700 hover:text-black">Contact</a>
      </div>
    {/if}
  </div>
</nav>

<style>
  .open {
    display: block;
  }
</style>


<script lang="ts">
  import {getData} from "$lib/api"
  let dataPromise= getData()
</script>

<div class='h-screen flex flex-col gap-y-10 items-center justify-center'>
  <h1 class="scroll-m-20 text-4xl font-extrabold tracking-tight lg:text-5xl">Welcome to Comhairle</h1>
  <p class="text-2xl">A platform about understanding what people want</p>

  {#await dataPromise}
    Loading data 
  {:then data}
    {data}
  {:catch error}
    <h2>Something went wrong</h2>
    {error}
  {/await}
</div>

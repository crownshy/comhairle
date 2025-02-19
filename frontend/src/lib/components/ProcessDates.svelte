<script lang="ts">
	import { formatDistance, format } from 'date-fns';
	let { startDate, endDate }: { startDate: Date; endDate: Date } = $props();
</script>

<aside class=" w-full rounded-3xl bg-green-950 p-4 text-white">
	<h3 class="font-bold">Opened at</h3>
	{format(startDate, 'EEEE d')}

	<h3 class="font-bold">Closes at</h3>
	{formatDistance(new Date(), endDate, { addSuffix: true })}
</aside>

<script lang="ts">
	import { format, formatDistanceToNow } from 'date-fns';
	let { startDate, endDate }: { startDate: Date; endDate: Date } = $props();
</script>

<aside class=" w-full rounded-3xl bg-green-950 p-4 text-white">
	<h3 class="font-bold">Opened</h3>
	{format(startDate, 'EEEE d yyyy')}

	<h3 class="font-bold">Closes</h3>
	{formatDistanceToNow(endDate, { addSuffix: true })}
</aside>

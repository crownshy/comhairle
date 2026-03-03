<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import * as Card from '$lib/components/ui/card';

	type Props={
		statement: String,
		onVote: (vote:"agree" | "disagree" | "pass")=>void
	}

	let {statement, onVote} : Props = $props()


</script>

<Card.Root class='w-full max-w-3xl'>
	<Card.Content class='w-full text-center'>
		{statement}
	</Card.Content>
	<Card.Footer>
		<div class='flex w-full flex-row justify-between'>
			<Button onclick={()=>onVote("agree")}>Agree</Button>
			<Button onclick={()=>onVote("disagree")} >Disagree</Button>
			<Button onclick={()=>onVote("pass")}>Pass</Button>
		</div>
	</Card.Footer>
</Card.Root>

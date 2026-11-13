<script lang="ts">
	import {
		Button,
		type ButtonVariant,
		type ButtonSize,
		type Props
	} from '$lib/components/ui/button';

	type HelperProps = Props & {
		content: string;
		variant: ButtonVariant;
		size: ButtonSize;
	};

	let props: HelperProps = $props();
	let content = props.content;
</script>

<Button variant={props.variant} size={props.size}>{content}</Button>

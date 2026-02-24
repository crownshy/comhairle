<script lang="ts">
	import * as Dialog from '$lib/components/ui/dialog';
	import * as Select from '$lib/components/ui/select';
	import * as Avatar from '$lib/components/ui/avatar';
	import Button from './ui/button/button.svelte';

	import Input from './ui/input/input.svelte';
	import Label from './ui/label/label.svelte';
	import Badge from './ui/badge/badge.svelte';
	import { Plus } from 'lucide-svelte';
	let role = $state('admin');

	let team = [
		{
			name: 'Stuart',
			role: 'Admin',
			image: 'https://crown-shy.com/assets/images/stuart_headshot.png'
		},
		{
			name: 'Shu',
			role: 'Moderator',
			image:
				'https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcT0F0EMjK8191_J1thWTaSOFDePYEI869DCag&s'
		},

		{
			name: 'Andy',
			role: 'Translator',
			image: 'http://www.andypaice.net/uploads/1/2/6/3/12637599/me_1_orig.png'
		}
	];
</script>

<div class="flex flex-col gap-4">
	<div class="flex items-center gap-2">
		<Input placeholder="Enter a name" class="flex-1" />
		<Dialog.Root>
			<Dialog.Trigger>
				<Button variant="default" class="shrink-0"><Plus class="h-4 w-4 mr-1" />Invite collaborator</Button>
			</Dialog.Trigger>
			<Dialog.Content>
				<Dialog.Header>
					<Dialog.Title>Add a collaborator</Dialog.Title>
				</Dialog.Header>

				<Label>Name</Label>
				<Input placeholder="search"></Input>

				<Label>Role</Label>
				<Select.Root bind:value={role} type="single">
					<Select.Trigger class="w-full">{role}</Select.Trigger>
					<Select.Content>
						<Select.Item value="Admin">Admin</Select.Item>
						<Select.Item value="Moderator">Moderator</Select.Item>
						<Select.Item value="Translator">Translator</Select.Item>
					</Select.Content>
				</Select.Root>
				<Button>Invite</Button>
			</Dialog.Content>
		</Dialog.Root>
	</div>
	<div class="flex flex-col gap-3">
		{#each team as team_member}
			<div class="flex items-center gap-3">
				<Avatar.Root class="h-10 w-10">
					<Avatar.Image src={team_member.image} alt={team_member.name}></Avatar.Image>
				</Avatar.Root>
				<span class="text-sm font-medium">{team_member.name}</span>
				<span class="rounded-full border px-2.5 py-0.5 text-xs font-medium">{team_member.role}</span>
			</div>
		{/each}
	</div>
</div>

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

<div class="flex w-full flex-row justify-between border-t-1 py-5">
	<p class="font-bold">Team</p>
	<div class="flex flex-col gap-2">
		{#each team as team_member}
			<div class="flex flex-row items-center gap-2">
				<Avatar.Root>
					<Avatar.Image src={team_member.image} alt={team_member.name}></Avatar.Image>
				</Avatar.Root>
				<p>{team_member.name}</p>
				<p class="rounded-2xl bg-[#C7AB86] px-4 py-2">{team_member.role}</p>
			</div>
		{/each}
		<Dialog.Root>
			<Dialog.Trigger><Button variant="default"><Plus />Add Colaborator</Button></Dialog.Trigger>
			<Dialog.Content>
				<Dialog.Header>
					<Dialog.Title>Add a colaborator</Dialog.Title>
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
</div>

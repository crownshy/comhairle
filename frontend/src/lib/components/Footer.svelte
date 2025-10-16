<script lang="ts">
	import * as m from '$lib/paraglide/messages';
</script>

<div class="bg-primary inline-flex w-full flex-col items-center justify-center gap-2.5">
	<div class="mx-auto flex w-full flex-col items-center justify-start gap-8 py-24">
		<div class="flex w-full max-w-full flex-col items-center justify-center gap-12 px-6">
			<div class="inline-flex h-20 w-full max-w-[1280px] items-center justify-between">
				<div class="flex flex-1 items-center justify-start gap-6">
					<div class="flex items-center justify-start gap-3">
						<div
							class="justify-start text-center font-['Inter'] text-2xl leading-7 font-medium text-lime-50"
						>
							Comhairle
						</div>
					</div>
					<div class="flex flex-1 items-center justify-center gap-1">
						<div class="flex items-center justify-start gap-8">
							<a
								href="/"
								class="justify-start font-['Inter'] text-lg leading-tight font-medium text-stone-300"
							>
								{m.home()}
							</a>
							<a
								href="/about"
								class="justify-start font-['Inter'] text-lg leading-tight font-medium text-stone-300"
							>
								{m.about()}
							</a>
							<a
								aria-disabled="true"
								class="justify-start font-['Inter'] text-lg leading-tight font-medium text-stone-300"
							>
								{m.contact()}
							</a>
							<a
								href="/rights"
								class="justify-start font-['Inter'] text-lg leading-tight font-medium text-stone-300"
							>
								{m.your_rights()}
							</a>
						</div>
					</div>
				</div>
				<div class="hidden flex-row items-center justify-between lg:flex">
					<div class="relative mr-2 h-12 w-28">
						<div class="bg-secondary absolute top-0 left-0 h-12 w-28 rounded-[100px]"></div>
						<div
							class="absolute top-[15px] left-[26px] justify-center text-center font-['Inter'] text-sm leading-tight font-medium text-white"
						>
							{m.subscribe()}
						</div>
					</div>
					<div class="relative h-12 w-80">
						<div
							class="border-secondary absolute top-0 left-0 h-12 w-80 rounded-[100px] border-2"
						></div>
						<div
							class="absolute top-[16px] left-[24px] justify-center font-['Inter'] text-sm leading-tight font-normal text-white"
						>
							{m.your_email()}
						</div>
					</div>
				</div>
			</div>
			<div
				data-orientation="Horizontal"
				class="flex flex-col items-start justify-start gap-2.5 self-stretch"
			>
				<div
					class="h-0 self-stretch outline outline-1 outline-offset-[-0.50px] outline-stone-700"
				></div>
			</div>
			<div class="inline-flex items-center justify-between self-stretch">
				<div class="text-mutted justify-start font-['Inter'] text-base leading-normal font-normal">
					Copyright {new Date().getFullYear()} © CrownShy
				</div>
				<div class="flex items-center justify-start gap-8">
					<a
						href="/rights/privacy"
						class="text-mutted justify-start font-['Inter'] text-base leading-normal font-normal"
					>
						{m.privacy_policy()}
					</a>
					<a
						href="/rights/tos"
						class="text-mutted justify-start font-['Inter'] text-base leading-normal font-normal"
					>
						{m.terms_of_service()}
					</a>
					<a
						href="/rights/cookies"
						class="text-mutted justify-start font-['Inter'] text-base leading-normal font-normal"
					>
						{m.cookies_settings()}
					</a>
				</div>
			</div>
		</div>
	</div>
</div>

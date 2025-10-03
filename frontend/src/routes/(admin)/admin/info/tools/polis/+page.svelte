<script lang="ts">
	import vtaiwan from '$lib/assets/vtaiwan.jpg';
	import climateuk from '$lib/assets/climageuk.jpg';
	import seattle from '$lib/assets/seattle_usa.jpg';
	import { ChevronRight } from 'lucide-svelte';
</script>

<div class=" prose inline-flex w-[1443px] flex-col items-center justify-center gap-2.5 py-8">
	<div class="flex flex-col items-center justify-center gap-2.5 self-stretch pb-12">
		<div class="flex flex-col items-start justify-start gap-12 self-stretch">
			<div class="inline-flex flex-wrap content-center items-center justify-start gap-2.5">
				<div
					data-state="Default"
					data-variant="Link"
					class="flex items-center justify-center gap-2.5"
				>
					<div
						class="text-base-muted-foreground justify-start font-['Inter'] text-sm leading-tight font-medium"
					>
						Home
					</div>
				</div>
				<div class="relative h-3.5 w-3.5 overflow-hidden">
					<ChevronRight size="sm" />
				</div>
				<div class="flex h-9 items-center justify-center gap-2.5">
					<div class="justify-start font-['Inter'] text-sm leading-tight font-medium text-black">
						Dashboard
					</div>
				</div>
				<div class="relative h-3.5 w-3.5 overflow-hidden">
					<ChevronRight size="sm" />
				</div>
				<div
					data-state="Default"
					data-variant="Link Current"
					class="flex items-center justify-center gap-2.5"
				>
					<div
						class="text-base-foreground justify-start font-['Inter'] text-sm leading-tight font-medium"
					>
						Toolbox
					</div>
				</div>
				<div class="relative h-3.5 w-3.5 overflow-hidden">
					<ChevronRight size="sm" />
				</div>
				<div
					class="text-base-foreground justify-start font-['Inter'] text-sm leading-tight font-medium"
				>
					Polis
				</div>
			</div>
			<div class="flex flex-col items-start justify-center gap-2">
				<div class="justify-start font-['Inter'] text-4xl leading-9 font-semibold text-black">
					Polis
				</div>
				<div
					class="text-base-muted-foreground w-[1278px] justify-start font-['Inter'] text-xl leading-7 font-semibold"
				>
					Polis is an open-source tool for large-scale, real-time conversation that helps groups
					discover their shared opinions and points of agreement. Unlike traditional surveys or
					forums, Polis uses machine learning to cluster participants’ responses, visualize opinion
					groups, and highlight areas of consensus. It has been used by governments, civil society,
					and organizations worldwide to make policymaking and collective decision-making more
					inclusive and evidence-based.
				</div>
			</div>
		</div>
	</div>
	<div class="flex flex-col items-start justify-start gap-9 self-stretch pb-12">
		<div
			class="outline-base-border h-0 self-stretch outline outline-1 outline-offset-[-0.50px]"
		></div>
		<div class="flex flex-col items-center justify-start gap-0.5 self-stretch">
			<div
				class="text-base-foreground justify-start self-stretch font-['Inter'] text-2xl leading-loose font-semibold"
			>
				How it works
			</div>
			<div
				class="text-base-muted-foreground justify-start self-stretch font-['Inter'] text-base leading-normal font-medium"
			>
				Placeholder for description
			</div>
		</div>
		<iframe
			width="560"
			height="315"
			src="https://www.youtube.com/embed/Aek5EDdDbds?si=EEbFguGWx2a8CRd7"
			title="YouTube video player"
			frameborder="0"
			allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share"
			referrerpolicy="strict-origin-when-cross-origin"
			allowfullscreen
		></iframe>
		<div class="w-[898px] justify-start">
			<span class="font-['Inter'] text-base leading-normal font-medium text-black"
				>Polis starts with a set of seed statements to frame a discussion. <br /><br />Participants
				respond by agreeing, disagreeing, or passing on each statement, and can also add their own.
				Polis then:<br /></span
			><span class="font-['Inter'] text-base leading-normal font-medium text-black"
				>Uses statistical clustering to group people with similar voting patterns.<br />Generates
				visualizations that show how opinions are distributed.<br />Identifies consensus
				statements—the points that cut across divides and unite groups.<br /></span
			><span class="font-['Inter'] text-base leading-normal font-medium text-black"
				><br />This combination of structured input and dynamic analysis makes it possible to have
				meaningful conversations with hundreds or thousands of participants at once, while surfacing
				insights that would be hard to see in unstructured debate.</span
			>
		</div>
	</div>
	<div class="flex flex-col items-start justify-start gap-9 self-stretch pb-12">
		<div
			class="outline-base-border h-0 self-stretch outline outline-1 outline-offset-[-0.50px]"
		></div>
		<div class="flex flex-col items-center justify-start gap-0.5 self-stretch">
			<div
				class="text-base-foreground justify-start self-stretch font-['Inter'] text-2xl leading-loose font-semibold"
			>
				Templates on Comhairle using Polis
			</div>
			<div
				class="text-base-muted-foreground justify-start self-stretch font-['Inter'] text-base leading-normal font-medium"
			>
				There is an optional customization capability for this workflow by adding the following
				features.
			</div>
		</div>
		<div class="flex flex-col items-center justify-start gap-12 self-stretch">
			<div class="inline-flex items-start justify-start gap-5 self-stretch">
				<div
					class="inline-flex flex-1 flex-col items-center justify-start gap-6 rounded-2xl bg-white p-6 shadow-[0px_2px_4px_0px_rgba(0,0,0,0.08)] outline outline-1 outline-offset-[-1px] outline-stone-300"
				>
					<div
						class="inline-flex w-full max-w-[1280px] flex-wrap content-start items-start justify-end gap-9"
					>
						<div class="inline-flex flex-1 flex-col items-start justify-start gap-6 pr-6">
							<div class="flex flex-col items-start justify-start gap-8 self-stretch">
								<div class="flex flex-col items-start justify-start gap-3 self-stretch">
									<div
										class="inline-flex flex-wrap content-start items-start justify-start gap-3 self-stretch"
									>
										<div class="flex items-center justify-start gap-1"></div>
									</div>
									<div
										class="text-Headings justify-start self-stretch font-['Inter'] text-xl leading-loose font-semibold"
									>
										Polis flow
									</div>
								</div>
							</div>
						</div>
						<div class="flex flex-wrap content-start items-start justify-end gap-9">
							<div class="flex items-center justify-end gap-1.5">
								<div
									class="justify-start text-right font-['Inter'] text-xs leading-snug font-medium text-black"
								>
									See more
								</div>
								<div class="relative h-4 w-4 overflow-hidden">
									<div class="absolute top-[2.81px] left-[1.69px] h-3 w-3.5 bg-black"></div>
								</div>
							</div>
						</div>
					</div>
					<div class="flex flex-col items-start justify-start gap-5 self-stretch">
						<div
							class="inline-flex flex-wrap content-start items-start justify-between self-stretch"
						>
							<div class="inline-flex flex-1 flex-col items-start justify-start gap-3">
								<div
									class="text-base-muted-foreground justify-start self-stretch font-['Inter'] text-base leading-normal font-medium"
								>
									Begin with introduction to the topic, follow with any tools of your choice to
									gather participants’ inputs, and end with a summary of the responses.
								</div>
							</div>
						</div>
					</div>
					<div class="flex flex-col items-start justify-start gap-4 self-stretch pr-6">
						<div class="flex w-[648px] flex-col items-start justify-start gap-2">
							<div class="inline-flex items-center justify-start gap-2">
								<div
									class="outline-tailwind-colors-base-transparent/0 flex h-4 w-4 items-center justify-center gap-1 overflow-hidden rounded-full bg-stone-300 px-2 py-0.5 outline outline-1 outline-offset-[-1px]"
								>
									<div
										class="justify-start font-['Inter'] text-xs leading-none font-medium text-gray-500"
									>
										1
									</div>
								</div>
								<div
									class="justify-start font-['Inter'] text-base leading-normal font-medium text-stone-900"
								>
									Introduction
								</div>
							</div>
							<div class="inline-flex items-center justify-start gap-2">
								<div
									class="outline-tailwind-colors-base-transparent/0 flex h-4 w-4 items-center justify-center gap-1 overflow-hidden rounded-full bg-stone-300 px-2 py-0.5 outline outline-1 outline-offset-[-1px]"
								>
									<div
										class="justify-start font-['Inter'] text-xs leading-none font-medium text-gray-500"
									>
										2
									</div>
								</div>
								<div
									class="justify-start font-['Inter'] text-base leading-normal font-medium text-stone-900"
								>
									Input (Polis)
								</div>
							</div>
							<div class="inline-flex items-center justify-start gap-2">
								<div
									class="outline-tailwind-colors-base-transparent/0 flex h-4 w-4 items-center justify-center gap-1 overflow-hidden rounded-full bg-stone-300 px-2 py-0.5 outline outline-1 outline-offset-[-1px]"
								>
									<div
										class="justify-start font-['Inter'] text-xs leading-none font-medium text-gray-500"
									>
										3
									</div>
								</div>
								<div
									class="justify-start font-['Inter'] text-base leading-normal font-medium text-stone-900"
								>
									Summary
								</div>
							</div>
						</div>
					</div>
					<div class="inline-flex items-start justify-start gap-4 self-stretch">
						<div
							data-show-left-icon="false"
							data-show-right-icon="false"
							data-variant="Outline"
							class="bg-base-background flex h-7 items-center justify-center gap-1 overflow-hidden rounded-lg px-2.5 shadow-[0px_1px_2px_0px_rgba(0,0,0,0.05)] outline outline-1 outline-offset-[-1px] outline-stone-300"
						>
							<div
								class="text-base-foreground justify-start font-['Inter'] text-sm leading-tight font-medium"
							>
								Basic survey tool
							</div>
						</div>
						<div
							data-show-left-icon="false"
							data-show-right-icon="false"
							data-variant="Outline"
							class="bg-base-background flex h-7 items-center justify-center gap-1 overflow-hidden rounded-lg px-2.5 shadow-[0px_1px_2px_0px_rgba(0,0,0,0.05)] outline outline-1 outline-offset-[-1px] outline-stone-300"
						>
							<div
								class="text-base-foreground justify-start font-['Inter'] text-sm leading-tight font-medium"
							>
								dialogical agent (RAG)
							</div>
						</div>
						<div
							data-show-left-icon="false"
							data-show-right-icon="false"
							data-variant="Outline"
							class="bg-base-background flex h-7 items-center justify-center gap-1 overflow-hidden rounded-lg px-2.5 shadow-[0px_1px_2px_0px_rgba(0,0,0,0.05)] outline outline-1 outline-offset-[-1px] outline-stone-300"
						>
							<div
								class="text-base-foreground justify-start font-['Inter'] text-sm leading-tight font-medium"
							>
								30-50 minutes
							</div>
						</div>
					</div>
					<div class="flex flex-col items-start justify-start gap-6 self-stretch">
						<div class="flex flex-col items-start justify-start gap-3 self-stretch">
							<div
								class="text-base-foreground justify-start font-['Inter'] text-sm leading-tight font-medium"
							>
								Last used
							</div>
							<div
								data-show-left-icon="false"
								data-show-right-icon="true"
								data-variant="Outline"
								class="inline-flex h-7 items-center justify-center gap-1 overflow-hidden rounded-lg bg-stone-100 px-2.5 shadow-[0px_1px_2px_0px_rgba(0,0,0,0.05)] outline outline-1 outline-offset-[-1px]"
							>
								<div
									class="text-base-foreground justify-start font-['Inter'] text-sm leading-none font-normal"
								>
									National Speed management Review (ongoing)
								</div>
								<div class="relative h-3 w-3 overflow-hidden">
									<div
										class="outline-base-muted-foreground absolute top-[2.25px] left-[1.50px] h-2 w-2 outline outline-[1.25px] outline-offset-[-0.62px]"
									></div>
								</div>
							</div>
							<div
								data-show-left-icon="false"
								data-show-right-icon="true"
								data-variant="Outline"
								class="inline-flex h-7 items-center justify-center gap-1 overflow-hidden rounded-lg bg-stone-100 px-2.5 shadow-[0px_1px_2px_0px_rgba(0,0,0,0.05)] outline outline-1 outline-offset-[-1px]"
							>
								<div
									class="text-base-foreground justify-start font-['Inter'] text-sm leading-none font-normal"
								>
									Scottish seabird conversation (ongoing)
								</div>
								<div class="relative h-3 w-3 overflow-hidden">
									<div
										class="outline-base-muted-foreground absolute top-[2.25px] left-[1.50px] h-2 w-2 outline outline-[1.25px] outline-offset-[-0.62px]"
									></div>
								</div>
							</div>
							<div
								data-show-left-icon="false"
								data-show-right-icon="true"
								data-variant="Outline"
								class="inline-flex h-7 items-center justify-center gap-1 overflow-hidden rounded-lg bg-stone-100 px-2.5 shadow-[0px_1px_2px_0px_rgba(0,0,0,0.05)] outline outline-1 outline-offset-[-1px]"
							>
								<div
									class="text-base-foreground justify-start font-['Inter'] text-sm leading-none font-normal"
								>
									Inshore fisheries management (four days ago)
								</div>
								<div class="relative h-3 w-3 overflow-hidden">
									<div
										class="outline-base-muted-foreground absolute top-[2.25px] left-[1.50px] h-2 w-2 outline outline-[1.25px] outline-offset-[-0.62px]"
									></div>
								</div>
							</div>
						</div>
					</div>
					<div
						data-show-left-icon="false"
						data-show-right-icon="false"
						data-size="lg"
						data-state="Default"
						data-variant="Default"
						class="inline-flex h-10 items-center justify-center gap-2 self-stretch rounded-md bg-stone-400 px-8 py-2 shadow-[0px_1px_2px_0px_rgba(0,0,0,0.05)]"
					>
						<div class="justify-center font-['Inter'] text-sm leading-tight font-medium text-white">
							Select
						</div>
					</div>
				</div>
				<div
					class="inline-flex flex-1 flex-col items-center justify-start gap-6 rounded-2xl bg-white p-6 shadow-[0px_2px_4px_0px_rgba(0,0,0,0.08)] outline outline-1 outline-offset-[-1px] outline-stone-300"
				>
					<div
						class="inline-flex w-full max-w-[1280px] flex-wrap content-start items-start justify-end gap-9"
					>
						<div class="inline-flex flex-1 flex-col items-start justify-start gap-6 pr-6">
							<div class="flex flex-col items-start justify-start gap-8 self-stretch">
								<div class="flex flex-col items-start justify-start gap-3 self-stretch">
									<div
										class="inline-flex flex-wrap content-start items-start justify-start gap-3 self-stretch"
									></div>
									<div
										class="text-Headings justify-start self-stretch font-['Inter'] text-xl leading-loose font-semibold"
									>
										Survey + Polis
									</div>
								</div>
							</div>
						</div>
						<div class="flex flex-wrap content-start items-start justify-end gap-9">
							<div class="flex items-center justify-end gap-1.5">
								<div
									class="justify-start text-right font-['Inter'] text-xs leading-snug font-medium text-black"
								>
									See more
								</div>
								<div class="relative h-4 w-4 overflow-hidden">
									<div class="absolute top-[2.81px] left-[1.69px] h-3 w-3.5 bg-black"></div>
								</div>
							</div>
						</div>
					</div>
					<div class="flex flex-col items-start justify-start gap-5 self-stretch">
						<div
							class="inline-flex flex-wrap content-start items-start justify-between self-stretch"
						>
							<div class="inline-flex flex-1 flex-col items-start justify-start gap-3">
								<div
									class="text-base-muted-foreground justify-start self-stretch font-['Inter'] text-base leading-normal font-medium"
								>
									Begin with introduction to the topic, follow with any tools of your choice to
									gather participants’ inputs, and end with a summary of the responses.
								</div>
							</div>
						</div>
					</div>
					<div class="flex flex-col items-start justify-start gap-4 self-stretch pr-6">
						<div class="flex w-[648px] flex-col items-start justify-start gap-2">
							<div class="inline-flex items-center justify-start gap-2">
								<div
									class="outline-tailwind-colors-base-transparent/0 flex h-4 w-4 items-center justify-center gap-1 overflow-hidden rounded-full bg-stone-300 px-2 py-0.5 outline outline-1 outline-offset-[-1px]"
								>
									<div
										class="justify-start font-['Inter'] text-xs leading-none font-medium text-gray-500"
									>
										1
									</div>
								</div>
								<div
									class="justify-start font-['Inter'] text-base leading-normal font-medium text-stone-900"
								>
									Introduction
								</div>
							</div>
							<div class="inline-flex items-center justify-start gap-2">
								<div
									class="outline-tailwind-colors-base-transparent/0 flex h-4 w-4 items-center justify-center gap-1 overflow-hidden rounded-full bg-stone-300 px-2 py-0.5 outline outline-1 outline-offset-[-1px]"
								>
									<div
										class="justify-start font-['Inter'] text-xs leading-none font-medium text-gray-500"
									>
										2
									</div>
								</div>
								<div
									class="justify-start font-['Inter'] text-base leading-normal font-medium text-stone-900"
								>
									Input (Survey and Polis)
								</div>
							</div>
							<div class="inline-flex items-center justify-start gap-2">
								<div
									class="outline-tailwind-colors-base-transparent/0 flex h-4 w-4 items-center justify-center gap-1 overflow-hidden rounded-full bg-stone-300 px-2 py-0.5 outline outline-1 outline-offset-[-1px]"
								>
									<div
										class="justify-start font-['Inter'] text-xs leading-none font-medium text-gray-500"
									>
										3
									</div>
								</div>
								<div
									class="justify-start font-['Inter'] text-base leading-normal font-medium text-stone-900"
								>
									Summary
								</div>
							</div>
						</div>
					</div>
					<div class="inline-flex items-start justify-start gap-4 self-stretch">
						<div
							data-show-left-icon="false"
							data-show-right-icon="false"
							data-variant="Outline"
							class="bg-base-background flex h-7 items-center justify-center gap-1 overflow-hidden rounded-lg px-2.5 shadow-[0px_1px_2px_0px_rgba(0,0,0,0.05)] outline outline-1 outline-offset-[-1px] outline-stone-300"
						>
							<div
								class="text-base-foreground justify-start font-['Inter'] text-sm leading-tight font-medium"
							>
								Basic survey tool
							</div>
						</div>
						<div
							data-show-left-icon="false"
							data-show-right-icon="false"
							data-variant="Outline"
							class="bg-base-background flex h-7 items-center justify-center gap-1 overflow-hidden rounded-lg px-2.5 shadow-[0px_1px_2px_0px_rgba(0,0,0,0.05)] outline outline-1 outline-offset-[-1px] outline-stone-300"
						>
							<div
								class="text-base-foreground justify-start font-['Inter'] text-sm leading-tight font-medium"
							>
								dialogical agent (RAG)
							</div>
						</div>
						<div
							data-show-left-icon="false"
							data-show-right-icon="false"
							data-variant="Outline"
							class="bg-base-background flex h-7 items-center justify-center gap-1 overflow-hidden rounded-lg px-2.5 shadow-[0px_1px_2px_0px_rgba(0,0,0,0.05)] outline outline-1 outline-offset-[-1px] outline-stone-300"
						>
							<div
								class="text-base-foreground justify-start font-['Inter'] text-sm leading-tight font-medium"
							>
								30-50 minutes
							</div>
						</div>
					</div>
					<div class="flex flex-col items-start justify-start gap-6 self-stretch">
						<div class="flex flex-col items-start justify-start gap-3 self-stretch">
							<div
								class="text-base-foreground justify-start font-['Inter'] text-sm leading-tight font-medium"
							>
								Last used
							</div>
							<div
								data-show-left-icon="false"
								data-show-right-icon="true"
								data-variant="Outline"
								class="inline-flex h-7 items-center justify-center gap-1 overflow-hidden rounded-lg bg-stone-100 px-2.5 shadow-[0px_1px_2px_0px_rgba(0,0,0,0.05)] outline outline-1 outline-offset-[-1px]"
							>
								<div
									class="text-base-foreground justify-start font-['Inter'] text-sm leading-none font-normal"
								>
									National Speed management Review (ongoing)
								</div>
								<div class="relative h-3 w-3 overflow-hidden">
									<div
										class="outline-base-muted-foreground absolute top-[2.25px] left-[1.50px] h-2 w-2 outline outline-[1.25px] outline-offset-[-0.62px]"
									></div>
								</div>
							</div>
							<div
								data-show-left-icon="false"
								data-show-right-icon="true"
								data-variant="Outline"
								class="inline-flex h-7 items-center justify-center gap-1 overflow-hidden rounded-lg bg-stone-100 px-2.5 shadow-[0px_1px_2px_0px_rgba(0,0,0,0.05)] outline outline-1 outline-offset-[-1px]"
							>
								<div
									class="text-base-foreground justify-start font-['Inter'] text-sm leading-none font-normal"
								>
									Scottish seabird conversation (ongoing)
								</div>
								<div class="relative h-3 w-3 overflow-hidden">
									<div
										class="outline-base-muted-foreground absolute top-[2.25px] left-[1.50px] h-2 w-2 outline outline-[1.25px] outline-offset-[-0.62px]"
									></div>
								</div>
							</div>
							<div
								data-show-left-icon="false"
								data-show-right-icon="true"
								data-variant="Outline"
								class="inline-flex h-7 items-center justify-center gap-1 overflow-hidden rounded-lg bg-stone-100 px-2.5 shadow-[0px_1px_2px_0px_rgba(0,0,0,0.05)] outline outline-1 outline-offset-[-1px]"
							>
								<div
									class="text-base-foreground justify-start font-['Inter'] text-sm leading-none font-normal"
								>
									Inshore fisheries management (four days ago)
								</div>
								<div class="relative h-3 w-3 overflow-hidden">
									<div
										class="outline-base-muted-foreground absolute top-[2.25px] left-[1.50px] h-2 w-2 outline outline-[1.25px] outline-offset-[-0.62px]"
									></div>
								</div>
							</div>
						</div>
					</div>
					<div
						data-show-left-icon="false"
						data-show-right-icon="false"
						data-size="lg"
						data-state="Default"
						data-variant="Default"
						class="inline-flex h-10 items-center justify-center gap-2 self-stretch rounded-md bg-stone-400 px-8 py-2 shadow-[0px_1px_2px_0px_rgba(0,0,0,0.05)]"
					>
						<div class="justify-center font-['Inter'] text-sm leading-tight font-medium text-white">
							Select
						</div>
					</div>
				</div>
			</div>
		</div>
	</div>
	<div class="flex flex-col items-start justify-start gap-9 self-stretch pb-12">
		<div
			class="outline-base-border h-0 self-stretch outline outline-1 outline-offset-[-0.50px]"
		></div>
		<div class="flex flex-col items-center justify-start gap-0.5 self-stretch">
			<div
				class="text-base-foreground justify-start self-stretch font-['Inter'] text-2xl leading-loose font-semibold"
			>
				Case Studies
			</div>
			<div
				class="text-base-muted-foreground justify-start self-stretch font-['Inter'] text-base leading-normal font-medium"
			>
				There is an optional customization capability for this workflow by adding the following
				features.
			</div>
		</div>
		<div class="inline-flex w-full items-start justify-start gap-6 self-stretch">
			<div class="inline-flex flex-1 flex-col items-start justify-start gap-4">
				<img class="h-[200px] w-[473px] self-stretch rounded-[40px]" src={vtaiwan} />
				<div class="flex flex-col items-start justify-start gap-4 self-stretch px-4">
					<div class="flex flex-col items-start justify-center gap-2 self-stretch">
						<div
							class="justify-start self-stretch font-['Inter'] text-2xl leading-loose font-semibold text-black"
						>
							Taiwan’s vTaiwan Process
						</div>
						<div
							class="justify-start self-stretch font-['Inter'] text-lg leading-7 font-medium text-stone-900"
						>
							How AI can be used responsibly in public services (e.g., healthcare, transport,
							policing).
						</div>
					</div>
					<div
						class="text-base-muted-foreground justify-start self-stretch font-['Inter'] text-lg leading-7 font-normal"
					>
						The Taiwanese government used Polis to engage citizens on complex issues like
						ride-sharing regulation (Uber) and online alcohol sales. The process brought together
						industry, civil society, and government, leading to consensus-driven policy
						recommendations.
					</div>
				</div>
			</div>
			<div class="inline-flex flex-1 flex-col items-start justify-start gap-4">
				<img class="h-[200px] w-[473px] self-stretch rounded-[40px]" src={climateuk} />
				<div class="flex flex-col items-start justify-start gap-4 self-stretch px-4">
					<div class="flex flex-col items-start justify-center gap-2 self-stretch">
						<div
							class="justify-start self-stretch font-['Inter'] text-2xl leading-loose font-semibold text-black"
						>
							Climate Assembly UK
						</div>
						<div
							class="justify-start self-stretch font-['Inter'] text-lg leading-7 font-medium text-stone-900"
						>
							How AI can be used responsibly in public services (e.g., healthcare, transport,
							policing).
						</div>
					</div>
					<div
						class="text-base-muted-foreground justify-start self-stretch font-['Inter'] text-lg leading-7 font-normal"
					>
						Polis was used as part of public engagement exercises around climate action, helping
						organizers understand where consensus existed among citizens on policy measures.
					</div>
				</div>
			</div>
			<div class="inline-flex flex-1 flex-col items-start justify-start gap-4">
				<img class=" h-[200px] w-[473px] self-stretch rounded-[40px]" src={seattle} />
				<div class="flex flex-col items-start justify-start gap-4 self-stretch px-4">
					<div class="flex flex-col items-start justify-center gap-2 self-stretch">
						<div
							class="justify-start self-stretch font-['Inter'] text-2xl leading-loose font-semibold text-black"
						>
							Seattle (USA) – Police Reform
						</div>
						<div
							class="justify-start self-stretch font-['Inter'] text-lg leading-7 font-medium text-stone-900"
						>
							How AI can be used responsibly in public services (e.g., healthcare, transport,
							policing).
						</div>
					</div>
					<div
						class="text-base-muted-foreground justify-start self-stretch font-['Inter'] text-lg leading-7 font-normal"
					>
						Polis was deployed to facilitate dialogue between residents and officials on policing
						and public safety, allowing diverse voices to be heard and common ground to emerge
						despite polarization.
					</div>
				</div>
			</div>
		</div>
	</div>
	<div class="h-44 w-[1280px] max-w-[1280px] pb-9"></div>
</div>

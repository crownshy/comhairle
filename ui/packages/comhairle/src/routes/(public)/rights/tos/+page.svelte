<script lang="ts">
	import Highlight from '$lib/components/Highlight.svelte';

	// Shared type scale for the Your Rights pages (matches the Jazz Up designs).
	const sectionTitle = 'text-2xl font-bold text-foreground md:text-3xl';
	const heading = 'text-xl font-semibold text-foreground md:text-2xl';
	const body = 'text-base leading-7 text-foreground';
	const list = 'flex list-disc flex-col gap-1 pl-6 text-base leading-7 text-foreground';
</script>

<svelte:head>
	<title>Terms of Service - Comhairle</title>
</svelte:head>

<section class="flex flex-col gap-8 md:gap-12">
	<h2 class={sectionTitle}>Terms of service</h2>

	<div class="flex flex-col gap-10 md:gap-14">
		<div class="flex flex-col gap-5">
			<h3 class={heading}>1. User Eligibility</h3>
			<p class={body}>
				You must be at least 13 years old to use our services. If you are under 18, you must
				have the consent of a parent or legal guardian. By accessing the website, you
				confirm that you meet these eligibility requirements.
			</p>
		</div>

		<div class="flex flex-col gap-5">
			<h3 class={heading}>2. Account Creation &amp; Management</h3>
			<ul class={list}>
				<li>
					Some features may require you to create an account. You are responsible for
					maintaining the confidentiality of your account credentials.
				</li>
				<li>You agree to provide accurate, complete, and updated information.</li>
				<li>
					We reserve the right to suspend or terminate accounts that violate these
					T&amp;C.
				</li>
			</ul>
		</div>

		<div class="flex flex-col gap-5">
			<h3 class={heading}>3. Acceptable Use Policy</h3>
			<p class={body}>You agree not to:</p>
			<ul class={list}>
				<li>Use the site for any illegal or unauthorized purpose.</li>
				<li>Interfere with the security, integrity, or performance of the website.</li>
				<li>Transmit any harmful or malicious software.</li>
				<li>
					Engage in harassment, discrimination, or the spread of knowingly misleading
					information.
				</li>
			</ul>
		</div>

		<div class="flex flex-col gap-5">
			<h3 class={heading}>4. Intellectual Property Rights</h3>
			<ul class={list}>
				<li>
					All content, including text, graphics, logos, and software, is owned by Crown
					Shy Ltd or licensed to us and is protected by intellectual property laws.
				</li>
				<li>
					Users may not reproduce, distribute, or modify any content without explicit
					permission.
				</li>
			</ul>
		</div>

		<div class="flex flex-col gap-5">
			<h3 class={heading}>5. User-Generated Content</h3>
			<p class={body}>
				Users may post content, including text, images, and other materials
				("Contributions"). By posting, you grant Comhairle a non-exclusive, royalty-free,
				perpetual, and worldwide license to use, display, and distribute your content. You
				represent and warrant that your Contributions:
			</p>
			<ul class={list}>
				<li>Are not spam, randomly generated, or created by automated systems.</li>
				<li>
					Do not contain deceptive, misleading, or unethical commercial content aimed at
					increasing traffic or search rankings for third-party sites.
				</li>
				<li>
					Do not promote illegal activities such as phishing, fraud, or impersonation.
				</li>
				<li>
					Are not pornographic, defamatory, or libelous and do not incite violence or harm
					against individuals or groups.
				</li>
				<li>
					Do not violate the privacy, publicity rights, or intellectual property rights of
					any third party.
				</li>
			</ul>
			<p class={body}>
				Comhairle reserves the right to remove, edit, or restrict access to any
				user-generated content at its discretion.
			</p>
			<p class={body}>
				Visitors accessing content posted by other users acknowledge that such content
				reflects the opinions and views of the contributors and not Comhairle or Crown Shy
				Ltd. Comhairle does not endorse or verify the accuracy of user-generated content.
				Visitors should report inappropriate or infringing content through the provided
				contact channels.
			</p>
		</div>

		<div class="flex flex-col gap-5">
			<h3 class={heading}>6. Privacy Considerations</h3>
			<p class={body}>
				Your privacy is important to us. Our <a
					href="/rights/privacy"
					class="text-primary underline">Privacy Policy</a
				> outlines how we collect, use, and protect your personal data. By using our services,
				you consent to our data practices.
			</p>
		</div>

		<div class="flex flex-col gap-5">
			<h3 class={heading}>7. Limitation of Liability</h3>
			<ul class={list}>
				<li>
					Comhairle and Crown Shy Ltd are not liable for any indirect, incidental, or
					consequential damages arising from your use of our website.
				</li>
				<li>We do not guarantee that the website will be error-free or uninterrupted.</li>
				<li>Users access the website at their own risk.</li>
			</ul>
		</div>

		<div class="flex flex-col gap-5">
			<h3 class={heading}>8. Dispute Resolution</h3>
			<ul class={list}>
				<li>Any disputes shall be governed by the laws of [Insert Jurisdiction].</li>
				<li>
					Disputes must first be resolved through good-faith negotiations. If unresolved,
					disputes may be subject to arbitration or legal proceedings.
				</li>
			</ul>
		</div>

		<div class="flex flex-col gap-5">
			<h3 class={heading}>9. Changes to These Terms</h3>
			<p class={body}>
				We may update these T&amp;C periodically. Changes will be posted on this page with a
				revised "Last Updated" date. Continued use after changes constitutes acceptance of
				the revised terms.
			</p>
		</div>

		<div class="flex flex-col gap-5">
			<h3 class={heading}>10. Contact Information</h3>
			<p class={body}>
				For any questions or concerns regarding these Terms &amp; Conditions, please contact
				us at:
			</p>
			<Highlight>
				Crown Shy Ltd &nbsp; Email:
				<a href="mailto:team@crown-shy.com" class="text-primary underline"
					>team@crown-shy.com</a
				>
			</Highlight>
			<p class={body}>
				By using our website, you acknowledge that you have read, understood, and agreed to
				these Terms &amp; Conditions.
			</p>
		</div>
	</div>
</section>

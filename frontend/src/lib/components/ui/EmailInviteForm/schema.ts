import { LoginBehaviour } from "$lib/api/api";
import { z } from "zod";

export const splitEmails = (emailStr: String) => emailStr.split(",").map(e => e.trim()).filter(Boolean)

export const emailsFormSchema = z.object({
	emails: z.string()
		.min(1, "Please enter at least one email")
		.refine(
			(emails) =>
				splitEmails(emails).every((e) => /^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(e)),
			{ message: 'One or more emails are invalid' }
		).default(""),
	expiresOption: z.enum(["1 day", "1 week", "1 month", "never", "custom", "never"]).default("never"),
	customExpire: z.string().optional()
});

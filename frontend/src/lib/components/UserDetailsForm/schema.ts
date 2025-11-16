import { z } from "zod";

export const userDetailsSchema = z.object({
	username: z.string().min(1, "Username is required").optional(),
	password: z.string().min(6, "Password must be at least 6 characters").optional(),
	confirmPassword: z.string().optional(),
}).refine((data) => {
	if (data.password && data.password !== data.confirmPassword) {
		return false;
	}
	return true;
}, {
	message: "Passwords don't match",
	path: ["confirmPassword"],
});

export type UserDetailsFormData = z.infer<typeof userDetailsSchema>;
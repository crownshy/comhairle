import { z } from "zod";

export const commonStepSchema = z.object({
	name: z.string().min(5, ""),
	description: z.string().min(10, ""),
	// openDate: z.date(),
	// closeDate: z.date()
})
// .refine((data) => data.closeDate > data.openDate,
// 	{
// 		message: "The close date must be after start date",
// 		path: ["endDate"], // error will be attached to endDate
// 	}
// );

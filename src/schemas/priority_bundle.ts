import { z } from "zod";

const OperationSchema = z.enum(["add", "sub", "mul", "div", "set"]);

export const PriorityBundleModifierSchema = z.object({
    id: z.number().optional(),
    bundle_id: z.number().optional(),
    grade: z.string().length(1, "Grade must be a single character"),
    target_key: z.string(),
    operation: OperationSchema,
    value: z.string(),
    created_at: z.string().optional(),
    updated_at: z.string().optional(),
});

export const PriorityBundleSkillSchema = z.object({
    id: z.number().optional(),
    bundle_id: z.number().optional(),
    grade: z.string().length(1, "Grade must be a single character"),
    attribute: z.string(),
    amount: z.number(),
    rating: z.number(),
});

export const PriorityBundleMetatypeSchema = z.object({
    id: z.number().optional(),
    bundle_id: z.number().optional(),
    grade: z.string().length(1, "Grade must be a single character"),
    special_points: z.number(),
    name: z.string(),
});

export const PriorityBundleQualitySchema = z.object({
    id: z.number().optional(),
    bundle_id: z.number().optional(),
    grade: z.string().length(1, "Grade must be a single character"),
    name: z.string(),
});

export type Operation = z.infer<typeof OperationSchema>;
export type PriorityBundleModifier = z.infer<
    typeof PriorityBundleModifierSchema
>;
export type PriorityBundleSkill = z.infer<typeof PriorityBundleSkillSchema>;
export type PriorityBundleMetatype = z.infer<
    typeof PriorityBundleMetatypeSchema
>;
export type PriorityBundleQuality = z.infer<typeof PriorityBundleQualitySchema>;
export type PriorityBundle = {
    id?: number;
    name?: string;
    grade: string;
    parent_bundle_id?: number;
    modifiers: PriorityBundleModifier[];
    skills: PriorityBundleSkill[];
    metatypes: PriorityBundleMetatype[];
    qualities: PriorityBundleQuality[];
    children: PriorityBundle[];
};
export type PriorityBundleArray = z.infer<typeof PriorityBundleArraySchema>;

export const PriorityBundleSchema: z.ZodType<PriorityBundle> = z.lazy(() =>
    z.object({
        id: z.number().optional(),
        name: z.string().optional(),
        grade: z.string().length(1, "Grade must be a single character"),
        parent_bundle_id: z.number().optional(),
        modifiers: z.array(PriorityBundleModifierSchema),
        skills: z.array(PriorityBundleSkillSchema),
        metatypes: z.array(PriorityBundleMetatypeSchema),
        qualities: z.array(PriorityBundleQualitySchema),
        children: z.array(PriorityBundleSchema),
    }),
);

export const PriorityBundleArraySchema = z.array(PriorityBundleSchema);

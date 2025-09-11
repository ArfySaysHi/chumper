import { z } from "zod";

const OperationSchema = z.enum(["add", "sub", "mul", "div", "set"]);
const PriorityGradeSchema = z.enum(["A", "B", "C", "D", "E", "*"]);

export const PriorityBundleModifierSchema = z.object({
  id: z.number().optional(),
  bundle_id: z.number().optional(),
  grade: PriorityGradeSchema,
  target_key: z.string(),
  operation: OperationSchema,
  value: z.string(),
  created_at: z.string().optional(),
  updated_at: z.string().optional(),
});

const defaultModifiers = {
  A: [],
  B: [],
  C: [],
  D: [],
  E: [],
  "*": [],
} as const;

export const PriorityBundleModifierMapSchema = z.preprocess(
  (val) => {
    if (typeof val !== "object" || val === null) return defaultModifiers;
    return { ...defaultModifiers, ...(val as Record<string, any>) };
  },
  z.object({
    A: z.array(PriorityBundleModifierSchema),
    B: z.array(PriorityBundleModifierSchema),
    C: z.array(PriorityBundleModifierSchema),
    D: z.array(PriorityBundleModifierSchema),
    E: z.array(PriorityBundleModifierSchema),
    "*": z.array(PriorityBundleModifierSchema),
  }),
);

export const PriorityBundleSkillSchema = z.object({
  id: z.number().optional(),
  bundle_id: z.number().optional(),
  grade: PriorityGradeSchema,
  attribute: z.string(),
  amount: z.number(),
  rating: z.number(),
});

export const PriorityBundleMetatypeSchema = z.object({
  id: z.number().optional(),
  bundle_id: z.number().optional(),
  grade: PriorityGradeSchema,
  special_points: z.number(),
  name: z.string(),
});

export const PriorityBundleQualitySchema = z.object({
  id: z.number().optional(),
  bundle_id: z.number().optional(),
  grade: PriorityGradeSchema,
  name: z.string(),
});

export type PriorityBundle = {
  id?: number;
  name?: string;
  grade: PriorityGrade;
  menu_order?: number;
  parent_bundle_id?: number;
  modifiers: PriorityBundleModifierMap;
  skills: PriorityBundleSkill[];
  metatypes: PriorityBundleMetatype[];
  qualities: PriorityBundleQuality[];
  children: PriorityBundle[];
};

export const PriorityBundleSchema: z.ZodType<PriorityBundle> = z.lazy(() =>
  z.object({
    id: z.number().optional(),
    name: z.string().optional(),
    grade: PriorityGradeSchema,
    menu_order: z.number().optional(),
    parent_bundle_id: z.number().optional(),
    modifiers: PriorityBundleModifierMapSchema,
    skills: z.array(PriorityBundleSkillSchema),
    metatypes: z.array(PriorityBundleMetatypeSchema),
    qualities: z.array(PriorityBundleQualitySchema),
    children: z.array(PriorityBundleSchema),
  }),
);

export const PriorityBundleArraySchema = z.array(PriorityBundleSchema);

export type Operation = z.infer<typeof OperationSchema>;
export type PriorityGrade = z.infer<typeof PriorityGradeSchema>;
export type PriorityBundleModifier = z.infer<
  typeof PriorityBundleModifierSchema
>;
export type PriorityBundleModifierMap = z.infer<
  typeof PriorityBundleModifierMapSchema
>;
export type PriorityBundleSkill = z.infer<typeof PriorityBundleSkillSchema>;
export type PriorityBundleMetatype = z.infer<
  typeof PriorityBundleMetatypeSchema
>;
export type PriorityBundleQuality = z.infer<typeof PriorityBundleQualitySchema>;
export type PriorityBundleArray = z.infer<typeof PriorityBundleArraySchema>;

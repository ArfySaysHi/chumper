import { z } from "zod";

const OperationSchema = z.enum(["Add", "Sub", "Mul", "Div", "Set"]);
const PriorityGradeSchema = z.enum(["A", "B", "C", "D", "E", "*"]);

export const PriorityBundleModifierSchema = z.object({
  id: z.number(),
  bundle_id: z.number(),
  target_key: z.string(),
  operation: OperationSchema,
  value: z.string(),
  created_at: z.string().optional(),
  updated_at: z.string().optional(),
});

const defaultGradeArrays = {
  A: [],
  B: [],
  C: [],
  D: [],
  E: [],
  "*": [],
} as const;

export const PriorityBundleModifierMapSchema = z.preprocess(
  (val) => {
    if (typeof val !== "object" || val === null) return defaultGradeArrays;
    return { ...defaultGradeArrays, ...(val as Record<string, any>) };
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
  id: z.number(),
  bundle_id: z.number(),
  attribute: z.string(),
  amount: z.number(),
  rating: z.number(),
});

export const PriorityBundleSkillMapSchema = z.preprocess(
  (val) => {
    if (typeof val !== "object" || val === null) return defaultGradeArrays;
    return { ...defaultGradeArrays, ...(val as Record<string, any>) };
  },
  z.object({
    A: z.array(PriorityBundleSkillSchema),
    B: z.array(PriorityBundleSkillSchema),
    C: z.array(PriorityBundleSkillSchema),
    D: z.array(PriorityBundleSkillSchema),
    E: z.array(PriorityBundleSkillSchema),
    "*": z.array(PriorityBundleSkillSchema),
  }),
);

export const PriorityBundleMetatypeSchema = z.object({
  id: z.number(),
  bundle_id: z.number(),
  special_points: z.number(),
  name: z.string(),
});

export const PriorityBundleMetatypeMapSchema = z.preprocess(
  (val) => {
    if (typeof val !== "object" || val === null) return defaultGradeArrays;
    return { ...defaultGradeArrays, ...(val as Record<string, any>) };
  },
  z.object({
    A: z.array(PriorityBundleMetatypeSchema),
    B: z.array(PriorityBundleMetatypeSchema),
    C: z.array(PriorityBundleMetatypeSchema),
    D: z.array(PriorityBundleMetatypeSchema),
    E: z.array(PriorityBundleMetatypeSchema),
    "*": z.array(PriorityBundleMetatypeSchema),
  }),
);

export const PriorityBundleQualitySchema = z.object({
  id: z.number(),
  bundle_id: z.number(),
  name: z.string(),
});

export const PriorityBundleQualityMapSchema = z.preprocess(
  (val) => {
    if (typeof val !== "object" || val === null) return defaultGradeArrays;
    return { ...defaultGradeArrays, ...(val as Record<string, any>) };
  },
  z.object({
    A: z.array(PriorityBundleQualitySchema),
    B: z.array(PriorityBundleQualitySchema),
    C: z.array(PriorityBundleQualitySchema),
    D: z.array(PriorityBundleQualitySchema),
    E: z.array(PriorityBundleQualitySchema),
    "*": z.array(PriorityBundleQualitySchema),
  }),
);

export type PriorityBundle = {
  id: number;
  name?: string;
  grade: PriorityGrade;
  parent_id?: number;
  modifiers: PriorityBundleModifier[];
  skills: PriorityBundleSkill[];
  metatypes: PriorityBundleMetatype[];
  qualities: PriorityBundleQuality[];
  options: PriorityBundle[];
};

export const PriorityBundleSchema: z.ZodType<PriorityBundle> = z.lazy(() =>
  z.object({
    id: z.number(),
    name: z.string().optional().default(""),
    grade: PriorityGradeSchema,
    menu_order: z.number().optional(),
    parent_bundle_id: z.number().optional(),
    modifiers: z.array(PriorityBundleModifierSchema),
    skills: z.array(PriorityBundleSkillSchema),
    metatypes: z.array(PriorityBundleMetatypeSchema),
    qualities: z.array(PriorityBundleQualitySchema),
    options: z.array(PriorityBundleSchema),
  }),
);

export const PriorityBundleMapSchema = z.preprocess(
  (val) => {
    if (typeof val !== "object" || val === null) return defaultGradeArrays;
    return { ...defaultGradeArrays, ...(val as Record<string, any>) };
  },
  z.object({
    A: z.array(PriorityBundleSchema),
    B: z.array(PriorityBundleSchema),
    C: z.array(PriorityBundleSchema),
    D: z.array(PriorityBundleSchema),
    E: z.array(PriorityBundleSchema),
    "*": z.array(PriorityBundleSchema),
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
export type PriorityBundleSkillMap = z.infer<
  typeof PriorityBundleSkillMapSchema
>;

export type PriorityBundleMetatype = z.infer<
  typeof PriorityBundleMetatypeSchema
>;
export type PriorityBundleMetatypeMap = z.infer<
  typeof PriorityBundleMetatypeMapSchema
>;

export type PriorityBundleQuality = z.infer<typeof PriorityBundleQualitySchema>;
export type PriorityBundleQualityMap = z.infer<
  typeof PriorityBundleQualityMapSchema
>;

export type PriorityBundleMap = z.infer<typeof PriorityBundleMapSchema>;
export type PriorityBundleArray = z.infer<typeof PriorityBundleArraySchema>;

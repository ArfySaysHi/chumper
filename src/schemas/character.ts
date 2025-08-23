import { z } from "zod";

export const CharacterSchema = z.object({
    id: z.number().optional(),
    name: z.string().min(1, "Character name is required"),
    metatype: z.string(),
    player_name: z.string().optional(),
    created_at: z.string().optional(),
    updated_at: z.string().optional(),
    status: z.enum(["Creation", "Active", "Archived"]),
});

export const CharacterSummarySchema = z.object({
    id: z.number().optional(),
    name: z.string().min(1, "Character name is required"),
    metatype: z.string(),
    player_name: z.string().nullable(),
    created_at: z.string().optional(),
    updated_at: z.string().optional(),
    status: z.enum(["Creation", "Active", "Archived"]),
});

export const CharacterArraySchema = z.array(CharacterSchema);
export const CharacterSummaryArraySchema = z.array(CharacterSummarySchema);

export type Character = z.infer<typeof CharacterSchema>;
export type CharacterArray = z.infer<typeof CharacterArraySchema>;
export type CharacterSummary = z.infer<typeof CharacterSummarySchema>;
export type CharacterSummaryArray = z.infer<typeof CharacterSummaryArraySchema>;
export type CharacterStatus = "Creation" | "Active" | "Archived";

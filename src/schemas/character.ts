import { z } from "zod";

export const CharacterSchema = z.object({
    id: z.number().optional(),
    name: z.string().min(1, "Character name is required"),
    metatype: z.string(),
    player_name: z.string().optional(),

    body: z.number().min(1),
    agility: z.number().min(1),
    reaction: z.number().min(1),
    strength: z.number().min(1),
    willpower: z.number().min(1),
    logic: z.number().min(1),
    intuition: z.number().min(1),
    charisma: z.number().min(1),

    edge: z.number().min(1).max(7),
    magic: z.number().min(1).optional(),
    resonance: z.number().min(1).optional(),

    karma_total: z.number().min(0).default(0),
    nuyen: z.number().min(0).default(0),
    created_at: z.string().optional(),
    updated_at: z.string().optional(),
    status: z.enum(["Creation", "Active", "Archived"]),
});

export const CharacterSummarySchema = z.object({
    id: z.number().optional(),
    name: z.string().min(1, "Character name is required"),
    metatype: z.string(),
    player_name: z.string().nullable(),
    karma_total: z.number().min(0).default(0),
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

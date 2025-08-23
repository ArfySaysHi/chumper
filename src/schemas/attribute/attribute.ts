import { z } from "zod";

export const AttributeSchema = z.object({
    body: z.number().min(1),
    agility: z.number().min(1),
    reaction: z.number().min(1),
    strength: z.number().min(1),
    willpower: z.number().min(1),
    logic: z.number().min(1),
    intuition: z.number().min(1),
    charisma: z.number().min(1),
    edge: z.number().min(1),
    magic: z.number().min(1).optional(),
    resonance: z.number().min(1).optional(),
});

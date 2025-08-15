export type PriorityCategory =
  | "Metatype"
  | "Attributes"
  | "Magic/Resonance"
  | "Skills"
  | "Resources";

export type Archetype =
  | "Street Samurai"
  | "Decker"
  | "Mage"
  | "Rigger"
  | "Adept"
  | "Face";

export interface PriorityList {
  A: PriorityCategory;
  B: PriorityCategory;
  C: PriorityCategory;
  D: PriorityCategory;
  E: PriorityCategory;
}

export interface Character {
  id: number;
  name: string;
  archetype: Archetype; // TODO: Change to tags later
  karma: number;
  nuyen: number;
  lastPlayed: string;
  avatar: string;
  priority: PriorityList;
}

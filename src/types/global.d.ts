declare namespace JSX {
  interface IntrinsicElements {
    "ion-icon": {
      name?: string;
      src?: string;
      className?: string;
      ios?: string;
      md?: string;
      "aria-hidden"?: string;
      "aria-label"?: string;
    };
  }
}

export enum PriorityOption {
  met = "Metatype",
  att = "Attributes",
  mag = "Magic/Resonance",
  ski = "Skills",
  res = "Resources",
}

export enum ArchetypeOption {
  sam = "Street Samurai",
  decker = "Decker",
  mage = "Mage",
  rigger = "Rigger",
  adept = "Adept",
  face = "Face",
}

interface PriorityList {
  A: PriorityOption;
  B: PriorityOption;
  C: PriorityOption;
  D: PriorityOption;
  E: PriorityOption;
}

// TODO: Keep up to date with backend architecture
export interface Character {
  id: number;
  name: string;
  archetype: ArchetypeOption; // TODO: Change to tags later
  karma: number;
  nuyen: number;
  lastPlayed: string;
  avatar: string;
  priority: PriorityList;
}

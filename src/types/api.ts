import type { Character } from "./character.ts";

export interface TauriCommands {
  get_characters: () => Promise<Character[]>; // Add error state
}

declare global {
  interface Window {
    __TAURI__: {
      invoke: <T extends keyof TauriCommands>(
        cmd: T,
        args?: Parameters<TauriCommands[T]>[0],
      ) => ReturnType<TauriCommands[T]>;
    };
  }
}

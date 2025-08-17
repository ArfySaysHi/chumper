import { useCallback, useState, useEffect, useRef } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen, UnlistenFn } from "@tauri-apps/api/event";
import {
  CharacterArray,
  CharacterArraySchema,
  CharacterStatus,
  Character,
} from "../schemas/character.ts";

interface UseCharactersOptions {
  status?: CharacterStatus;
  enableRealtimeUpdates?: boolean;
}

interface UseCharactersReturn {
  characters: CharacterArray;
  loading: boolean;
  error: string | null;
  refetch: () => Promise<void>;
  isEmpty: boolean;
  count: number;
}

const useCharacters = ({
  status,
  enableRealtimeUpdates = true,
}: UseCharactersOptions = {}): UseCharactersReturn => {
  const [characters, setCharacters] = useState<CharacterArray>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const isMountedRef = useRef(true);
  const unlistenFns = useRef<UnlistenFn[]>([]);

  const listCharacters = useCallback(async (showLoading = true) => {
    try {
      if (showLoading && isMountedRef.current) {
        setLoading(true);
        setError(null);
      }

      const endpoint = status ? "list_characters_by_status" : "list_characters";
      const params = status ? { status } : undefined;

      const data = await invoke(endpoint, params);
      const validatedData = CharacterArraySchema.parse(data);

      if (isMountedRef.current) {
        setCharacters(validatedData);
        setError(null);
      }
    } catch (e) {
      console.error(`useCharacters: ${e}`);

      if (e instanceof Error) setError(e.message);
      else setError("Failed to load characters");
    } finally {
      if (isMountedRef.current && showLoading) setLoading(false);
    }
  }, []);

  useEffect(() => {
    if (!enableRealtimeUpdates) return;

    const realtimeUpdates = async () => {
      const unlistenCreated = await listen<{
        id: number;
        character: Character;
      }>("character_created", () => console.log("CREATED A CHARACTER"));

      unlistenFns.current = [unlistenCreated];
    };

    realtimeUpdates();

    return () => {
      unlistenFns.current.forEach((unlisten) => unlisten());
      unlistenFns.current = [];
    };
  }, []);

  const refetch = useCallback(async () => {
    await listCharacters(true);
  }, [listCharacters]);

  useEffect(() => {
    listCharacters();
  }, []);

  return {
    characters,
    loading,
    error,
    refetch,
    isEmpty: characters.length === 0,
    count: characters.length,
  };
};

export default useCharacters;

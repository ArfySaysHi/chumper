import { useCallback, useState, useEffect, useRef } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen, UnlistenFn } from "@tauri-apps/api/event";
import {
  CharacterStatus,
  CharacterSummaryArray,
  CharacterSummaryArraySchema,
  CharacterSummary,
} from "../schemas/character.ts";

interface UseCharactersOptions {
  status?: CharacterStatus;
  enableRealtimeUpdates?: boolean;
}

interface UseCharactersReturn {
  characters: CharacterSummaryArray;
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
  const [characters, setCharacters] = useState<CharacterSummaryArray>([]);
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
      const validatedData = CharacterSummaryArraySchema.parse(data);

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

    const setupRealtimeUpdates = async () => {
      try {
        const unlistenCreated = await listen<CharacterSummary>(
          "character_created",
          async () => await refetch(),
        );

        unlistenFns.current = [unlistenCreated];
      } catch (error) {
        console.error("Failed to set up event listeners:", error);
      }
    };

    setupRealtimeUpdates();

    // Cleanup function
    return () => {
      unlistenFns.current.forEach((unlisten) => {
        try {
          unlisten();
        } catch (error) {
          console.error("Error cleaning up listener:", error);
        }
      });
      unlistenFns.current = [];
    };
  }, [enableRealtimeUpdates]); // Only re-run if enableRealtimeUpdates changes

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

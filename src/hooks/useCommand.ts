import { useState, useCallback } from "react";
import { invoke } from "@tauri-apps/api/core";

const useCommand = <T>(command: string) => {
    const [isLoading, setIsLoading] = useState(false);

    const execute = useCallback(
        async (args: any): Promise<T> => {
            if (isLoading) throw new Error("Command already running");

            setIsLoading(true);
            try {
                return await invoke<T>(command, args);
            } finally {
                setIsLoading(false);
            }
        },
        [command, isLoading],
    );

    return { execute, isLoading };
};

export default useCommand;

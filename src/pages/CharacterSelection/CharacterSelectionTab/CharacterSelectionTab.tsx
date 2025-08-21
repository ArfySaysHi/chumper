import { useState, MouseEvent, useEffect } from "react";
import useCharacters from "../../../hooks/useCharacters";
import CharacterCard from "../../../components/ui/CharacterCard/CharacterCard.tsx";
import { CharacterStatus, CharacterSummary } from "../../../schemas/character";
import Loading from "../../../components/ui/Loading/Loading.tsx";
import { AnimatePresence, motion } from "framer-motion";
import useCommand from "../../../hooks/useCommand.ts";
import "./CharacterSelectionTab.scss";

interface CharacterSelectionTabProps {
  status: CharacterStatus;
}
const enter = { opacity: 1, y: 0 };
const exit = { opacity: 0, y: -8 };
const initial = { opacity: 0, y: 8 };

const CharacterSelectionTab = ({ status }: CharacterSelectionTabProps) => {
  const [selected, setSelected] = useState<CharacterSummary[]>([]);

  const { characters, loading } = useCharacters({ status });
  const { execute: deleteCharacter } = useCommand("delete_character");
  const { execute: archiveCharacter } = useCommand("archive_character");

  const listVariants = {
    animate: { transition: { staggerChildren: 0.04 } },
    exit: {},
  };

  const onClick = (e: MouseEvent, cs: CharacterSummary) => {
    e.preventDefault();
    if (e.shiftKey) setSelected((prev) => [...prev, cs]);
    else setSelected([cs]);
  };

  const deleteSingle = (cs: CharacterSummary) =>
    cs.status === "Archived"
      ? deleteCharacter({ id: cs.id })
      : archiveCharacter({ id: cs.id });

  const deleteSelected = () => selected.forEach((cs) => deleteSingle(cs));

  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key !== "Delete") return;
      deleteSelected();
    };

    document.addEventListener("keydown", handleKeyDown);

    return () => {
      document.removeEventListener("keydown", handleKeyDown);
    };
  }, [selected]);

  return (
    <section className="character-selection-tab">
      <AnimatePresence mode="wait" initial={false}>
        {loading ? (
          <motion.div
            key="loading"
            initial={initial}
            animate={enter}
            exit={exit}
            transition={{ duration: 0.3, ease: "easeOut" }}
            className="character-selection-tab__grid"
          >
            <Loading />
          </motion.div>
        ) : (
          <motion.div
            key="grid"
            initial={initial}
            animate={enter}
            exit={exit}
            transition={{ duration: 0.3, ease: "easeOut" }}
            layout
          >
            <motion.div
              className="character-selection-tab__grid"
              variants={listVariants}
              initial="initial"
              animate="animate"
              exit="exit"
            >
              {characters.map((character) => (
                <CharacterCard
                  key={character.id}
                  character={character}
                  onClick={onClick}
                  onDoubleClick={() =>
                    console.log("Load character:", character.name)
                  }
                  onDelete={() => deleteSingle(character)}
                  selected={selected.includes(character)}
                />
              ))}
            </motion.div>
          </motion.div>
        )}
      </AnimatePresence>
    </section>
  );
};

export default CharacterSelectionTab;

import useCharacters from "../../../hooks/useCharacters";
import CharacterCard from "../../../components/ui/CharacterCard/CharacterCard.tsx";
import { CharacterStatus } from "../../../schemas/character";
import Loading from "../../../components/ui/Loading/Loading.tsx";
import { Variants, Transition, AnimatePresence, motion } from "framer-motion";

interface CharacterSelectionTabProps {
  status: CharacterStatus;
}
const enter = { opacity: 1, y: 0 };
const exit = { opacity: 0, y: -8 };
const initial = { opacity: 0, y: 8 };

const itemTransition: Transition = { duration: 0.3, ease: "easeOut" };
export const itemVariants: Variants = {
  initial: { opacity: 0, y: 8, scale: 0.995 },
  animate: {
    opacity: 1,
    y: 0,
    scale: 1,
    transition: itemTransition,
  },
  exit: { opacity: 0, y: -8, transition: { duration: 0.2 } },
};

const CharacterSelectionTab = ({ status }: CharacterSelectionTabProps) => {
  const { characters, loading } = useCharacters({ status });

  const listVariants = {
    animate: { transition: { staggerChildren: 0.04 } },
    exit: {},
  };

  return (
    <section className="character-selection__characters">
      <AnimatePresence mode="wait" initial={false}>
        {loading ? (
          <motion.div
            key="loading"
            initial={initial}
            animate={enter}
            exit={exit}
            transition={{ duration: 0.3, ease: "easeOut" }}
            className="character-grid"
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
              className="character-grid"
              variants={listVariants}
              initial="initial"
              animate="animate"
              exit="exit"
            >
              {characters.map((character) => (
                <CharacterCard
                  key={character.id}
                  character={character}
                  onClick={() => {}}
                  onDoubleClick={() =>
                    console.log("Load character:", character.name)
                  }
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

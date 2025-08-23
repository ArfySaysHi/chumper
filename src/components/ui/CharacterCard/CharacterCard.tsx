import { SyntheticEvent, MouseEvent } from "react";
import { Delete, Spa, CalendarToday, CurrencyYen } from "@mui/icons-material";
import Button from "../Button/Button.tsx";
import "./CharacterCard.scss";
import { CharacterSummary } from "../../../schemas/character.ts";
import { motion, Transition, Variants } from "framer-motion";

interface CharacterCardProps {
  character: CharacterSummary;
  onClick: (e: MouseEvent, cs: CharacterSummary) => void;
  onDoubleClick: (e: SyntheticEvent) => void;
  onDelete: (e: SyntheticEvent) => void;
  selected?: boolean;
}

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

const CharacterCard = ({
  character,
  onClick,
  onDoubleClick,
  onDelete,
  selected = false,
}: CharacterCardProps) => {
  return (
    <motion.div
      key={character.id}
      className={`character-card ${selected ? "character-card--selected" : ""}`}
      onClick={(e) => onClick(e, character)}
      onDoubleClick={onDoubleClick}
      variants={itemVariants}
      initial="initial"
      animate="animate"
      exit="exit"
      layout
    >
      <div className="character-card__header">
        <div className="character-card__avatar">A</div>
        <div className="character-card__info">
          <h3 className="character-card__name">{character.name}</h3>
          <span className={`character-card__archetype`}></span>
        </div>
        <div className="character-card__actions">
          <Button variant="icon" size="xs" title="Delete" destructive>
            <Delete onClick={onDelete} />
          </Button>
        </div>
      </div>

      <div className="character-card__stats">
        <div className="stat-item">
          <Spa className="stat-item__icon stat-item__icon--karma" />
          <span className="stat-item__label">Karma</span>
          <span className="stat-item__value text-karma">0</span>
        </div>
        <div className="stat-item">
          <CurrencyYen className="stat-item__icon stat-item__icon--nuyen" />
          <span className="stat-item__label">Nuyen</span>
          <span className="stat-item__value text-nuyen">0</span>
        </div>
      </div>

      <div className="character-card__footer">
        <div className="character-card__last-played">
          <CalendarToday className="character-card__calendar-icon" />
          <span>Last played: Date Here</span>
        </div>
      </div>
    </motion.div>
  );
};

export default CharacterCard;

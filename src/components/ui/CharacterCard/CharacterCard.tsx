import { SyntheticEvent } from "react";
import { Delete, Spa, CalendarToday, CurrencyYen } from "@mui/icons-material";
import Button from "../Button/Button.tsx";
import "./CharacterCard.scss";
import { CharacterSummary } from "../../../schemas/character.ts";

interface CharacterCardProps {
  character: CharacterSummary;
  onClick: (e: SyntheticEvent) => void;
  onDoubleClick: (e: SyntheticEvent) => void;
  selected?: boolean;
}

const CharacterCard = ({
  character,
  onClick,
  onDoubleClick,
  selected = false,
}: CharacterCardProps) => {
  return (
    <div
      key={character.id}
      className={`character-card ${selected ? "character-card--selected" : ""}`}
      onClick={onClick}
      onDoubleClick={onDoubleClick}
    >
      <div className="character-card__header">
        <div className="character-card__avatar">A</div>
        <div className="character-card__info">
          <h3 className="character-card__name">{character.name}</h3>
          <span className={`character-card__archetype`}></span>
        </div>
        <div className="character-card__actions">
          <Button variant="icon" size="xs" title="Delete" destructive>
            <Delete />
          </Button>
        </div>
      </div>

      <div className="character-card__stats">
        <div className="stat-item">
          <Spa className="stat-item__icon stat-item__icon--karma" />
          <span className="stat-item__label">Karma</span>
          <span className="stat-item__value text-karma">
            {character.karma_total}
          </span>
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
    </div>
  );
};

export default CharacterCard;

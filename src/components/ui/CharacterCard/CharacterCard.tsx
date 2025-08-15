import { SyntheticEvent } from "react";
import { Delete, Spa, CalendarToday, CurrencyYen } from "@mui/icons-material";
import Button from "../Button/Button.tsx";
import "./CharacterCard.scss";
import { Character, ArchetypeOption } from "../../../types/global.d.ts";

interface CharacterCardProps {
  character: Character;
  onClick: (e: SyntheticEvent) => void;
  onDoubleClick: (e: SyntheticEvent) => void;
  selected?: boolean;
}

interface ArchetypeMap {
  [k: string]: string;
}

const CharacterCard = ({
  character,
  onClick,
  onDoubleClick,
  selected = false,
}: CharacterCardProps) => {
  // TODO: Move to formatter file later
  const formatDate = (dateString: string) => {
    return new Date(dateString).toLocaleDateString("en-US", {
      month: "short",
      day: "numeric",
      year: "numeric",
    });
  };

  const getArchetypeClass = (archetype: ArchetypeOption) => {
    const archetypeMap: ArchetypeMap = {
      Decker: "archetype--decker",
      "Street Samurai": "archetype--samurai",
      Mage: "archetype--mage",
      Rigger: "archetype--rigger",
      Adept: "archetype--adept",
      Face: "archetype--face",
    };

    return archetypeMap[archetype] || "archetype--default";
  };

  return (
    <div
      key={character.id}
      className={`character-card ${selected ? "character-card--selected" : ""}`}
      onClick={onClick}
      onDoubleClick={onDoubleClick}
    >
      <div className="character-card__header">
        <div className="character-card__avatar">{character.avatar}</div>
        <div className="character-card__info">
          <h3 className="character-card__name">{character.name}</h3>
          <span
            className={`character-card__archetype ${getArchetypeClass(character.archetype)}`}
          >
            {character.archetype}
          </span>
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
          <span className="stat-item__value text-karma">{character.karma}</span>
        </div>
        <div className="stat-item">
          <CurrencyYen className="stat-item__icon stat-item__icon--nuyen" />
          <span className="stat-item__label">Nuyen</span>
          <span className="stat-item__value text-nuyen">
            {character.nuyen.toLocaleString()}
          </span>
        </div>
      </div>

      <div className="character-card__priority">
        <span className="character-card__priority-label">Priority:</span>
        <div className="priority-badges">
          {Object.entries(character.priority).map(([priority]) => (
            <span
              key={priority}
              className={`priority-badge priority-badge--${priority.toLowerCase()}`}
            >
              {priority}
            </span>
          ))}
        </div>
      </div>

      <div className="character-card__footer">
        <div className="character-card__last-played">
          <CalendarToday className="character-card__calendar-icon" />
          <span>Last played: {formatDate(character.lastPlayed)}</span>
        </div>
      </div>
    </div>
  );
};

export default CharacterCard;

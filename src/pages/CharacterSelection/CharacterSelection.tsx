import { useState } from "react";
import {
  Add as Plus,
  GpsFixed as Target,
  Bolt as Zap,
  Spa,
  Download,
  Upload,
  Settings,
} from "@mui/icons-material";
import "./CharacterSelection.scss";
import CreateCard from "../../components/ui/CreateCard/CreateCard.tsx";
import CharacterCard from "../../components/ui/CharacterCard/CharacterCard.tsx";
import Button from "../../components/ui/Button/Button.tsx";
import { ArchetypeOption, PriorityOption } from "../../types/global.d.ts";
import TabGroup from "../../components/ui/TabGroup/TabGroup.tsx";

// Mock data - replace with your actual data layer
const mockCharacters: Character[] = [
  {
    id: 1,
    name: "Nyx Blackwire",
    archetype: ArchetypeOption.decker,
    karma: 45,
    nuyen: 12500,
    lastPlayed: "2024-08-10",
    avatar: "🔌",
    priority: {
      A: PriorityOption.ski,
      B: PriorityOption.att,
      C: PriorityOption.mag,
      D: PriorityOption.met,
      E: PriorityOption.res,
    },
  },
  {
    id: 2,
    name: "Razor Chrome",
    archetype: ArchetypeOption.sam,
    karma: 28,
    nuyen: 3200,
    lastPlayed: "2024-08-12",
    avatar: "⚔️",
    priority: {
      A: PriorityOption.att,
      B: PriorityOption.ski,
      C: PriorityOption.res,
      D: PriorityOption.met,
      E: PriorityOption.mag,
    },
  },
  {
    id: 3,
    name: "Echo Mindbridge",
    archetype: ArchetypeOption.mage,
    karma: 67,
    nuyen: 8900,
    lastPlayed: "2024-08-09",
    avatar: "🔮",
    priority: {
      A: PriorityOption.mag,
      B: PriorityOption.att,
      C: PriorityOption.ski,
      D: PriorityOption.met,
      E: PriorityOption.res,
    },
  },
  {
    id: 4,
    name: "Booga Mindbridge",
    archetype: ArchetypeOption.decker,
    karma: 67,
    nuyen: 8900,
    lastPlayed: "2024-08-09",
    avatar: "🔮",
    priority: {
      A: PriorityOption.mag,
      B: PriorityOption.att,
      C: PriorityOption.ski,
      D: PriorityOption.met,
      E: PriorityOption.res,
    },
  },
  {
    id: 5,
    name: "Booga Mindbridge",
    archetype: ArchetypeOption.decker,
    karma: 67,
    nuyen: 8900,
    lastPlayed: "2024-08-09",
    avatar: "🔮",
    priority: {
      A: PriorityOption.mag,
      B: PriorityOption.att,
      C: PriorityOption.ski,
      D: PriorityOption.met,
      E: PriorityOption.res,
    },
  },
];

const CharacterSelection = () => {
  const [characters, _setCharacters] = useState(mockCharacters);
  const [selectedCharacter, setSelectedCharacter] = useState(mockCharacters[0]);

  return (
    <div className="character-selection">
      <header className="character-selection__header">
        <div className="character-selection__brand">
          <h1 className="character-selection__title">Characters</h1>
          <span className="character-selection__version">v1.0.0-alpha</span>
        </div>
        <div className="character-selection__toolbar">
          <Button variant="icon" size="xs" title="Settings">
            <Settings />
          </Button>
          <Button variant="icon" size="xs" title="Import Character">
            <Upload />
          </Button>
          <Button variant="icon" size="xs" title="Export Character">
            <Download />
          </Button>
        </div>
      </header>

      <main className="character-selection__main container">
        {/* <section className="character-selection__create-section">
            <h2 className="section-header">Initialize New Runner</h2>
            <div className="grid grid-cols-3 gap-4">
            <CreateCard onClick={() => {}}>
            <Plus className="create-card__icon" />
            </CreateCard>
            <CreateCard
            variant="disabled"
            title="Sum to Ten"
            subtitle="Coming Soon"
            onClick={() => {}}
            >
            <Target className="create-card__icon" />
            </CreateCard>
            <CreateCard
            variant="disabled"
            title="Karma Generation"
            subtitle="Coming Soon"
            onClick={() => {}}
            >
            <Spa className="create-card__icon" />
            </CreateCard>
            </div>
            </section>
          */}
        <TabGroup
          tabs={[
            {
              key: "tab1",
              label: "Active Runners",
              content: (
                <section className="character-selection__characters">
                  <div className="character-selection__characters-header">
                    <h2 className="section-header">
                      Active Runners ({characters.length})
                    </h2>
                    <p className="character-selection__instructions">
                      Click to select • Double-click to load
                    </p>
                  </div>

                  <div className="character-grid">
                    {characters.map((character) => (
                      <CharacterCard
                        key={character.id}
                        character={character}
                        selected={selectedCharacter?.id === character.id}
                        onClick={() => {}}
                        onDoubleClick={() =>
                          console.log("Load character:", character.name)
                        }
                      />
                    ))}
                  </div>
                </section>
              ),
            },
            {
              key: "tab2",
              label: "Soon-to-be Runners",
              content: (
                <section className="character-selection__characters">
                  <div className="character-selection__characters-header">
                    <h2 className="section-header">
                      Soon-to-be Runners ({characters.length})
                    </h2>
                    <p className="character-selection__instructions">
                      Click to select • Double-click to load
                    </p>
                  </div>

                  <div className="character-grid">
                    {characters.map((character) => (
                      <CharacterCard
                        key={character.id}
                        character={character}
                        selected={selectedCharacter?.id === character.id}
                        onClick={() => {}}
                        onDoubleClick={() =>
                          console.log("Load character:", character.name)
                        }
                      />
                    ))}
                  </div>
                </section>
              ),
            },
          ]}
        />
      </main>
    </div>
  );
};

export default CharacterSelection;

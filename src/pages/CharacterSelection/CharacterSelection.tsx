import { Download, Upload, Settings, Add } from "@mui/icons-material";
import "./CharacterSelection.scss";
import CharacterCard from "../../components/ui/CharacterCard/CharacterCard.tsx";
import Button from "../../components/ui/Button/Button.tsx";
import { ArchetypeOption, PriorityOption } from "../../types/global.d.ts";
import TabGroup from "../../components/ui/TabGroup/TabGroup.tsx";
import CreateCard from "../../components/ui/CreateCard/CreateCard.tsx";

const mockCreationCharacters: Character[] = [
  {
    id: 7,
    name: "Silas Quickstep",
    archetype: ArchetypeOption.mage,
    karma: 52,
    nuyen: 7400,
    lastPlayed: "2024-08-11",
    avatar: "🥷",
    priority: {
      A: PriorityOption.ski,
      B: PriorityOption.att,
      C: PriorityOption.res,
      D: PriorityOption.met,
      E: PriorityOption.mag,
    },
  },
  {
    id: 8,
    name: "Maris Shadowgrasp",
    archetype: ArchetypeOption.sam,
    karma: 41,
    nuyen: 9800,
    lastPlayed: "2024-08-14",
    avatar: "⚔️",
    priority: {
      A: PriorityOption.att,
      B: PriorityOption.res,
      C: PriorityOption.met,
      D: PriorityOption.ski,
      E: PriorityOption.mag,
    },
  },
];

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
        <TabGroup
          tabs={[
            {
              key: "tab1",
              label: "Active Runners",
              content: (
                <section className="character-selection__characters">
                  <div className="character-grid">
                    {mockCharacters.map((character, i) => (
                      <CharacterCard
                        key={character.id}
                        character={character}
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
                  <div className="character-grid">
                    {mockCreationCharacters.map((character, i) => (
                      <CharacterCard
                        key={character.id}
                        character={character}
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
        <div className="character-selection__creation_type">
          <TabGroup
            tabs={[
              {
                key: "cs-creation",
                label: "Initialize New Runner",
                content: (
                  <section className="character-selection__characters">
                    <div className="character-grid">
                      <CreateCard onClick={() => {}}>
                        <Add className="create-card__icon" />
                      </CreateCard>
                    </div>
                  </section>
                ),
              },
            ]}
          />
        </div>
      </main>
    </div>
  );
};

export default CharacterSelection;

import { Download, Upload, Settings } from "@mui/icons-material";
import "./CharacterSelection.scss";
import CharacterCard from "../../components/ui/CharacterCard/CharacterCard.tsx";
import Button from "../../components/ui/Button/Button.tsx";
import TabGroup from "../../components/ui/TabGroup/TabGroup.tsx";
import { Character } from "../../types/character.ts";
import { invoke } from "@tauri-apps/api/core";

const mockCreationCharacters: Character[] = [
  {
    id: 7,
    name: "Silas Quickstep",
    archetype: "Mage",
    karma: 52,
    nuyen: 7400,
    lastPlayed: "2024-08-11",
    avatar: "🥷",
    priority: {
      A: "Skills",
      B: "Attributes",
      C: "Resources",
      D: "Metatype",
      E: "Magic/Resonance",
    },
  },
  {
    id: 8,
    name: "Maris Shadowgrasp",
    archetype: "Street Samurai",
    karma: 41,
    nuyen: 9800,
    lastPlayed: "2024-08-14",
    avatar: "⚔️",
    priority: {
      A: "Attributes",
      B: "Resources",
      C: "Metatype",
      D: "Skills",
      E: "Magic/Resonance",
    },
  },
];

// Mock data - replace with your actual data layer
const mockCharacters: Character[] = [
  {
    id: 1,
    name: "Nyx Blackwire",
    archetype: "Decker",
    karma: 45,
    nuyen: 12500,
    lastPlayed: "2024-08-10",
    avatar: "🔌",
    priority: {
      A: "Skills",
      B: "Attributes",
      C: "Magic/Resonance",
      D: "Metatype",
      E: "Resources",
    },
  },
  {
    id: 2,
    name: "Razor Chrome",
    archetype: "Street Samurai",
    karma: 28,
    nuyen: 3200,
    lastPlayed: "2024-08-12",
    avatar: "⚔️",
    priority: {
      A: "Attributes",
      B: "Skills",
      C: "Resources",
      D: "Metatype",
      E: "Magic/Resonance",
    },
  },
  {
    id: 3,
    name: "Echo Mindbridge",
    archetype: "Mage",
    karma: 67,
    nuyen: 8900,
    lastPlayed: "2024-08-09",
    avatar: "🔮",
    priority: {
      A: "Magic/Resonance",
      B: "Attributes",
      C: "Skills",
      D: "Metatype",
      E: "Resources",
    },
  },
  {
    id: 4,
    name: "Booga Mindbridge",
    archetype: "Decker",
    karma: 67,
    nuyen: 8900,
    lastPlayed: "2024-08-09",
    avatar: "🔮",
    priority: {
      A: "Magic/Resonance",
      B: "Attributes",
      C: "Skills",
      D: "Metatype",
      E: "Resources",
    },
  },
  {
    id: 5,
    name: "Booga Mindbridge",
    archetype: "Decker",
    karma: 67,
    nuyen: 8900,
    lastPlayed: "2024-08-09",
    avatar: "🔮",
    priority: {
      A: "Magic/Resonance",
      B: "Attributes",
      C: "Skills",
      D: "Metatype",
      E: "Resources",
    },
  },
];

const CharacterSelection = () => {
  const char_fetch = async (command: string) => {
    try {
      const res = await invoke(command);
      console.log(res);
    } catch (err) {
      console.log(err);
    }
  };

  const makeGoHappen = async () => {
    try {
      const res = await invoke("create_character", {
        character: {
          name: "Ooga Booga",
          metatype: "Human",
          player_name: "Arfy",
          karma_total: 34,
        },
      });
      console.log(res);
    } catch (e) {
      console.error(e);
    }
  };

  return (
    <div className="character-selection">
      <header className="character-selection__header">
        <div className="character-selection__brand">
          <h1 className="character-selection__title">Characters</h1>
          <span className="character-selection__version">v1.0.0-alpha</span>
        </div>
        <div className="character-selection__toolbar">
          <Button
            onClick={makeGoHappen}
            variant="icon"
            size="xs"
            title="Settings"
          >
            <Settings />
          </Button>
          <Button
            onClick={() => char_fetch("list_characters")}
            variant="icon"
            size="xs"
            title="Import Character"
          >
            <Download />
          </Button>
          <Button variant="icon" size="xs" title="Export Character">
            <Upload />
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
                    {mockCharacters.map((character) => (
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
                    {mockCreationCharacters.map((character) => (
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
      </main>
    </div>
  );
};

export default CharacterSelection;

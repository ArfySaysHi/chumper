import { useState } from "react";
import { Download, Upload, Settings } from "@mui/icons-material";
import "./CharacterSelection.scss";
import CharacterCard from "../../components/ui/CharacterCard/CharacterCard.tsx";
import Button from "../../components/ui/Button/Button.tsx";
import TabGroup from "../../components/ui/TabGroup/TabGroup.tsx";
import useCharacters from "../../hooks/useCharacters.ts";
import useCommand from "../../hooks/useCommand.ts";

const CharacterSelection = () => {
  const { characters, loading } = useCharacters();
  const [counter, setCounter] = useState(0);

  const { execute: createCharacter, isLoading } =
    useCommand("create_character");

  const handleCreateChar = async () => {
    try {
      const res = await createCharacter({
        character: { name: `name_${counter}`, metatype: "Human" },
      });
      setCounter(counter + 1);
      console.log(res);
    } catch (e) {
      console.log(e);
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
            onClick={handleCreateChar}
            variant="icon"
            size="xs"
            title="Settings"
          >
            <Settings />
          </Button>
          <Button variant="icon" size="xs" title="Import Character">
            <Download />
          </Button>
          <Button variant="icon" size="xs" title="Export Character">
            <Upload />
          </Button>
        </div>
      </header>

      <main className="character-selection__main container">
        {!(loading || isLoading) ? (
          <TabGroup
            tabs={[
              {
                key: "tab1",
                label: "Active Runners",
                content: (
                  <section className="character-selection__characters">
                    <div className="character-grid">
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
                    </div>
                  </section>
                ),
              },
            ]}
          />
        ) : (
          <div>Loading...</div>
        )}
      </main>
    </div>
  );
};

export default CharacterSelection;

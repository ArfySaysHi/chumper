import { Download, Upload, Settings } from "@mui/icons-material";
import "./CharacterSelection.scss";
import CharacterCard from "../../components/ui/CharacterCard/CharacterCard.tsx";
import Button from "../../components/ui/Button/Button.tsx";
import TabGroup from "../../components/ui/TabGroup/TabGroup.tsx";
import useCharacters from "../../hooks/useCharacters.ts";
import { invoke } from "@tauri-apps/api/core";

const CharacterSelection = () => {
  const { characters, loading } = useCharacters();

  const initDb = async () => {
    try {
      await invoke("initialize_database");
    } catch (err) {
      console.error(err);
    }
  };

  // Need a yaml read import ugh
  const makeMetatype = async () => {
    try {
      const res = await invoke("import_metatypes", {
        path: "./core_data/metatypes.yaml",
      });
      console.log(res);
    } catch (e) {
      console.error(e);
    }
  };

  const listMetatypes = async () => {
    try {
      const res = await invoke("list_metatypes");
      console.log(res);
    } catch (e) {
      console.error(e);
    }
  };

  if (loading) return <div>Loading...</div>;

  return (
    <div className="character-selection">
      <header className="character-selection__header">
        <div className="character-selection__brand">
          <h1 className="character-selection__title">Characters</h1>
          <span className="character-selection__version">v1.0.0-alpha</span>
        </div>
        <div className="character-selection__toolbar">
          <Button onClick={initDb} variant="icon" size="xs" title="Settings">
            <Settings />
          </Button>
          <Button
            onClick={makeMetatype}
            variant="icon"
            size="xs"
            title="Import Character"
          >
            <Download />
          </Button>
          <Button
            onClick={listMetatypes}
            variant="icon"
            size="xs"
            title="Export Character"
          >
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
      </main>
    </div>
  );
};

export default CharacterSelection;

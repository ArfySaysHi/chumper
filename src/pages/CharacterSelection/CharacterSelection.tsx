import { useState } from "react";
import { Download, Upload, Settings } from "@mui/icons-material";
import "./CharacterSelection.scss";
import Button from "../../components/ui/Button/Button.tsx";
import TabGroup from "../../components/ui/TabGroup/TabGroup.tsx";
import useCommand from "../../hooks/useCommand.ts";
import CharacterSelectionTab from "./CharacterSelectionTab/CharacterSelectionTab.tsx";
import ModalContainer from "../../components/layout/ModalContainer.tsx";

const CharacterSelection = () => {
  const [counter, setCounter] = useState(0);
  const [modalVisible, setModalVisible] = useState(false);

  const { execute: createCharacter } = useCommand("create_character");

  const handleCreateChar = async () => {
    try {
      await createCharacter({
        character: { name: `name_${counter}`, metatype: "Human" },
      });
      setCounter(counter + 1);
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

      {modalVisible && (
        <ModalContainer onClose={() => setModalVisible(false)}>
          <div>Cool Content for modal wow</div>
        </ModalContainer>
      )}
      <button onClick={() => setModalVisible(true)}>Modal Toggle</button>
      <main className="character-selection__main container">
        <TabGroup
          tabs={[
            {
              key: "tab1",
              label: "Active Runners",
              content: <CharacterSelectionTab status="Active" />,
            },
            {
              key: "tab2",
              label: "Soon-to-be Runners",
              content: <CharacterSelectionTab status="Creation" />,
            },
            {
              key: "tab3",
              label: "Archived Runners",
              content: <CharacterSelectionTab status="Archived" />,
            },
          ]}
        />
      </main>
    </div>
  );
};

export default CharacterSelection;

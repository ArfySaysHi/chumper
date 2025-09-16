import { useState } from "react";
import { Download, Upload, Settings } from "@mui/icons-material";
import "./CharacterSelection.scss";
import Button from "../../components/ui/Button/Button.tsx";
import TabGroup from "../../components/ui/TabGroup/TabGroup.tsx";
import CharacterSelectionTab from "./CharacterSelectionTab/CharacterSelectionTab.tsx";
import { AnimatePresence } from "framer-motion";
import PriorityModal from "../../components/modals/PriorityModal/PriorityModal.tsx";

const CharacterSelection = () => {
  const [priorityModalVisible, setPriorityModalVisible] = useState(true);

  return (
    <div className="character-selection">
      <AnimatePresence>
        {priorityModalVisible && (
          <PriorityModal onClose={() => setPriorityModalVisible(false)} />
        )}
      </AnimatePresence>
      <header className="character-selection__header">
        <div className="character-selection__brand">
          <h1 className="character-selection__title">Characters</h1>
          <span className="character-selection__version">v1.0.0-alpha</span>
        </div>
        <div className="character-selection__toolbar">
          <Button
            onClick={() => setPriorityModalVisible(true)}
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

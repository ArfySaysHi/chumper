import { ReactNode, useState, useRef, useEffect } from "react";
import "./TabGroup.scss";
import { AnimatePresence, motion } from "framer-motion";

interface Tab {
  key: string;
  label: string;
  content: ReactNode;
  disabled?: boolean;
}

interface TabGroupProps {
  tabs?: Tab[];
}

const defaultTabContent: Tab[] = [
  { key: "tab1", label: "Tab One", content: <div>Tab One</div> },
  { key: "tab2", label: "Tab Two", content: <div>Tab Two</div> },
];

const TabGroup = ({ tabs = defaultTabContent }: TabGroupProps) => {
  const [currentTab, setCurrentTab] = useState(0);
  const [sliderOffset, setSliderOffset] = useState({ width: 0, left: 0 });
  const tabRefs = useRef<(HTMLButtonElement | null)[]>([]);

  useEffect(() => {
    const tabElement = tabRefs.current[currentTab];
    if (tabElement) {
      const { offsetWidth, offsetLeft } = tabElement;
      setSliderOffset({ width: offsetWidth, left: offsetLeft });
    }
  }, [currentTab, tabs]);

  const adjustedSliderWidth = `${sliderOffset.width / 3}px`;
  const tab = tabs[currentTab];

  return (
    <div className="tab-group">
      <div className="tab-group__tabs">
        {tabs.map((tab, i) => (
          <button
            key={tab.key}
            tabIndex={tabs.length === 1 ? -1 : 1}
            ref={(el) => (tabRefs.current[i] = el)}
            className={`tab-group__tabs__tab tab-group__tabs__tab${currentTab === i ? "--active" : ""}`}
            onClick={() => setCurrentTab(i)}
          >
            {tab.label}
          </button>
        ))}
        <span
          className="tab-group__tabs__slider"
          style={{
            width: adjustedSliderWidth,
            transform: `translateX(${sliderOffset.left}px)`,
            left: adjustedSliderWidth,
          }}
        />
      </div>
      <div className="tab-group__content">
        <AnimatePresence mode="wait">
          <motion.div
            key={tab.key}
            initial={{ opacity: 0, y: 8 }}
            animate={{ opacity: 1, y: 0 }}
            exit={{ opacity: 0, y: -8 }}
            transition={{ duration: 0.3 }}
          >
            {tab.content}
          </motion.div>
        </AnimatePresence>
      </div>
    </div>
  );
};

export default TabGroup;

import { ReactNode, useState, useRef, useEffect } from "react";
import "./TabGroup.scss";

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
  {
    key: "tab3",
    label: "Tab Three",
    content: <div>Tab Three</div>,
    disabled: true,
  },
  { key: "tab4", label: "Tab Four", content: <div>Tab Four</div> },
  { key: "tab5", label: "Tab Five", content: <div>Tab Five</div> },
  { key: "tab6", label: "Tab Six", content: <div>Tab Six</div> },
  { key: "tab7", label: "Tab Seven", content: <div>Tab Seven</div> },
  {
    key: "tab8",
    label: "BLAHBLAHBLAHBLAHBLAHBLAHBLABH",
    content: <div>Tab Eight</div>,
  },
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

  return (
    <div className="tab-group">
      <div className="tab-group__tabs">
        {tabs.map((tab, i) => (
          <button
            key={tab.key}
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
        {tabs.map((tab, i) => (
          <div
            key={tab.key}
            role="tabpanel"
            aria-labelledby={`tab-${tab.key}`}
            className={`tab-group__content__container${currentTab !== i ? "--hidden" : ""}`}
          >
            {tab.content}
          </div>
        ))}
      </div>
    </div>
  );
};

export default TabGroup;

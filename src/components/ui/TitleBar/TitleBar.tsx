import { SyntheticEvent } from "react";
import "./TitleBar.scss";
import { Close, CropSquare, Remove } from "@mui/icons-material";
import { getCurrentWindow } from "@tauri-apps/api/window";

const TitleBar = () => {
  const appWindow = getCurrentWindow();

  const onMinimizeClick = (e: SyntheticEvent) => {
    e.preventDefault();
    appWindow.minimize();
  };

  const onMaximizeClick = (e: SyntheticEvent) => {
    e.preventDefault();
    appWindow.toggleMaximize();
  };

  const onCloseClick = (e: SyntheticEvent) => {
    e.preventDefault();
    appWindow.close();
  };

  return (
    <div className="title-bar">
      <div className="title-bar__menu"></div>
      <div className="title-bar__window-controls">
        <button
          onClick={onMinimizeClick}
          className="title-bar__window-controls__button title-bar__window-controls__button--primary"
          tabIndex={-1}
        >
          <Remove />
        </button>
        <button
          onClick={onMaximizeClick}
          className="title-bar__window-controls__button title-bar__window-controls__button--primary"
          tabIndex={-1}
        >
          <CropSquare />
        </button>
        <button
          onClick={onCloseClick}
          className="title-bar__window-controls__button title-bar__window-controls__button--primary"
          tabIndex={-1}
        >
          <Close />
        </button>
      </div>
    </div>
  );
};

export default TitleBar;

import { SyntheticEvent } from "react";
import "./TitleBar.scss";
import IconButton from "../IconButton/IconButton.tsx";
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
        <IconButton onClick={onMinimizeClick}>
          <Remove />
        </IconButton>
        <IconButton onClick={onMaximizeClick}>
          <CropSquare />
        </IconButton>
        <IconButton onClick={onCloseClick}>
          <Close />
        </IconButton>
      </div>
    </div>
  );
};

export default TitleBar;

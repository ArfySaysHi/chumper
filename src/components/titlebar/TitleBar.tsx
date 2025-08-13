import { SyntheticEvent } from "react";
import "./TitleBar.scss";
import IconButton from "../iconbutton/IconButton.tsx";
import { Close, CropSquare, Remove, Menu } from "@mui/icons-material";
import { getCurrentWindow } from "@tauri-apps/api/window";

const TitleBar = () => {
  const appWindow = getCurrentWindow();

  const onMenuClick = (e: SyntheticEvent) => {
    e.preventDefault();
    console.info("TODO: Menu implementation");
  };

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
      <div className="title-bar__menu">
        <IconButton onClick={() => {}}>
          <Menu onClick={onMenuClick} />
        </IconButton>
      </div>
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

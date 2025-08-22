import "./App.scss";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { useState, useEffect } from "react";
import TitleBar from "./components/ui/TitleBar/TitleBar.tsx";
import { RouterProvider } from "react-router";
import router from "./routes.tsx";
import ActiveElementDebugger from "./components/debug/ActiveElementDebugger/ActiveElementDebugger.tsx";

const App = () => {
  const [isReady, setIsReady] = useState(false);
  const appWindow = getCurrentWindow();

  useEffect(() => {
    const showWindow = async () => {
      try {
        await new Promise((res) => setTimeout(res, 100));
        await appWindow.show();

        setIsReady(true);
      } catch (err) {
        console.error("Error showing window:", err);
        setIsReady(true);
      }
    };
    showWindow();
  }, []);

  if (!isReady)
    return (
      <div className="anti-flash">
        <div className="loading-screen">
          <div className="spinner"></div>
        </div>
      </div>
    );

  return (
    <main className="app-container">
      <TitleBar />
      <div className="app-content">
        <RouterProvider router={router} />
      </div>
      <ActiveElementDebugger />
    </main>
  );
};

export default App;

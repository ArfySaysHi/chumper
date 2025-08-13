import "./App.scss";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { useState, useEffect } from "react";
import TitleBar from "./components/titlebar/TitleBar.tsx";
import { RouterProvider } from "react-router";
import router from "./routes.tsx";

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
    <main>
      <TitleBar />
      <RouterProvider router={router} />
    </main>
  );
};

export default App;

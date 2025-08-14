import { createBrowserRouter } from "react-router";
import CharacterSelection from "./pages/CharacterSelection/CharacterSelection.tsx";

const router = createBrowserRouter([
  {
    path: "/",
    element: <CharacterSelection />,
  },
]);

export default router;

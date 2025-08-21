import { createBrowserRouter } from "react-router";
import CharacterSelection from "./pages/CharacterSelection/CharacterSelection.tsx";
import CharacterCreation from "./pages/CharacterCreation/CharacterCreation.tsx";

const router = createBrowserRouter([
  {
    path: "/",
    element: <CharacterSelection />,
  },
  {
    path: "/creation/:id",
    element: <CharacterCreation />,
  },
]);

export default router;

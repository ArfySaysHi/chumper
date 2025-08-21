import "./CharacterCreation.scss";
import { useParams, useNavigate } from "react-router";

const CharacterCreation = () => {
  const navigate = useNavigate();
  const { id } = useParams();

  console.log("Character ID", id);
  return (
    <div>
      <div></div>
      <button
        onClick={() => navigate("/", { replace: true })}
        style={{ background: "white", marginTop: "20px" }}
      >
        Back
      </button>
    </div>
  );
};

export default CharacterCreation;

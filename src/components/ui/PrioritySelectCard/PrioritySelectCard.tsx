import "./PrioritySelectCard.scss";
import {
  Bolt,
  AccessibilityNew,
  Psychology,
  AutoAwesome,
  AccountBalance,
} from "@mui/icons-material";

interface PrioritySelectCardProps {
  title: string;
  icon?: string;
  selectedGrade?: string;
  onGradeSelect?: (grade: string) => void;
}

interface IconArray {
  [k: string]: React.ReactNode;
}

const icons: IconArray = {
  Attributes: <Bolt />,
  Metatype: <AccessibilityNew />,
  Skills: <Psychology />,
  "Magic/Resonance": <AutoAwesome />,
  Resources: <AccountBalance />,
};

const PrioritySelectCard = ({
  title = "Attributes",
  selectedGrade,
  onGradeSelect,
}: PrioritySelectCardProps) => {
  const grades = ["A", "B", "C", "D", "E"];

  return (
    <div className="priority-select-card">
      <div className="priority-select-card__icon">{icons[title]}</div>

      <div className="priority-select-card__info">
        <h3 className="priority-select-card__info__title">{title}</h3>
      </div>

      <div className="priority-select-card__options">
        {grades.map((grade) => (
          <button
            key={grade}
            className={`priority-select-card__options__grade ${
              selectedGrade === grade
                ? "priority-select-card__options__grade--selected"
                : ""
            }`}
            onClick={() => onGradeSelect?.(grade)}
          >
            {grade}
          </button>
        ))}
      </div>
    </div>
  );
};

export default PrioritySelectCard;

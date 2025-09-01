import "./PrioritySelectCard.scss";
import { PlusOne } from "@mui/icons-material";

interface PrioritySelectCardProps {
  title: string;
  description: string;
  icon?: React.ReactNode;
  selectedGrade?: string;
  onGradeSelect?: (grade: string) => void;
}

const PrioritySelectCard = ({
  title = "Attributes",
  description = "Physical and mental capabilities",
  icon = <PlusOne />,
  selectedGrade,
  onGradeSelect,
}: PrioritySelectCardProps) => {
  const grades = ["A", "B", "C", "D", "E"];

  return (
    <div className="priority-select-card">
      <div className="priority-select-card__icon">{icon}</div>

      <div className="priority-select-card__info">
        <h3 className="priority-select-card__info__title">{title}</h3>
        <p className="priority-select-card__info__description">{description}</p>
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

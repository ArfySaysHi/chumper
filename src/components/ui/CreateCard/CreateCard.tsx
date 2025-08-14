import { SyntheticEvent, ReactNode } from "react";
import "./CreateCard.scss";

interface CreateCardProps {
  onClick: (e: SyntheticEvent) => void;
  variant?: "primary" | "disabled";
  title?: string;
  subtitle?: string;
  children: ReactNode;
}

const CreateCard = ({
  onClick,
  variant = "primary",
  title = "Create Character",
  subtitle = "Priority System",
  children,
}: CreateCardProps) => (
  <button
    tabIndex={variant === "primary" ? 1 : -1}
    disabled={variant === "disabled"}
    onClick={onClick}
    className={`create-card create-card--${variant}`}
  >
    {children}
    <div className="create-card__content">
      <h3 className="create-card__title">{title}</h3>
      <p className="create-card__subtitle">{subtitle}</p>
    </div>
  </button>
);

export default CreateCard;

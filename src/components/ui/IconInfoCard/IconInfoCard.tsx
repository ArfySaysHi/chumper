import { ReactNode } from "react";
import "./IconInfoCard.scss";

interface IconInfoCard {
  title?: string;
  description?: string;
  onClick?: () => void;
  badge?: string;
  disabled?: boolean;
  children: ReactNode;
}

const IconInfoCard = ({
  title = "Info Title",
  description = "Info Description",
  onClick = () => {},
  disabled = false,
  children,
  badge,
}: IconInfoCard) => (
  <button className="icon-info-card" onClick={onClick} disabled={disabled}>
    <div className="icon-info-card__icon-wrap">
      <div className="icon-info-card__icon-wrap__icon">{children}</div>
      <div className="icon-info-card__info">
        <div className="icon-info-card__header">
          <h3 className="icon-info-card__title">{title}</h3>
          {badge && <span className="icon-info-card__badge">{badge}</span>}
        </div>
        <p className="icon-info-card__description">{description}</p>
      </div>
    </div>
  </button>
);
export default IconInfoCard;

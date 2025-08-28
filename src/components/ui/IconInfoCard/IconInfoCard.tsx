import { ReactNode } from "react";

interface IconInfoCard {
  title?: string;
  description?: string;
  children: ReactNode;
}

const IconInfoCard = ({
  title = "Info Title",
  description = "Info Description",
  children,
}: IconInfoCard) => (
  <div className="icon-info-card">
    <div className="icon-info-card__icon-container">{children}</div>
    <div className="icon-info-card__content">
      <h3 className="icon-info-card__content__title">{title}</h3>
      <p className="icon-info-card__content__description">{description}</p>
    </div>
  </div>
);

export default IconInfoCard;

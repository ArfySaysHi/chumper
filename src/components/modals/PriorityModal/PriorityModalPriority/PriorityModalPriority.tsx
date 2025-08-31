import "./PriorityModalPriority.scss";
import { PlusOne } from "@mui/icons-material";

interface PriorityModalPriorityProps {
  setCurrentStep: (step: string) => void;
}

const PriorityModalPriority = ({
  setCurrentStep,
}: PriorityModalPriorityProps) => {
  return (
    <div className="priority-modal-priority">
      <div className="priority-modal-priority__header">
        <div className="priority-modal-priority__header__back">Back</div>
        <h1 className="priority-modal-priority__header__title">
          Priority Generation
        </h1>
      </div>
      <div className="priority-modal-priority__content">
        <div className="priority-modal-priority__content__priorities">
          <div className="priority-select-card">
            <div className="priority-select-card__icon">
              <PlusOne />
            </div>
            <div className="priority-select-card__info">
              <h2 className="priority-select-card__info__title">Attributes</h2>
              <p className="priority-select-card__info__description">
                Physical and mental capabilities
              </p>
            </div>
            <div className="priority-select-card__options">
              {["A", "B", "C", "D", "E"].map((g) => (
                <button className="priority-select-card__options__grade">
                  {g}
                </button>
              ))}
            </div>
          </div>
        </div>
        <div className="priority-modal-priority__content__info">
          <div className="priority-details">
            <h2 className="priority-details__header">Selection Details</h2>
            <div className="priority-details__divider"></div>
            <div className="priority-details__content">
              <div className="priority-detail-card">
                <div className="priority-detail-card__header">
                  <div className="priority-detail-card__icon">
                    <PlusOne />
                  </div>
                  <div className="priority-detail-card__header__details">
                    <h3 className="priority-detail-card__header__details__title">
                      Attributes
                    </h3>
                    <div className="priority-detail-card__header__details__description">
                      Level 3
                    </div>
                  </div>
                  <div className="priority-detail-card__info">
                    16 Attribute Points
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default PriorityModalPriority;

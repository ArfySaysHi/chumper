import "./PriorityModalPriority.scss";
import { PlusOne } from "@mui/icons-material";
import PrioritySelectCard from "../../../ui/PrioritySelectCard/PrioritySelectCard.tsx";
import type { MethodSpec } from "../../../../types/creation.ts";
import useCommand from "../../../../hooks/useCommand.ts";
import { useEffect } from "react";

interface PriorityModalPriorityProps {
  setCurrentStep: (step: string) => void;
  methodSpec: MethodSpec;
}

const PriorityModalPriority = ({
  setCurrentStep,
  methodSpec,
}: PriorityModalPriorityProps) => {
  const { execute: listMetatypes } = useCommand("list_priority_bundles");

  useEffect(() => {
    listMetatypes({}).then(console.log);
  }, []);

  return (
    <div className="priority-modal-priority">
      <div className="priority-modal-priority__header">
        <div
          className="priority-modal-priority__header__back"
          onClick={() => setCurrentStep("method")}
        >
          Back
        </div>
        <h1 className="priority-modal-priority__header__title">
          {methodSpec.title}
        </h1>
      </div>
      <div className="priority-modal-priority__content">
        <div className="priority-modal-priority__content__priorities">
          <PrioritySelectCard />
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

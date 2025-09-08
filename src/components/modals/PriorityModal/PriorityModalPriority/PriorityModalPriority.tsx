import "./PriorityModalPriority.scss";
import { PlusOne } from "@mui/icons-material";
import PrioritySelectCard from "../../../ui/PrioritySelectCard/PrioritySelectCard.tsx";
import useCommand from "../../../../hooks/useCommand.ts";
import { useEffect, useState } from "react";
import type {
  PriorityBundleArray,
  PriorityBundle,
} from "../../../../schemas/priority_bundle.ts";
import { PriorityBundleArraySchema } from "../../../../schemas/priority_bundle.ts";

interface PriorityModalPriorityProps {
  setCurrentStep: (step: string) => void;
  title: string;
}

const PriorityModalPriority = ({
  setCurrentStep,
  title,
}: PriorityModalPriorityProps) => {
  const { execute: listPriorityBundles } = useCommand("list_priority_bundles");
  const [bundles, setBundles] = useState<PriorityBundleArray>();

  useEffect(() => {
    listPriorityBundles({}).then((res) => {
      const validatedRes = PriorityBundleArraySchema.parse(res);
      setBundles(validatedRes.sort());
    });
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
        <h1 className="priority-modal-priority__header__title">{title}</h1>
      </div>
      <div className="priority-modal-priority__content">
        <div className="priority-modal-priority__content__priorities">
          {bundles &&
            bundles.map((b: PriorityBundle) => (
              <PrioritySelectCard
                key={b.id || "Default ID"}
                title={b.name || "No Title"}
              />
            ))}
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

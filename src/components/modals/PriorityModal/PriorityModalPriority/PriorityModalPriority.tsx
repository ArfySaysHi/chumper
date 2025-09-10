import "./PriorityModalPriority.scss";
import PrioritySelectCard from "../../../ui/PrioritySelectCard/PrioritySelectCard.tsx";
import useCommand from "../../../../hooks/useCommand.ts";
import { useEffect, useState } from "react";
import type {
  PriorityBundleArray,
  PriorityBundle,
  PriorityBundleModifier,
} from "../../../../schemas/priority_bundle.ts";
import { PriorityBundleArraySchema } from "../../../../schemas/priority_bundle.ts";
import CharacterStatBlock from "../../../ui/CharacterStatBlock/CharacterStatBlock.tsx";
import Button from "../../../ui/Button/Button.tsx";

interface PriorityModalPriorityProps {
  setCurrentStep: (step: string) => void;
  title: string;
}

interface PriorityGrades {
  [k: string]: string;
}

const PriorityModalPriority = ({
  setCurrentStep,
  title,
}: PriorityModalPriorityProps) => {
  const { execute: listPriorityBundles } = useCommand("list_priority_bundles");
  const [bundles, setBundles] = useState<PriorityBundleArray>();
  const [grades, setGrades] = useState<PriorityGrades>({});

  useEffect(() => {
    listPriorityBundles({}).then((res) => {
      const validatedRes = PriorityBundleArraySchema.parse(res);
      setBundles(
        validatedRes.sort((a, b) => (a.menu_order || 0) - (b.menu_order || 0)),
      );
    });
  }, []);

  const getGradeBenefits = (name: string, grade: string) => {
    if (!bundles) return {};

    const bundle = bundles.find((g: PriorityBundle) => g.name === name);

    if (!bundle) return {};
    const modifiers = bundle.modifiers.filter(
      (m: PriorityBundleModifier) => m.grade === grade,
    );
    console.log(modifiers);
  };

  return (
    <div className="priority-modal-priority">
      <div className="priority-modal-priority__header">
        <h1 className="priority-modal-priority__header__title">{title}</h1>
      </div>
      <div className="priority-modal-priority__content">
        <div className="priority-modal-priority__content__priorities">
          {bundles &&
            bundles.map((b: PriorityBundle) => (
              <PrioritySelectCard
                key={b.id || ""}
                title={b.name || ""}
                selectedGrade={grades[b.name || ""] || ""}
                onGradeSelect={(opt) => {
                  setGrades((prev) => ({
                    ...prev,
                    [b.name || ""]: opt,
                  }));
                  getGradeBenefits(b.name || "", opt);
                }}
              />
            ))}
        </div>
        <div className="priority-modal-priority__content__info">
          <CharacterStatBlock />
        </div>
      </div>
      <div className="priority-modal-priority__controls">
        <Button>Submit</Button>
        <Button variant="secondary" onClick={() => setCurrentStep("method")}>
          Back
        </Button>
      </div>
    </div>
  );
};

export default PriorityModalPriority;

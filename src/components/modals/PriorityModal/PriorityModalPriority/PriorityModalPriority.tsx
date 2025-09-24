import "./PriorityModalPriority.scss";
import PrioritySelectCard from "../../../ui/PrioritySelectCard/PrioritySelectCard.tsx";
import useCommand from "../../../../hooks/useCommand.ts";
import { useEffect, useState } from "react";
import type {
  PriorityBundleArray,
  PriorityBundle,
  PriorityGrade,
} from "../../../../schemas/priority_bundle.ts";
import { PriorityBundleArraySchema } from "../../../../schemas/priority_bundle.ts";
import CharacterStatBlock from "../../../ui/CharacterStatBlock/CharacterStatBlock.tsx";
import Button from "../../../ui/Button/Button.tsx";

interface PriorityModalPriorityProps {
  setCurrentStep: (step: string) => void;
  title: string;
}

interface CharacterCreatePriorityOption {
  grade: PriorityGrade;
  bundle_id: number;
  chosen_bundle_id?: number;
}

interface PriorityGrades {
  [k: number]: CharacterCreatePriorityOption;
}

const PriorityModalPriority = ({
  setCurrentStep,
  title,
}: PriorityModalPriorityProps) => {
  const { execute: listPriorityBundles } = useCommand("list_priority_bundles");
  const { execute: createCharacter } = useCommand("create_character");
  const [bundles, setBundles] = useState<PriorityBundleArray>();
  const [grades, setGrades] = useState<PriorityGrades>({});

  useEffect(() => {
    listPriorityBundles({ params: {} }).then((res) => {
      const validatedRes = PriorityBundleArraySchema.parse(res);
      setBundles(validatedRes.sort((a, b) => (a.id || 0) - (b.id || 0)));
    });
  }, []);

  const getGradeBenefits = (id: number, _grade: PriorityGrade) => {
    if (!bundles) return {};

    const bundle = bundles.find((g: PriorityBundle) => g.id === id);

    if (!bundle) return {};
  };

  const submit = () => {
    createCharacter({
      params: {
        priority_system: "Core",
        priorities: grades,
        metatype_id: 0,
        skill_selections: [],
      },
    }).then(console.log);
  };

  return (
    <div className="priority-modal-priority">
      <div className="priority-modal-priority__header">
        <h1 className="priority-modal-priority__header__title">{title}</h1>
      </div>
      <div className="priority-modal-priority__content">
        <div className="priority-modal-priority__content__priorities">
          {/* {bundles &&
              bundles.map((b: PriorityBundle) => {
              const name = b.name ?? "Unknown Bundle";
              const key = String(b.id);

              return (
              <PrioritySelectCard
              key={key}
              title={name}
              selectedGrade={grades?.[b.id]?.grade}
              onGradeSelect={(opt) => {
              setGrades((prev) => ({
              ...prev,
              [key]: { grade: opt },
              }));
              console.log(getGradeBenefits(b.id, opt));
              }}
              />
              );
              })}
            */}{" "}
        </div>
        <div className="priority-modal-priority__content__info">
          <CharacterStatBlock />
        </div>
      </div>
      <div className="priority-modal-priority__controls">
        <Button onClick={submit}>Submit</Button>
        <Button variant="secondary" onClick={() => setCurrentStep("method")}>
          Back
        </Button>
      </div>
    </div>
  );
};

export default PriorityModalPriority;

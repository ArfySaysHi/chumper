import "./PriorityModalMethod.scss";
import IconInfoCard from "../../../ui/IconInfoCard/IconInfoCard.tsx";
import { Abc, PlusOne, Spa } from "@mui/icons-material";

const methods = [
  {
    id: "priority",
    title: "Priority System",
    description:
      "Classic approach - assign one priority level (A-E) to each category",
    icon: Abc,
  },
  {
    id: "sum-to-ten",
    title: "Sum-to-Ten",
    description:
      "Flexible point allocation - distribute 10 points across categories",
    icon: PlusOne,
  },
  {
    id: "karma-gen",
    title: "Karma Generation",
    description: "Point-buy system using karma for complete customization",
    icon: Spa,
  },
];

interface PriorityModalMethodProps {
  setCurrentStep: (step: string) => void;
}

const PriorityModalMethod = ({ setCurrentStep }: PriorityModalMethodProps) => {
  return (
    <div className="priority-modal-method">
      <div className="priority-modal-method__header">
        <h2 className="priority-modal-method__header__title">
          Character Generation
        </h2>
        <p className="priority-modal-method__header__subheader">
          Select your preferred character creation method
        </p>
      </div>
      <div className="priority-modal-method__options">
        {methods.map((m, i) => (
          <IconInfoCard
            key={m.id}
            title={m.title}
            onClick={() => setCurrentStep(m.id)}
            description={m.description}
            disabled={i > 0}
          >
            <m.icon />
          </IconInfoCard>
        ))}
      </div>
    </div>
  );
};

export default PriorityModalMethod;
